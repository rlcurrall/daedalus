use serde::{Deserialize, Serialize};
use tsync::tsync;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
pub struct Paginated<T> {
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub data: Vec<T>,
}
