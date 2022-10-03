#![allow(non_snake_case)]

use crate::model::ui::QuestionObject;
use crate::question::Question;
use crate::{model::raw::RawQuestionObject, search::*};
use dioxus::{prelude::*, router::use_route};

pub fn Post(cx: Scope) -> Element {
    //Get from router
    let post_id = use_route(&cx).last_segment()?;
    let id = &post_id.to_string();

    // State
    let questions: &UseState<Vec<QuestionObject>> = use_state(&cx, || vec![]);
    let search_txt = use_state(&cx, || "".to_string());

    // Query server
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
            if search_txt.get().is_empty() {
                let q = post.clone().transform();
                return cx.render(rsx! {
                        div {
                            class: "container",
                            Search {txt: search_txt.clone(), questions: questions.clone()},
                            div {
                                Question{
                                    post: q.post.clone(),
                                    answers: q.answers.clone(),
                                    comments: q.comments.clone()
                                }
                            }
                        }
                });
            } else {
                return cx.render(rsx! {
                    div {
                        class: "container",
                        Search{txt: search_txt.clone(), questions: questions.clone()},
                        div {
                            questions.get().iter().map(|q| {
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
                });
            }
        }
        Some(Err(_)) => return cx.render(rsx! {div {"internal error!"}}),
        None => return cx.render(rsx! {div {"No record!{id}"}}),
    };
}
