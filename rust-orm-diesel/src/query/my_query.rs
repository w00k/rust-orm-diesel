use diesel::{pg::PgConnection};

use crate::models::{
    countries::Country,
    users::{NewUser, User},
};

// Insert NewUser
pub fn new_user(mut conn: PgConnection, new_user: NewUser) {
    println!("START: new user");

    let result = User::create_user(&mut conn, &new_user);

    if result.is_ok() {
        let msg_user = result.unwrap();
        println!(
            "{} {} {} {}",
            msg_user.id, msg_user.name, msg_user.code_country, msg_user.number
        );

        println!("END: new user");
    } else {
        println!("Error: new user {}\n", result.err().unwrap());
    }
}

// Select all User
pub fn select_users(mut conn: PgConnection) {
    println!("START: select all users");

    let all_users = User::select_all(&mut conn);

    if let Ok(users) = all_users {
        for user in users {
            println!(
                "{} {} {} {}",
                user.id, user.name, user.code_country, user.number
            )
        }

        print!("END: select all users\n");
    } else {
        println!("Error: select all users {}\n", all_users.err().unwrap());
    }
}

// Select all users names
pub fn select_users_names(mut conn: PgConnection) {
    println!("START: select names users");

    let all_users_names = User::select_all_names(&mut conn);

    if let Ok(names) = all_users_names {
        for name in names {
            println!("{}", name);
        }
        println!("END: select names users\n");
    } else {
        println!(
            "Error: select names users {}\n",
            all_users_names.err().unwrap()
        );
    }
}

pub fn select_all_users_and_countries(mut conn: PgConnection) {
    Country::get_join(&mut conn);
}

pub fn select_all_users_and_countries_to_struct(mut conn: PgConnection) {
    println!("START: select all users with countries to struct");
    let user_country_vec = Country::get_join_to_struct(&mut conn);
    for user_country in user_country_vec {
        println!(
            "{} {} {} {}",
            user_country.name, user_country.country_name, user_country.number, user_country.capital
        );
    }
    println!("END: select all users with countries to struct\n");
}

pub fn update_user(mut conn: PgConnection, my_user: User) {
    println!("START: update user");
    let update_row = User::update(&mut conn, my_user);

    if update_row.is_ok() {
        let user_updated = update_row.unwrap();
        println!("update_user {} {} {} {}", user_updated.id, user_updated.name, user_updated.code_country, user_updated.number);
        println!("END: update user\n");
    } else {
        println!("Error: update user {}\n", update_row.err().unwrap());
    } 
}

pub fn delete_user(mut conn: PgConnection, delete_id: i32) {
    println!("START: delete user");
    let rows_deleted = User::delete(&mut conn, delete_id);
    println!("rows affected {}", rows_deleted);
    println!("START: delete user");
}
