use chrono::NaiveDate;
use rusqlite::Connection;

pub struct SavedRecord {
    pub record_id: u32,
    pub client_id: u32,
    pub date: NaiveDate,
    pub time: u32,
    pub description: String,
}

pub struct Client {
    pub client_id: u32,
    pub client_name: String,
}

impl Client {
    pub fn fetch_all(conn: &Connection) -> Vec<Client> {
        let stmt = conn.execute("SELECT * FROM clients");
    }
}