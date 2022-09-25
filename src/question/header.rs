#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct HeaderProps {
    id: u64,
    _title: String,
}

pub fn Header(cx: Scope<HeaderProps>) -> Element {
    let id = cx.props.id;
    
    cx.render(rsx! {
        div {
            class:"row-question-header",
            h4 {
                a {
                    href:"/question/{id}",
                    onclick: move |_| {
                        
                    },
                    "{id}-{cx.props._title}"
                }
            }
        }
    })
}
