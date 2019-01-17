use pg_trgm;

use cucumber_rust::steps;
use diesel::{
  prelude::*,
  sql_query,
};

use super::super::setup::Tests;

steps! {Tests => {
  given "Running init code" |_world, _step| { };

  when "I init the database" |world, _step| {
    pg_trgm::init(world.con())
      .expect("pg_trgm::init failed");
  };

  then "the function exists" |world, _step| {
    let res = diesel::select(pg_trgm::concat_ws(" ", "Hello,", "World!"))
      .first::<String>(world.con())
      .expect("");
    assert_eq!(res, "Hello, World!");

    let res = diesel::select(pg_trgm::array_to_string(" ", vec!["Hello", "long", "furby"]))
      .first::<String>(world.con())
      .expect("");
    assert_eq!(res, "Hello long furby");
  };
}}
