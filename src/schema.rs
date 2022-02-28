table! {
    member (id) {
        id -> Varchar,
        user_id -> Varchar,
        team_id -> Varchar,
    }
}

table! {
    team (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(member -> team (team_id));
joinable!(member -> users (user_id));

allow_tables_to_appear_in_same_query!(
    member,
    team,
    users,
);
