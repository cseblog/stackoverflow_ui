use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct AnswerCountProps {
    count: u64,
}

pub fn AnswerCount(cx: Scope<AnswerCountProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "answer-count",
            span {
                "{cx.props.count}"
            }
            span {
                " anwers"
            }
        }
    })
}
