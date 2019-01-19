use diesel::{prelude::*, Insertable, Queryable};
use std::env;

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

pub fn con() -> PgConnection {
  let db_url = env::var("DATABASE_URL").expect("failed to find the 'DATABASE_URL' env variable");

  PgConnection::establish(&db_url).expect("failed to connect to the db")
}
