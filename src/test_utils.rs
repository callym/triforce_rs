use diesel::{prelude::*, Insertable, Queryable};
use lazy_static;
use std::{
  env,
  sync::{Condvar, Mutex},
};

lazy_static! {
  pub static ref INIT: (Mutex<bool>, Condvar) = { (Mutex::new(false), Condvar::new()) };
}

use crate::schema::{test_similarity, test_similarity_arr};

#[derive(Debug, Queryable, Insertable, Eq, PartialEq)]
#[table_name = "test_similarity"]
pub struct TestSimilarity {
  pub id: i32,
  pub test_case: String,
}

#[derive(Debug, Queryable, Insertable, Eq, PartialEq)]
#[table_name = "test_similarity_arr"]
pub struct TestSimilarityArr {
  pub id: i32,
  pub test_case: Vec<String>,
}

pub fn con_init() -> PgConnection {
  let db_url = env::var("DATABASE_URL").expect("failed to find the 'DATABASE_URL' env variable");

  PgConnection::establish(&db_url).expect("failed to connect to the db")
}

pub fn con() -> PgConnection {
  let &(ref lock, ref cvar) = &*INIT;

  let mut started = lock.lock().unwrap();
  while !*started {
    started = cvar.wait(started).unwrap();
  }

  con_init()
}
