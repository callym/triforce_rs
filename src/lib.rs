#[macro_use] extern crate diesel;

pub mod functions;
pub mod predicates;
pub mod types;

pub mod similar;

pub use functions::*;

pub fn init(conn: &diesel::PgConnection) -> diesel::QueryResult<()> {
  use diesel::connection::SimpleConnection;

  conn.batch_execute(include_str!("init.sql"))
}
