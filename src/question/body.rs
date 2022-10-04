use dioxus::prelude::*;

use crate::question::answer_count::*;
use crate::question::vote_count::*;
use crate::QuestionObject;

pub fn Body(cx: Scope<QuestionObject>) -> Element {
    let answers = cx.props.answers.clone();
    let comments = cx.props.comments.clone();

    let com = if comments.len() > 0 {
        cx.render(rsx!{
            h6{
                "Comments"
            }
        })
    } else {
        cx.render(rsx!{
            h6{}
        })
    };

    let ans = if answers.len() > 0 {
        cx.render(rsx!{
            h6 {
                "Answers"
            }
        })
    } else {
        cx.render(rsx!(h6{}))
    };

    cx.render(
        rsx! {
        div {
            class: "row answer vote-answer-block",
            div {
                class: "col-2 vote-answer-block",
                VoteCount {
                    count: cx.props.post.view_count
                }
                AnswerCount {
                    count: cx.props.post.answer_count
                }
            }

            div {
                class: "col-10 row-question-body",
                dangerous_inner_html:"{cx.props.post._body}",
                com,
                comments.iter().map(|item| {
                    rsx!(
                        div {
                            key: "{item.id}", 
                            class: "comment-text",
                            dangerous_inner_html: "{item.text}",
                        }
                        
                    )
                })
            }

            div {
                class:"answers",
                ans,
                answers.iter().map(|item| {
                    rsx!(
                        div {
                            class: "row question-page answer",
                            key: "{item.id}",
                            div {
                                class: "col-2 vote-answer-block",
                                VoteCount {
                                    count: item.view_count
                                }
                                AnswerCount {
                                    count: item.answer_count
                                }
                            }
                            div {
                                class: "col-10 answercell post-layout--right",
                                div {
                                    class: "s-prose js-post-body",
                                    dangerous_inner_html: "{item._body}"
                                }

                            }
                        }
                    )
                })
            }
        }
    }
    )
}