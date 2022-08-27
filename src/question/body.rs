use dioxus::prelude::*;

use crate::question::vote_count::*;
use crate::question::answer_count::*;

#[derive(PartialEq, Props)]
pub struct BodyProps{
    vote_count: u32,
    answer_count: u32,
    text: String
}

pub fn Body(cx: Scope<BodyProps>) -> Element {
    cx.render(rsx!{
        div { 
            class: "row answer vote-answer-block",
            div { 
                class: "col-2 vote-answer-block",
                VoteCount {
                    count: cx.props.vote_count
                }
                AnswerCount {
                    count: cx.props.answer_count
                }
            }
            div { 
                class: "col-10 row-question-body",
                "{cx.props.text}"
            }
        }
    })
}
