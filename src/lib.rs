mod utils;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
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
    let mut r = Router::new();

    r.add("Hello");
    r.remove("Hello");
    r.add("Hello from WASM");
    r.log(0);
    r.get_fragment()
}

struct Router {
    routes: Vec<String>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn add(&mut self, route: &str) {
        self.routes.push(String::from(route));
    }

    pub fn remove(&mut self, route: &str) {
        self.routes.retain(|r| r != route)
    }

    pub fn log(&self, num: usize) {
        console_log(self.routes[num].as_str());
    }

    pub fn get_fragment(self) {
        let l = web_sys::window().unwrap().location();
        log(l.pathname().unwrap().as_str());

        // Temporary for Demo
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("path")
            .unwrap()
            .dyn_ref::<HtmlElement>()
            .expect("#path should be an `HtmlElement`")
            .set_inner_html(l.pathname().unwrap().as_str());
    }
}
