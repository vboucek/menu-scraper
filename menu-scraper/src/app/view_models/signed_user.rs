use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SignedUser {
    pub username: String,
    pub profile_picture: Option<String>,
}
