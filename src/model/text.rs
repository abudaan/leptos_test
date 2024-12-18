#[cfg(feature = "ssr")]
use crate::database::ssr::db;
use leptos::{logging, prelude::ServerFnError, server};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::PgId;

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
    if text.title.is_empty() || text.content.is_empty() {
        Err(ServerFnError::new(
            "Please fill out both title and content!",
        ))
        // Ok("Please fill out both title and content!".to_string())
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
    let r = Text::update(&text, &pool).await;
    match r {
        Ok(_uuid) => Ok("ok".to_string()),
        Err(error) => Ok(error.to_string()),
    }
}

#[server(Delete)]
pub async fn delete(id: Uuid) -> Result<String, ServerFnError> {
    let pool = db().await?;
    let r = Text::delete(id, &pool).await;
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

#[cfg(feature = "ssr")]
impl Text {
    async fn get_all(pool: &PgPool) -> Result<Vec<Text>, sqlx::Error> {
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
