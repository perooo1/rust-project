CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE loans(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
    book_id INTEGER NOT NULL REFERENCES books(id),
    user_id VARCHAR(36) NOT NULL REFERENCES users(id),
    loan_date DATE NOT NULL,
    return_deadline DATE NOT NULL,
    is_returned BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);