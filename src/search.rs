use std::vec;

use crate::QuestionItem;
use crate::APPSTATE;
use dioxus::prelude::*;
use gloo::net::http::Request;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PostRow {
    Id: u64,
    Body: String,
    Title: String,
    OwnerUserId: u64,
    LastEditorUserId: u64,
    PostTypeId: u32,
    AcceptedAnswerId: u64,
    Score: u32,
    ParentId: u64,
    ViewCount: u32,
    AnswerCount: u32,
    CommentCount: u32,
    OwnerDisplayName: String,
    LastEditorDisplayName: String,
    Tags: String,
}

pub fn tranform(post: &PostRow) -> QuestionItem {
    let p = QuestionItem {
        id: post.Id,
        _body: post.Body.clone(),
        _title: post.Title.clone(),
        answer: post.AnswerCount,
        post_type_id: post.PostTypeId,
        vote: post.Score,
    };
    return p;
}

pub fn Search(cx: Scope<()>) -> Element {
    let state = use_state(&cx, || "".to_string());
    let questions = use_atom_ref(&cx, APPSTATE);

    //Get list question
    let txt = state.get();
    let post_request = use_future(&cx, txt, |txt| async move {
        reqwest::Client::new()
            .post("http://localhost:9990/search")
            .body(txt)
            .send()
            .await
            .unwrap()
            .json::<Vec<PostRow>>()
            .await
    });

    let status = match post_request.value() {
        Some(Ok(post_list)) => {
            let mut count = 0;
            questions.write().clear();
            for item in post_list.into_iter() {
                let q = tranform(item);
                if count < 20 {
                    questions.write().push(q);
                    count = count + 1;
                }
            }
        }
        Some(Err(err)) => {}
        None => {}
    };

    cx.render(rsx! {
        div {
            class: "row height d-flex justify-content-center align-items-center",
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
                            state.set(evt.value.clone());
                        }
                    },
                    button {
                        class:"btn btn-primary",
                        // onclick: move |_| app_state.search_txt = "".to_string(),
                        "Search"
                    }
                }
            }
        }
    })
}
