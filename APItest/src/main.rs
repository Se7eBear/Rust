use std::net::TcpListener;

use axum::{
    routing::get,
    response::Json,
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct Resposta{
    mensagem: String,
    status: String,
}

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get(raiz))
        .route("/api/status", get(get_status));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("servidor rodando em http://localhost:3000");

    axum::serve(listener, app). await.unwrap();
}

async fn raiz() -> &'static str{
    "Bem-vindo a API em Rust!"
}

async fn get_status() -> Json<Resposta>{
    let resposta = Resposta{
        mensagem: "Tudo funcionando perfeitamente".to_string(),
        status: "online".to_string(),
    };
    Json(resposta)
}