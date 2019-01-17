use diesel::sql_types::*;
/*
sql_function! {
  /// Returns a number that indicates how similar the two arguments are.
  /// The range of the result is zero (indicating that the two strings are completely dissimilar)
  /// to one (indicating that the two strings are identical).
  fn similarity(x: Text, y: Text) -> Float;
}

sql_function! {
  /// Returns an array of all the trigrams in the given string.
  /// (In practice this is seldom useful except for debugging.)
  fn show_trgm(x: Text) -> Array<Text>;
}

sql_function! {
  /// Returns a number that indicates the greatest similarity between the set of trigrams in the first string
  /// and any continuous extent of an ordered set of trigrams in the second string.
  /// For details, see the explanation below.
  fn word_similarity(x: Text, y: Text) -> Float;
}

sql_function! {
  /// Same as word_similarity(text, text), but forces extent boundaries to match word boundaries.
  /// Since we don't have cross-word trigrams, this function actually returns greatest similarity
  /// between first string and any continuous extent of words of the second string.
  fn strict_word_similarity(x: Text, y: Text) -> Float;
}

sql_function! {
  /// Returns the current similarity threshold used by the % operator.
  /// This sets the minimum similarity between two words for them to be considered similar enough to be misspellings of each other, for example (deprecated).
  #[deprecated]
  fn show_limit() -> Float;
}

sql_function! {
  /// Sets the current similarity threshold that is used by the % operator.
  /// The threshold must be between 0 and 1 (default is 0.3).
  /// Returns the same value passed in (deprecated).
  #[deprecated]
  fn set_limit(x: Float) -> Float;
}

sql_function! {
  /// Sets the current similarity threshold that is used by the % operator.
  /// The threshold must be between 0 and 1 (default is 0.3).
  /// Returns the same value passed in (deprecated).
  #[deprecated]
  fn set_limit(x: Float) -> Float;
}
*/
sql_function! {
  /// Concats all the strings in the array,
  /// using the supplied delimiter,
  /// so they can be used in a search query.
  #[sql_name="f_array_to_string"]
  fn array_to_string(x: Text, y: Array<Text>) -> Text;
}

macro_rules! concat_ws {
  ($name: ident, $($i: ident),+ $(,)?) => {
    sql_function! {
      /// Concats all the strings,
      /// using the supplied delimiter,
      /// so they can be used in a search query.
      #[sql_name="f_concat_ws"]
      fn $name(ws: Text $(, $i: Text)*) -> Text;
    }
  };
}

concat_ws!(concat_ws,     a, b);
concat_ws!(concat_ws_3,   a, b, c);
concat_ws!(concat_ws_4,   a, b, c, d);
concat_ws!(concat_ws_5,   a, b, c, d, e);
concat_ws!(concat_ws_6,   a, b, c, d, e, f);
concat_ws!(concat_ws_7,   a, b, c, d, e, f, g);
concat_ws!(concat_ws_8,   a, b, c, d, e, f, g, h);
concat_ws!(concat_ws_9,   a, b, c, d, e, f, g, h, i);
concat_ws!(concat_ws_10,  a, b, c, d, e, f, g, h, i, j);
concat_ws!(concat_ws_11,  a, b, c, d, e, f, g, h, i, j, k);
concat_ws!(concat_ws_12,  a, b, c, d, e, f, g, h, i, j, k, l);
concat_ws!(concat_ws_13,  a, b, c, d, e, f, g, h, i, j, k, l, m);
concat_ws!(concat_ws_14,  a, b, c, d, e, f, g, h, i, j, k, l, m, n);
concat_ws!(concat_ws_15,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o);
concat_ws!(concat_ws_16,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p);
concat_ws!(concat_ws_17,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q);
concat_ws!(concat_ws_18,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r);
concat_ws!(concat_ws_19,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s);
concat_ws!(concat_ws_20,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t);
concat_ws!(concat_ws_21,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);
concat_ws!(concat_ws_22,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v);
concat_ws!(concat_ws_23,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w);
concat_ws!(concat_ws_24,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x);
concat_ws!(concat_ws_25,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y);
concat_ws!(concat_ws_26,  a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z);
