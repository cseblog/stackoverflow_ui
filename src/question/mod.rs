use dioxus::prelude::*;

use crate::model::ui::QuestionObject;

pub mod body;
pub mod header;

pub mod answer_count;
pub mod vote_count;


pub fn Question(cx: Scope<QuestionObject>) -> Element {
    cx.render(rsx! {
        div {
            header::Header{
                key: "head",
                post: cx.props.post.clone(),
                answers: vec![],
                comments: vec![],
            }
            body::Body{
                    key: "body",
                    post: cx.props.post.clone(),
                    answers: cx.props.answers.clone(),
                    comments: cx.props.comments.clone()
                }
            }
    })
}
