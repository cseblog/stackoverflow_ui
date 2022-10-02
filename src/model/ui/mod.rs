use dioxus::prelude::Props;

#[derive(Default, Debug, PartialEq, Clone, Props)]
pub struct QuestionObject {
    pub post: PostObject,
    pub answers: Vec<PostObject>,
    pub comments: Vec<CommentObject>,
}

#[derive(Default, Debug, PartialEq, Clone, Props)]
pub struct PostObject {
    pub id: u64,
    pub vote_count: u64,
    pub answer_count: u64,
    pub comment_count: u64,
    pub _title: String,
    pub _body: String,
    pub _tags: String,
    pub timestamp: String,
}

#[derive(Default, Debug, PartialEq, Clone, Props)]
pub struct CommentObject {
    pub id: u64,
    pub post_id: u64,
    pub text: String,
    pub timestamp: String,
}