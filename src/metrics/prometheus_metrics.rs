use prometheus::{
    CounterVec, Encoder, HistogramOpts, HistogramVec, IntCounter, IntGauge, Opts, Registry,
    TextEncoder,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct PrometheusMetrics {
    pub registry: Arc<Registry>,
    pub http_requests_total: CounterVec,
    pub http_request_duration_seconds: HistogramVec,
    pub http_requests_in_flight: IntGauge,
    pub database_connections_active: IntGauge,
    pub url_shortening_total: IntCounter,
    pub url_redirects_total: IntCounter,
    pub telegram_messages_processed: IntCounter,
    pub errors_total: CounterVec,
}

impl PrometheusMetrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Arc::new(Registry::new());

        // HTTP метрики
        let http_requests_total = CounterVec::new(
            Opts::new("http_requests_total", "Total number of HTTP requests")
                .namespace("url_shortener"),
            &["method", "status", "path"],
        )?;

        let http_request_duration_seconds = HistogramVec::new(
            HistogramOpts::new(
                "http_request_duration_seconds",
                "HTTP request duration in seconds",
            )
            .namespace("url_shortener")
            .buckets(vec![
                0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ]),
            &["method", "path"],
        )?;

        let http_requests_in_flight = IntGauge::with_opts(
            Opts::new(
                "http_requests_in_flight",
                "Number of HTTP requests currently being processed",
            )
            .namespace("url_shortener"),
        )?;

        // База данных метрики
        let database_connections_active = IntGauge::with_opts(
            Opts::new(
                "database_connections_active",
                "Number of active database connections",
            )
            .namespace("url_shortener"),
        )?;

        // Бизнес метрики
        let url_shortening_total = IntCounter::with_opts(
            Opts::new("url_shortening_total", "Total number of URLs shortened")
                .namespace("url_shortener"),
        )?;

        let url_redirects_total = IntCounter::with_opts(
            Opts::new("url_redirects_total", "Total number of URL redirects")
                .namespace("url_shortener"),
        )?;

        let telegram_messages_processed = IntCounter::with_opts(
            Opts::new(
                "telegram_messages_processed_total",
                "Total number of Telegram messages processed",
            )
            .namespace("url_shortener"),
        )?;

        // Ошибки
        let errors_total = CounterVec::new(
            Opts::new("errors_total", "Total number of errors").namespace("url_shortener"),
            &["error_type", "component"],
        )?;

        // Регистрируем все метрики
        registry.register(Box::new(http_requests_total.clone()))?;
        registry.register(Box::new(http_request_duration_seconds.clone()))?;
        registry.register(Box::new(http_requests_in_flight.clone()))?;
        registry.register(Box::new(database_connections_active.clone()))?;
        registry.register(Box::new(url_shortening_total.clone()))?;
        registry.register(Box::new(url_redirects_total.clone()))?;
        registry.register(Box::new(telegram_messages_processed.clone()))?;
        registry.register(Box::new(errors_total.clone()))?;

        Ok(Self {
            registry,
            http_requests_total,
            http_request_duration_seconds,
            http_requests_in_flight,
            database_connections_active,
            url_shortening_total,
            url_redirects_total,
            telegram_messages_processed,
            errors_total,
        })
    }

    /// Экспортирует метрики в формате Prometheus
    pub fn export(&self) -> Result<String, prometheus::Error> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer).unwrap())
    }

    /// Увеличивает счетчик HTTP запросов
    pub fn inc_http_requests(&self, method: &str, status: u16, path: &str) {
        self.http_requests_total
            .with_label_values(&[method, &status.to_string(), path])
            .inc();
    }

    /// Записывает время выполнения HTTP запроса
    pub fn observe_http_request_duration(&self, duration: f64, method: &str, path: &str) {
        self.http_request_duration_seconds
            .with_label_values(&[method, path])
            .observe(duration);
    }

    /// Увеличивает/уменьшает количество запросов в обработке
    pub fn inc_requests_in_flight(&self) {
        self.http_requests_in_flight.inc();
    }

    pub fn dec_requests_in_flight(&self) {
        self.http_requests_in_flight.dec();
    }

    /// Записывает количество активных соединений с БД
    pub fn set_database_connections(&self, count: i64) {
        self.database_connections_active.set(count);
    }

    /// Увеличивает счетчик сокращенных URL
    pub fn inc_url_shortening(&self) {
        self.url_shortening_total.inc();
    }

    /// Увеличивает счетчик переходов по сокращенным URL
    pub fn inc_url_redirects(&self) {
        self.url_redirects_total.inc();
    }

    /// Увеличивает счетчик обработанных сообщений Telegram
    pub fn inc_telegram_messages(&self) {
        self.telegram_messages_processed.inc();
    }

    /// Записывает ошибку
    pub fn inc_errors(&self, error_type: &str, component: &str) {
        self.errors_total
            .with_label_values(&[error_type, component])
            .inc();
    }
}

impl Default for PrometheusMetrics {
    fn default() -> Self {
        Self::new().expect("Failed to create Prometheus metrics")
    }
}
