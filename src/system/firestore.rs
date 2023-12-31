use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, DateTime, Utc};

use crate::system::uuid::UuidResource;
use crate::system::difficulty;
use crate::system::state::{GameState, DataReadingState, LoginPopupState, LoginState};
use crate::system::egui::IsLoginOpen;

pub const PROJECT_ID: &str = "minesweeper-86284";
pub struct FirestorePlugin;

impl Plugin for FirestorePlugin {
    fn build(&self, app: &mut App) {
        self.build_default(app);
        app
            .init_resource::<LoginDone>()
            .init_resource::<RankingDataResource>()
            .init_resource::<RankingDataTempResource>()
            .add_state::<DataReadingState>()
            .add_state::<LoginPopupState>()
            .add_state::<LoginState>()
            .add_systems(Startup, platform::init_firestore)
            .add_systems(Update, login_check)
            .add_systems(OnEnter(GameState::Win), platform::add_ranking)
            .add_systems(OnEnter(DataReadingState::Ready), platform::read_ranking)
            .add_systems(OnEnter(LoginState::Done), platform::login_done);
    }
}

#[derive(Resource)]
pub struct LoginDone{
    pub done: Arc<Mutex<bool>>,
    pub id: Arc<Mutex<Option<String>>>,
}

impl Default for LoginDone {
    fn default() -> Self {
        Self {
            done: Arc::new(Mutex::new(false)),
            id: Arc::new(Mutex::new(None)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingData {
    pub id: String,
    pub time: f32,
    pub difficulty: String,
    pub created_at: u64,
}

impl RankingData {
    pub fn timestamp_to_date(&self) -> String {
        let ts = self.created_at;
        let naive_datetime = NaiveDateTime::from_timestamp_opt(ts as i64, 0).unwrap();
        let utc_datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
        utc_datetime.format("%Y-%m-%d").to_string()
    }
}

impl PartialOrd for RankingData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RankingData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.partial_cmp(&other.time).unwrap_or(Ordering::Equal)
            .then_with(|| self.created_at.cmp(&other.created_at))
    }
}

impl PartialEq for RankingData {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.created_at == other.created_at
    }
}

impl Eq for RankingData {}

#[derive(Resource)]
pub struct RankingDataTempResource {
    pub time: f32,
    pub difficulty: String,
    pub saved: bool,
}

impl Default for RankingDataTempResource {
    fn default() -> Self 
    {
        Self {
            time: 0.0,
            difficulty: difficulty::Difficulty::Hard.to_string(),
            saved: false,
        }
    }
}

#[derive(Debug, Resource)]
pub struct RankingDataResource {
    pub data: Arc<Mutex<Vec<RankingData>>>,
    pub is_done: Arc<Mutex<bool>>,
}

impl Default for RankingDataResource {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(Vec::new())),
            is_done: Arc::new(Mutex::new(false)),
        }
    }
}

impl RankingDataResource {
    pub fn get_sorted_by_difficulty(&self, difficulty: &str, is_show_my_ranking: bool, id: Option<String>) -> (Vec<RankingData>, usize) {
        let lock = self.data.lock().unwrap();
        let mut sorted_data: Vec<RankingData> = lock.iter().filter(|&item| item.difficulty == difficulty).cloned().collect();
        
        sorted_data.sort();
        if is_show_my_ranking && id.is_some() {
            let id = id.clone();
            let id = id.unwrap();
            if let Some(index) = sorted_data.iter().position(|item| item.id == id) {
                let start = if index < 9 { 0 } else { index - 9 };
                let end = usize::min(start + 20, sorted_data.len());

                let truncated_data = sorted_data[start..end].to_vec();

                return (truncated_data, start + 1);
            } else {
                sorted_data.truncate(100);
                (sorted_data, 1)
            }
        } else {
            sorted_data.truncate(100);
            (sorted_data, 1)
        }
    }
}

