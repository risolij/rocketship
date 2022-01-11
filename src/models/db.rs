use diesel::PgConnection;

#[database("rocket_db")]
pub struct DbPool(PgConnection);
