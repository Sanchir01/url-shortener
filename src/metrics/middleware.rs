use crate::metrics::PrometheusMetrics;
use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::{sync::Arc, time::Instant};

pub async fn metrics_middleware(
    State(metrics): State<Arc<PrometheusMetrics>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().to_string();
    let path = request.uri().path().to_string();

    // Увеличиваем счетчик активных запросов
    metrics.inc_requests_in_flight();

    // Обрабатываем запрос
    let response = next.run(request).await;

    // Уменьшаем счетчик активных запросов
    metrics.dec_requests_in_flight();

    // Записываем метрики
    let duration = start.elapsed().as_secs_f64();
    let status = response.status().as_u16();

    metrics.observe_http_request_duration(duration, &method, &path);
    metrics.inc_http_requests(&method, status, &path);

    // Если статус ошибки, записываем метрику ошибки
    if status >= 400 {
        let error_type = if status >= 500 {
            "server_error"
        } else {
            "client_error"
        };
        metrics.inc_errors(error_type, "http");
    }

    response
}

/// Handler для эндпоинта /metrics
pub async fn metrics_handler(State(metrics): State<Arc<PrometheusMetrics>>) -> Response {
    match metrics.export() {
        Ok(metrics_output) => Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain; version=0.0.4; charset=utf-8")
            .body(Body::from(metrics_output))
            .unwrap(),
        Err(err) => {
            log::error!("Failed to export metrics: {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to export metrics"))
                .unwrap()
        }
    }
}
