use dioxus::prelude::*;
use crate::server::{Credentials, AuthError};
use crate::views::routes::Routes;
use crate::server::use_auth;

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
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
                Ok(_) => {
                    nav.push(Routes::Home {}).expect("Navigation should work");
                },
                Err(e) => {
                    log::error!("Login failed: {}", e);
                    // Optionally show error to user
                }
            }
        });
    };

    rsx! {
        form { onsubmit,
            div {
                div {
                    label { "Email" }
                    input {
                        r#type: "email",
                        value: "{email}",
                        oninput: move |e| email.set(e.value().clone()),
                    }
                }
                div {
                    label { "Password" }
                    input {
                        r#type: "password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value().clone()),
                    }
                }
                button { r#type: "submit", "Login" }
                if let Some(e) = error.read().as_ref() {
                    div {
                        div { style: "color: red;", "{e}" }
                    }
                }
            }
        }
    }
}