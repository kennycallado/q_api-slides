use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::slides::model::{NewSlide, Slide};
use crate::app::modules::slides::services::repository as slides_repository;

pub async fn put_update_admin(db: Db, _admin: UserInClaims, id: i32, slide: NewSlide) -> Result<Json<Slide>, Status> {
    let slide = slides_repository::update(&db, id, slide).await;

    match slide {
        Ok(slide) => Ok(Json(slide)),
        Err(_) => {
            println!("Error: put_update_admin; Slide not updated");
            Err(Status::InternalServerError)
        }
    }
}
