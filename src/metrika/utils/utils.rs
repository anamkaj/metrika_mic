use chrono::NaiveDate;
use std::{error::Error, time::Duration};
use tokio::time::sleep;

//* Таймер*/
//*______________________________________________________________________ */
pub async fn timer(sec: u64) -> Result<(), Box<dyn Error>> {
    let duration = Duration::from_secs(sec);
    sleep(duration).await;

    Ok(())
}

//* Формирование даты для запроса статистики Я Метрики */
pub async fn calc_data(start: String, end: String) -> Result<(String, String), Box<dyn Error>> {
    //* Формирование даты */
    // let final_data = today
    //     .checked_sub_signed(Duration::days(20)) //*Отнимаем сколько нужно дней для формирования начальой даты */
    //     .expect("Ошибка при вычитании дней");

    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%d").unwrap();

    Ok((start_date.to_string(), end_date.to_string()))
}
