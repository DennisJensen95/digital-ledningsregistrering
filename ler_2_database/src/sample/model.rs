#![allow(proc_macro_derive_resolution_fallback)]
use crate::schema::clients;
use chrono::NaiveDateTime;

// Database structs
#[derive(Debug, Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "clients"]
pub struct Client {
    pub id: i32,
    pub company: String,
    pub email: String,
    pub data_file: String,
    #[serde(skip)] // we're removing password from being show in the response
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "clients"]
pub struct NewClient {
    pub email: String,
    pub company: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientAuth {
    pub email: String,
    pub password: String,
}

// Upload download structs
#[derive(Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub time: u64,
    pub err: String,
}
