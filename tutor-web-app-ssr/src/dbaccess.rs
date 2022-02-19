use crate::errors::EzyTutorError;
use crate::models::*;
use sqlx::postgres::PgPool;

pub async fn get_user_record(pool: &PgPool, username: String) -> Result<User, EzyTutorError> {
    let user_row = sqlx::query_as!(
        User,
        "SELECT * FROM users where username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user_row {
        Ok(user)
    } else {
        Err(EzyTutorError::NotFound("User name not found".into()))
    }
}

pub async fn post_new_user(pool: &PgPool, new_user: User) -> Result<User, EzyTutorError> {
    let user_row= sqlx::query_as!(User,"insert into users (username, tutor_id, user_password) values ($1,$2,$3) returning username, tutor_id, user_password",
    new_user.username, new_user.tutor_id, new_user.user_password)
    .fetch_one(pool)
    .await?;

    Ok(user_row)
}