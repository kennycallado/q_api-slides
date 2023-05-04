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
            // debería llamar a question para pedir la pregunta
            let pub_question = PubQuestion {
                id: slide.question_id.unwrap(),
                question_type: QuestionType::Input,
                question: String::from("blabla?"),
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

pub async fn get_multiple_admin(db: &Db, _admin: UserInClaims, ids: Vec<i32>) -> Result<Json<Vec<SlideExpanded>>, Status> {
    let slides = slides_repository::get_slides_by_ids(&db, ids).await;

    match slides {
        Ok(slides)  => {
            let mut slides_expanded: Vec<SlideExpanded> = Vec::new();

            // debería llamar a question para pedir las preguntas
            for slide in slides {
                let pub_question = match slide.question_id {
                    Some(id) => { 
                        Some(PubQuestion {
                            id,
                            question_type: QuestionType::Input,
                            question: String::from("blabla?"),
                        })
                    },
                    None => {
                        None
                    }
                };

                let slide_expanded = SlideExpanded {
                    id: slide.id,
                    slide_type: slide.slide_type,
                    title: slide.title,
                    content: slide.content,
                    question: pub_question,
                };

                slides_expanded.push(slide_expanded);
            }

            Ok(Json(slides_expanded))
        },
        Err (_)     => {
            println!("Error: get_multiple_admin; Slides not found");
            Err(Status::InternalServerError)
        }
    }
}
