use gloo_net::http::Request;
use gloo_timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html, Callback};

#[derive(PartialEq)]
struct Cursor {
    visible: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { visible: false }
    }

    pub fn invert(&self) -> Self {
        Cursor {
            visible: !self.visible,
        }
    }
}

#[function_component]
fn Editor() -> Html {
    let content = use_state_eq(|| String::from("Hello, it's me"));
    let cursor = use_state_eq(|| Cursor::new());
    let _cursor_interval = {
        let cursor = cursor.clone();
        Interval::new(1_000, move || cursor.set(cursor.invert()))
    };

    let on_content_change = {
        let content = content.clone();
        Callback::from(move |content_change: InputEvent| {
            let char = content_change.as_string().unwrap();
            content.set(char)
        })
    };
    let on_content_key_up = {
        let content = content.clone();
        Callback::from(move |content_change: KeyboardEvent| {
            let char = content_change.code();
            content.set(char)
        })
    };

    html! {
        <div class="editor">
            <div id="editor-query">
                <span>
                    {(*content).clone()}
                </span>
            </div>
            <div class={classes!("mouse-cursor-text", "cursor", (!(cursor).visible).then(|| Some("hidden")))}></div>
            <textarea
                class="inputarea mouse-cursor-text"
                role="textbox"
                oninput={on_content_change.clone()}
                onkeydown={on_content_key_up.clone()}
            ></textarea>
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    let response = use_state_eq(|| String::from(""));

    let on_run_click = {
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
                <button id="run-query" class="run-button" type="button" onclick={on_run_click}>
                    <p>
                        { "Run" }
                    </p>
                </button>
            </div>
            <div class="workspace">
                <Editor />
                <div id="output">
                    {(*response).clone()}
                </div>
            </div>
        </>
    }
}
