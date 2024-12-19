use axum::Router;
use axum::routing::post;
use crate::api::auth::routes::auth_login_handler;

mod services;
mod routes;
mod schemas;


pub fn router() -> Router{
    Router::new().route("/login/", post(auth_login_handler))
}