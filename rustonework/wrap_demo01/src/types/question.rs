use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Question {
    pub(crate) id: QuestionId,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewQuestion{
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct QuestionId(pub i32);