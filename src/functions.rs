#![allow(proc_macro_derive_resolution_fallback)]
use diesel::sql_types::*;

sql_function! {
  /// Returns a number that indicates how similar the two arguments are.
  /// The range of the result is zero (indicating that the two strings are completely dissimilar)
  /// to one (indicating that the two strings are identical).
  fn similarity(x: Text, y: Text) -> Float;
}

pub type Similarity<X, Y> = similarity::HelperType<X, Y>;

sql_function! {
  /// Returns an array of all the trigrams in the given string.
  /// (In practice this is seldom useful except for debugging.)
  fn show_trgm(x: Text) -> Array<Text>;
}

pub type ShowTrgm<X> = show_trgm::HelperType<X>;

#[test]
fn show_trgm_test() {
  use crate::test_utils::con;
  use diesel::{self, prelude::*};

  let res = diesel::select(show_trgm("trigram"))
    .get_result::<Vec<String>>(&con())
    .unwrap();

  for tri in vec!["  t", " tr", "tri", "rig", "igr", "gra", "ram", "am "] {
    assert!(res.contains(&String::from(tri)));
  }
}

sql_function! {
  /// Returns a number that indicates the greatest similarity between the set of trigrams in the first string
  /// and any continuous extent of an ordered set of trigrams in the second string.
  /// For details, see the explanation below.
  fn word_similarity(x: Text, y: Text) -> Float;
}

pub type WordSimilarity<X, Y> = word_similarity::HelperType<X, Y>;

sql_function! {
  /// Same as word_similarity(text, text), but forces extent boundaries to match word boundaries.
  /// Since we don't have cross-word trigrams, this function actually returns greatest similarity
  /// between first string and any continuous extent of words of the second string.
  fn strict_word_similarity(x: Text, y: Text) -> Float;
}

pub type StrictWordSimilarity<X, Y> = strict_word_similarity::HelperType<X, Y>;

no_arg_sql_function!(show_limit, Float, "Returns the current similarity threshold used by the % operator. This sets the minimum similarity between two words for them to be considered similar enough to be misspellings of each other, for example (deprecated).");

#[test]
fn show_limit_test() {
  use crate::test_utils::con;
  use diesel::{self, prelude::*};

  let res = diesel::select(show_limit)
    .get_result::<f32>(&con())
    .unwrap();

  assert_eq!(res, 0.3);
}

sql_function! {
  /// Sets the current similarity threshold that is used by the % operator.
  /// The threshold must be between 0 and 1 (default is 0.3).
  /// Returns the same value passed in (deprecated).
  #[deprecated]
  fn set_limit(x: Float) -> Float;
}

pub type SetLimit<X> = set_limit::HelperType<X>;

#[test]
fn set_limit_test() {
  #![allow(deprecated)]
  use crate::test_utils::con;
  use diesel::{self, prelude::*};

  let default = diesel::select(show_limit)
    .get_result::<f32>(&con())
    .unwrap();

  let test = 0.55;

  let res = diesel::select(set_limit(test))
    .get_result::<f32>(&con())
    .unwrap();

  assert_eq!(res, test);

  diesel::select(set_limit(default)).execute(&con()).unwrap();
}

sql_function! {
  /// Concats all the strings in the array,
  /// using the supplied delimiter,
  /// so they can be used in a search query.
  #[sql_name="f_concat_ws"]
  fn concat_null(x: Text, y: Nullable<Text>) -> Text;
}

pub type ConcatNull<X, Y> = concat_null::HelperType<X, Y>;

