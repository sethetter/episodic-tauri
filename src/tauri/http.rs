use wasm_bindgen::prelude::*;
use js_sys::{JsString, Object, Reflect};
use web_sys::console;
use url::Url;

type Result<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = http, js_name = request)]
    async fn tauri_request(opts: &JsValue) -> Result<JsValue>;
}

enum ResponseType { Text, Json, Binary }

pub async fn request_text(url: Url, body: Option<&str>) -> Result<String> {
    request(url, body, ResponseType::Text).await
}

pub async fn request_json(url: Url, body: Option<&str>) -> Result<String> {
    request(url, body, ResponseType::Json).await
}

pub async fn request_binary(url: Url, body: Option<&str>) -> Result<String> {
    request(url, body, ResponseType::Binary).await
}

async fn request(url: Url, body: Option<&str>, resp_type: ResponseType) -> Result<String> {
    let req_opts = Object::new();

    // Set response type.
    let resp_type_num = match resp_type {
        ResponseType::Text => 2.0,
        ResponseType::Json => 1.0,
        ResponseType::Binary => 3.0,
    };
    Reflect::set(&req_opts, &"responseType".into(), &JsValue::from_f64(resp_type_num))?;

    // Set method.
    // TODO: method should be providable
    Reflect::set(&req_opts, &"method".into(), &JsValue::from_str("GET"))?;

    // Set URL.
    Reflect::set(&req_opts, &"url".into(), &JsValue::from_str(url.as_str()))?;

    // Set body.
    if body.is_some() {
        // TODO ...
    }

    let response = tauri_request(&req_opts.into()).await?;
    Ok(response.as_string().unwrap())
}
