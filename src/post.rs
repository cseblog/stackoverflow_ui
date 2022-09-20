use dioxus::prelude::*;

pub fn Post (cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h3 {
                "This is detail page"
            }
        }
    })
}