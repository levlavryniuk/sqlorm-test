mod posts;
mod users;

use crate::users::User;
use crate::users::{UserExecutor, UserRelations};

#[tokio::main]
async fn main() {
    let pool = sqlorm::Pool::connect("postgres://test:test@localhost:6666")
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await.expect("Migrate faied");

    let user = User::query()
        .filter(User::EMAIL.eq("levi@levi.io".into()))
        .with_posts()
        .fetch_one(&pool)
        .await
        .unwrap();

    let posts = user.posts.unwrap();
}
