use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode, response::Html, routing::get};
use minijinja::{Environment, context};
use tower_http::services::ServeDir;

struct AppState {
    env: Environment<'static>,
}

async fn index_handler(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let tmpl = state.env.get_template("index.html").unwrap();

    let rendered = tmpl
        .render(context! {
            message => "Hello, World!"
        })
        .unwrap();

    Ok(Html(rendered))
}

async fn about_handler(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let tmpl = state.env.get_template("about.html").unwrap();

    let rendered = tmpl
        .render(context! {
            message => "htmx is working!"
        })
        .unwrap();

    Ok(Html(rendered))
}

fn create_router() -> Router {
    let mut env = Environment::new();
    env.add_template("base.html", include_str!("../ui/templates/base.html"))
        .unwrap();
    env.add_template("index.html", include_str!("../ui/templates/index.html"))
        .unwrap();
    env.add_template("about.html", include_str!("../ui/templates/about.html"))
        .unwrap();

    let app_state = Arc::new(AppState { env });

    Router::new()
        .route("/", get(index_handler))
        .route("/about", get(about_handler))
        .nest_service("/dist", ServeDir::new("dist"))
        .with_state(app_state)
}

#[tokio::main]
async fn main() {
    let app = create_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
