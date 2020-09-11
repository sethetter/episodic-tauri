use wasm_bindgen::prelude::*;
use yew::prelude::*;
use components::app::App as AppComponent;

mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<AppComponent>::new().mount_to_body();
}
