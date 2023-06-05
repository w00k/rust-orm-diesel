use diesel::pg::PgConnection;

use crate::models::users::{NewUser, User};

pub fn new_user(mut conn: PgConnection, new_user: NewUser) {

  let result =  User::create_user(&mut conn, &new_user);
  
  if result.is_ok() {
    let msg_user = result.unwrap();
    println!("{} {} {} {}", msg_user.id, msg_user.name, msg_user.code_country, msg_user.number);
  }

}
