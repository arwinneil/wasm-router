mod demo_data;
mod router;
mod utils;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen]
pub fn console_log(s: &str) {
    log(s);
}

#[wasm_bindgen]
pub fn main() {
    let mut r = router::Router::new();

    r.add("/", update_page);
    r.add("/faq", update_page);
    r.add("/about", update_page);
    r.add("/lets/party", update_page);
    r.add("404", not_found);
    r.add_hook("on_loaded", loaded);
    r.init();
}

pub fn update_page(s: &str) {
    let data = demo_data::DemoData::new();

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("path")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .expect("#path should be an `HtmlElement`")
        .set_inner_html(s);

    let content: String;

    match s {
        "/" => content = data.home_content,
        "/about" => content = data.about_content,
        "/lets/party" => content = data.lets_party_content,
        "/faq" => content = data.faq_content,
        _ => content = s.to_owned(),
    }

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("content")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .expect("#path should be an `HtmlElement`")
        .set_inner_html(content.as_str());
}

pub fn not_found(s: &str) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("path")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .expect("#path should be an `HtmlElement`")
        .set_inner_html("being handled as a 404 Not Found page");

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("content")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .expect("#path should be an `HtmlElement`")
        .set_inner_html("");
}

pub fn loaded() {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("body")
        .unwrap()
        .dyn_ref::<Element>()
        .unwrap()
        .class_list()
        .remove_1("loading");
}
