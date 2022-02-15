use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use tera::Tera;
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 127.0.0.1:8080!");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/templates/**/*"))
        .unwrap();

        App::new()
            .data(tera)
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", "Bob");
    let s = tmpl
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn form(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
        let s = tmpl
        .render("form.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

async fn handle_get_tutors(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let tutors: Vec<Tutor> = vec![
        Tutor {
            name: String::from("Tutor 1"),
        },
        Tutor {
            name: String::from("Tutor 2"),
        },
        Tutor {
            name: String::from("Tutor 3"),
        },
        Tutor {
            name: String::from("Tutor 4"),
        },
        Tutor {
            name: String::from("Tutor 5"),
        },
    ];

    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutors);
    let rendered_html = tmpl
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered_html))
}

async fn handle_post_tutor(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<Tutor>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &params.name);
    ctx.insert("text", "Welcome!");
    let s = tmpl
        .render("user.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/form").route(web::get().to(form)))
            .service(web::resource("/tutors")
                .route(web::post().to(handle_post_tutor))
                .route(web::get().to(handle_get_tutors)))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::{ServiceResponse, Service};
    use actix_web::http::{header::CONTENT_TYPE, HeaderValue, StatusCode};
    use actix_web::web::Form;
    use actix_web::test::{self, TestRequest};

    #[actix_rt::test]
    async fn handle_post_1_unit_test() {
        let params = Form(
            Tutor {
                name: "Terry".to_string()
            }
        );

        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/templates/**/*"
        )).unwrap();

        let webdata_tera = web::Data::new(tera);
        let resp = handle_post_tutor(webdata_tera, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/html")
        );
        resp.body();
    }

    #[actix_rt::test]
    async fn handle_post_1_integration_test() {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/templates/**/*"
        )).unwrap();
        let mut app = test::init_service(
            App::new().data(tera).configure(app_config)
        ).await;

        let req = TestRequest::post()
            .uri("/tutors")
            .set_form(&Tutor {
                name: "Terry".to_string(),
            }).to_request();
        
        let resp: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/html"));
    }
}