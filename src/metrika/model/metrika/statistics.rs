use dotenv::dotenv;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Pool;
use sqlx::Postgres;
use std::error::Error;

use crate::metrika::model::error::error::ErrorResponseMetrika;
use crate::metrika::model::metrika::goal_client::Goal;
use crate::metrika::model::metrika::goal_client::GoalClient;
use crate::metrika::utils::utils::calc_data;
use crate::metrika::utils::utils::timer;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatGoals {
    pub query: Query,
    pub data: Vec<Daum>,
    pub total_rows: i64,
    pub total_rows_rounded: bool,
    pub sampled: bool,
    pub contains_sensitive_data: bool,
    pub sample_share: f64,
    pub sample_size: i64,
    pub sample_space: i64,
    pub data_lag: i64,
    pub totals: Vec<f64>,
    pub min: Vec<f64>,
    pub max: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Query {
    pub ids: Vec<i64>,
    pub dimensions: Vec<String>,
    pub metrics: Vec<String>,
    pub sort: Vec<String>,
    pub date1: String,
    pub date2: String,
    pub filters: String,
    pub limit: i64,
    pub offset: i64,
    pub group: String,
    pub benchmarks_version: String,
    pub auto_group_size: String,
    pub attr_name: String,
    pub quantile: String,
    pub offline_window: String,
    pub attribution: String,
    pub currency: String,
    pub funnel_window: String,
    pub adfox_event_id: String,
    pub funnel_pattern: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub dimensions: Vec<Dimension>,
    pub metrics: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    pub id: String,
    pub name: String,
}

//__________________________________________________
//* Результирующая структура для клиента  */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultStatGoals {
    pub date1: String,
    pub date2: String,
    pub len_goals: i32,
    pub goal: Vec<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub goal_id: i64,
    pub name: String,
    pub status: bool,
    pub metrics: f64,
}

impl StatGoals {
    pub async fn get_stat_goals(
        id_count: i64,
        start: String,
        end: String,
        pool: Pool<Postgres>,
    ) -> Result<ResultStatGoals, Box<dyn Error>> {
        println!("Получение статистики по целям");
        dotenv().ok();

        let goals: GoalClient = GoalClient::get_goals_counter(id_count, pool).await?;

        let goals_true: Vec<&Goal> = goals.goals.iter().filter(|x| x.status == true).collect();

        if goals_true.is_empty() {
            return Err("Нет выбранных целей".into());
        }

        println!("Количество целей: {}", goals_true.len());

        let access_token: String = std::env::var("ACCESS_TOKEN").unwrap();
        let cookie_session: String = std::env::var("COOKIES_SESSION").unwrap();

        //*Формирование даты */
        let (start, end): (String, String) = calc_data(start, end).await?;

        //*Запрос на получения статистики по целям */
        let filter_metrika_url: &str = "dimensions=ym:s:goalDimension&filters=(ym:s:AUTOMATICAdvEngine=='ya_direct' or ym:s:AUTOMATICAdvEngine=='ya_undefined') and (ym:s:isRobot=='No')";

        let mut url: String = format!(
        "https://api-metrika.yandex.ru/stat/v1/data?date1={}&date2={}&ids={}&accuracy=full&{}&metrics=",
        start,end, id_count, filter_metrika_url
    );

        //*Формирование url запроса для запроса конверсий по целям  */
        let len_arr: usize = goals_true.len();
        for (index, x) in goals_true.iter().enumerate() {
            let id: i64 = x.goal_id;
            let _ = timer(index.try_into().unwrap()).await;

            if index + 1 == len_arr {
                let form_str: String = format!("ym:s:goal{}visits", id);
                url.push_str(&form_str);
            } else {
                let form_str: String = format!("ym:s:goal{}visits,", id);
                url.push_str(&form_str);
            }
        }

        let client: reqwest::Client = reqwest::Client::builder().build()?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/x-yametrika+json".parse()?);
        headers.insert("Authorization", access_token.parse()?);
        headers.insert("Cookie", cookie_session.parse()?);

        let request: reqwest::RequestBuilder =
            client.request(reqwest::Method::GET, url).headers(headers);

        let response: reqwest::Response = request.send().await?;
        println!("{:?}", response);

        if response.status() != reqwest::StatusCode::OK {
            let error: String = response.text().await?;
            let json_error: ErrorResponseMetrika = serde_json::from_str(&error)?;
            let msg: String = json_error.message;
            return Err(msg.into());
        }

        let body: String = response.text().await?;

        let pars_json: StatGoals = serde_json::from_str(&body).expect("Ошибка сериализации JSON");

        let data: ResultStatGoals = ResultStatGoals::transform_data(pars_json, goals_true).await?;

        Ok(data)
    }
}

impl ResultStatGoals {
    pub async fn transform_data(
        data: StatGoals,
        goals: Vec<&Goal>,
    ) -> Result<ResultStatGoals, Box<dyn Error>> {
        let mut result_array: Vec<Data> = Vec::new();

        for x in data.data.iter() {
            let id: String = x.dimensions.iter().map(|x| x.id.clone()).next().unwrap();

            result_array.push(Data {
                goal_id: x
                    .dimensions
                    .iter()
                    .map(|x| x.id.clone())
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap(),
                name: goals
                    .iter()
                    .filter(|f| f.goal_id.to_string() == id)
                    .next()
                    .unwrap()
                    .name
                    .clone(),
                status: goals
                    .iter()
                    .filter(|f| f.goal_id.to_string() == id)
                    .next()
                    .unwrap()
                    .status,
                metrics: x.metrics.iter().sum(),
            })
        }

        let result: ResultStatGoals = ResultStatGoals {
            date1: data.query.date1.clone() as String,
            date2: data.query.date2.clone() as String,
            len_goals: goals.len() as i32,
            goal: result_array,
        };

        Ok(result)
    }
}
