use dioxus::prelude::{*};

pub mod header;
pub mod body;

pub mod answer_count;
pub mod vote_count;

#[derive(Props, PartialEq)]
pub struct QuestionProps {
    vote: u32,
    answer: u32,
    _title: String,
    _body: String
}

pub fn Question(cx: Scope<QuestionProps>) -> Element {
    cx.render(rsx!{
        div { 
            header::Header{
                _title: cx.props._title.clone()
            }
            body::Body{
                vote: cx.props.vote,
                answer: cx.props.answer,
                _body: cx.props._body.clone()
            }
        }
    })
}
