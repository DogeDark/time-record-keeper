use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub struct SavedRecord {
    pub date: NaiveDate,
    pub time: u32,
    pub description: String,
}
