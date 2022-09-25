#![allow(non_snake_case)]

use dioxus::{
    prelude::*,
    router::{Route, Router},
};

mod home;
mod post;
mod question;
mod search;

use home::Home;
use post::Post;

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

pub static QUESTION_LISTS: AtomRef<Vec<QuestionItem>> = |_| vec![];
fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            Route { to: "/", Home{}},
            Route { to: "/question/:id", Post {}}
        }
    })
}
