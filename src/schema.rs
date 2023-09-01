// @generated automatically by Diesel CLI.

diesel::table! {
    movement (id) {
        id -> Uuid,
        amount -> Numeric,
        description -> Nullable<Varchar>,
        #[max_length = 6]
        effect -> Nullable<Varchar>,
        account_id -> Uuid,
    }
}
