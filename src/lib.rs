use wasm_bindgen::prelude::*;
use yew::prelude::*;
use components::app::App as AppComponent;
use web_sys::console;
use url::Url;

mod components;
mod data;
mod tauri;
mod tmdb;

#[wasm_bindgen]
pub async fn run_app() {
    let tmdb_client = tmdb::TmdbClient::new("874c5d5a093ae7e43035cd9a4bd4939c");
    let resp = tmdb_client.search_tv("Righteous").await.unwrap();
    let resp_json = serde_json::to_string(&resp).unwrap();
    console::log_1(&resp_json.into());
    App::<AppComponent>::new().mount_to_body();
}
