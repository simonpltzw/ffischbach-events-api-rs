// @generated automatically by Diesel CLI.

diesel::table! {
    EventManagers (Id) {
        Id -> Int4,
        ManagerId -> Int4,
        #[max_length = 20]
        EventId -> Varchar,
        CreatedAt -> Timestamptz,
    }
}

diesel::table! {
    Events (Id) {
        #[max_length = 20]
        Id -> Varchar,
        #[max_length = 1000]
        Description -> Nullable<Varchar>,
        PublicKey -> Text,
        CreatedBy -> Text,
        CreatedAt -> Timestamptz,
        Completed -> Bool,
        EncryptedPrivateKey -> Text,
    }
}

diesel::table! {
    Groups (Id) {
        Id -> Int4,
        Category -> Int4,
        Approved -> Nullable<Bool>,
        #[max_length = 20]
        EventId -> Varchar,
        CreatedAt -> Timestamptz,
        EncryptedName -> Bytea,
        #[max_length = 64]
        HashedName -> Varchar,
    }
}

diesel::table! {
    Managers (Id) {
        Id -> Int4,
        #[max_length = 100]
        Email -> Varchar,
        CreatedAt -> Timestamptz,
    }
}

diesel::table! {
    Participants (Id) {
        Id -> Int4,
        EncryptedData -> Bytea,
        IsContact -> Bool,
        VIP -> Nullable<Bool>,
        GroupId -> Int4,
        CreatedAt -> Timestamptz,
    }
}

diesel::joinable!(EventManagers -> Events (EventId));
diesel::joinable!(EventManagers -> Managers (ManagerId));
diesel::joinable!(Groups -> Events (EventId));
diesel::joinable!(Participants -> Groups (GroupId));

diesel::allow_tables_to_appear_in_same_query!(
    EventManagers,
    Events,
    Groups,
    Managers,
    Participants,
);
