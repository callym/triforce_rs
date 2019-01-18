use crate::functions::{array_to_string, ArrayToString};
use crate::predicates::*;

use diesel::{
  expression::{dsl::any, AsExpression, Expression},
  pg::expression::array_comparison::Any,
  sql_types::*,
};

type ArrayToStringTy<T> = ArrayToString<<&'static str as AsExpression<Text>>::Expression, T>;

pub trait TrgmQueryExtensions: Expression<SqlType = Text> + Sized {
  fn similar_to<T: AsExpression<Text>>(self, other: T) -> Similarity<Self, T::Expression> {
    Similarity::new(self, other.as_expression())
  }

  fn similar_to_arr<T: AsExpression<Array<Text>>>(
    self,
    other: T,
  ) -> Similarity<Self, ArrayToStringTy<T::Expression>> {
    Similarity::new(self, array_to_string(" ", other))
  }

  fn distance<T: AsExpression<Text>>(self, other: T) -> Distance<Self, T::Expression> {
    Distance::new(self, other.as_expression())
  }

  fn distance_arr<T: AsExpression<Array<Text>>>(
    self,
    other: T,
  ) -> Distance<Self, ArrayToStringTy<T::Expression>> {
    Distance::new(self, crate::functions::array_to_string(" ", other))
  }
}

impl<T: Expression<SqlType = Text>> TrgmQueryExtensions for T {}

pub trait TrgmQueryArrayExtensions: Expression<SqlType = Array<Text>> + Sized {
  fn similar_to_any<T: AsExpression<Text>>(self, other: T) -> Similarity<T::Expression, Any<Self>> {
    Similarity::new(other.as_expression(), any(self))
  }
}

impl<T: Expression<SqlType = Array<Text>>> TrgmQueryArrayExtensions for T {}

#[test]
fn similarity_test() {
  use crate::schema::test_similarity::dsl::*;
  use crate::test_utils::*;
  use diesel::prelude::*;

  let test_string = "similar";

  let res: Vec<_> = test_similarity
    .order_by(test_case.distance(test_string))
    .load::<TestSimilarity>(&con())
    .unwrap()
    .into_iter()
    .map(|r| r.test_case)
    .take(2)
    .collect();

  assert_eq!(res, vec!["similar", "simulacra"]);
}

#[test]
fn similarity_any_test() {
  use crate::schema::test_similarity_arr::dsl::*;
  use crate::test_utils::*;
  use diesel::prelude::*;

  let test_string = "similar";

  let res: Vec<_> = test_similarity_arr
    .filter(test_case.similar_to_any(test_string))
    .load::<TestSimilarityArr>(&con())
    .unwrap()
    .into_iter()
    .map(|r| r.test_case)
    .take(2)
    .collect();

  assert_eq!(
    res,
    vec![vec!["similar", "different"], vec!["similar", "textiles"]]
  );
}

#[test]
fn similarity_test_db_arr() {
  use crate::schema::test_similarity_arr::dsl::*;
  use crate::test_utils::*;
  use diesel::prelude::*;

  let test_string = "similar textiles";

  let res: Vec<_> = test_similarity_arr
    .filter(array_to_string(" ", test_case).similar_to(test_string))
    .load::<TestSimilarityArr>(&con())
    .unwrap()
    .into_iter()
    .map(|r| r.test_case)
    .take(2)
    .collect();

  assert_eq!(res, vec![vec!["similar", "textiles"]]);
}

#[test]
fn similarity_test_arr() {
  use crate::schema::test_similarity::dsl::*;
  use crate::test_utils::*;
  use diesel::prelude::*;

  let test_string = vec!["this", "is", "a", "whole", "length", "of", "text"];

  let res: Vec<_> = test_similarity
    .order_by(test_case.distance_arr(&test_string))
    .load::<TestSimilarity>(&con())
    .unwrap()
    .into_iter()
    .map(|r| r.test_case)
    .take(2)
    .collect();

  assert_eq!(
    res,
    vec![
      "this is a whole length of text",
      "this is a whole bunch of textiles"
    ]
  );
}
