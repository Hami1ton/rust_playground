use diesel::{prelude::*};

use crate::model::User;

pub async fn get_users(conn: &mut SqliteConnection) -> Vec<User> { 
    use crate::schema::user::dsl::user;

    let get_users = user
        .select(User::as_select())
        .load(conn);
    // println!("{:?}", get_users);
    
    return get_users.expect("error");
}
