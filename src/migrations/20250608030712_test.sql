-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS metrics (
    id UUID DEFAULT generateUUIDv4(),
    url String,
    timestamp DateTime,
    status_code UInt32,
    response_time_ms UInt64,
    content_length UInt64,
    content_type String,
    performance_dns_lookup_time UInt64,
    performance_tcp_connect_time UInt64,
    performance_tls_handshake_time UInt64,
    performance_first_byte_time UInt64,
    performance_download_time UInt64,
    performance_total_time UInt64,
    seo_title String,
    seo_description String,
    seo_keywords Array(String),
    seo_h1_count UInt32,
    seo_h2_count UInt32,
    seo_img_count UInt32,
    seo_link_count UInt32,
    seo_has_robots_meta UInt8,
    error String
    )
    ENGINE = MergeTree()
    PARTITION BY toYYYYMM(timestamp)
    ORDER BY (timestamp, url)
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd
