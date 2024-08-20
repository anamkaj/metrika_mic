use super::{
    counter_info::{CountInform, CounterInfo},
    goals::GoalCounter,
};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};
use std::error::Error;

#[derive(Default, Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct GoalClient {
    pub counter_id: i64,
    pub goals: Vec<Goal>,
}

//* Структура целей для клиента  */
#[derive(Default, Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Goal {
    pub goal_id: i64,
    pub name: String,
    pub status: bool,
}

impl GoalClient {
    //* Трансформация обьекта целей из API метрики */
    pub async fn transform_goals(
        goals: GoalCounter,
        id: i64,
    ) -> Result<GoalClient, Box<dyn Error>> {
        let mut data: Vec<Goal> = Vec::new();

        for goal in goals.goals {
            if goal.type_field == "action" {
                data.push(Goal {
                    goal_id: goal.id,
                    name: goal.name.clone(),
                    status: false,
                });
            }

            //* Автоцели */
            if goal.type_field == "form" {
                data.push(Goal {
                    goal_id: goal.id,
                    name: goal.name.clone(),
                    status: false,
                });
            }

            if goal.type_field == "messenger" {
                data.push(Goal {
                    goal_id: goal.id,
                    name: goal.name.clone(),
                    status: false,
                });
            }
            if goal.type_field == "phone" {
                data.push(Goal {
                    goal_id: goal.id,
                    name: goal.name.clone(),
                    status: false,
                });
            }
            if goal.type_field == "social" {
                data.push(Goal {
                    goal_id: goal.id,
                    name: goal.name.clone(),
                    status: false,
                });
            }

            //*Коллтрекинг */
            if goal.type_field == "call" {
                data.push(Goal {
                    goal_id: goal.id,
                    name: goal.name.clone(),
                    status: false,
                });
            }

            //* Составная цель */
            if goal.type_field == "step" {
                let name: String = goal.name;
                let id: i64 = goal.id;
                data.push(Goal {
                    goal_id: id,
                    name: name.clone(),
                    status: false,
                });

                goal.steps.iter().for_each(|x| {
                    let step_name = x.name.clone();
                    let step_id = x.id;
                    data.push(Goal {
                        goal_id: step_id,
                        name: format!("{} -> {}", name, step_name),
                        status: false,
                    });
                });
            }
        }

        let result: GoalClient = GoalClient {
            counter_id: id,
            goals: data,
        };

        Ok(result)
    }
}

impl GoalClient {
    //*Добавление целей в базу */
    pub async fn add_goals_in_db(
        data: &GoalClient,
        pool: Pool<Postgres>,
    ) -> Result<(), Box<dyn Error>> {
        println!("Добавление целей счетчика {}", data.counter_id);

        let count_info: CountInform = CountInform::get_info_counter(data.counter_id).await?;
        let counter: CounterInfo = count_info.counter;

        let insert_count: &str =
        "INSERT INTO counters_metrika (counter_id, status, owner_login, name, site, site_two, domain)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (counter_id)
        DO UPDATE SET
            status = EXCLUDED.status,
            owner_login = EXCLUDED.owner_login,
            name = EXCLUDED.name,
            site = EXCLUDED.site,
            site_two = EXCLUDED.site_two,
            domain = EXCLUDED.domain;";

        sqlx::query(&insert_count)
            .bind(counter.id)
            .bind(&counter.status)
            .bind(&counter.owner_login)
            .bind(&counter.name)
            .bind(&counter.site)
            .bind(&counter.site2.site)
            .bind(&counter.site2.domain)
            .execute(&pool)
            .await?;

        let insert_goal: &str = "
            INSERT INTO goals (goal_id, name, fk_counters_metrika_counter_id,status)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (goal_id)
            DO UPDATE SET
                goal_id = EXCLUDED.goal_id,
                name = EXCLUDED.name,
                fk_counters_metrika_counter_id = EXCLUDED.fk_counters_metrika_counter_id,
                status = EXCLUDED.status;";

        for goal in &data.goals {
            sqlx::query(&insert_goal)
                .bind(&goal.goal_id)
                .bind(&goal.name)
                .bind(data.counter_id)
                .bind(goal.status)
                .execute(&pool)
                .await?;
        }

        Ok(())
    }

    //*Получение целей по конкретному счетчику из базы
    pub async fn get_goals_counter(
        id: i64,
        pool: Pool<Postgres>,
    ) -> Result<GoalClient, Box<dyn Error>> {
        let get_goals: &str = "
            WITH counter_check AS (
            SELECT goal_id, name, status
            FROM goals
            WHERE fk_counters_metrika_counter_id = $1)
            SELECT goal_id, name, status FROM counter_check;";

        let goals: Vec<Goal> = sqlx::query_as(&get_goals).bind(id).fetch_all(&pool).await?;

        let result: GoalClient = GoalClient {
            goals,
            counter_id: id,
        };

        Ok(result)
    }
}

impl Goal {
    pub async fn update_goals_in_db(
        data: &Vec<Goal>,
        pool: Pool<Postgres>,
    ) -> Result<(), Box<dyn Error>> {
        let insert_goal: &str = "
        UPDATE goals
        SET
        status = $2
        WHERE goal_id = $1;";

        for goal in data {
            sqlx::query(&insert_goal)
                .bind(goal.goal_id)
                .bind(goal.status)
                .execute(&pool)
                .await?;
        }
        Ok(())
    }
}
