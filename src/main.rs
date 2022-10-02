#![allow(non_snake_case)]

use dioxus::{
    prelude::*,
    router::{Route, Router},
};

mod home;
mod post_detail;
mod question;
mod search;
mod model;

use crate::model::ui::*;

// An array of current question
pub static QUESTION_LISTS: AtomRef<Vec<QuestionObject>> = |_| vec![];
pub static SEARCH_TXT: AtomRef<String> = |_| "".to_string();

fn main() {
    wasm_logger::init(wasm_logger::Config::default());    
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    log::info!("Some info");
    cx.render(rsx! {
        Router {
            Route { to: "/", home::Home{}},
            Route { to: "/question/:id", post_detail::Post {}}
        }
    })
}
