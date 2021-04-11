// Third party imports
use crate::db;
use argon2::{self, Config};
use diesel;
use diesel::prelude::*;

// Application components
use crate::sample::model::{Client, ClientAuth, NewClient};
use crate::schema::clients;

impl Client {
    pub fn find_all() -> Result<Vec<Self>, r2d2::Error> {
        let conn = db::connection()?;
        let clients_retrieved = clients::table.load::<Client>(&*conn);
        Ok(clients_retrieved.unwrap())
    }

    pub fn find_client(email: String) -> Result<Self, r2d2::Error> {
        let conn = db::connection()?;
        let client = clients::table
            .filter(clients::email.eq(email))
            .get_result(&*conn);

        Ok(client.unwrap())
    }

    pub fn auth(client: ClientAuth) -> Result<Client, argon2::Error> {
        let conn = db::connection().unwrap();
        let database_client = clients::table
            .filter(clients::email.eq(client.email))
            .get_result(&*conn);

        let client_data = match database_client {
            Ok(database_client) => Client::from(database_client),
            Err(e) => {
                debug!("{:?}", e);
                return Err(argon2::Error::IncorrectType);
            }
        };

        let matches = argon2::verify_encoded(&client_data.password, client.password.as_bytes());
        if matches.unwrap() {
            Ok(client_data)
        } else {
            Err(argon2::Error::DecodingFail)
        }
    }

    pub fn create_client(mut client: NewClient) -> Result<Self, r2d2::Error> {
        let conn = db::connection()?;
        let salt = b"randomsalt";
        let config = Config::default();
        client.password = argon2::hash_encoded(client.password.as_bytes(), salt, &config).unwrap();
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
