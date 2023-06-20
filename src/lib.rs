extern crate web_sys;
use std::str;
use wasm_bindgen::prelude::*;

use ipconfig;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen]
pub fn console_log() {
    let message = "Hello from Rust!";
    log(message);
}
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub async fn fetch_data() -> Result<JsValue, JsValue> {
    // 异步操作，获取数据
    // 这里只是一个示例，你可以根据自己的需求进行修改
    let result = "Some data".to_string();

    // 创建一个 Promise
    let promise = Promise::new(&mut |resolve, _| {
        // 在异步操作完成后，调用 resolve 将结果传递给 Promise
        resolve
            .call1(&JsValue::NULL, &JsValue::from_str(&result))
            .unwrap();
    });

    // 将 Promise 转换为 Future
    let future = JsFuture::from(promise);
    let js_result = future.await.unwrap();
    Ok(js_result)
}

#[wasm_bindgen]
pub async fn fetch_data1() -> Promise {
    // 异步操作，获取数据
    // 这里只是一个示例，你可以根据自己的需求进行修改
    let result = "Some data".to_string();

    // 创建一个 Promise
    let promise = Promise::new(&mut |resolve, _| {
        // 在异步操作完成后，调用 resolve 将结果传递给 Promise
        resolve
            .call1(&JsValue::NULL, &JsValue::from_str(&result))
            .unwrap();
    });

    // 将 Promise 转换为 Future
    let future = JsFuture::from(promise);

    // 等待 Future 完成并返回结果
    future.await.unwrap().into()
}

#[wasm_bindgen]
pub fn internal_ip() {
    for adapter in ipconfig::get_adapters()? {
        println!("Ip addresses: {:#?}", adapter.ip_addresses());
        println!("Dns servers: {:#?}", adapter.dns_servers());
    }
}
