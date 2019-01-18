use crate::predicates::*;

use diesel::{
  expression::{AsExpression, Expression},
  sql_types::*,
};

pub trait TrgmQueryExtensions: Expression<SqlType = Text> + Sized {
  fn similarity<T: AsExpression<Text>>(self, other: T) -> Similarity<Self, T::Expression> {
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

impl<T: Expression<SqlType = Text>> TrgmQueryExtensions for T {}

#[test]
fn similarity_test() {
  use crate::schema::test_similarity::dsl::*;
  use crate::test_utils::*;
  use diesel::prelude::*;

  let test_string = "similar";

  let res = test_similarity
    .filter(test_case.distance(test_string).eq(0.0))
    .first::<TestSimilarity>(&con())
    .unwrap();

  assert_eq!(res.test_case, test_string);
}
