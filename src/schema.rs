table! {
    friends (id) {
        id -> Int4,
        user_id -> Int4,
        friend_id -> Int4,
    }
}

table! {
    room_members (id) {
        id -> Int4,
        room_id -> Int4,
        member_id -> Int4,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        gender -> Nullable<Int2>,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    friends,
    room_members,
    rooms,
    users,
);
