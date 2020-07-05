#[derive(Queryable, Serialize, Deserialize)]
pub struct TrailorData {
    pub id: i32,
    pub trailor: i32,
    pub timestamp: i32,
    pub temperature: Option<i32>,
}
