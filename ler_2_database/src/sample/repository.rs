#![allow(proc_macro_derive_resolution_fallback)]
use crate::sample::model::Client;
use crate::sample::model::NewClient;
use crate::schema::clients;
use crate::schema::clients::dsl::*;
use diesel;
use diesel::prelude::*;
pub fn create_client(new_client: NewClient, conn: &PgConnection) -> QueryResult<Client> {
    return diesel::insert_into(clients::table)
        .values(&new_client)
        .get_result(conn);
}
pub fn show_clients(connection: &PgConnection) -> QueryResult<Vec<Client>> {
    //posts.filter(published.eq(true))
    return clients.limit(5).load::<Client>(&*connection);
}
pub fn get_client(post_id: i32, connection: &PgConnection) -> QueryResult<Client> {
    return clients::table
        .find(post_id)
        .get_result::<Client>(connection);
}
pub fn update_client(
    client_id: i32,
    client: Client,
    connection: &PgConnection,
) -> QueryResult<Client> {
    return diesel::update(clients::table.find(client_id))
        .set(&client)
        .get_result(connection);
}
pub fn delete_client(client_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    return diesel::delete(clients::table.find(client_id)).execute(connection);
}
