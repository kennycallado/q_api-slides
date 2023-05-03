use std::fmt;

// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::{sql_types::Text, pg::Pg, row::Row, expression::AsExpression, helper_types::AsExprOf, deserialize::FromSqlRow};

use crate::app::providers::interfaces::question::PubQuestion;

use crate::database::schema::slides;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[diesel(treat_none_as_null = true)]
#[serde(crate = "rocket::serde")]
pub struct Slide {
    pub id: i32,
    pub slide_type: SlideType,
    pub title: String,
    pub content: Option<String>,
    pub question_id: Option<i32>,
}

impl From<(i32, String, String, Option<String>, Option<i32>)> for Slide {
    fn from(value: (i32, String, String, Option<String>, Option<i32>)) -> Self {
        Slide {
            id: value.0,
            slide_type: match value.1.as_ref() {
                "content" => SlideType::Content,
                "input"   => SlideType::Input,
                _ => panic!("Unknown question type"),
            },
            title: value.2,
            content: value.3,
            question_id: value.4,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SlideExpanded {
    pub id: i32,
    pub slide_type: SlideType,
    pub title: String,
    pub content: Option<String>,
    pub question: Option<PubQuestion>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = slides)]
#[diesel(treat_none_as_null = true)]
#[serde(crate = "rocket::serde")]
pub struct NewSlide {
    pub slide_type: SlideType,
    pub title: String,
    pub content: Option<String>,
    pub question_id: Option<i32>,
}

impl From<Slide> for NewSlide {
    fn from(slide: Slide) -> Self {
        NewSlide {
            slide_type: slide.slide_type,
            title: slide.title,
            content: slide.content,
            question_id: slide.question_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SlideType {
    Content,
    Input,
}

impl fmt::Display for SlideType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
                SlideType::Content => "content",
                SlideType::Input => "input",
            }
        )
    }
}

impl FromSqlRow<Text, Pg> for SlideType {
    fn build_from_row<'a>(row: &impl Row<'a, Pg>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match String::build_from_row(row)?.as_ref() {
            "content" => Ok(SlideType::Content),
            "input"   => Ok(SlideType::Input),
            v => Err(format!("Unknown value {} for SlideType found", v).into()),
        }
    }
}

impl AsExpression<Text> for SlideType {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}

impl<'a> AsExpression<Text> for &'a SlideType {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}

// impl From<UpdateSlide> for NewSlide {
//     fn from(slide: UpdateSlide) -> Self {
//         NewSlide {
//             slide_type: slide.slide_type,
//             title: slide.title,
//             content: slide.content,
//         }
//     }
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(crate = "rocket::serde")]
// pub struct UpdateSlide {
//     pub id: i32,
//     pub slide_type: String,
//     pub title: String,
//     pub content: Option<String>,
// }
