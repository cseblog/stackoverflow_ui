use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct VoteCountProps {
    count: u64,
}

pub fn VoteCount(cx: Scope<VoteCountProps>) -> Element {
    cx.render(rsx! {
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
