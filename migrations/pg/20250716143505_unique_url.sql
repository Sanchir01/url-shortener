-- +goose Up
-- +goose StatementBegin
ALTER TABLE url
ADD CONSTRAINT url_url_unique UNIQUE (url);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd
