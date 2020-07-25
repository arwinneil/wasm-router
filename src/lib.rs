mod router;
mod utils;
use wasm_bindgen::prelude::*;

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
pub fn test() {
    let mut r = router::router::Router::new();

    r.add("/faq", handle_faq);
    r.add("/about", handle_about);
    r.add("/lets/party", handle_lets_party);
    r.add("/", handle_root);

    r.load_page();
}

pub fn handle_root(s: &str) {
    console_log(&["Hit handler for ", s, " route"].concat());
}

pub fn handle_about(s: &str) {
    console_log(&["Hit handler for ", s, " route"].concat());
}

pub fn handle_faq(s: &str) {
    console_log(&["Hit handler for ", s, " route"].concat());
}

pub fn handle_lets_party(s: &str) {
    console_log(&["Hit handler for ", s, " route"].concat());
}
