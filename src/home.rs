#![allow(non_snake_case)]
use crate::model::ui::QuestionObject;
use crate::question::Question;
use crate::search::Search;
use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let questions: &UseState<Vec<QuestionObject>> = use_state(&cx,|| vec![]);
    let search_txt = use_state(&cx, || "".to_string());
    cx.render(rsx! {
        div {
            class: "container",
            Search{txt: search_txt.clone(), questions: questions.clone()},
            div {
                class: "row height d-flex justify-content-center align-items-center",
                div {
                    class:"col-2",
                }
                div {
                    class: "col-10",
                    questions.get().iter().map(|q| {
                        rsx!{
                            div {
                                key: "{q.post.id}",
                                class:"s-post-summary",
                                Question{
                                    key: "{q.post.id}",
                                    post: q.post.clone(),
                                    answers: q.answers.clone(),
                                    comments: q.comments.clone()
                                }
                            }
                        }
                    })
                }
            }
        }
    })
}
