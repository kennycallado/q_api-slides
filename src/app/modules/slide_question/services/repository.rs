use rocket::http::Status;
use rocket::State;

use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;
use crate::app::providers::interfaces::helpers::fetch::Fetch;

use crate::app::providers::interfaces::question::PubQuestion;

pub async fn get_questions_by_ids(fetch: &State<Fetch>, ids: Vec<i32>) -> Result<Vec<PubQuestion>, Status> {
    // Prepare robot token
    let robot_token = match Fetch::robot_token().await {
        Ok(robot_token) => robot_token,
        Err(_) => return Err(Status::InternalServerError),
    };

    // Prepare questions url
    let question_url = ConfigGetter::get_entity_url("question")
        .unwrap_or("http://localhost:8011/api/v1/question".to_string())
        + "/multiple";

    // Request question
    let client = fetch.client.lock().await;
    let res = client
        .post(&question_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .json(&ids)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                let questions = res.json::<Vec<PubQuestion>>().await;
                match questions {
                    Ok(questions) => Ok(questions),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn get_question_by_id(fetch: &State<Fetch>, id: i32) -> Result<PubQuestion, Status> {
    // Prepare robot token
    let robot_token = match Fetch::robot_token().await {
        Ok(robot_token) => robot_token,
        Err(_) => return Err(Status::InternalServerError),
    };

    // Prepare questions url
    let question_url = ConfigGetter::get_entity_url("question")
        .unwrap_or("http://localhost:8011/api/v1/question".to_string())
        + "/"
        + &id.to_string();

    // Request question
    let client = fetch.client.lock().await;
    let res = client
        .get(&question_url)
        .header("Authorization", robot_token)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status().is_success() {
                let question = res.json::<PubQuestion>().await;
                match question {
                    Ok(question) => Ok(question),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
