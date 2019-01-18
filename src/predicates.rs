#![allow(proc_macro_derive_resolution_fallback)]

use diesel::{pg::Pg, sql_types::*};

diesel_infix_operator!(Similarity, " % ", backend: Pg);

diesel_infix_operator!(SimilarityWord, " <% ", backend: Pg);
diesel_infix_operator!(SimilarityWordCom, " %> ", backend: Pg);

diesel_infix_operator!(SimilarityStrictWord, " <<% ", backend: Pg);
diesel_infix_operator!(SimilarityStrictWordCom, " %>> ", backend: Pg);

diesel_infix_operator!(Distance, " <-> ", Float, backend: Pg);

diesel_infix_operator!(DistanceWord, " <<-> ", Float, backend: Pg);
diesel_infix_operator!(DistanceWordCom, " <->> ", Float, backend: Pg);

diesel_infix_operator!(DistanceStrictWord, " <<<-> ", Float, backend: Pg);
diesel_infix_operator!(DistanceStrictWordCom, " <->>> ", Float, backend: Pg);
