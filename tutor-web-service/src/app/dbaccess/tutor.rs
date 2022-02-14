use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use sqlx::postgres::{PgPool};

pub async fn get_all_tutors_db(
    pool: &PgPool
) -> Result<Vec<Tutor>, EzyTutorError> {
    Ok(sqlx::query_as!(
        Tutor,
        "SELECT * FROM ezy_tutor_c6",
    )
    .fetch_all(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("No tutors found".into()))?)
}

pub async fn get_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Tutor, EzyTutorError> {
    Ok(sqlx::query_as!(
        Tutor,
        "SELECT * FROM ezy_tutor_c6 WHERE tutor_id = $1",
        tutor_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?)
}

pub async fn post_new_tutor_db(
    pool: &PgPool,
    new_tutor: NewTutor,
) -> Result<Tutor, EzyTutorError> {
    Ok(sqlx::query_as!(
        Tutor,
        "INSERT INTO ezy_tutor_c6(tutor_id, tutor_name, tutor_pic_url, tutor_profile) \
        VALUES (DEFAULT, $1, $2, $3) RETURNING \
        tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor.tutor_name, new_tutor.tutor_pic_url, new_tutor.tutor_profile,
    )
    .fetch_one(pool)
    .await
    .map_err(|err| EzyTutorError::NotFound(format!("{:?}", err)))?)
}

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    update_tutor: UpdateTutor,
) -> Result<Tutor, EzyTutorError> {
    let current_tutor_info = sqlx::query_as!(
        Tutor,
        "SELECT * FROM ezy_tutor_c6 WHERE tutor_id = $1",
        tutor_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound(format!("Tutor with id {} not found", tutor_id)))?;

    let new_info = Tutor {
        tutor_id: current_tutor_info.tutor_id,
        tutor_name: update_tutor.tutor_name.unwrap_or(current_tutor_info.tutor_name),
        tutor_pic_url: update_tutor.tutor_pic_url.unwrap_or(current_tutor_info.tutor_pic_url),
        tutor_profile: update_tutor.tutor_profile.unwrap_or(current_tutor_info.tutor_profile),
    };

    Ok(sqlx::query_as!(
        Tutor,
        "UPDATE ezy_tutor_c6 set tutor_name = $1, tutor_pic_url = $2, tutor_profile = $3 \
        WHERE tutor_id = $4 \
        RETURNING tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_info.tutor_name, new_info.tutor_pic_url, new_info.tutor_profile, new_info.tutor_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound(format!("Failed to update tutor with ID {}", tutor_id)))?)
}

pub async fn delete_tutor_db(
    pool: &PgPool,
    tutor_id: i32
) -> Result<String, EzyTutorError> {
    Ok(sqlx::query!(
        "DELETE FROM ezy_tutor_c6 WHERE tutor_id = $1",
        tutor_id,
    )
    .execute(pool)
    .await
    .map(|count| format!("Deleted {:?} tutor(s)", count))
    .map_err(|_err| EzyTutorError::NotFound(format!("Tutor with id {} not found", tutor_id)))?)
}
