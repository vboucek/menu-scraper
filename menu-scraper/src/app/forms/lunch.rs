use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetLunchPreviewsQuery {
    pub date: NaiveDate,
    pub menu_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateLunchFormData {
    #[serde(rename = "create-lunch-date-picker")]
    pub date: NaiveDate,
}
