// @generated automatically by Diesel CLI.

diesel::table! {
    albums (album_id) {
        album_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        artist_id -> Int4,
    }
}

diesel::table! {
    artists (artist_id) {
        artist_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    songs (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        album_id -> Int4,
    }
}

diesel::joinable!(albums -> artists (artist_id));
diesel::joinable!(songs -> albums (album_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    songs,
);
