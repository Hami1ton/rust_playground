use diesel::{prelude::*};

use crate::model::User;

pub async fn get_users(conn: &mut SqliteConnection) -> Vec<User> { 
    use crate::schema::user::dsl::user;

    let get_users = user
        .select(User::as_select())
        .load(conn);
    
    return get_users.expect("error");
}

pub async fn add_user(conn: &mut SqliteConnection, id: i32, name: String) {
    use crate::schema::user::dsl::user;

    let new_user = User {
        id: id,
        name: name.clone(),
    };

    let _ = diesel::insert_into(user).values(&new_user).execute(conn);
}
