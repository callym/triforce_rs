use std::{
  default::Default,
  env,
  ops::Drop,
  process::Command,
};

use cucumber_rust::World;
use dotenv::dotenv;
use diesel::prelude::*;

pub struct Tests {
  con: Option<PgConnection>,
}

impl Tests {
  pub fn con(&self) -> &PgConnection {
    self.con.as_ref().unwrap()
  }
}

impl World for Tests { }

impl Default for Tests {
  fn default() -> Self {
    dotenv().ok();

    Command::new("docker-compose")
      .arg("up")
      .arg("-d")
      .spawn()
      .expect("'docker-compose up' failed to run")
      .wait()
      .expect("'docker-compose up' exited with an error");

    std::thread::sleep_ms(5_000);

    let con = PgConnection::establish(&env::var("DATABASE_URL")
      .expect("failed to find the 'DATABASE_URL' env variable"))
      .or_else(|e| {
        //down();
        Err(e)
      })
      .expect("failed to connect to the db");

    let con = Some(con);

    Self {
      con,
    }
  }
}

fn down() {
  Command::new("docker-compose")
    .arg("down")
    .spawn()
    .expect("'docker-compose down' failed to run")
    .wait()
    .expect("'docker-compose down' exited with an error");
}

impl Drop for Tests {
  fn drop(&mut self) {
    let con = self.con.take();
    drop(con);

    down();
  }
}
