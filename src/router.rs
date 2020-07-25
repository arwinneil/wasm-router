use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub struct Router {
    routes: Vec<(String, fn(s: &str))>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn add(&mut self, route: &str, handler: fn(s: &str)) {
        self.routes.push((String::from(route), handler));
    }

    pub fn remove(&mut self, route: &str) {
        self.routes.retain(|r| r.0 != route)
    }

    fn get_fragment() -> String {
        let l = web_sys::window().unwrap().location();
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

        return l.pathname().unwrap();
    }

    pub fn load_page(&self) {
        let current_path = Self::get_fragment();

        if self.routes.iter().any(|r| r.0 == current_path) {
            let index = self
                .routes
                .iter()
                .position(|r| r.0 == current_path)
                .unwrap();
            self.routes[index].1(self.routes[index].0.as_str());
        }
    }}
