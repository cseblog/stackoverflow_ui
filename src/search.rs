use std::vec;

use dioxus::prelude::*;
use gloo::net::http::Request;
use crate::QuestionItem;
use crate::APPSTATE;

#[derive(Debug, PartialEq, Clone,serde::Serialize,serde::Deserialize)]
struct PostRow {
    Id: u64,
    Body: String,
    Title: String
}

pub fn Search(cx: Scope<()>) -> Element {
    
    let post_request = use_future(&cx, (), |_| async move {
        reqwest::Client::new().post("http://localhost:9990/search")
        .body("law").send().await.unwrap()
        .json::<Vec<PostRow>>().await
    });
    
    let questions = use_atom_ref(&cx, APPSTATE);
    cx.render(
        rsx! {
        div {
            class: "row height d-flex justify-content-center align-items-center", 
            div {
                class: "col-8",
                div {
                    class: "search",
                    input {
                        class: "form-control",
                        placeholder: "Search question!",
                        oninput: move |evt| {
                            let txt = evt.value;
                            let result = post_request.restart();
                            questions.write().push(QuestionItem{
                                id: 10,
                                post_type_id: 1,
                                vote: 100,
                                answer: 33,
                                _title: "What journals do not allow open access to published material?".to_string(),
                                _body: "<p>Regardless of the field or subject, \"is\" there a website or organization that has rules on how to section and sub-section a document?\
                                 I have a document on which I have randomly chosen a layout.</p> <p>project document outline:</p>\
                                  <pre><code>1. Define Goals to stakeholders 2. Product/ Objective/ Expectations 3. Scope 4. Time Frame with phases and processes 5. Project Budget 6. Risk and Benefits 7. Team members and organizational chart w/ roles and responsibilities matrix 8. Rules of Communication 9. extra docs include: a. action and issue tracker b. project charter c. project organization d. project roles and responsibilities e. project plan f. project budget breakdown g. stakeholder matrix h. risk log i. project communication plan j. scope statement/requirement specification k. change request tracker l. design document m. possible obstacles &amp; problems 10. Full visualization of organization and process. </code></pre> <p>This layout is completely a guess, and I am wondering if there is a guide to how to section and subsection, some law or rule to do sectioning or sub-sectioning the proper way that anyone from any field can follow</p>".to_string()
                            })
                        }
                    }

                    button {
                        class:"btn btn-primary",
                        // onclick: move |_| app_state.search_txt = "".to_string(),
                        "Search"
                    }
                }
            }
        }
    }
    )
}