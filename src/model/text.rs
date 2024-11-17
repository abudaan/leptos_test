use leptos::{expect_context, server, ServerFnError};
use macros::New;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Error, PgPool};
#[cfg(feature = "ssr")]
use tokio::time::{sleep, Duration};
use uuid::Uuid;

use crate::model::PgId;

// #[server(GetAllTexts, "/api", "GetJson", "text")]
#[server]
pub async fn get_all_texts() -> Result<Vec<Text>, ServerFnError> {
    let db = expect_context::<PgPool>();

    sleep(Duration::from_millis(1000)).await;

    Text::get_all(&db).await.map_err(|x| {
        tracing::error!("problem while fetching home texts: {x:?}");
        ServerFnError::new("Problem while fetching home texts")
    })
}

#[derive(Serialize, Deserialize, Debug, Clone, New, Default)]
#[new(derive(Deserialize, Clone))]
pub struct Text {
    #[new(skip)]
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[cfg(feature = "ssr")]
impl Text {
    pub async fn get_all(db: &PgPool) -> Result<Vec<Text>, Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT *
            FROM text
            GROUP BY id
            "#
        )
        .fetch_all(db)
        .await
    }

    pub async fn get_one(id: Uuid, db: &PgPool) -> Result<Self, sqlx::Error> {
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

    pub async fn add(new_text: NewText, db: &PgPool) -> Result<Uuid, sqlx::Error> {
        Ok(sqlx::query_as!(
            PgId,
            r#"
            INSERT INTO text (title, content, published) VALUES ($1, $2, $3)
            RETURNING id
            "#,
            new_text.title,
            new_text.content,
            new_text.published,
        )
        .fetch_one(db)
        .await?
        .id)
    }

    pub async fn update(id: Uuid, text: NewText, db: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Text,
            r#"
            UPDATE text
            SET title = $2, content = $3, published = $4
            WHERE id = $1
            RETURNING *
            "#,
            id,
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
