use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::app::providers::interfaces::helpers::fetch::Fetch;
use crate::app::providers::interfaces::question::PubQuestion;

use crate::app::modules::slide_question::services::repository as sq_repository;

use crate::app::modules::slides::model::SlideExpanded;
use crate::app::modules::slides::services::repository as slides_repository;

pub async fn get_show_admin(
    fetch: &State<Fetch>,
    db: Db,
    _admin: UserInClaims,
    id: i32,
) -> Result<Json<SlideExpanded>, Status> {
    let slide = slides_repository::get_by_id(&db, id).await;

    match slide {
        Ok(slide) => {
            let pub_question = match slide.question_id {
                Some(id) => {
                    let question = sq_repository::get_question_by_id(fetch, id).await;
                    match question {
                        Ok(question) => Some(PubQuestion {
                            id: question.id,
                            question_type: question.question_type,
                            question: question.question,
                        }), // Quizá es un error no un none
                        Err(_) => None,
                    }
                } // Quizá es un error no un none
                None => None,
            };

            let slide_expanded = SlideExpanded {
                id: slide.id,
                slide_type: slide.slide_type,
                title: slide.title,
                content: slide.content,
                question: pub_question,
            };

            Ok(Json(slide_expanded))
        }
        Err(_) => {
            println!("Error: get_show_admin; Slide not found");
            Err(Status::InternalServerError)
        }
    }
}

pub async fn get_multiple_admin(
    fetch: &State<Fetch>,
    db: &Db,
    _admin: UserInClaims,
    ids: Vec<i32>,
) -> Result<Json<Vec<SlideExpanded>>, Status> {
    let slides = slides_repository::get_slides_by_ids(&db, ids).await;

    match slides {
        Ok(slides) => {
            let question_ids: Vec<i32> = slides.iter().filter_map(|slide| slide.question_id).collect();
            let questions = sq_repository::get_questions_by_ids(fetch, question_ids).await;

            if let Err(_) = questions {
                println!("Error: get_multiple_admin; Questions not found");
                return Err(Status::InternalServerError);
            }
            let questions = questions.unwrap();

            let mut slides_expanded: Vec<SlideExpanded> = Vec::new();

            for slide in slides {
                let pub_question = match slide.question_id {
                    Some(id) => {
                        let question = questions.iter().find(|question| question.id == id);
                        match question {
                            Some(question) => Some(PubQuestion {
                                id: question.id,
                                question_type: question.question_type.clone(),
                                question: question.question.clone(),
                            }),
                            None => None,
                        }
                    }
                    None => None,
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
        }
        Err(_) => {
            println!("Error: get_multiple_admin; Slides not found");
            Err(Status::InternalServerError)
        }
    }
}
