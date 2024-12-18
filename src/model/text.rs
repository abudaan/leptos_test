#[cfg(feature = "ssr")]
use crate::database::ssr::db;
use crate::{database::AppState, error_template::AppError};
use leptos::{
    logging,
    prelude::{use_context, ServerFnError},
    server,
};
// use macros::New;
use serde::{Deserialize, Serialize};
// use snafu::{ensure, Snafu};
#[cfg(feature = "ssr")]
use sqlx::PgPool;
#[cfg(feature = "ssr")]
use tokio::time::{sleep, Duration};
use uuid::Uuid;

use crate::model::PgId;

// #[cfg(feature = "ssr")]
// pub async fn get_pool() -> Result<PgPool, ServerFnError> {
//     if let Some(state) = use_context::<AppState>() {
//         if let Some(pool) = state.pool {
//             Ok(pool)
//         } else {
//             tracing::error!("No database");
//             Err(ServerFnError::new(format!(
//                 "No database connection {}",
//                 state.db_error.unwrap_or_default()
//             )))
//         }
//     } else {
//         tracing::error!("No context");
//         Err(ServerFnError::new("No context available"))
//     }
// }

// #[server(Json)]
#[server(GetAllTexts, "/api", "GetJson", "get_all_texts")]
pub async fn get_all_texts() -> Result<Vec<Text>, ServerFnError> {
    // let pool = get_pool().await?;
    let pool = db().await?;
    Text::get_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Problem while fetching texts: {}", e)))
}

#[server(GetOne)]
pub async fn get_one(id: Uuid) -> Result<Text, ServerFnError> {
    // let pool = get_pool().await?;
    let pool = db().await?;
    Text::get_one(id, &pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Problem while fetching text {} {}", id, e)))
}

#[server(AddOrUpdate)]
pub async fn add_or_update(text: NewText) -> Result<String, ServerFnError> {
    if let Some(uuid) = text.id {
        update(Text {
            id: uuid,
            title: text.title,
            content: text.content,
            published: text.published,
        })
        .await
    } else {
        add(text).await
    }
}

#[server(Add)]
pub async fn add(text: NewText) -> Result<String, ServerFnError> {
    let pool = db().await?;
    // logging::log!(
    //     "add {} {}",
    //     text.title,
    //     text.title.is_empty() || text.content.is_empty()
    // );
    if text.title.is_empty() || text.content.is_empty() {
        // Err(ServerFnError::new(
        //     "Please fill out both title and content!",
        // ))
        Ok("Please fill out both title and content!".to_string())
    } else {
        // Text::add(text, &pool)
        //     .await
        //     .map_err(|e| ServerFnError::new(format!("Problem while adding text {}", e)))
        let r = Text::add(text, &pool).await;
        match r {
            Ok(_uuid) => Ok("ok".to_string()),
            Err(error) => Ok(error.to_string()),
        }
    }
}

#[server(Update)]
pub async fn update(text: Text) -> Result<String, ServerFnError> {
    let pool = db().await?;
    // Text::update(&text, &pool)
    //     .await
    //     .map_err(|e| ServerFnError::new(format!("Problem while updating text {} {}", text.id, e)))
    let r = Text::update(&text, &pool).await;
    match r {
        Ok(_uuid) => Ok("ok".to_string()),
        Err(error) => Ok(error.to_string()),
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
// #[new(derive(Deserialize, Clone))]
pub struct Text {
    // #[new(skip)]
    pub id: Uuid,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub published: bool,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct NewText {
    pub id: Option<Uuid>,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub published: bool,
}

// #[derive(Debug, Snafu)]
// enum AppError {
//     #[snafu(display("No database connection"))]
//     Database,
//     #[snafu(display("Context not found"))]
//     Context,
// }

#[cfg(feature = "ssr")]
impl Text {
    // pub fn get_pool(&self) -> Result<PgPool, AppError> {
    //     ensure!(use_context::<AppState>().is_some(), ContextSnafu);
    //     let state = use_context::<AppState>().unwrap();
    //     ensure!(state.pool.is_some(), DatabaseSnafu);
    //     Ok(state.pool.unwrap())
    // }

    // pub fn get_pool(&self) -> PgPool {
    //     let state = use_context::<AppState>().unwrap();
    //     state.pool.unwrap()
    // }

    async fn get_all(pool: &PgPool) -> Result<Vec<Text>, sqlx::Error> {
        // let pool = &self.get_pool();
        sqlx::query_as!(
            Self,
            r#"
            SELECT *
            FROM text
            GROUP BY id
            "#
        )
        .fetch_all(pool)
        .await
    }

    async fn get_one(id: Uuid, db: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT *
            FROM text
            WHERE id = $1
            GROUP BY id
            "#,
            id
        )
        .fetch_one(db)
        .await
    }

    async fn add(text: NewText, db: &PgPool) -> Result<Uuid, sqlx::Error> {
        Ok(sqlx::query_as!(
            PgId,
            r#"
            INSERT INTO text (title, content, published) VALUES ($1, $2, $3)
            RETURNING id
            "#,
            text.title,
            text.content,
            text.published,
        )
        .fetch_one(db)
        .await?
        .id)
    }

    async fn update(text: &Text, db: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Text,
            r#"
            UPDATE text
            SET title = $2, content = $3, published = $4
            WHERE id = $1
            RETURNING *
            "#,
            text.id,
            text.title,
            text.content,
            text.published,
        )
        .fetch_one(db)
        .await
    }

    /**
     * Delete the text
     */
    pub async fn delete(id: Uuid, db: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM text WHERE id = $1
            "#,
            id
        )
        .execute(db)
        .await?;

        Ok(())
    }

    /**
     * Delete all texts
     */
    pub async fn delete_all(db: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM text
            "#,
        )
        .execute(db)
        .await?;

        Ok(())
    }
}
