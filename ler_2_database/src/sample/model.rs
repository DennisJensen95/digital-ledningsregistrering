#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::clients;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "clients"]
pub struct Client {
    pub id: i32,
    pub user_name: String,
    pub data_file: String,
}
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "clients"]
pub struct NewClient {
    pub user_name: String,
    pub data_file: String,
}
