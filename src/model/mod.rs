// mod answer_option;
// mod block;
// mod call;
// mod event;
// mod feedback;
// mod question;
// mod round;
// mod stats;
// mod tag;
pub mod test;
pub mod text;

// pub use answer_option::{AnswerOption, NewAnswerOption};
// pub use block::{Block, NewBlock};
// pub use call::{Call, NewCall};
// pub use event::NewEvent;
// pub use feedback::{Feedback, NewFeedback};
// pub use question::{NewQuestion, Question};
// pub use round::{NewRound, Round};
use serde::{Deserialize, Serialize};
// pub use stats::{parse_date_string, Stats};
// pub use tag::{NewTag, Tag};
pub use text::{NewText, Text};
use uuid::Uuid;

pub type Seconds = i32;

#[derive(Deserialize, Clone, Debug, Copy)]
pub struct PgId {
    pub id: Uuid,
}

/*
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "decision", rename_all = "snake_case")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Decision {
    Approved,
    NotApproved,
    FurtherQuestions,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "round_type", rename_all = "snake_case")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoundType {
    Round1,
    Round2,
    Category,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "round_state", rename_all = "snake_case")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoundState {
    Initial,
    Running,
    Scores,
    Failed,
    Feedback,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, sqlx::Type)]
#[sqlx(type_name = "answer_type", rename_all = "snake_case")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnswerType {
    Correct,
    Incomplete,
    Incorrect,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
pub struct StatRequest {
    region: String,
    role: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Score {
    rank: Option<i64>,
    region: Option<String>,
    role: Option<String>,
    time: Option<i32>,
    user: Uuid,
    score: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderboardResponse {
    scores: Vec<Score>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyRankingResponse {
    rank: Option<i64>,
    before: Vec<Score>,
    after: Vec<Score>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AveragesResponse {
    average_cases_total: Option<i32>,
    average_calls_total: Option<i32>,
    average_cases_filtered: Option<i32>,
    average_calls_filtered: Option<i32>,
}
*/
