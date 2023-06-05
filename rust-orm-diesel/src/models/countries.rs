use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::models::schema::countries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Countries {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub capital: String,
}