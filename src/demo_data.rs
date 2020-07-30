pub struct DemoData {
    pub home_content: String,
  //  about_content: String,
    pub faq_content: String,
    //lets_party_content: String,
}

impl DemoData {
    pub fn new() -> DemoData {
        DemoData{
                home_content : r##"
                <section>
        
                <aside>
                    <img alt="HTML only" src="./img/routing.svg" height="200px" >
                    <h3>Routing with clean URLs</h3>
                    <p>without page reload, and with full brower history navigation! Non existant routes can also be handled to display an error message. Try it out : </p>
                    <p><a href="#thisisatest"><em>Non-existant Page↗</em></a></p>
                </aside>
                <aside>
                    <img alt="Lifecycle Hooks" src="./img/hooks.svg" height="200px">
                <h3>Lifecycle Hooks  <sup>In Progress</sup></h3>
                    <p>Inject code your at specific stages of routing to a page. This page's uses an <code>on_loaded</code> hook to hide the loading animation when the page on initial load.
                    </p>
                </aside>
                <aside>

                    <img alt="Example code" src="./img/example.svg" height="200px">
                    <h3>Feedback</h3>
                    <p>Still a work in progress, feel free to play and break this demo. I'm new to Rust the inner workings of the Browser APIs. Feedback is welcome on the GitHub repository. Be sure to leave a ⭐ if you're visiting.
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
                    <p><summary>Because it scares the crap out of their dogs.</summary></p>
                </details>
                ".to_string()
                            
            }

    }
}
