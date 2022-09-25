use crate::QuestionItem;
use crate::QUESTION_LISTS;
use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PostRow {
    Id: u64,
    Body: String,
    Title: String,
    OwnerUserId: String,
    LastEditorUserId: String,
    PostTypeId: String,
    AcceptedAnswerId: String,
    Score: String,
    ParentId: String,
    ViewCount: String,
    AnswerCount: String,
    CommentCount: String,
    OwnerDisplayName: String,
    LastEditorDisplayName: String,
    Tags: String,
}

pub fn tranform(post: &PostRow) -> QuestionItem {
    let p = QuestionItem {
        id: post.Id,
        _body: post.Body.clone(),
        _title: post.Title.clone(),
        answer: post.AnswerCount.clone(),
        post_type_id: post.PostTypeId.clone(),
        vote: post.Score.clone(),
    };
    return p;
}

pub fn Search(cx: Scope<()>) -> Element {  
    let state = use_state(&cx, || "".to_string());
    let questions = use_atom_ref(&cx, QUESTION_LISTS);

    // Get list question
    let txt = state.get();

    let post_request 
            = use_future(&cx, txt, |txt| async move {
        reqwest::Client::new()
            .post("http://localhost:9990/search")
            .body(txt)
            .send()
            .await
            .unwrap()
            .json::<Vec<PostRow>>()
            .await
    });

    match post_request.value() {
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
        Some(Err(_)) => {}
        None => {}
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
                            state.set(evt.value.clone());
                        }
                    },
                }
            }
        }
    })
}
