use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct AnswerCountProps {
    count: u32,
}

pub fn AnswerCount(cx: Scope<AnswerCountProps>) -> Element {
    cx.render(
        rsx! {
        div {
            class: "answer-count",
            div {
                "{cx.props.count}"
            }
            div {
                "anwers"
            }
        }
    }
    )
}