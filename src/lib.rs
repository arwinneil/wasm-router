mod router;
mod utils;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

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

    r.add("/faq", update_page);
    r.add("/about", update_page);
    r.add("/lets/party", update_page);
    r.add("/", update_page);
    r.init();
    r.load_page();
}

pub fn update_page(s: &str) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("path")
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .expect("#path should be an `HtmlElement`")
        .set_inner_html(s);
}
