use dioxus::prelude::{*};

pub mod header;
pub mod body;

pub mod answer_count;
pub mod vote_count;

#[derive(Props, PartialEq)]
pub struct QuestionProps {
    vote_count: u32,
    answer_count: u32,
    _title: String,
    text: String
}

pub fn Question(cx: Scope<QuestionProps>) -> Element {
    cx.render(rsx!{
        div { 
            header::Header{
                _title: cx.props._title.clone()
            }
            body::Body{
                vote_count: cx.props.vote_count,
                answer_count: cx.props.answer_count,
                text: cx.props.text.clone()
            }
        }
    })
}
