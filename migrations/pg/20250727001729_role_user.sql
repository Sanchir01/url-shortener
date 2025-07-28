-- +goose Up
-- +goose StatementBegin

CREATE TYPE user_role AS ENUM ('admin', 'user', 'moderator');

ALTER TABLE users ADD COLUMN IF NOT EXISTS role user_role NOT NULL DEFAULT 'user';

-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE users DROP COLUMN IF EXISTS role;
DROP TYPE IF EXISTS user_role;
-- +goose StatementEnd
