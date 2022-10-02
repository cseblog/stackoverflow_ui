use dioxus::prelude::*;
use crate::Question;
use crate::question::vote_count::*;
use crate::question::answer_count::*;

pub fn Answer(cx: Scope<Post>) -> Element {
    cx.render(rsx! {
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
                dangerous_inner_html:"{cx.props._body}"
            }
        }
    })
}
