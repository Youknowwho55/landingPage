use dioxus::prelude::*;
use crate::server::{Credentials, AuthError, AuthClient};
use crate::views::routes::Routes;
use crate::server::use_auth;


#[component]
pub fn Login() -> Element {
    let email = use_signal(|| String::new());
    let password = use_signal(|| String::new());
    let error = use_signal::<Option<AuthError>>(|| None);
    let nav = use_navigator();
    let auth = use_auth();

    let onsubmit = move |_| {
        let email = email.read().clone();
        let password = password.read().clone();
        let nav = nav.clone();
        let auth = auth.clone();
        
        spawn(async move {
            match auth.login(&email, &password).await {
                Ok(_) => nav.push(Routes::Home {}),
                Err(e) => error.set(Some(e)),
            }
        });
    };

    rsx! {
        form { onsubmit: onsubmit,
            div {
                label { "Email" }
                input {
                    r#type: "email",
                    value: "{email}",
                    oninput: move |e| email.set(e.value.clone()),
                }
            }
            div {
                label { "Password" }
                input {
                    r#type: "password",
                    value: "{password}",
                    oninput: move |e| password.set(e.value.clone()),
                }
            }
            button { r#type: "submit", "Login" }
            error.read().as_ref().map(|e| rsx! {
                div { style: "color: red;", "{e}" }
            })
        }
    }
}