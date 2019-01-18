-- Your SQL goes here
CREATE TABLE test_similarity (
  id SERIAL PRIMARY KEY,
  test_case TEXT NOT NULL
);

INSERT INTO test_similarity
  (test_case)
VALUES
  ('similar'),
  ('different'),
  ('this is a whole length of text'),
  ('simulacra'),
  ('this is a whole bunch of textiles')
;

CREATE TABLE test_similarity_arr (
  id SERIAL PRIMARY KEY,
  test_case TEXT[] NOT NULL
);

INSERT INTO test_similarity_arr
  (test_case)
VALUES
  ('{"similar", "different"}'),
  ('{"similar", "textiles"}'),
  ('{"different", "zebras"}')
;
