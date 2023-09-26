use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::database::connection::Db;

use crate::app::providers::models::question::PubQuestion;
use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::slides::model::SlideExpanded;

use crate::app::modules::slide_question::services::repository as sq_repository;
use crate::app::modules::slides::services::repository as slides_repository;
use crate::app::modules::media::services::repository as media_repository;
use crate::app::modules::media::model::Media;

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
                        }),
                        Err(_) => None,
                    }
                },
                None => None,
            };

            let media = match slide.media_id {
                Some(id) => {
                    let media = media_repository::get_by_id(&db, id).await;
                    match media {
                        Ok(media) => Some(Media {
                            id: media.id,
                            name: media.name,
                            media_type: media.media_type,
                            url: media.url,
                        }),
                        Err(_) => None,
                    }
                },
                None => None,
            };

            let slide_expanded = SlideExpanded {
                id: slide.id,
                slide_type: slide.slide_type,
                title: slide.title,
                media,
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
            let questions = match sq_repository::get_questions_by_ids(fetch, question_ids).await {
                Ok(questions) => questions,
                Err(_) => return Err(Status::InternalServerError),
            };

            let mut slides_expanded: Vec<SlideExpanded> = Vec::new();
            for slide in slides {
                let pub_question = match slide.question_id {
                    Some(id) => {
                        match questions.iter().find(|question| question.id == id) {
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

                let media = match slide.media_id {
                    Some(id) => {
                        let media = media_repository::get_by_id(&db, id).await;
                        match media {
                            Ok(media) => Some(Media {
                                id: media.id,
                                name: media.name,
                                media_type: media.media_type,
                                url: media.url,
                            }),
                            Err(_) => None,
                        }
                    },
                    None => None,
                };

                let slide_expanded = SlideExpanded {
                    id: slide.id,
                    slide_type: slide.slide_type,
                    title: slide.title,
                    media,
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
