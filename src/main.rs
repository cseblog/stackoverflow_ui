use dioxus::{ prelude::* };

mod question;
mod search;
fn main() {
    dioxus::web::launch(app);
}


#[derive(Debug, PartialEq, Clone)]
pub struct QuestionItem {
    pub id: u64,
    pub post_type_id: u32,
    pub vote: u32,
    pub answer: u32,
    pub _title: String,
    pub _body: String,
    // pub answerList: Vec<AnswerItem>
}

pub static APPSTATE: AtomRef<Vec<QuestionItem>> = |_| vec![];
fn app(cx: Scope) -> Element {
    let questions = use_atom_ref(&cx, APPSTATE);
    cx.render(
        rsx! {
        div {
            class: "container",
            search::Search {}

            div {
                questions.read().iter().map(|q| rsx!{
                    question::Question {
                        vote: q.vote,
                        answer: q.answer,
                        _title: q._title.clone(),
                        _body: q._body.clone()
                    }
                })
            }
        }
    }
    )
}