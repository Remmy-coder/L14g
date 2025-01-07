use chrono::Local;
use diesel::{
    query_dsl::methods::{FilterDsl, FindDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use nanoid::nanoid;
use validator::Validate;

use crate::{
    models::{Client, NewClient, UpdateClient},
    schema,
    utils::{hash_password, ErrorResponse},
};

pub fn create_client(
    conn: &mut SqliteConnection,
    new_client: &NewClient,
) -> Result<Client, ErrorResponse> {
    new_client.validate().map_err(|e| ErrorResponse {
        error: e.to_string(),
    })?;

    let id = nanoid!();
    let hashed_password = hash_password(new_client.password).map_err(|e| ErrorResponse {
        error: e.to_string(),
    })?;
    let now = Local::now().naive_local();

    let new_client = NewClient {
        id: &id,
        email: new_client.email,
        password: &hashed_password,
        role: new_client.role,
        created_at: Some(now),
        updated_at: Some(now),
    };

    let client = diesel::insert_into(schema::clients::table)
        .values(&new_client)
        .returning(Client::as_returning())
        .get_result(conn)
        .map_err(|e| ErrorResponse {
            error: e.to_string(),
        })?;

    Ok(client)
}

pub fn get_client(conn: &mut SqliteConnection, client_id: &str) -> Result<Client, ErrorResponse> {
    use crate::schema::clients::dsl::*;
    clients
        .find(client_id)
        .get_result::<Client>(conn)
        .map_err(|e| ErrorResponse {
            error: e.to_string(),
        })
}

pub fn get_client_email(
    conn: &mut SqliteConnection,
    client_email: &str,
) -> Result<Client, ErrorResponse> {
    use crate::schema::clients::dsl::*;
    clients
        .filter(email.eq(client_email))
        .first::<Client>(conn)
        .map_err(|e| ErrorResponse {
            error: e.to_string(),
        })
}

pub fn get_clients(conn: &mut SqliteConnection) -> Result<Vec<Client>, ErrorResponse> {
    use crate::schema::clients::dsl::*;
    clients.load::<Client>(conn).map_err(|e| ErrorResponse {
        error: e.to_string(),
    })
}

pub fn edit_client(
    conn: &mut SqliteConnection,
    client_id: &str,
    updated_client: &UpdateClient,
) -> Result<Client, ErrorResponse> {
    use crate::schema::clients::dsl::*;
    updated_client.validate().map_err(|e| ErrorResponse {
        error: e.to_string(),
    })?;

    let now = Local::now().naive_local();

    diesel::update(clients.find(client_id))
        .set((
            email.eq(&updated_client.email),
            role.eq(&updated_client.role),
            updated_at.eq(now),
        ))
        .get_result::<Client>(conn)
        .map_err(|e| ErrorResponse {
            error: e.to_string(),
        })
}
