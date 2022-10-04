use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct VoteCountProps {
    count: u64,
}

pub fn VoteCount(cx: Scope<VoteCountProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "vote-count",
            span {
                "{cx.props.count}"
            }
            span {
                " votes"
            }
        }
    })
}
