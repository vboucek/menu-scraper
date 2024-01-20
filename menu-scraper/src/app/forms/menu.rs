use crate::app::forms::ordering::{Ordering, RestaurantOrderingMethod};
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MenuListQuery {
    pub date: NaiveDate,
    pub method: RestaurantOrderingMethod,
    pub ordering: Ordering,
    pub page: usize,
}
