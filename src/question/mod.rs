use dioxus::prelude::*;

pub mod body;
pub mod header;

pub mod answer_count;
pub mod vote_count;

#[derive(Props, PartialEq)]
pub struct QuestionProps {
    id: u64,
    vote: String,
    answer: String,
    _title: String,
    _body: String,
}

pub fn Question(cx: Scope<QuestionProps>) -> Element {
    cx.render(rsx! {
        div {
            header::Header{
                id: cx.props.id,
                _title: cx.props._title.clone()
            }
            body::Body{
                vote: cx.props.vote.clone(),
                answer: cx.props.answer.clone(),
                _body: cx.props._body.clone()
            }
        }
    })
}
