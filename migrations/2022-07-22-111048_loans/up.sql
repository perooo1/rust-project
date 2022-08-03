CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE loans(
    id VARCHAR(36) DEFAULT uuid_generate_v4() NOT NULL PRIMARY KEY,
    book_id INTEGER NOT NULL REFERENCES books(id),
    user_id VARCHAR(36) NOT NULL REFERENCES users(id),
    loan_date DATE NOT NULL DEFAULT NOW(),
    return_deadline DATE NOT NULL DEFAULT (NOW() + '14 DAYS'::interval),
    is_returned BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON loans
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();