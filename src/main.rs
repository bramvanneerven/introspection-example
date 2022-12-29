use async_graphql::{http::GraphiQLSource, *};
use async_graphql_poem::GraphQL;
use once_cell::sync::Lazy;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
use std::sync::Mutex;

static NUMBER: Lazy<Mutex<i32>> = Lazy::new(Default::default);

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn number(&self) -> i32 {
        *NUMBER.lock().unwrap()
    }
}

#[derive(OneofObject)]
enum AddInput {
    Tens(i32),
    Hundreds(i32),
    Thousands(i32),
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add(&self, input: AddInput) -> i32 {
        let mut number = NUMBER.lock().unwrap();
        let value = match input {
            AddInput::Tens(value) => value * 10,
            AddInput::Hundreds(value) => value * 100,
            AddInput::Thousands(value) => value * 1000,
        };
        *number += value;
        *number
    }
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8001");
    Server::new(TcpListener::bind("127.0.0.1:8001"))
        .run(app)
        .await
        .unwrap();
}
