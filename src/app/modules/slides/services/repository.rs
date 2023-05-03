use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::slides;


use crate::app::modules::slides::model::{NewSlide, Slide};

pub async fn get_all(db: &Db) -> Result<Vec<Slide>, diesel::result::Error> {
    let result = db
        .run(move |conn| slides::table.load::<(i32, String, String, Option<String>, Option<i32>)>(conn))
        .await?;

    let slides: Vec<Slide> = result.into_iter().map(|s| s.into()).collect();

    Ok(slides)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Slide, diesel::result::Error> {
    let slide: Slide = db
        .run(move |conn| {
                slides::table.filter(slides::id.eq(id)).first::<(i32, String, String, Option<String>, Option<i32>)>(conn)
            })
        .await?
        .into();

    Ok(slide.into())
}

pub async fn create(db: &Db, new_slide: NewSlide) -> Result<Slide, diesel::result::Error> {
    let result = db
        .run(move |conn| {
            diesel::insert_into(slides::table)
                .values(&new_slide)
                .get_result::<(i32, String, String, Option<String>, Option<i32>)>(conn)
        })
        .await?;

    Ok(result.into())
}

pub async fn update(
    db: &Db,
    id: i32,
    new_slide: NewSlide,
) -> Result<Slide, diesel::result::Error> {
    let result = db
        .run(move |conn| {
            diesel::update(slides::table)
                .filter(slides::id.eq(id))
                .set(&new_slide)
                .get_result::<(i32, String, String, Option<String>, Option<i32>)>(conn)
        })
        .await?;

    Ok(result.into())
}
