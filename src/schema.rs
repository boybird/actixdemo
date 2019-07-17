table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
