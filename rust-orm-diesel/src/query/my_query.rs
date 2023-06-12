use diesel::{pg::PgConnection};

use crate::models::{users::{NewUser, User}, countries::{Country}};

// Insert NewUser
pub fn new_user(mut conn: PgConnection, new_user: NewUser) {
    let result = User::create_user(&mut conn, &new_user);

    if result.is_ok() {
        let msg_user = result.unwrap();
        println!(
            "{} {} {} {}",
            msg_user.id, msg_user.name, msg_user.code_country, msg_user.number
        );
    }
}

// Select all User
pub fn select_users(mut conn: PgConnection) {
    let all_users = User::select_all(&mut conn);

    if let Ok(users) = all_users {
        for user in users {
            println!(
                "{} {} {} {}",
                user.id, user.name, user.code_country, user.number
            )
        }
    } else {
        println!("Error: {}", all_users.err().unwrap());
    }
}

// Select all users names
pub fn select_users_names(mut conn: PgConnection) {
    let all_users_names = User::select_all_names(&mut conn);

    if let Ok(names) = all_users_names {
        for name in names {
            println!("{}", name);
        }
    } else {
        println!("Error: {}", all_users_names.err().unwrap());
    }
}

pub fn select_all_users_and_countries(mut conn: PgConnection) {
  Country::get_join(&mut conn);
}
