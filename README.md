[![Netlify Status](https://api.netlify.com/api/v1/badges/4b512164-5c8f-4739-8dc6-6a4e4deb0395/deploy-status)](https://app.netlify.com/sites/wasm-router-poc/deploys)
# wasm-router 🦀🚀

Playground for a WebAssembly frontend router written in Rust, live [here](https://wasm-router.netlify.app/).

## Current Status ⚒

- Hybrid router combinng `hash` & `history` style routing modes, delivering clean URLs without the need to reload page
- Rust function path handlers

## Example Usage
```rust
#[wasm_bindgen]
pub fn main() {
    let mut r = router::Router::new();

    r.add("/faq", update_page);
    r.add("/about", update_page);
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

```

## Building
```
wasm-pack build -t web
```
