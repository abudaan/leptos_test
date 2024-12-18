pub mod test;
pub mod text;

use serde::Deserialize;
pub use text::{NewText, Text};
use uuid::Uuid;

pub type Seconds = i32;

#[derive(Deserialize, Clone, Debug, Copy)]
pub struct PgId {
    pub id: Uuid,
}
