use crate::{models::users::NewUser, query::my_query::new_user};

mod connection;
mod models;
mod query;


fn main() {
    println!("Init!");
    let mut _pool = connection::connection::establish_connection();

    // insert data
    let user_doo = NewUser {
        name: "John Doe",
        code_country: "US",
        number: &1000,
    };

    new_user(_pool, user_doo);

    // end insert

}
