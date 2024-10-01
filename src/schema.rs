// @generated automatically by Diesel CLI.

diesel::table! {
    #[sql_name = "EventManagers"]
    event_managers (id) {
        #[sql_name = "Id"]
        id -> Int4,
        #[sql_name = "ManagerId"]
        manager_id -> Int4,
        #[sql_name = "EventId"]
        #[max_length = 20]
        event_id -> Varchar,
        #[sql_name = "CreatedAt"]
        created_at -> Timestamptz,
    }
}

diesel::table! {
    #[sql_name = "Events"]
    events (id) {
        #[sql_name = "Id"]
        #[max_length = 20]
        id -> Varchar,
        #[sql_name = "Description"]
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        #[sql_name = "PublicKey"]
        public_key -> Text,
        #[sql_name = "CreatedBy"]
        created_by -> Text,
        #[sql_name = "CreatedAt"]
        created_at -> Timestamptz,
        #[sql_name = "Completed"]
        completed -> Bool,
        #[sql_name = "EncryptedPrivateKey"]
        encrypted_private_key -> Text,
    }
}

diesel::table! {
    #[sql_name = "Groups"]
    groups (id) {
        #[sql_name = "Id"]
        id -> Int4,
        #[sql_name = "Category"]
        category -> Int4,
        #[sql_name = "Approved"]
        approved -> Nullable<Bool>,
        #[sql_name = "EventId"]
        #[max_length = 20]
        event_id -> Varchar,
        #[sql_name = "CreatedAt"]
        created_at -> Timestamptz,
        #[sql_name = "EncryptedName"]
        encrypted_name -> Bytea,
        #[sql_name = "HashedName"]
        #[max_length = 64]
        hashed_name -> Varchar,
    }
}

diesel::table! {
    #[sql_name = "Managers"]
    managers (id) {
        #[sql_name = "Id"]
        id -> Int4,
        #[sql_name = "Email"]
        #[max_length = 100]
        email -> Varchar,
        #[sql_name = "CreatedAt"]
        created_at -> Timestamptz,
    }
}

diesel::table! {
    #[sql_name = "Participants"]
    participants (id) {
        #[sql_name = "Id"]
        id -> Int4,
        #[sql_name = "EncryptedData"]
        encrypted_data -> Bytea,
        #[sql_name = "IsContact"]
        is_contact -> Bool,
        #[sql_name = "VIP"]
        vip -> Nullable<Bool>,
        #[sql_name = "GroupId"]
        group_id -> Int4,
        #[sql_name = "CreatedAt"]
        created_at -> Timestamptz,
    }
}

diesel::joinable!(event_managers -> events (event_id));
diesel::joinable!(event_managers -> managers (manager_id));
diesel::joinable!(groups -> events (event_id));
diesel::joinable!(participants -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    event_managers,
    events,
    groups,
    managers,
    participants,
);
