use dioxus::{prelude::*};

use crate::APPSTATE;
use crate::QuestionItem;


#[derive(Props, PartialEq)]
pub struct SearchProps {
    score: u32
}

pub fn Search(cx: Scope<SearchProps>) -> Element {
    
    // let dog = use_future(&cx, (), |_| async move {
    //     reqwest::get("https://dog.ceo/api/breeds/image/random")
    //         .await
    //         .unwrap()
    //         .json::<RandomDog>()
    //         .await
    // });
    
    let questions = use_atom_ref(&cx, APPSTATE);
    cx.render(rsx!{
        div {
            class: "row height d-flex justify-content-center align-items-center", 
            div {
                class: "col-8",
                div {
                    class: "search",
                    input {
                        class: "form-control",
                        placeholder: "Search question!",
                        oninput: move |evt| questions.write().push(QuestionItem{
                            vote: 100,
                            answer: 33,
                            _title: "sear".to_string(),
                            _description: "dfadfad".to_string()
                        })
                    }
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