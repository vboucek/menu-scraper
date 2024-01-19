use chrono::NaiveDate;
use db::db::models::LunchWithGroup;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LunchPreviewView {
    pub id: Uuid,
    pub date: NaiveDate,
    pub group_id: Uuid,
    pub menu_id: Uuid,
    pub group_name: String,
}

impl LunchPreviewView {
    pub fn new(lunch: LunchWithGroup, menu_id: Uuid) -> Self {
        LunchPreviewView {
            id: lunch.id,
            date: lunch.date,
            group_id: lunch.group_id,
            group_name: lunch.group_name,
            menu_id,
        }
    }
}
