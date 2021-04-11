table! {
    accounts (id) {
        id -> Int4,
        compound_id -> Varchar,
        user_id -> Int4,
        provider_type -> Varchar,
        provider_id -> Varchar,
        provider_account_id -> Varchar,
        refresh_token -> Nullable<Text>,
        access_token -> Nullable<Text>,
        access_token_expires -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    clients (id) {
        id -> Int4,
        company -> Varchar,
        email -> Text,
        data_file -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        expires -> Timestamptz,
        session_token -> Varchar,
        access_token -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        email_verified -> Nullable<Timestamptz>,
        image -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    verification_requests (id) {
        id -> Int4,
        identifier -> Varchar,
        token -> Varchar,
        expires -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    clients,
    sessions,
    users,
    verification_requests,
);
