use diesel::prelude::*;

use super::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::models::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub code_country: String,
    pub number: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::models::schema::users)]
pub struct NewUser<'a> {
    pub name: &'a str, 
    pub code_country: &'a str, 
    pub number: &'a i32
}

impl User {
    pub fn create_user<'a> (
        conn: &mut PgConnection, 
        user_new: &NewUser) -> Result<User, diesel::result::Error> {
            diesel::insert_into(users::table).values(user_new).get_result::<User>(conn)
        }
}