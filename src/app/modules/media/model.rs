use core::fmt;
use diesel::{
    deserialize::FromSqlRow, expression::AsExpression, helper_types::AsExprOf, pg::Pg, row::Row, sql_types::Text
};
use serde::{Deserialize, Serialize};

use crate::database::schema::media;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = media)]
#[serde(crate = "rocket::serde")]
pub struct Media {
    pub id: i32,
    pub name: Option<String>,
    pub media_type: MediaType,
    pub url: String,
}

impl From<(i32, Option<String>, String, String)> for Media{
    fn from(value: (i32, Option<String>, String, String)) -> Self {
        Self {
            id: value.0,
            name: value.1,
            media_type: value.2.into(),
            url: value.3,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = media)]
#[serde(crate = "rocket::serde")]
pub struct NewMedia {
    pub name: Option<String>,
    pub media_type: MediaType,
    pub url: String,
}

impl From<Media> for NewMedia {
    fn from(value: Media) -> Self {
        Self {
            name: value.name,
            media_type: value.media_type,
            url: value.url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Video,
    Image,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                MediaType::Video => "video",
                MediaType::Image => "image",
            }
        )
    }
}

impl From<String> for MediaType {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "video" => MediaType::Video,
            "image" => MediaType::Image,
            v => panic!("Unknown value {} for MediaType found", v),
        }
    }
}

impl FromSqlRow<Text, Pg> for MediaType {
    fn build_from_row<'a>(row: &impl Row<'a, Pg>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match String::build_from_row(row)?.as_ref() {
            "video" => Ok(MediaType::Video),
            "image" => Ok(MediaType::Image),
            v => Err(format!("Unknown value {} for MediaType found", v).into()),
        }
    }
}

impl AsExpression<Text> for MediaType {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}

impl<'a> AsExpression<Text> for &'a MediaType {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}
