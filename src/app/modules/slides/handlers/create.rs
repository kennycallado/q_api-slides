use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::slides::model::{NewSlide, Slide};
use crate::app::modules::slides::services::repository as slides_repository;

pub async fn post_create_admin(
    db: Db,
    _admin: UserInClaims,
    new_slide: NewSlide,
) -> Result<Json<Slide>, Status> {
    let slide = slides_repository::create(&db, new_slide).await;

    match slide {
        Ok(slide) => Ok(Json(slide)),
        Err(_) => {
            println!("Error: post_create_admin; Slide not created");
            Err(Status::InternalServerError)
        }
    }
}
