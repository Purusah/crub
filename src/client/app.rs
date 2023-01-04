use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html, Callback};

#[function_component]
pub fn App() -> Html {
    let response = use_state_eq(|| String::from(""));

    let onclick = {
        let response = response.clone();
        Callback::from(move |_: MouseEvent| {
            let response = response.clone();
            spawn_local(async move {
                let result = Request::get("https://httpbin.org/ip")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                response.set(result);
            });
        })
    };

    html! {
        <>
            <div>
                <button id="run-query" class="run-button" type="button" onclick={onclick}>
                    <p>
                        { "Run" }
                    </p>
                </button>
            </div>
            <div class="editor">
                <div id="editor-query" contenteditable="true"></div>
                <div id="editor-output">
                    {(*response).clone()}
                </div>
            </div>
        </>
    }
}
