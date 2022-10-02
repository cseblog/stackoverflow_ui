use crate::model::ui::CommentObject;


#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct RawCommentObject {
    pub Id: u64,
    pub PostId: String,
    pub UserId: String,
    pub Score: String,
    pub ContentLicense: String,
    pub UserDisplayName: String,
    pub Text: String,
    pub CreationDate: String,
}

impl RawCommentObject {
    pub fn transform(self) -> CommentObject {
        let c = CommentObject {
            id: self.Id,
            post_id: self.PostId.parse().unwrap(),
            text: self.Text,
            timestamp: self.CreationDate,
        };
        return c;
    }
}