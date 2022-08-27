use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct VoteCountProps{
    count: u32
}

pub fn VoteCount(cx: Scope<VoteCountProps>) -> Element {
    cx.render(rsx!{
        div {
            class: "vote-count",
            div {
                "{cx.props.count}"
            }
            div {
                "votes"
            }
        }
    })
}