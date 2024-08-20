use crate::metrika::model::error::error::ErrorResponseMetrika;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::error::Error;

use super::goal_client::GoalClient;

//* получение целей на счетчике */
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct GoalCounter {
    pub goals: Vec<Goal>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Goal {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub conditions: Vec<Condition>,
    #[serde(default)]
    pub steps: Vec<Step>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Step {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub conditions: Vec<Condition2>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

impl GoalCounter {
    pub async fn api_get_goals(
        id: i64,
        pool: Pool<Postgres>,
    ) -> Result<GoalClient, Box<dyn Error>> {
        dotenv().ok();

        //* Если счетчик есть в базе , вернет с него цели  */
        let goals_check: GoalClient = GoalClient::get_goals_counter(id, pool.clone()).await?;

        if !goals_check.goals.is_empty() {
            return Ok(goals_check);
        }

        println!("Добавление целей счетчика {:?}", goals_check);

        let access_token: String = std::env::var("ACCESS_TOKEN").unwrap();
        let cookie_session: String = std::env::var("COOKIES_SESSION").unwrap();
        let url: String = format!(
            "https://api-metrika.yandex.net/management/v1/counter/{}/goals",
            id
        );

        let client: reqwest::Client = reqwest::Client::builder().build()?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/x-yametrika+json".parse()?);
        headers.insert("Authorization", access_token.parse()?);
        headers.insert("Cookie", cookie_session.parse()?);

        let request: reqwest::RequestBuilder =
            client.request(reqwest::Method::GET, url).headers(headers);

        let response: reqwest::Response = request.send().await?;

        if response.status() != reqwest::StatusCode::OK {
            let error: String = response.text().await?;
            let json_error: ErrorResponseMetrika = serde_json::from_str(&error)?;
            let msg: String = json_error.message;
            return Err(msg.into());
        }
        let body: String = response.text().await?;
        let pars_json: GoalCounter = serde_json::from_str(&body)?;

        //* Трансформация обьекта целей из API метрики */
        let data: GoalClient = GoalClient::transform_goals(pars_json, id).await?;

        //* Добавление целей в базу */
        let _ = GoalClient::add_goals_in_db(&data, pool).await;

        Ok(data)
    }
}
