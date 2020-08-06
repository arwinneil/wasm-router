use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

pub struct Router {
    routes: HashMap<String, fn(s: &str)>,
    hooks: HashMap<String, fn()>, //To improve implementation
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
            hooks: HashMap::new(),
        }
    }

    pub fn init(&self) {
        //Route initial URL
        Self::route(self.routes.clone(), Self::get_fragment().as_str());

        let routes: HashMap<String, fn(s: &str)> = self.routes.clone();

        //Hash routing forward in history and URL rewrite
        let handle_hash = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
            let l: String = web_sys::window()
                .unwrap()
                .location()
                .hash()
                .unwrap()
                .chars()
                .skip(1)
                .collect();

            log(&["hash handle : ", l.as_str()].concat());

            let h = web_sys::window().unwrap().history().unwrap();
            h.replace_state_with_url(&JsValue::NULL, "", Some(l.as_str()));

            Self::route(routes.clone(), l.as_str())
        }) as Box<dyn Fn(_)>);

        web_sys::window()
            .unwrap()
            .set_onhashchange(Some(handle_hash.as_ref().unchecked_ref()));

        handle_hash.forget();

        let routes: HashMap<String, fn(s: &str)> = self.routes.clone();

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

            Self::route(routes.clone(), l.as_str())
        }) as Box<dyn Fn(_)>);

        web_sys::window()
            .unwrap()
            .set_onpopstate(Some(handle_pop.as_ref().unchecked_ref()));

        handle_pop.forget();

        Self::run_hook(self.hooks.clone(), "on_loaded");
    }

    pub fn add(&mut self, route: &str, handler: fn(s: &str)) {
        self.routes.insert(String::from(route), handler);
    }

    pub fn remove(&mut self, route: &str) {
        self.routes.remove(route);
    }

    pub fn add_hook(&mut self, hook: &str, handler: fn()) {
        self.hooks.insert(String::from(hook), handler);
    }

    fn get_fragment() -> String {
        return web_sys::window().unwrap().location().pathname().unwrap();
    }

    fn run_hook(hooks: HashMap<String, fn()>, hook: &str) {
        log(hook);

        match hooks.get(hook) {
            Some(handler) => handler(),
            None => (),
        }
    }

    fn route(routes: HashMap<String, fn(&str)>, destination: &str) {
        match routes.get(destination) {
            Some(route_handler) => route_handler(destination),
            None => match routes.get("404") {
                Some(route_handler) => route_handler(destination),
                None => {
                    log("Page not found!");
                }
            },
        }
    }
}
