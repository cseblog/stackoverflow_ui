use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct AnswerProps {
    answer_count: u32,
    vote_count: u32,
    text: String
}

pub fn Answer(cx: Scope<AnswerProps>) -> Element {
    cx.render(rsx!{
        div {
            class: "row answer vote-answer-block",
            div {
                class: "col-2 vote-answer-block"
                h3 {
                    "This answer text"
                }
            }
        }
    })
}        