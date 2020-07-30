[![Netlify Status](https://api.netlify.com/api/v1/badges/4b512164-5c8f-4739-8dc6-6a4e4deb0395/deploy-status)](https://app.netlify.com/sites/wasm-router-poc/deploys)
# wasm-router 🦀🚀

Playground for a WebAssembly frontend router written in Rust, live [here](https://wasm-router.netlify.app/).

## Current Status ⚒

- Hybrid router combinng `hash` & `history` style routing modes, delivering clean URLs without the need to reload page
- DOM updated using Rust function pointers & [web-sys](https://crates.io/crates/web-sys)
- 404 Page if route does not exist
- Supports hooks at different stages of the routing lifecycle (WIP)

## Example Usage
```rust
#[wasm_bindgen]pub fn main() {
    let mut r = router::Router::new();

    r.add("/", update_home);
    r.add("/about", update_about);
    r.add_hook("on_loaded", loaded);
    r.init();
}

pub fn update_home(s: &str) { /* DOM Manipulation */}
pub fn update_about(s: &str) { /* DOM Manipulation */}
pub fn loaded() { /* DOM Manipulation */}

```

## Building
```
wasm-pack build -t web
```
