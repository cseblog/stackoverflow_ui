use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct HeaderProps{
    id: u64,
    _title: String
}

pub fn Header(cx: Scope<HeaderProps>) -> Element {
    cx.render(rsx!{
        div { 
            class:"row-question-header",
            h4 {
                a {
                    href:"/{cx.props.id}",
                    "{cx.props._title}"
                }
            }
        }
    })
}

