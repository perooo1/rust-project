CREATE TABLE books(
    id SERIAL NOT NULL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    authors VARCHAR(255) NOT NULL,
    isbn VARCHAR(255) NOT NULL,
    language_code VARCHAR(255) NOT NULL,
    num_pages INTEGER NOT NULL,
    publication_date DATE NOT NULL,
    publisher VARCHAR(255) NOT NULL,
    is_loaned BOOLEAN NOT NULL DEFAULT false
);