-- +goose Up
-- +goose StatementBegin
    INSERT INTO url (alias, url) VALUES ('test', 'http://site.com');
    INSERT INTO url (alias, url) VALUES ('test2', 'http://site.com');
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd
