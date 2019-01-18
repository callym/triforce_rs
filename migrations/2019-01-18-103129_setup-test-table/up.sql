-- Your SQL goes here
CREATE TABLE test_similarity (
  id SERIAL PRIMARY KEY,
  test_case TEXT NOT NULL
);

INSERT INTO test_similarity
  (test_case)
VALUES
  ('similar'),
  ('different')
;
