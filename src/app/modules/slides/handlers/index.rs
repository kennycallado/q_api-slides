use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::slides::model::Slide;
use crate::app::modules::slides::services::repository as slides_repository;

pub async fn get_index_admin(db: Db, _admin: UserInClaims) -> Result<Json<Vec<Slide>>, Status> {
    let slides = slides_repository::get_all(&db).await;

    match slides {
        Ok(slides) => Ok(Json(slides)),
        Err(_) => Err(Status::InternalServerError),
    }
}
