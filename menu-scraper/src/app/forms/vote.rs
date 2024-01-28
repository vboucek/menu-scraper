use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddVoteFormData {
    pub menu_id: Uuid,
    pub lunch_id: Uuid,
}
