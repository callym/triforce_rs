table! {
    test_similarity (id) {
        id -> Int4,
        test_case -> Text,
    }
}

table! {
    test_similarity_arr (id) {
        id -> Int4,
        test_case -> Array<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    test_similarity,
    test_similarity_arr,
);
