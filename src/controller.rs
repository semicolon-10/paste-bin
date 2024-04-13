use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use crate::db_service::DBService;
use crate::model::Data;

pub async fn get_content_by_token(service: Extension<DBService>, Path(token): Path<String>)
 -> Result<Json<String>, StatusCode> {
    match service.get_content(token).await {
        Ok(content) => Ok(Json(content)),
        Err(ex) => {
            eprintln!("{:?}",ex);
            Err(StatusCode::NO_CONTENT)
        }
    }
}

pub async fn store_content(service: Extension<DBService>, Json(data): Json<Data>)
 -> Result<Json<String>, StatusCode> {
    match service.store_content(data.content).await {
        Ok(token) => Ok(Json(token)),
        Err(ex) => {
            eprintln!("{:?}",ex);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
