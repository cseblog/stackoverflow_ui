#![allow(non_snake_case)]
use crate::question::Question;
use crate::search::Search;
use crate::QUESTION_LISTS;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let questions = use_atom_ref(&cx, QUESTION_LISTS);
    cx.render(rsx! {
        div {
            class: "container",
            Search{},
            div {
                questions.read().iter().map(|q| {
                    rsx!{
                        Question{
                            key: "{q.post.id}",
                            post: q.post.clone(),
                            answers: q.answers.clone(),
                            comments: q.comments.clone()
                        }
                    }
                })
            }
        }
    })
}
