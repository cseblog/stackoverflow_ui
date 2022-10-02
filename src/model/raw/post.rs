use crate::model::ui::PostObject;


#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct RawPost {
    pub Id: u64,
    pub Body: String,
    pub Title: String,
    pub OwnerUserId: String,
    pub LastEditorUserId: String,
    pub PostTypeId: String,
    pub AcceptedAnswerId: String,
    pub Score: String,
    pub ParentId: String,
    pub ViewCount: String,
    pub AnswerCount: String,
    pub CommentCount: String,
    pub OwnerDisplayName: String,
    pub LastEditorDisplayName: String,
    pub Tags: String,
    pub CreationDate: String,
}

impl RawPost {
    pub fn transform(self) ->PostObject {
        let p = PostObject {
            id: self.Id,
            timestamp: self.CreationDate,
            _body: self.Body.clone(),
            _title: self.Title.clone(),
            _tags: self.Tags.clone(),
            answer_count: 0,
            comment_count: 0,
            vote_count: 0,
        };
        return p;
    }    
}