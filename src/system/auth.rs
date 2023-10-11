use bevy::prelude::*;
use serde::{Serialize, Deserialize};

pub struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Config>()
            .add_systems(Startup, load_config_file);
    }
}

#[derive(Debug, Resource, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub redirect_uri: String,
    pub client_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            redirect_uri: String::new(),
            client_secret: String::new(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_config_file(
    mut config: ResMut<Config>,
) {
    use crate::utils::yaml::load_yaml_from_file;
    let config_from_yaml = load_yaml_from_file::<Config>("config.yaml");
    config.client_id = config_from_yaml.client_id;
    config.redirect_uri = config_from_yaml.redirect_uri;
}

#[cfg(target_arch = "wasm32")]
pub fn load_config_file(
    mut config: ResMut<Config>,
) {
    // TODO: load from wasm config
    config.client_id = "687056820470-5ujnqbk4pblq6huvl7nb42ggmi24dt8h.apps.googleusercontent.com".to_string();
    config.redirect_uri = "https://oauth-6kvyjc5h2a-uc.a.run.app".to_string();
}

pub fn initiate_google_login(config: &Config, uuid: &str) {
    let client_id = &config.client_id;
    let redirect_uri = &config.redirect_uri;
    let uuid = uuid;
    let oauth_url = format!("https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&state={}"
        ,client_id
        ,redirect_uri,
        uuid);
    
    webbrowser::open(&oauth_url).unwrap();
}
