// Third party imports
use crate::db;
use diesel;
use diesel::prelude::*;

// Application components
use crate::sample::model::{Client, NewClient};
use crate::schema::clients;

impl Client {
    pub fn find_all() -> Result<Vec<Self>, r2d2::Error> {
        let conn = db::connection()?;
        let clients_retrieved = clients::table.load::<Client>(&*conn);
        Ok(clients_retrieved.unwrap())
    }

    pub fn create_client(client: NewClient) -> Result<Self, r2d2::Error> {
        let conn = db::connection()?;
        let client = NewClient::from(client);
        let client = diesel::insert_into(clients::table)
            .values(client)
            .get_result(&*conn);
        Ok(client.unwrap())
    }

    pub fn update(id: i32, client: Client) -> Result<Self, r2d2::Error> {
        let conn = db::connection()?;
        let client = diesel::update(clients::table)
            .filter(clients::id.eq(id))
            .set(client)
            .get_result(&*conn);
        Ok(client.unwrap())
    }

    pub fn delete(id: i32) -> Result<usize, r2d2::Error> {
        let conn = db::connection()?;
        let res = diesel::delete(clients::table.filter(clients::id.eq(id))).execute(&*conn);
        Ok(res.unwrap())
    }
}
