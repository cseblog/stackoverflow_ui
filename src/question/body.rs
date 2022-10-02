use dioxus::prelude::*;

use crate::QuestionObject;
use crate::question::vote_count::*;
use crate::question::answer_count::*;

pub fn Body(cx: Scope<QuestionObject>) -> Element {

    let answers = cx.props.answers.clone();
    let comments = cx.props.comments.clone();
    
    cx.render(rsx!{
        div { 
            class: "row answer vote-answer-block",
            div { 
                class: "col-2 vote-answer-block",
                VoteCount {
                    count: cx.props.post.vote_count
                }
                AnswerCount {
                    count: cx.props.post.answer_count
                }
            }

            div { 
                class: "col-10 row-question-body",
                dangerous_inner_html:"{cx.props.post._body}",
                h5 {
                    "Comments"
                },
                comments.iter().map(|item| {
                    rsx!(
                        div {
                            div {
                                "{item.text}",
                            }
                        }
                    )
                })

            }

            div {
                h3 {
                    "Answers"
                }
                answers.iter().map(|item| {
                    rsx!(
                        div {
                            class: "row answer vote-answer-block",
                            div {
                                class: "col-2 vote-answer-block",
                                VoteCount {
                                    count: item.vote_count
                                }
                                AnswerCount {
                                    count: item.answer_count
                                }
                            }
                            div {
                                class: "col-10 row-question-body",
                                "{item._body}"
                            }
                        }
                    )
                })
            }
        }
    })
}
