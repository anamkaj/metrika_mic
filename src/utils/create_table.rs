use sqlx::{Pool, Postgres};

pub async fn create_table(pool: &Pool<Postgres>) -> Result<String, Box<dyn std::error::Error>> {
    let check_table: &str = "SELECT EXISTS (
    SELECT 1
    FROM pg_tables
    WHERE schemaname = 'public'
    AND tablename = 'counters_metrika'
);";

    let row: (bool,) = sqlx::query_as(&check_table).fetch_one(pool).await?;
    let table_exists: bool = row.0;

    if table_exists {
        return Ok("Table already exists".to_string());
    }

    if !table_exists {
        let counters_metrika: &str = "
            CREATE TABLE public.counters_metrika (
            id bigserial NOT NULL,
            counter_id int8 NOT NULL,
            status varchar NULL,
            owner_login varchar NULL,
            name varchar NULL,
            site varchar NULL,
            site_two varchar NULL,
            domain varchar NULL,
            CONSTRAINT counters_metrika_counter_id_key UNIQUE (counter_id),
            CONSTRAINT counters_metrika_pkey PRIMARY KEY (id));";

        sqlx::query(&counters_metrika)
            .execute(pool)
            .await
            .expect("Error creating table");

        let goals: &str = "
        CREATE TABLE public.goals (
            id bigserial NOT NULL,
            goal_id int8 NOT NULL,
            name text NULL,
            status bool NOT NULL,
            fk_counters_metrika_counter_id int8 NOT NULL,
            CONSTRAINT goals_goal_id_key UNIQUE (goal_id),
            CONSTRAINT goals_pkey PRIMARY KEY (id),
            CONSTRAINT goals_fk_counters_metrika_counter_id_fkey FOREIGN KEY (fk_counters_metrika_counter_id) REFERENCES public.counters_metrika(counter_id));";

        sqlx::query(&goals)
            .execute(pool)
            .await
            .expect("Error creating table");
    }

    Ok("Table created successfully!".to_string())
}
