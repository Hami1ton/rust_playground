use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::user;

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable, Insertable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
}

