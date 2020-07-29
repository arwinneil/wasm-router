use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

pub struct Router {
    routes: Vec<(String, fn(s: &str))>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn init(&self) {
        let routes: Vec<(String, fn(s: &str))> = self.routes.clone();

        //Hash routing forward in history and URL rewrite
        let handle_hash = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
            let l = web_sys::window().unwrap().location();
            let l: String = l.hash().unwrap().chars().skip(1).collect();
            log(&["hash handle : ", l.as_str()].concat());

            let h = web_sys::window().unwrap().history().unwrap();
            h.replace_state_with_url(&JsValue::NULL, "", Some(l.as_str()));

            if routes.iter().any(|r| r.0 == l) {
                let index = routes.iter().position(|r| r.0 == l).unwrap();
                routes[index].1(routes[index].0.as_str());
            } else {
                log("hit nothing");
            }
        }) as Box<dyn Fn(_)>);

        web_sys::window()
            .unwrap()
            .set_onhashchange(Some(handle_hash.as_ref().unchecked_ref()));

        handle_hash.forget();

        let routes: Vec<(String, fn(s: &str))> = self.routes.clone();

        //Routing for navigating in history and escaping hash routes
        let handle_pop = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
            let l = Self::get_fragment();

            if web_sys::window()
                .unwrap()
                .location()
                .hash()
                .unwrap()
                .chars()
                .count()
                > 0
            {
                log("hash detected");
                return ();
            }

            log(&["pop handle : ", l.as_str()].concat());

            if routes.iter().any(|r| r.0 == l) {
                let index = routes.iter().position(|r| r.0 == l).unwrap();
                routes[index].1(routes[index].0.as_str());
            } else {
                log("hit nothing");
            }
        }) as Box<dyn Fn(_)>);

        web_sys::window()
            .unwrap()
            .set_onpopstate(Some(handle_pop.as_ref().unchecked_ref()));

        handle_pop.forget();
    }

    pub fn add(&mut self, route: &str, handler: fn(s: &str)) {
        self.routes.push((String::from(route), handler));
    }

    pub fn remove(&mut self, route: &str) {
        self.routes.retain(|r| r.0 != route)
    }

    fn get_fragment() -> String {
        return web_sys::window().unwrap().location().pathname().unwrap();
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
    }
}
