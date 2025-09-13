mod e;
use e::User;

use crate::e::UserRelations;
use crate::e::{Post, UserExecutor};

#[tokio::main]
async fn main() {
    let pool = sqlorm::Pool::connect(":memory:").await.unwrap();

    sqlx::query(
        r#"
create table users (
    "id" integer primary key autoincrement,
    "email" text not null unique,
    "username" text not null,
    "first_name" text not null,
    "last_name" text not null,
    "created_at" datetime not null,
    "updated_at" datetime not null
);


create table "posts" (
    id integer primary key autoincrement,
    title text not null,
    body text not null,
    creator_id integer not null, 

    created_at datetime not null,
    updated_at datetime not null,
    foreign key (creator_id) references users("id")
)
"#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let user = User {
        email: "levi@levi.io".to_string(),
        username: "levi".to_string(),
        first_name: "Levi".to_string(),
        last_name: "Levi".to_string(),
        ..Default::default()
    };
    let user = user.save(&pool).await.unwrap();

    for i in 0..5 {
        let post = Post {
            title: i.to_string(),
            creator_id: user.id,
            ..Default::default()
        };
        post.save(&pool).await.unwrap();
    }

    let user = User::query()
        .filter(User::EMAIL.eq("levi@levi.io".into()))
        .with_posts()
        .fetch_one(&pool)
        .await
        .unwrap();

    let posts = &user.posts.unwrap();
    for post in posts {
        let own = post.creator(&pool).await.unwrap();
    }
}
