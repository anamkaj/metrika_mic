use dotenv::dotenv;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

//* Информация о счетчике */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountInform {
    pub counter: CounterInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CounterInfo {
    pub id: i64,
    pub status: String,
    pub owner_login: String,
    pub name: String,
    pub site: String,
    pub site2: Site2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Site2 {
    pub site: String,
    pub domain: String,
}

impl CountInform {
    pub async fn get_info_counter(id: i64) -> Result<CountInform, Box<dyn Error>> {
        println!("Запрос к API. Получение информации о счетчике {}", id);
        dotenv().ok();

        let access_token: String = std::env::var("ACCESS_TOKEN").unwrap();
        let cookie_session: String = std::env::var("COOKIES_SESSION").unwrap();

        let url: String = format!(
            "https://api-metrika.yandex.net/management/v1/counter/{}",
            id
        );

        let client: reqwest::Client = reqwest::Client::builder().build().unwrap();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-yametrika+json"
                .parse()
                .expect("ошибка парсинга заголовка"),
        );
        headers.insert(
            "Authorization",
            access_token.parse().expect("ошибка парсинга заголовка"),
        );
        headers.insert(
            "Cookie",
            cookie_session.parse().expect("ошибка парсинга заголовка"),
        );

        let request: reqwest::RequestBuilder =
            client.request(reqwest::Method::GET, url).headers(headers);

        let response: reqwest::Response = request
            .send()
            .await
            .expect("Ошибка запроса получения массива счетчиков");

        let body: String = response
            .text()
            .await
            .expect("Ошибка извлечения тела запроса");

        let pars_json: CountInform = serde_json::from_str(&body).expect("Ошибка сериализации JSON");

        Ok(pars_json)
    }
}
