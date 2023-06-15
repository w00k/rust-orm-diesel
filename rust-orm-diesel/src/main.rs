use crate::{
    models::users::{NewUser, User},
    query::my_query::{new_user, select_all_users_and_countries, select_users, select_users_names, select_all_users_and_countries_to_struct, update_user, delete_user},
};

mod connection;
mod models;
mod query;

fn main() {
    println!("Start!");

    // insert data
    let mut pool = connection::connection::establish_connection();

    let user_doo = NewUser {
        name: "John Doe",
        code_country: "US",
        number: &1000,
    };

    new_user(pool, user_doo);

    // select all users
    pool = connection::connection::establish_connection();

    select_users(pool);

    // select all user names
    pool = connection::connection::establish_connection();

    select_users_names(pool);

    // inner_join
    pool = connection::connection::establish_connection();

    select_all_users_and_countries(pool);

    // inner_join
    pool = connection::connection::establish_connection();
    select_all_users_and_countries_to_struct(pool);

    // update
    let my_user = User {
        id: 7,
        name: "John Doe!!!!".to_owned(),
        code_country: "US".to_owned(),
        number: 50000,
    };
    pool = connection::connection::establish_connection();
    update_user(pool, my_user);

    // delete 
    let delete_id = 8;
    pool = connection::connection::establish_connection();
    delete_user(pool, delete_id);
    
}
