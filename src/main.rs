mod args;
mod cli;
mod scanner;
mod stdout;

use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fm_dns=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .context("error while starting server")?;

    //Cli::run().await;

    //let output = Arc::new(OutputPort::default());

    //let (_actor, handle) = Actor::spawn(None, Scanner, output.clone())
    //    .await
    //    .expect("Failed to start ping-pong actor");

    //let (_actor, handle) = Actor::spawn(None, StdoutPublisher, output)
    //    .await
    //    .expect("Failed to start ping-pong actor");
    //handle
    //    .await
    //    .expect("Ping-pong actor failed to exit properly");
    Ok(())
}

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
