use axum::{
    extract::{State, Form, Json},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post, Router},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, sync::Arc};
use serde::{Deserialize, Serialize};

use askama::Template;
use std::io;
use tokio::net::TcpListener;

struct AppStateInner {
    pool: PgPool,
}


impl AppStateInner {
    async fn new() -> Self {
        dotenvy::dotenv().expect("Could not load the .env file!");
        let database_url =
            env::var("DATABASE_URL").expect("The environment variable DATABASE_URL is missing!");

        let pool = PgPoolOptions::new()
            .connect(&database_url)
            .await
            .expect("Failed to connect to the database!");

        Self { pool }
    }
}

type AppState = Arc<AppStateInner>;


#[derive(Template, Default)]
#[template(path = "form.html")]
struct FormTemplate<'a> {
    name: &'a str,
    email: &'a str,
    message: &'a str,
    error_message: &'a str,
}

#[derive(Deserialize, Serialize)]
struct FormFields {
    name: String,
    email: String,
    message: String,
}

impl FormFields {
    async fn insert_into_db(&self, pool: &PgPool) {
        sqlx::query!(
            "INSERT INTO submissions(name, email, message) VALUES ($1, $2, $3)",
            &self.name,
            &self.email,
            &self.message
        )
        .execute(pool)
        .await
        .expect("Failed to insert a submission into the database!");

        println!("Inserted a submission into the database!");
    }

    async fn get_all_submissions(pool: &PgPool) -> Vec<Self> {
        sqlx::query_as!(Self, "SELECT name, email, message FROM submissions")
            .fetch_all(pool)
            .await
            .expect("Failed to get submissions from the database!")
    }
}

#[derive(Template)]
#[template(path = "success.html")]
struct SuccessTemplate<'a> {
    name: &'a str,
}

#[tokio::main]
async fn main() -> io::Result<()> {

    let state = Arc::new(AppStateInner::new().await);

    let router = Router::new()
    .route("/", get(index))
    .route("/submit", post(submit))
    .route("/all-submissions", get(all_submissions))
    .route("/db_status", get("Status Ok1"))
    .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Starting Rust-Axum server ....");
    println!("Running server ....");
    println!("Listening on http://{}", listener.local_addr()?);
    println!("CTRL click on link to open");
    println!("CTRL c to close connection");


    axum::serve(listener, router).await
}


fn render_template(template: impl Template) -> Response {
    match template.render() {
        Ok(rendered) => Html(rendered).into_response(),
        Err(e) => {
            eprintln!("Failed to render a template: {e:?}");

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn index() -> Response {
    let template = FormTemplate::default();
    render_template(template)
}

async fn submit(state: State<AppState>, fields: Form<FormFields>) -> Response {
    if fields.name.len() < 2 || fields.email.len() < 3 || !fields.email.contains('@') {
        let template = FormTemplate {
            name: &fields.name,
            email: &fields.email,
            message: &fields.message,
            error_message: "Invalid input!",
        };
        return render_template(template);
    }

    fields.insert_into_db(&state.pool).await;

    let template = SuccessTemplate { name: &fields.name };
    render_template(template)
}

async fn all_submissions(state: State<AppState>) -> Json<Vec<FormFields>> {
    Json(FormFields::get_all_submissions(&state.pool).await)
}