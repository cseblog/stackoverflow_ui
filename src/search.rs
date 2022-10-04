use crate::model::raw::post::RawPost;
use crate::QuestionObject;
use dioxus::prelude::*;

#[inline_props]
pub fn Search(
    cx: Scope,
    txt: UseState<String>,
    questions: UseState<Vec<QuestionObject>>,
) -> Element {
    // Get list question
    let search_str = txt.get();
    let post_request = use_future(&cx, search_str, |search_str| async move {
        reqwest::Client::new()
            .post("http://localhost:9990/search")
            .body(search_str)
            .send()
            .await
            .unwrap()
            .json::<Vec<RawPost>>()
            .await
    });

    match post_request.value() {
        Some(Ok(post_list)) => {
            let mut question_result = vec![];
            for p in post_list.into_iter() {
                let pp = p.clone();
                let q = pp.transform();
                question_result.push(QuestionObject {
                    post: q.clone(),
                    answers: vec![],
                    comments: vec![],
                });
            }
            questions.set(question_result);
        }
        Some(Err(_)) => return cx.render(rsx! {div{"No record"}}),
        None => return cx.render(rsx! {div{"No record"}}),
    };

    cx.render(rsx! {
        div {
            class: "row height header d-flex justify-content-center align-items-center",
            div {
                class:"col-2 home-link",
                a {
                    href: "/",
                    "StackOverflow"
                }
            },
            div {
                class: "col-10",
                div {
                    class: "search",
                    input {
                        class: "form-control",
                        placeholder: "Search question!",
                        value: "{txt}",
                        autofocus: "true",
                        oninput: move |evt| {
                            txt.set(evt.value.clone())
                        }
                    },
                }
            }
        }
    })
}
