
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::project)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Movement {
    pub id: uuid::uuid,
    amount: f64,
    description: String,
    effect: String,
    account_id: String
}

