CREATE EXTENSION IF NOT EXISTS "pg_trgm";

CREATE OR REPLACE FUNCTION f_concat_ws(text, VARIADIC text[])
  RETURNS text LANGUAGE sql IMMUTABLE AS 'SELECT array_to_string($2, $1)';

CREATE OR REPLACE FUNCTION f_array_to_string(text, text[])
  RETURNS text LANGUAGE sql IMMUTABLE AS 'SELECT array_to_string($2, $1)';
