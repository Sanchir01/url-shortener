-- +goose Up
-- +goose StatementBegin
CREATE INDEX IF NOT EXISTS  idx_alias ON url(alias);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd
