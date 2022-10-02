
use crate::QuestionObject;

use crate::QUESTION_LISTS;
use crate::SEARCH_TXT;
use crate::model::raw::post::RawPost;
use crate::model::ui::PostObject;
use dioxus::prelude::*;


pub fn Search(cx: Scope<()>) -> Element {
    let state = use_state(&cx, || "".to_string());
    
    let questions = use_atom_ref(&cx, QUESTION_LISTS);

    // Get list question
    let txt = state.get();
    let post_request = use_future(&cx, txt, |txt| async move {
        reqwest::Client::new()
            .post("http://localhost:9990/search")
            .body(txt)
            .send()
            .await
            .unwrap()
            .json::<Vec<RawPost>>()
            .await
    });

    match post_request.value() {
        Some(Ok(post_list)) => {
            questions.write().clear();
            for p in post_list.into_iter() {
                let pp = p.clone();
                let q = pp.transform();
                questions.write().push(QuestionObject {
                    post: q.clone(),
                    answers: vec![],
                    comments: vec![],
                });
            }
        }
        Some(Err(_)) => return cx.render(rsx!{div{"No record"}}),
        None => return cx.render(rsx!{div{"No record"}}),
    };

    cx.render(rsx! {
        div {
            class: "row height d-flex justify-content-center align-items-center",
            div {
                class:"col-4",
                a {
                    href: "/",
                    "home"
                }
            },
            div {
                class: "col-8",
                div {
                    class: "search",
                    input {
                        class: "form-control",
                        placeholder: "Search question!",
                        value: "{state}",
                        autofocus: "true",
                        oninput: move |evt| {
                            state.set(evt.value.clone())
                        }
                    },
                }
            }
        }
    })
}
