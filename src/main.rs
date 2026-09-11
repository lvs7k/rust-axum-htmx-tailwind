use std::{fs, path::Path, sync::Arc};

use axum::{Router, extract::State, http::StatusCode, response::Html, routing::get};
use minijinja::{Environment, Value, context};
use tower_http::services::ServeDir;

struct AppState {
    env: Environment<'static>,
}

async fn index_handler(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    render(
        &state,
        "index.html",
        context! { message => "Hello, World!" },
    )
}

async fn about_handler(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    render(
        &state,
        "about.html",
        context! { message => "htmx is working!" },
    )
}

fn render(state: &AppState, name: &str, ctx: Value) -> Result<Html<String>, StatusCode> {
    let tmpl = state
        .env
        .get_template(name)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tmpl.render(ctx)
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn create_router() -> Router {
    let app_state = Arc::new(AppState {
        env: load_templates(),
    });

    Router::new()
        .route("/", get(index_handler))
        .route("/about", get(about_handler))
        .nest_service("/dist", ServeDir::new("dist"))
        .with_state(app_state)
}

fn load_templates() -> Environment<'static> {
    let mut env = Environment::new();
    let template_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("ui/templates");

    for entry in fs::read_dir(template_dir).expect("ui/templates を開けない") {
        let path = entry.expect("ディレクトリ走査に失敗").path();
        if path.extension().is_some_and(|e| e == "html") {
            let name = path
                .file_name()
                .expect("ファイル名を取得できない")
                .to_string_lossy()
                .into_owned();
            let content = fs::read_to_string(&path).expect("テンプレート読込に失敗");
            env.add_template_owned(name, content)
                .expect("テンプレート登録に失敗");
        }
    }

    env
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
