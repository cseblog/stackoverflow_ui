pub mod comment;
pub mod post;

use self::comment::RawCommentObject;
use self::post::RawPost;
use super::ui::{CommentObject, PostObject, QuestionObject};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct RawQuestionObject {
    pub VoteCount: u64,
    pub Post: RawPost,
    pub Answers: Vec<RawAnswerObject>,
    pub Comments: Vec<RawCommentObject>,
}

impl RawQuestionObject {
    pub fn transform(self) -> QuestionObject {
        let post = self.Post.clone();

        let mut ui_answers = Vec::new();
        for answer in self.Answers.into_iter() {
            ui_answers.push(answer.transform());
        }

        let mut ui_comments = Vec::new();
        for uicomment in self.Comments.into_iter() {
            ui_comments.push(uicomment.transform());
        }

        let p = QuestionObject {
            answers: ui_answers,
            comments: ui_comments,
            post: post.transform(),
        };
        return p;
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct RawAnswerObject {
    pub VoteCount: u64,
    pub Post: RawPost,
    pub Comments: Vec<RawCommentObject>,
}

impl RawAnswerObject {
    pub fn transform(self) -> PostObject {
        let mut p = self.Post.transform();
        p.view_count = self.VoteCount;
        return p;
    }

    // pub fn getUiComments(self) -> Vec<CommentObject> {
    //     let mut comments = Vec::new();
    //     for c in self.Comments.into_iter() {
    //         let po = c.transform();
    //         comments.push(po);
    //     }
    //     return comments;
    // }
}
