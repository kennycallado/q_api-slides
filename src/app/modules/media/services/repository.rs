use diesel::prelude::*;

use crate::app::modules::media::model::{Media, NewMedia};
use crate::database::connection::Db;
use crate::database::schema::media;

pub async fn get_by_id(db: &Db, id: i32) -> Result<Media, diesel::result::Error> {
    let media: Media = db.run(move |conn| {
        media::table
            .filter(media::id.eq(id))
            .first::<(i32, Option<String>, String, String)>(conn)
    }).await?.into();

    Ok(media)
}

pub async fn create(db: &Db, new_media: NewMedia) -> Result<Media, diesel::result::Error> {
    let media: Media = db.run(move |conn| {
        diesel::insert_into(media::table)
            .values(&new_media)
            .get_result::<(i32, Option<String>, String, String)>(conn)
    }).await?.into();

    Ok(media)
}

pub async fn update(db: &Db, id: i32, new_media: NewMedia) -> Result<Media, diesel::result::Error> {
    let media: Media = db.run(move |conn| {
        diesel::update(media::table)
            .filter(media::id.eq(id))
            .set(&new_media)
            .get_result::<(i32, Option<String>, String, String)>(conn)
    }).await?.into();

    Ok(media)
}
