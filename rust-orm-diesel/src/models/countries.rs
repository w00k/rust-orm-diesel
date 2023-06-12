use diesel::prelude::*;

use super::schema::{countries, users};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::models::schema::countries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Country {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub capital: String,
}

pub struct UserCountry {
    pub name: String,
    pub country_name: String,
    pub number: i32,
    pub capital: String,
}

impl From<(String, String, i32, String)> for UserCountry {
    fn from(tuple: (String, String, i32, String)) -> Self {
        UserCountry {
            name: tuple.0,
            country_name: tuple.1,
            number: tuple.2,
            capital: tuple.3,
        }
    }
}

impl Country {
    pub fn get_join(conn: &mut PgConnection) {
        println!("START: get join");

        let data: Vec<(String, String, i32, String)> = countries::table
            .inner_join(users::table.on(users::code_country.eq(countries::code)))
            .select((
                users::name,
                countries::name,
                users::number,
                countries::capital,
            ))
            .load(conn)
            .expect("Error loading data");

        for d in data {
            println!("{} {} {} {}", d.0, d.1, d.2, d.3);
        }

        println!("END: get join\n");
    }

    pub fn get_join_to_struct(conn: &mut PgConnection) -> Vec<UserCountry> {
        println!("\nSTART: get join to struct");

        let mut user_country_vec: Vec<UserCountry> = vec![];

        let data_vec: Result<Vec<(String, String, i32, String)>, diesel::result::Error> =
            countries::table
                .inner_join(users::table.on(users::code_country.eq(countries::code)))
                .select((
                    users::name,
                    countries::name,
                    users::number,
                    countries::capital,
                ))
                .get_results(conn);

        if data_vec.is_ok() {
            user_country_vec = data_vec
                .unwrap()
                .into_iter()
                .map(|tuple| tuple.into())
                .collect();
            println!("END: get join to struct\n");
            return user_country_vec;
        }
        println!("Error: get join to struct {}\n", data_vec.err().unwrap());
        return user_country_vec;
    }
}
