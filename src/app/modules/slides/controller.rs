use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::interfaces::helpers::fetch::Fetch;
use crate::config::database::Db;

use crate::app::modules::slides::handlers::{create, index, show, update};

use crate::app::modules::slides::model::{NewSlide, Slide, SlideExpanded};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        option_index,
        option_show,
        get_index,
        get_index_none,
        post_multiple,
        post_multiple_none,
        get_show,
        get_show_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
    ]
}

#[options("/")]
pub async fn option_index() -> Status {
    Status::Ok
}

#[options("/<_id>", rank = 101)]
pub async fn option_show(_id: i32) -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: Db, claims: AccessClaims) -> Result<Json<Vec<Slide>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(db, claims.0.user).await,
        _ => {
            println!("Error: get_index; Role not handled");
            Err(Status::BadRequest)
        }
    }
}

#[get("/", rank = 2)]
pub fn get_index_none() -> Status {
    Status::Unauthorized
}

#[post("/multiple", data = "<slide_ids>", rank = 1)]
pub async fn post_multiple(
    fetch: &State<Fetch>,
    db: Db,
    claims: AccessClaims,
    slide_ids: Json<Vec<i32>>,
) -> Result<Json<Vec<SlideExpanded>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_multiple_admin(fetch, &db, claims.0.user, slide_ids.into_inner()).await,
        "robot" => show::get_multiple_admin(fetch, &db, claims.0.user, slide_ids.into_inner()).await,
        _ => {
            println!("Error: get_show; Role not handled");
            Err(Status::BadRequest)
        }
    }
}

#[post("/multiple", data = "<_slide_ids>", rank = 2)]
pub fn post_multiple_none(_slide_ids: Json<Vec<i32>>) -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 101)]
pub async fn get_show(
    fetch: &State<Fetch>,
    db: Db,
    claims: AccessClaims,
    id: i32,
) -> Result<Json<SlideExpanded>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(fetch, db, claims.0.user, id).await,
        "robot" => show::get_show_admin(fetch, db, claims.0.user, id).await,
        _ => {
            println!("Error: get_show; Role not handled");
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>", rank = 102)]
pub fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_slide>")]
pub async fn post_create(db: Db, claims: AccessClaims, new_slide: Json<NewSlide>) -> Result<Json<Slide>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_create_admin(db, claims.0.user, new_slide.into_inner()).await,
        _ => {
            println!("Error: post_create; Role not handled");
            Err(Status::BadRequest)
        }
    }
}

#[post("/", data = "<_new_slide>", rank = 2)]
pub fn post_create_none(_new_slide: Json<NewSlide>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_slide>", rank = 101)]
pub async fn put_update(
    db: Db,
    claims: AccessClaims,
    id: i32,
    new_slide: Json<NewSlide>,
) -> Result<Json<Slide>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_admin(db, claims.0.user, id, new_slide.into_inner()).await,
        _ => {
            println!("Error: put_update; Role not handled");
            Err(Status::BadRequest)
        }
    }
}

#[put("/<_id>", data = "<_new_slide>", rank = 102)]
pub fn put_update_none(_id: i32, _new_slide: Json<NewSlide>) -> Status {
    Status::Unauthorized
}
