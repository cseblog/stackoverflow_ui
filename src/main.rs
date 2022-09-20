use dioxus::{ prelude::*, router::{Route, Router} };

mod question;
mod search;
mod home;
mod post;

#[derive(Debug, PartialEq, Clone)]
pub struct QuestionItem {
    pub id: u64,
    pub post_type_id: String,
    pub vote: String,
    pub answer: String,
    pub _title: String,
    pub _body: String,
    // pub answerList: Vec<AnswerItem>
}

pub static APPSTATE: AtomRef<Vec<QuestionItem>> = |_| vec![];

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {

    let questions = use_atom_ref(&cx, APPSTATE);
    cx.render(rsx! {
        Router {
            Route { to: "/", children: Home{}}, 
            Route { to: "/question", children: Post {}}
        }
    })
}
