use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let response = use_state_eq(|| String::from(""));
    {
        let response = response.clone();
        use_effect(|| {
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
            ()
        });
    }

    html! {
        <>
            <div>
                <p>
                    {(*response).clone()}
                </p>
            </div>
            <div id="text-input" contenteditable="true"></div>
            <div>
                <button id="run-query" class="run-button" type="button">
                    <p>
                        { "Run" }
                    </p>
                </button>
            </div>
        </>
    }
}
