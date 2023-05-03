use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::providers::interfaces::question::{PubQuestion, QuestionType};

use crate::app::modules::slides::model::SlideExpanded;
use crate::app::modules::slides::services::repository as slides_repository;

pub async fn get_show_admin(db: Db, _admin: UserInClaims, id: i32) -> Result<Json<SlideExpanded>, Status> {
    let slide = slides_repository::get_by_id(&db, id).await;

    match slide {
        Ok(slide) => {
            let pub_question = PubQuestion {
                id: slide.question_id.unwrap(),
                question_type: QuestionType::Input,
                question: String::from("blabla"),
            };

            let slide_expanded = SlideExpanded {
                id: slide.id,
                slide_type: slide.slide_type,
                title: slide.title,
                content: slide.content,
                question: Some(pub_question),
            };


            Ok(Json(slide_expanded))
        },
        Err(_) => {
            println!("Error: get_show_admin; Slide not found");
            Err(Status::InternalServerError)
        }
    }
}