pub fn login_check(
    login_done: Res<LoginDone>,
    current_state: Res<State<LoginState>>,
    mut next_state: ResMut<NextState<LoginState>>
) {
    let login_done = login_done.done.clone();
    let login_done = login_done.lock().unwrap();
    if *login_done && *current_state == LoginState::Not {
        next_state.set(LoginState::Done);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod platform {
    use super::*;
    use firestore::*;
    use bevy_tokio_tasks::TokioTasksRuntime;
    use bevy_tokio_tasks::TaskContext;
    use chrono::Utc;

    const TARGET_ID_BY_DOC_IDS: FirestoreListenerTarget = FirestoreListenerTarget::new(17_u32);
    const LOGIN_COLLECTION: &str = "login";
    const RANKING_COLLECTION: &str = "ranking";
    
    impl FirestorePlugin {
        pub fn build_default(&self, app: &mut App) {
            app
                .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
                .init_resource::<FirestoreResource>();
        }
    }

    #[derive(Resource)]
    pub struct FirestoreResource {
        pub db: Arc<Mutex<Option<FirestoreDb>>>,
    }

    impl Default for FirestoreResource {
        fn default() -> Self {
            Self {
                db: Arc::new(Mutex::new(None)),
            }
        }
    }
    
    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct LoginStructure {
        #[serde(alias = "_firestore_id")]
        doc_id: Option<String>,
        user_id: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct RankingStructure {
        pub id: String,
        pub time: f32,
        pub difficulty: String,
        pub created_at: firestore::FirestoreTimestamp,
    }

    pub fn init_firestore(
        runtime: ResMut<TokioTasksRuntime>,
        firestore: Res<FirestoreResource>,
        login_done: Res<LoginDone>,
        uuid: Res<UuidResource>,
    ) {
        let db = firestore.db.clone();
        let id = login_done.id.clone();
        let login_done = login_done.done.clone();
        let uuid = uuid.uuid.to_string();
        
        runtime.spawn_background_task(move |mut ctx| async move {
            init(db.clone()).await.expect("Init and Listen failed");
            listen_login(db.clone(), uuid.as_str(), &mut ctx, id.clone()).await.expect("Init and Listen failed");
            let mut login_done: std::sync::MutexGuard<'_, bool> = login_done.lock().unwrap();
            *login_done = true;

            info!("login done: {}", *id.clone().lock().unwrap().as_ref().unwrap());
        });
    }

    pub async fn init(db: Arc<Mutex<Option<FirestoreDb>>>) -> Result<(), Box<dyn std::error::Error>> {
        let firebase_db = FirestoreDb::with_options_service_account_key_file(
            FirestoreDbOptions::new(PROJECT_ID.to_string()),
            "./key/firebase_key.json".into(),
        )
        .await?;

        let mut locked_db = db.lock().unwrap();
        *locked_db = Some(firebase_db);
        Ok(())
    }

    pub async fn listen_login(db: Arc<Mutex<Option<FirestoreDb>>>, uuid: &str, ctx: &mut TaskContext, id: Arc<Mutex<Option<String>>>) -> Result<(), Box<dyn std::error::Error>> {
        info!("listen start: {}", uuid);
        let firestore_db = {
            let locked_db = db.lock().unwrap();
            locked_db.as_ref().ok_or_else(|| Box::<dyn std::error::Error>::from("FirestoreDb is None"))?.clone()
        };
        
        let mut listener = firestore_db.create_listener(
            FirestoreTempFilesListenStateStorage::new() // or FirestoreMemListenStateStorage or your own implementation 
        ).await?;
        
        firestore_db.fluent()
        .select()
        .by_id_in(LOGIN_COLLECTION)
        .batch_listen([uuid])
        .add_target(TARGET_ID_BY_DOC_IDS, &mut listener)?;

        let wait_done = Arc::new(Mutex::new(false));
        let wait_done_clone = wait_done.clone();
        let id_clone = id.clone();

        listener
            .start(move |event|  {
            let wait_done = wait_done_clone.clone();
            let id = id_clone.clone();
            async move {
                match event {
                    FirestoreListenEvent::DocumentChange(ref doc_change) => {
                        info!("Doc changed: {:?}", doc_change);
                        if let Some(doc) = &doc_change.document {
                            let obj = FirestoreDb::deserialize_doc_to::<LoginStructure>(doc);
                            if obj.is_err() {
                                return Ok(());
                            }
                            let obj = obj.unwrap();
                            info!("As object: {:?}", obj);
                            *wait_done.lock().unwrap() = true;
                            *id.lock().unwrap() = Some(obj.user_id.clone());
                        }
                    }
                    _ => { }
                }
        
                Ok(())
            }
        })
        .await?;
        
        while *wait_done.lock().unwrap() == false {
            ctx.sleep_updates(10).await;
        }

        listener.shutdown().await?;
        info!("listen done");
        
        Ok(())
    }

    pub async fn add_ranking_to_db(
        db: Arc<Mutex<Option<FirestoreDb>>>,
        id: Arc<Mutex<Option<String>>>,
        time: f32,
        difficulty: String,
    ) { 
        let firestore_db = {
            let locked_db = db.lock().unwrap();
            locked_db.as_ref().ok_or_else(|| Box::<dyn std::error::Error>::from("FirestoreDb is None")).expect("Firestore Db Initialize Exception").clone()
        };

        let ranking_structure = RankingStructure {
            id: id.lock().unwrap().clone().unwrap(),
            time: time,
            difficulty: difficulty,
            created_at: firestore::FirestoreTimestamp(Utc::now()),
        };

        let _object_returned: RankingStructure = firestore_db.fluent()
            .insert()
            .into(RANKING_COLLECTION)
            .generate_document_id()
            .object(&ranking_structure)
            .execute()
            .await.expect("Insert failed");
    }

    pub fn add_ranking(
        runtime: ResMut<TokioTasksRuntime>,
        firestore: Res<FirestoreResource>,
        login_done: Res<LoginDone>,
        difficulty: Res<difficulty::Difficulty>,
        timer: Res<crate::system::timer::platform::Timer>,
        mut ranking_data_temp: ResMut<RankingDataTempResource>,
        mut is_login_open: ResMut<IsLoginOpen>,
    ) {
        let db = firestore.db.clone();
        let id = login_done.id.clone();
        let login_done = login_done.done.clone();
        let time = timer.get_milli_sec() as f32 / 1000.0;
        let difficulty = difficulty.clone();
        
        if login_done.lock().unwrap().clone() {
            runtime.spawn_background_task(move |_ctx| async move {     
                add_ranking_to_db(db, id, time, difficulty.to_string()).await;
            });
        } else {
            *ranking_data_temp = RankingDataTempResource {
                time: time,
                difficulty: difficulty.to_string(),
                saved: true,
            };
            *is_login_open = IsLoginOpen(true);
        }
    }

    pub fn login_done(
        runtime: ResMut<TokioTasksRuntime>,
        firestore: Res<FirestoreResource>,
        login_done: Res<LoginDone>,
        ranking_data_temp: Res<RankingDataTempResource>
    ) {
        let has_temp = ranking_data_temp.saved;
        if has_temp {
            info!("login done with temp data");
            let db = firestore.db.clone();
            let id = login_done.id.clone();
            let time = ranking_data_temp.time;
            let difficulty = ranking_data_temp.difficulty.clone();
            runtime.spawn_background_task(move |_ctx| async move {     
                add_ranking_to_db(db, id, time, difficulty).await;
            });
        }
    }
    
    pub async fn read_ranking_from_db(db: Arc<Mutex<Option<FirestoreDb>>>) -> Result<Vec<RankingData>, Box<dyn std::error::Error>> {
        info!("read ranking start");
        let firestore_db = {
            let locked_db = db.lock().unwrap();
            locked_db.as_ref().ok_or_else(|| Box::<dyn std::error::Error>::from("FirestoreDb is None"))?.clone()
        };

        let results = firestore_db.fluent()
            .select()
            .from(RANKING_COLLECTION)
            .query()
            .await?;

        let results = results.iter().map(|doc| {
            let obj = FirestoreDb::deserialize_doc_to::<RankingStructure>(doc);
            if obj.is_err() {
                return RankingData {
                    id: String::new(),
                    time: 0.0,
                    difficulty: String::new(),
                    created_at: 0,
                };
            }

            let obj = obj.unwrap();
            RankingData {
                id: obj.id,
                time: obj.time,
                difficulty: obj.difficulty,
                created_at: obj.created_at.0.timestamp() as u64,
            }
        }).collect::<Vec<RankingData>>();

        let results = results.iter().filter_map(|item| {
            if item.created_at == 0 {
                None
            } else {
                Some(item.clone())
            }
        }).collect::<Vec<RankingData>>();

        info!("read ranking done");
        
        Ok(results)
    }

    pub fn read_ranking(
        runtime: ResMut<TokioTasksRuntime>,
        firestore: Res<FirestoreResource>,
        data_resource: ResMut<RankingDataResource>,
        mut next_state: ResMut<NextState<DataReadingState>>,
    ) {
        let db = firestore.db.clone();
        let data_done = data_resource.is_done.clone();
        let data_resource = data_resource.data.clone();
        next_state.set(DataReadingState::Done);

        runtime.spawn_background_task(move |_ctx| async move {
            let result = read_ranking_from_db(db.clone()).await.expect("read_ranking_from_db async failed");
            *data_resource.lock().unwrap() = result;
            *data_done.lock().unwrap() = true;
        });
    }
}


#[cfg(target_arch = "wasm32")]
pub mod platform {
    use super::*;
    use bevy_wasm_tasks::WASMTasksPlugin;
    use bevy_wasm_tasks::WASMTasksRuntime;
    use wasm_bindgen::prelude::*;
    use js_sys::JsString;
    use wasm_bindgen_futures::JsFuture;
    use serde_wasm_bindgen::from_value;
    
    impl FirestorePlugin {
        pub fn build_default(&self, app: &mut App) {
            app
                .add_plugins(WASMTasksPlugin);
        }
    }

    #[wasm_bindgen]
    extern "C" {
        fn listen_login_js(uuid: JsString) -> js_sys::Promise;
    }

    pub async fn listen_login(uuid: String, id: Arc<Mutex<Option<String>>>) -> Result<JsValue, JsValue> {
        let uuid_js = JsString::from(uuid);
        let promise = listen_login_js(uuid_js);
        let result = JsFuture::from(promise).await?;
        info!("listen done: {:?}", result);
        *id.lock().unwrap() = result.as_string();
        Ok(result)
    }

    pub fn init_firestore(
        runtime: ResMut<WASMTasksRuntime>,
        login_done: Res<LoginDone>,
        uuid: Res<UuidResource>,
    ) {
        let id = login_done.id.clone();
        let login_done = login_done.done.clone();
        let uuid = uuid.uuid.to_string();
        
        runtime.spawn_background_task(move |_ctx| async move {      
            listen_login(uuid, id.clone()).await.expect("call js async failed");
            let mut login_done = login_done.lock().unwrap();
            *login_done = true;

            info!("login done: {}", *id.clone().lock().unwrap().as_ref().unwrap());
        });
    }
    
    #[wasm_bindgen]
    extern "C" {
        fn add_ranking_js(id: JsString, time: f32, difficulty: JsString) -> js_sys::Promise;
    }

    pub async fn add_ranking_to_db(id: String, time: f32, difficulty: String) -> Result<JsValue, JsValue> {
        let id = JsString::from(id);
        let difficulty = JsString::from(difficulty);
        let promise = add_ranking_js(id, time, difficulty);
        let result = JsFuture::from(promise).await?;
        Ok(result)
    }

    pub fn add_ranking(
        runtime: ResMut<WASMTasksRuntime>,
        login_done: Res<LoginDone>,
        difficulty: Res<difficulty::Difficulty>,
        timer: Res<crate::system::timer::platform::Timer>,
        mut ranking_data_temp: ResMut<RankingDataTempResource>,
        mut is_login_open: ResMut<IsLoginOpen>,
    ) {
        let id = login_done.id.clone();
        let login_done = login_done.done.clone();
        let time = timer.get_milli_sec() as f32 / 1000.0;
        let difficulty = difficulty.clone();
        
        if login_done.lock().unwrap().clone() {
            runtime.spawn_background_task(move |_ctx| async move {        
                add_ranking_to_db(id.lock().unwrap().clone().unwrap(), time, difficulty.to_string()).await.expect("Insert failed");
            });
        } else {
            *ranking_data_temp = RankingDataTempResource {
                time: time,
                difficulty: difficulty.to_string(),
                saved: true,
            };
            *is_login_open = IsLoginOpen(true);
        }
    }
    
    pub fn login_done(
        runtime: ResMut<WASMTasksRuntime>,
        login_done: Res<LoginDone>,
        ranking_data_temp: Res<RankingDataTempResource>
    ) {
        let has_temp = ranking_data_temp.saved;
        if has_temp {
            info!("login done with temp data");
            let id = login_done.id.clone();
            let time = ranking_data_temp.time;
            let difficulty = ranking_data_temp.difficulty.clone();
            
            runtime.spawn_background_task(move |_ctx| async move {        
                add_ranking_to_db(id.lock().unwrap().clone().unwrap(), time, difficulty).await.expect("Insert failed");
            });
        }
    }

    #[wasm_bindgen]
    extern "C" {
        fn read_ranking_js() -> js_sys::Promise;
    }

    pub async fn read_ranking_from_db() -> Result<Vec<RankingData>, JsValue> {
        let promise = read_ranking_js();
        let result_jsvalue = JsFuture::from(promise).await?;
        
        let result: Vec<RankingData> = from_value(result_jsvalue).map_err(|e| {
            JsValue::from_str(&format!("Failed to deserialize: {:?}", e))
        })?;
        
        info!("read ranking done");
        Ok(result)
    }

    pub fn read_ranking(
        runtime: ResMut<WASMTasksRuntime>,
        data_resource: ResMut<RankingDataResource>,
        mut next_state: ResMut<NextState<DataReadingState>>,
    ) {
        let data_done = data_resource.is_done.clone();
        let data_resource = data_resource.data.clone();
        next_state.set(DataReadingState::Done);

        runtime.spawn_background_task(move |_ctx| async move {
            let result = read_ranking_from_db().await.expect("call js async failed");
            *data_resource.lock().unwrap() = result;
            *data_done.lock().unwrap() = true;
        });
    }
}