use dioxus::prelude::*;

use crate::question::vote_count::*;
use crate::question::answer_count::*;

#[derive(PartialEq, Props)]
pub struct BodyProps{
    vote: String,
    answer: String,
    _body: String
}

pub fn Body(cx: Scope<BodyProps>) -> Element {
    cx.render(rsx!{
        div { 
            class: "row answer vote-answer-block",
            div { 
                class: "col-2 vote-answer-block",
                VoteCount {
                    count: cx.props.vote.clone()
                }
                AnswerCount {
                    count: cx.props.answer.clone()
                }
            }
            div { 
                class: "col-10 row-question-body",
                dangerous_inner_html:"{cx.props._body}"
            }
        }
    })
}
