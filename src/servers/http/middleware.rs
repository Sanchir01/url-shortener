use axum::http::{Request, Response};
use std::convert::Infallible;
use std::task::{Context, Poll};
use tower::{Layer, Service};

// 1. Сам middleware-слой
#[derive(Clone)]
pub struct HelloSanchirLayer;

impl<S> Layer<S> for HelloSanchirLayer {
    type Service = HelloSanchirMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HelloSanchirMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct HelloSanchirMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for HelloSanchirMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<axum::body::Body>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    ReqBody: Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        println!("👋 Hello Sanchir!");
        self.inner.call(req)
    }
}
