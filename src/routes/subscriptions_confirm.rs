use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
// The parameter of type web::Query<> instructs actix-web to only call the handler if the extractions was successful.
// Otherwise it returns a 400 Bad Request
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
