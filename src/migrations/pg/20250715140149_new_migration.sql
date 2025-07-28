-- +goose Up
-- +goose StatementBegin
INSERT INTO url (alias, url) VALUES
  ('alias1', 'https://example.com/1'),
  ('alias2', 'https://example.com/2'),
  ('alias3', 'https://example.com/3'),
  ('alias4', 'https://example.com/4'),
  ('alias5', 'https://example.com/5'),
  ('alias6', 'https://example.com/6'),
  ('alias7', 'https://example.com/7'),
  ('alias8', 'https://example.com/8'),
  ('alias9', 'https://example.com/9'),
  ('alias10', 'https://example.com/10');
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd
