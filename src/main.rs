use dioxus::{prelude::*};

pub mod question;
pub mod search;
fn main() {
    dioxus::web::launch(app);
}


// #[derive(Debug, PartialEq, Clone)]
// pub struct AnswerItem {
//     vote: u32,
//     _title: String,
//     _description: String
// }

#[derive(Debug, PartialEq, Clone)]
pub struct QuestionItem {
    pub vote: u32,
    pub answer: u32,
    pub _title: String,
    pub _description: String
    // pub answerList: Vec<AnswerItem>
}

pub static APPSTATE: AtomRef<Vec<QuestionItem>> = |_| vec![
    QuestionItem {
        vote: 100,
        answer: 50,
        _title: "test ttitle".to_string(),
        _description: "test_description".to_string()
    }
];

fn app(cx: Scope) -> Element {
    let questions = use_atom_ref(&cx, APPSTATE);
    cx.render(rsx!{
        div {
            class: "container",
            search::Search {
                score: 100
            }

            div {
                questions.read().iter().map(|q| rsx!{
                    question::Question {
                        vote_count: q.vote,
                        answer_count: q.answer,
                        _title: q._title.clone(),
                        text: q._description.clone()
                    }
                })
            }
        }
    })
}
