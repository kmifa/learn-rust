use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Users {
    id: i32,
    name: String,
    age: i32,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/users")]
async fn show_users() -> impl Responder {
    let users = vec![
        Users {
            id: 1,
            name: "Taro".to_string(),
            age: 20,
        },
        Users {
            id: 2,
            name: "Jiro".to_string(),
            age: 21,
        },
    ];
    HttpResponse::Ok().json(users)
}

pub fn get_config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/get")
        .service(hello)
        .service(show_users)
        .route("hey", web::get().to(manual_hello));

    cfg.service(
        web::resource("/get")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(scope);
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, http::header::ContentType, test, App, Error};

    use super::*;

    #[actix_web::test]
    async fn test_hello_get() -> Result<(), Error> {
        let scope = web::scope("/get").service(hello);
        let app = test::init_service(App::new().service(scope)).await;

        let req = test::TestRequest::get()
            .uri("/get/hello")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

        Ok(())
    }

    #[actix_web::test]
    async fn test_manual_hello_get() -> Result<(), Error> {
        let app =
            test::init_service(App::new().route("/get/hey", web::get().to(manual_hello))).await;
        let req = test::TestRequest::get().uri("/get/hey").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().is_success(), true);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hey there!"##);

        Ok(())
    }

    #[actix_web::test]
    async fn test_scope_api() {
        let scope = web::scope("/get").service(show_users);
        let app = test::init_service(App::new().service(scope)).await;

        let req = test::TestRequest::get().uri("/get/users").to_request();
        let resp = test::call_service(&app, req).await;

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            body_bytes,
            r#"[{"id":1,"name":"Taro","age":20},{"id":2,"name":"Jiro","age":21}]"#
        );
    }
}
