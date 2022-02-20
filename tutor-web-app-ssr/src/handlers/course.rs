use actix_web::{web, Error, HttpResponse, Result};
use crate::state::AppState;
use crate::models::{NewCourse, NewCourseResponse, UpdateCourse, UpdateCourseResponse};
use serde_json::json;

pub async fn handle_insert_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    web::Path(tutor_id): web::Path<i32>,
    params: web::Json<NewCourse>,
) -> Result<HttpResponse, Error> {
    let new_course = json!({
        "tutor_id": tutor_id,
        "course_name": &params.course_name,
        "course_description": &params.course_description,
        "course_format": &params.course_format,
        "course_duration": &params.course_duration,
        "course_price": &params.course_price,
        "course_language": &params.course_language,
        "course_level": &params.course_level,
    });
    let awc_client = awc::Client::default();
    let res = awc_client
        .post("http://localhost:3000/courses/")
        .send_json(&new_course)
        .await?
        .body()
        .await?;
    println!("Finished call: {:?}", res);
    let course_response: NewCourseResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(course_response))
}

pub async fn handle_update_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
    web::Path((tutor_id, course_id)): web::Path<(i32, i32)>,
    params: web::Json<UpdateCourse>,
) -> Result<HttpResponse, Error> {
    let update_course = json!({
        "course_name": &params.course_name,
        "course_description": &params.course_description,
        "course_format": &params.course_format,
        "course_duration": &params.course_duration,
        "course_structure": &params.course_structure,
        "course_price": &params.course_price,
        "course_language": &params.course_language,
        "course_level": &params.course_level,
    });

    let awc_client = awc::Client::default();
    let update_url = format!("http://localhost:3000/courses/{}/{}", tutor_id, course_id);
    println!("{}", update_url);
    let res = awc_client
        .put(update_url)
        .send_json(&update_course)
        .await?
        .body()
        .await?;
    println!("{:?}", res);
    let course_response: UpdateCourseResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
    Ok(HttpResponse::Ok().json(course_response))
}

pub async fn handle_delete_course(
    _tmpl: web::Data<tera::Tera>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Got delete request"))
}