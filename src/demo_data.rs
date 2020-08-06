pub struct DemoData {
    pub home_content: String,
    pub about_content: String,
    pub faq_content: String,
    pub lets_party_content: String,
}

impl DemoData {
    pub fn new() -> DemoData {
        DemoData{
                home_content : r##"
                <section>
        
                <aside>
                    <img alt="HTML only" src="./img/routing.svg" height="200px" >
                    <h3>Routing with clean URLs</h3>
                    <p>without page reload, and with full brower history navigation! Non existant routes can also be handled to display an error message. Try it out on this  <a href="#thisisatest"> non existant page ➡ </a> or any random URL.</p>
                </aside>
                <aside>
                    <img alt="Lifecycle Hooks" src="./img/hooks.svg" height="200px">
                <h3>Lifecycle Hooks  <sup>In Progress</sup></h3>
                    <p>Inject code your at specific stages of routing to a page. This page's uses an <code>on_loaded</code> hook to hide the loading animation on initial load.
                    </p>
                </aside>
                <aside>
                    <img alt="Example code" src="./img/example.svg" height="200px">
                    <h3>Feedback</h3>
                    <p>Still a work in progress, feel free to play and break this demo. I'm new to Rust the inner workings of the Browser API. Feedback welcome on <a href="https://github.com/arwinneil/wasm-router">GitHub↗</a>. Be sure to leave a ⭐ if you're visiting.
                    </p>

                </section>"##.to_string(),

                faq_content : 
                "<h2>Frequently Asked Questions</h2>
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
                    <summary>Why dont blind people skydive?</summary>
                    <p>Because it scares the crap out of their dogs.</p>
                </details>
                ".to_string(),

                about_content : r#"

                    <h1> Example Usage 👨‍💻</h1>
                    <p>Routing "/" and "/about" with a an <code>on_loaded</code> hook in Rust using wasm-router </p>
                <pre>
                <code>
pub fn main() {
    let mut r = router::Router::new();

    r.add("/", update_home);
    r.add("/about", update_about);
    r.add_hook("on_loaded", loaded);
    r.init();
}

pub fn update_home(s: &str) { /* DOM Manipulation */}
pub fn update_about(s: &str) { /* DOM Manipulation */}
pub fn loaded() { /* DOM Manipulation */}

                </code>
                </pre>
                "#.to_string(),
                
                lets_party_content : r#"
                <img style=" display: block;
                    margin-left: auto;
                    margin-right: auto;"
                    src="https://cultofthepartyparrot.com/parrots/hd/parrot.gif">
                </img>

                <header>
                <a href="https://cultofthepartyparrot.com/" target="_blank"><h1> Join the cult </h1></a>
                </header>"#.to_string()
            }
    }
}
