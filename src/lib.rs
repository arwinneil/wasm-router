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

    r.add("/", update_page);
    r.add("/faq", update_page);
    r.add("/about", update_page);
    r.add("/lets/party", update_page);
    r.add("404", not_found);
    r.init();
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

    let content: String;

    match s {
        "/about" => content = "Welcome".to_string(),
        "/lets/party" => {
            content = r#"<iframe src="https://giphy.com/embed/FYx64DDl2ElWw" width="100%" frameBorder="0" class="giphy-embed" allowFullScreen></iframe><p><a href="https://giphy.com/gifs/trippy-party-weed-FYx64DDl2ElWw">via GIPHY</a></p>"#.to_string()
        },
        "/faq" =>{
            content="<h2>Frequently Asked Questions</h2>
            <details>
                <summary> You know why you never see elephants hiding up in trees?</summary>
                <p>Because they’re really good at it.</p>
            </details>
            <details>
                <summary>Why aren’t koalas actual bears?</summary>
                <p>The don’t meet the koalafications.</p>
            </details>
            <details>
                <summary>What do you call bears with no ears?</summary>
                <p>B</p>
            </details>
            <details>
                <p><summary>Because it scares the crap out of their dogs.</summary></p>
            </details>
            
            ".to_string()
        }

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
