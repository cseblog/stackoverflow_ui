#![allow(non_snake_case)]

use crate::QUESTION_LISTS;
use crate::question::Question;
use crate::{model::raw::RawQuestionObject, search::*};
use dioxus::{prelude::*, router::use_route};

pub fn Post(cx: Scope) -> Element {
    let state = use_state(&cx, || 0);
    let post_id = use_route(&cx).last_segment()?;
    let questions = use_atom_ref(&cx, QUESTION_LISTS);
    // if !questions.read().is_empty() {
    //     return cx.render(rsx! {
    //         div {
    //             class: "container",
    //             Search{},
    //             div {
    //                 questions.read().iter().map(|q| {
    //                     rsx!{
    //                         Question{
    //                             key: "{q.post.id}",
    //                             post: q.post.clone(),
    //                             answers: q.answers.clone(),
    //                             comments: q.comments.clone()
    //                         }
    //                     }
    //                 })
    //             }
    //         }
    //     })
    // }

    let id = &post_id.to_string();
    let post_request = use_future(&cx, id, |id| async move {
        reqwest::Client::new()
            .post(format!("http://localhost:9990/search/{}", id))
            .send()
            .await
            .unwrap()
            .json::<RawQuestionObject>()
            .await
    });

    match post_request.value() {
        Some(Ok(post)) => {
            let q = post.clone().transform();
            return cx.render(rsx! {
                    div {
                        class: "container",
                        Search {},
                        div {
                            Question{
                                post: q.post.clone(),
                                answers: q.answers.clone(),
                                comments: q.comments.clone()
                            }
                        }
                    }
            });
        }
        Some(Err(_)) => return cx.render(rsx! {div {"internal error!"}}),
        None => return cx.render(rsx! {div {"No record!{id}"}}),
    };
}
