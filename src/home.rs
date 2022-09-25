#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::QUESTION_LISTS;
use crate::question;
use crate::search::Search;

pub fn Home(cx: Scope) -> Element {
    let questions = use_atom_ref(&cx, QUESTION_LISTS);
    cx.render(rsx! {
        div {
            class: "container",
            Search {},
            div {
                questions.read().iter().map(|q| rsx!{
                    question::Question {
                        id: q.id,
                        vote: q.vote.clone(),
                        answer: q.answer.clone(),
                        _title: q._title.clone(),
                        _body: q._body.clone()
                    }
                })
            }
        }
    })
}
