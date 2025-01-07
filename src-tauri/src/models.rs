use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use validator::Validate;

use crate::enums::ClientRole;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::clients)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Client {
    pub id: String,
    pub email: String,
    pub password: String,
    pub role: ClientRole,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Validate)]
#[diesel(table_name = crate::schema::clients)]
pub struct NewClient<'a> {
    pub id: &'a str,
    #[validate(email)]
    pub email: &'a str,
    #[validate(length(min = 8))]
    pub password: &'a str,
    pub role: &'a ClientRole,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset, Validate)]
#[diesel(table_name = crate::schema::clients)]
pub struct UpdateClient<'a> {
    #[validate(email)]
    pub email: &'a str,
    pub role: &'a ClientRole,
    pub updated_at: Option<NaiveDateTime>,
}