#[test]
fn concat_null_test_null() {
  use crate::test_utils::con;
  use diesel::{self, prelude::*};

  let res = diesel::select(concat_null("", Option::<&'static str>::None))
    .first::<String>(&con())
    .unwrap();

  assert_eq!(res, "");
}

#[test]
fn concat_null_test_some() {
  use crate::test_utils::con;
  use diesel::{self, prelude::*};

  let res = diesel::select(concat_null("", Some("Hello, World!")))
    .first::<String>(&con())
    .unwrap();

  assert_eq!(res, "Hello, World!");
}

sql_function! {
  /// Concats all the strings in the array,
  /// using the supplied delimiter,
  /// so they can be used in a search query.
  #[sql_name="f_array_to_string"]
  fn array_to_string(x: Text, y: Array<Text>) -> Text;
}

pub type ArrayToString<X, Y> = array_to_string::HelperType<X, Y>;

#[test]
fn array_to_string_test() {
  use crate::test_utils::con;
  use diesel::{self, prelude::*};

  let res = diesel::select(array_to_string(" ", vec!["Hello", "long", "furby"]))
    .first::<String>(&con())
    .unwrap();

  assert_eq!(res, "Hello long furby");
}

macro_rules! concat_ws {
  ($ty_name: ident, $name: ident, $($i: ident),+| $($ty_i: ident),+) => {
    sql_function! {
      /// Concats all the strings,
      /// using the supplied delimiter,
      /// so they can be used in a search query.
      #[sql_name="f_concat_ws"]
      fn $name(ws: Text $(, $i: Text)*) -> Text;
    }

    #[allow(non_camel_case_types)]
    pub type $ty_name<WS $(, $ty_i)*> = $name::HelperType<WS $(, $ty_i)*>;

  };
}

#[test]
fn concat_ws_test() {
  use crate::test_utils::con;
  use diesel::{self, prelude::*};

  let res = diesel::select(concat_ws(" ", "Hello,", "World!"))
    .first::<String>(&con())
    .unwrap();

  assert_eq!(res, "Hello, World!");
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod concat_ws_mod {
  use diesel::sql_types::*;

  concat_ws!(ConcatWs,   concat_ws,    a,  b| a,  b);
  concat_ws!(ConcatWs3,  concat_ws_3,  a, b, c| a, b, c);
  concat_ws!(ConcatWs4,  concat_ws_4,  a, b, c, d| a, b, c, d);
  concat_ws!(ConcatWs5,  concat_ws_5,  a, b, c, d, e| a, b, c, d, e);
  concat_ws!(ConcatWs6,  concat_ws_6,  a, b, c, d, e, f| a, b, c, d, e, f);
  concat_ws!(ConcatWs7,  concat_ws_7,  a, b, c, d, e, f, g| a, b, c, d, e, f, g);
  concat_ws!(ConcatWs8,  concat_ws_8,  a, b, c, d, e, f, g, h| a, b, c, d, e, f, g, h);
  concat_ws!(ConcatWs9,  concat_ws_9,  a, b, c, d, e, f, g, h, i| a, b, c, d, e, f, g, h, i);
  concat_ws!(ConcatWs10, concat_ws_10, a, b, c, d, e, f, g, h, i, j| a, b, c, d, e, f, g, h, i, j);
  concat_ws!(ConcatWs11, concat_ws_11, a, b, c, d, e, f, g, h, i, j, k| a, b, c, d, e, f, g, h, i, j, k);
  concat_ws!(ConcatWs12, concat_ws_12, a, b, c, d, e, f, g, h, i, j, k, l| a, b, c, d, e, f, g, h, i, j, k, l);
  concat_ws!(ConcatWs13, concat_ws_13, a, b, c, d, e, f, g, h, i, j, k, l, m| a, b, c, d, e, f, g, h, i, j, k, l, m);
  concat_ws!(ConcatWs14, concat_ws_14, a, b, c, d, e, f, g, h, i, j, k, l, m, n| a, b, c, d, e, f, g, h, i, j, k, l, m, n);
  concat_ws!(ConcatWs15, concat_ws_15, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o);
  concat_ws!(ConcatWs16, concat_ws_16, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p);
  concat_ws!(ConcatWs17, concat_ws_17, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q);
  concat_ws!(ConcatWs18, concat_ws_18, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r);
  concat_ws!(ConcatWs19, concat_ws_19, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s);
  concat_ws!(ConcatWs20, concat_ws_20, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t);
  concat_ws!(ConcatWs21, concat_ws_21, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);
  concat_ws!(ConcatWs22, concat_ws_22, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v);
  concat_ws!(ConcatWs23, concat_ws_23, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w);
  concat_ws!(ConcatWs24, concat_ws_24, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x);
  concat_ws!(ConcatWs25, concat_ws_25, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y);
  concat_ws!(ConcatWs26, concat_ws_26, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z| a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z);
}

pub use concat_ws_mod::*;
