#![allow(non_snake_case)]

use crate::question::Question;
use crate::search::*;
// use crate::QUESTION_LISTS;

use dioxus::{
    prelude::*,
    router::{use_route},
};

pub fn Post(cx: Scope) -> Element {
    let post_id = use_route(&cx).last_segment()?;
    // let questions = use_atom_ref(&cx, QUESTION_LISTS);

    let id = &post_id.to_string();
    let post_request = use_future(&cx, id, |id| async move {
        reqwest::Client::new()
            .post(format!("http://localhost:9990/search/{}", id))
            .send()
            .await
            .unwrap()
            .json::<PostRow>()
            .await
    });

    match post_request.value() {
        Some(Ok(post)) => {
            let q = tranform(post);
            return cx.render(rsx! {
                    div {
                        class: "container",
                        Search {},
                        div {
                            Question {
                                id: q.id,
                                vote: q.vote.clone(),
                                answer: q.answer.clone(),
                                _title: q._title.clone(),
                                _body: q._body.clone()
                            }
                        }
                    }
            });
        }
        Some(Err(_)) => return cx.render(rsx! {div {"failed"}}),
        None => return cx.render(rsx! {div {"failed"}}),
    };
}
