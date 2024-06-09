use axum::{
	routing::{get,post},
	response::IntoResponse,
	http::StatusCode, Json, Router
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
	let x = "Bye";
	let y = String::from("planet");
    println!("{}, {}!", x, y);
	let app = Router::new()
		.route("/", get(root))
		.route("/users", post(create));
	let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}

async fn root()-> &'static str {
	"hello"
}

async fn create(
	Json(payload): Json<CreateUser>
) -> impl IntoResponse {
	let user = CreateUser {
		name: payload.name,
		age: payload.age
	};
	(StatusCode::CREATED, Json(user))


}

#[derive(Deserialize, Serialize)]
struct CreateUser {
	 name: String, 
	 age: i32
}

