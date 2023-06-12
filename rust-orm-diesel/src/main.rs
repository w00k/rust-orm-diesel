use crate::{
    models::users::NewUser,
    query::my_query::{new_user, select_all_users_and_countries, select_users, select_users_names, select_all_users_and_countries_to_struct},
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
    
}
