table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        authors -> Varchar,
        isbn -> Varchar,
        language_code -> Varchar,
        num_pages -> Int4,
        publication_date -> Date,
        publisher -> Varchar,
        is_loaned -> Bool,
    }
}

table! {
    loans (id) {
        id -> Varchar,
        book_id -> Int4,
        user_id -> Varchar,
        loan_date -> Date,
        return_deadline -> Date,
        is_returned -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        pass -> Varchar,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(loans -> books (book_id));
joinable!(loans -> users (user_id));

allow_tables_to_appear_in_same_query!(
    books,
    loans,
    users,
);
