use chrono::{DateTime, Utc};
use sqlorm::table;

#[table(name = "users")]
#[derive(Debug, Clone, Default)]
pub struct User {
    #[sql(pk)]
    #[sql(relation(has_many -> Post, relation ="posts", on = creator_id))]
    pub id: i64,
    #[sql(unique)]
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    #[sql(timestamp(created_at, chrono::Utc::now()))]
    pub created_at: DateTime<Utc>,
    #[sql(timestamp(updated_at, chrono::Utc::now()))]
    pub updated_at: DateTime<Utc>,
}

#[table(name = "posts")]
#[derive(Debug, Clone, Default)]
pub struct Post {
    #[sql(pk)]
    pub id: i64,
    pub title: String,
    pub body: String,
    #[sql(relation(belongs_to -> User, relation = "creator", on = id))]
    pub creator_id: i64,
    #[sql(timestamp(created_at, chrono::Utc::now()))]
    pub created_at: DateTime<Utc>,
    #[sql(timestamp(updated_at, chrono::Utc::now()))]
    pub updated_at: DateTime<Utc>,
}
