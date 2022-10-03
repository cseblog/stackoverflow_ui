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
