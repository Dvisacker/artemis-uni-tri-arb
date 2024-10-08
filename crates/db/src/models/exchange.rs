use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::exchanges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbExchange {
    pub id: i32,
    pub chain: String,
    pub factory_address: Option<String>,
    pub exchange_name: String,
    pub exchange_type: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::exchanges)]
pub struct NewDbExchange {
    pub chain: String,
    pub factory_address: Option<String>,
    pub exchange_name: String,
    pub exchange_type: String,
}
