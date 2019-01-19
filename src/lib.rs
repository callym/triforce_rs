#![cfg_attr(test, allow(proc_macro_derive_resolution_fallback))]

#[macro_use]
extern crate diesel;

pub mod functions;
pub mod predicates;
pub mod types;

pub mod similar;

pub use functions::*;
pub use predicates::*;
pub use similar::*;

pub fn init(conn: &diesel::PgConnection) -> diesel::QueryResult<()> {
  use diesel::connection::SimpleConnection;

  conn.batch_execute(include_str!("init.sql"))
}

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
pub mod schema;

#[test]
#[ignore]
pub fn init_test() {
  init(&test_utils::con()).expect("pg_trgm::init failed");
}
