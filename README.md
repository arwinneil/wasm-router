[![Netlify Status](https://api.netlify.com/api/v1/badges/4b512164-5c8f-4739-8dc6-6a4e4deb0395/deploy-status)](https://app.netlify.com/sites/wasm-router-poc/deploys)
# wasm-router-poc ⚒

Playground for a WebAssembly frontend router written in Rust, live [here](https://wasm-router-poc.netlify.app/).

## Current Status

- "history" style router
- Rust function path handlers

## Usage
```rust
pub fn main() {
    let mut r = router::Router::new();
    
    r.add("/", handle_root);
    r.add("/about", handle_about);

    r.load_page();
}

pub fn handle_root(s: &str) {
    console_log(&["Hit handler for ", s, " route"].concat());
}

pub fn handle_about(s: &str) {
    console_log(&["Hit handler for ", s, " route"].concat());
}
```

## Building
```
wasm-pack build -t web
```
