use crate::predicates::*;

use diesel::{
  expression::{
    AsExpression,
    Expression,
  },
  sql_types::*,
};

pub trait TrgmQueryExtensions: Expression<SqlType=Text> + Sized {
  fn similar<T: AsExpression<Text>>(self, other: T) -> Similarity<Self, T::Expression> {
    Similarity::new(self, other.as_expression())
  }
/*
  fn similar_arr<T: AsExpression<Array<Text>>>(self, other: T) -> Similarity<Self, T::Expression> {
    Similarity::new(self, crate::functions::array_to_string(other))
  }*/

  fn distance<T: AsExpression<Text>>(self, other: T) -> Distance<Self, T::Expression> {
    Distance::new(self, other.as_expression())
  }
/*
  fn distance_arr<T: AsExpression<Array<Text>>>(self, other: T) -> Distance<Self, T::Expression> {
    Distance::new(self, crate::functions::array_to_string(other))
  }*/
}

impl<T: Expression<SqlType=Text>> TrgmQueryExtensions for T { }
