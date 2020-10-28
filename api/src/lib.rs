extern crate reqwest;
extern crate select;
extern crate serde_json;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::databases::diesel;

pub mod db;
pub mod genius;

#[database("typesync")]
pub struct DbPool(diesel::MysqlConnection);
