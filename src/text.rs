use leptos::{expect_context, server, ServerFnError};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[server(GetAllTexts, "/api/text", "GetJson")]
pub async fn get_all_texts(amount: u32) -> Result<Vec<Text>, ServerFnError> {
    let amount = i64::from(amount);
    let db = expect_context::<PgPool>();

    Text::get_all(&db).await.map_err(|x| {
        tracing::error!("problem while fetching home texts: {x:?}");
        ServerFnError::new("Problem while fetching home texts")
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    id: Uuid,
    title: String,
    content: String,
    published: bool,
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
}
