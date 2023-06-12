use diesel::prelude::*;

use super::{schema::{countries, users}};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::models::schema::countries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Country {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub capital: String,
}

impl Country {
    pub fn get_join(conn: &mut PgConnection) {
        let data: Vec<(String, String, i32, String)> = countries::table.inner_join(users::table.on(users::code_country.eq(countries::code))).select((users::name, countries::name, users::number, countries::capital)).load(conn).expect("Error loading data");

        for d in data {
            println!("{} {} {} {}", d.0, d.1, d.2, d.3);
        }
    }
}
