use wasm_bindgen::prelude::*;
use yew::prelude::*;
use components::app::App as AppComponent;
use web_sys::console;
use url::Url;

mod components;
mod data;
mod tauri;

#[wasm_bindgen]
pub async fn run_app() {
    let url = Url::parse("https://sethetter.com").unwrap();
    let resp = tauri::http::request_text(url, None).await;
    console::log_1(&resp.unwrap().into());
    App::<AppComponent>::new().mount_to_body();
}
