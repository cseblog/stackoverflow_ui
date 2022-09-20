use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h3 {
                "This is home page"
            }
        }
    })
}
