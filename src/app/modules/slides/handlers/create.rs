use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::slides::model::{NewSlide, Slide, PostSlide};
use crate::app::modules::slides::services::repository as slides_repository;
use crate::app::modules::media::services::repository as media_repository;

pub async fn post_create_admin(db: Db, _admin: UserInClaims, post_slide: PostSlide) -> Result<Json<Slide>, Status> {
    let media_id;
    if let Some(new_media) = post_slide.media {
        let media = media_repository::create(&db, new_media).await.unwrap();

        media_id = Some(media.id);
    } else { media_id = None; };

    let new_slide = NewSlide {
        slide_type: post_slide.slide_type,
        title: post_slide.title,
        media_id,
        content: post_slide.content,
        question_id: post_slide.question_id,
    };

    let slide = slides_repository::create(&db, new_slide).await;

    match slide {
        Ok(slide) => Ok(Json(slide)),
        Err(_) => {
            println!("Error: post_create_admin; Slide not created");
            Err(Status::InternalServerError)
        }
    }
}
