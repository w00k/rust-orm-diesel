use diesel::prelude::*;

use super::schema::users;

use super::schema::users::dsl::*;

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
    pub number: &'a i32,
}

impl User {
    pub fn create_user<'a>(
        conn: &mut PgConnection,
        user_new: &NewUser,
    ) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(user_new)
            .get_result::<User>(conn)
    }

    pub fn select_all<'a>(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        return users.load::<User>(conn);
    }

    pub fn select_all_names<'a>(
        conn: &mut PgConnection,
    ) -> Result<Vec<String>, diesel::result::Error> {
        return users.select(name).load::<String>(conn);
    }

    pub fn update<'a>(
        conn: &mut PgConnection,
        my_user: User,
    ) -> Result<User, diesel::result::Error> {
        let update = diesel::update(users.filter(users::id.eq(my_user.id)))
            .set((
                name.eq(my_user.name),
                code_country.eq(my_user.code_country),
                number.eq(my_user.number),
            ))
            .get_result(conn);

        return update;
    }
}
