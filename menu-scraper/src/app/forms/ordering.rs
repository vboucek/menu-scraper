use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Ordering {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RestaurantOrderingMethod {
    #[serde(rename = "price")]
    Price,
    #[serde(rename = "range")]
    Range,
}
