#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::model::ui::QuestionObject;

pub fn Header(cx: Scope<QuestionObject>) -> Element {
    let id = cx.props.post.id;
    cx.render(rsx! {
        div {
            class:"row-question-header",
            h3 {
                class:"fs-headline1 ow-break-word mb8 flex--item fl1",
                a {
                    class:"hyperlink",
                    href:"/question/{id}",
                    onclick: move |_| {
                    },
                    "{cx.props.post._title}"
                }
            }
        }
    })
}
