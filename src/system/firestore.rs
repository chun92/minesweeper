use bevy::prelude::*;
use firestore::*;
use std::sync::{Arc, Mutex};
use bevy_tokio_tasks::TokioTasksRuntime;
use bevy_tokio_tasks::TaskContext;
use serde::{Deserialize, Serialize};

use crate::system::uuid::UuidResource;

pub const PROJECT_ID: &str = "minesweeper-86284";
const TARGET_ID_BY_DOC_IDS: FirestoreListenerTarget = FirestoreListenerTarget::new(17_u32);
const LOGIN_COLLECTION: &str = "login";
pub struct FirestorePlugin;
impl Plugin for FirestorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
            .init_resource::<LoginDone>()
            .init_resource::<FirestoreResource>()
            .add_systems(Startup, init_firestore);
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

#[derive(Debug, Clone, Deserialize, Serialize)]
struct LoginStructure {
    #[serde(alias = "_firestore_id")]
    doc_id: Option<String>,
    user_id: String,
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
    
    runtime.spawn_background_task(|mut ctx| async move {
        init(db.clone()).await.expect("Init and Listen failed");
        listen_login(db.clone(), uuid.as_str(), &mut ctx, id.clone()).await.expect("Init and Listen failed");
        let mut login_done = login_done.lock().unwrap();
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
                        let obj: LoginStructure =
                            FirestoreDb::deserialize_doc_to::<LoginStructure>(doc)
                                .expect("Deserialized object");
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