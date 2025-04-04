use dioxus::prelude::*;
use serde_json::json;

#[component]
pub fn PostForm() -> Element {
    let title = use_signal( || String::new());
    let body = use_signal( || String::new());

    rsx! {
        form {
            onsubmit: move |_| async {
                let payload = json!(
                    { "title" : title.current().as_str(), "body" : body.current().as_str() }
                );
                let _ = reqwest::Client::new()
                    .post("http://localhost:8080/api/posts")
                    .json(&payload)
                    .send()
                    .await;
            },
            input {
                value: "{title}",
                oninput: move |e| title.set(e.value().clone()),
            }
            textarea {
                value: "{body}",
                oninput: move |e| body.set(e.value().clone()),
            }
            button { "Submit Post" }
        }
    }
}