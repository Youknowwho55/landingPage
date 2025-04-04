use dioxus::prelude::*;

#[component]
pub fn NotFound() -> Element {
    rsx! {
        p { "this pages is not found" }
    }
}
