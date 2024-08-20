use serde::Deserialize;
use crate::metrika::model::metrika::goal_client::Goal;

//* Структура ответа на POST запрос получения целей счетчика */
#[derive(Debug, Deserialize, Clone)]
pub struct RequestGoals {
    pub id_count: i64,
}

//* Структура ответа на POST запрос получение статиситки*/
#[derive(Debug, Deserialize, Clone)]
pub struct RequestStatatics {
    pub id_count: i64,
    pub date_start: String,
    pub date_end: String,
}
//* Обновление целей */
#[derive(Debug, Deserialize, Clone)]
pub struct GoalsUpdate {
    pub goals: Vec<Goal>,
}
