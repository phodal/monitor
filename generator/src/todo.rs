use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoResponse {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    pub value: Vec<TodoItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoItem {
    #[serde(rename = "@odata.etag")]
    pub odata_etag: String,
    pub importance: String,
    pub is_reminder_on: bool,
    pub status: String,
    pub title: String,
    pub created_date_time: String,
    pub last_modified_date_time: String,
    pub id: String,
    pub body: TodoBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoBody {
    pub content: String,
    pub content_type: String,
}
