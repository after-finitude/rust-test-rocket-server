table! {
  users (id) {
      id -> Int4,
      username -> Text,
      email -> Text,
      bio -> Nullable<Text>,
      image -> Nullable<Text>,
      hash -> Text,
  }
}

allow_tables_to_appear_in_same_query!(users,);
