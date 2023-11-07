use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// This struct represents state
struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Serialize, Deserialize)]
struct Users {
    id: i32,
    name: String,
    age: i32,
}

async fn index_with_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1; // <- Lock counter

    // format!("Request number: {}", counter) // <- Response with counter
    format!("Request number: {counter}") // <- response with count
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/app-name")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("users")]
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
        Users {
            id: 3,
            name: "Saburo".to_string(),
            age: 22,
        },
    ];
    HttpResponse::Ok().json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        let scope = web::scope("/api").service(show_users);
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(counter.clone())
            .service(hello)
            .service(index)
            // .service(hello)
            .service(echo)
            .service(scope)
            .route("/hey", web::get().to(manual_hello))
            .route("index_with_state", web::get().to(index_with_state))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, http::header::ContentType, test, App, Error};

    use super::*;

    #[actix_web::test]
    async fn test_hello_get() -> Result<(), Error> {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

        Ok(())
    }

    #[actix_web::test]
    async fn test_index_get() -> Result<(), Error> {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    app_name: String::from("Actix Web"),
                }))
                .service(index),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/app-name")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("どうですか{:?}", resp.status());
        assert_eq!(resp.status().is_success(), true);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello Actix Web!"##);

        Ok(())
    }

    #[actix_web::test]
    async fn test_echo_post() -> Result<(), Error> {
        let app = test::init_service(App::new().service(echo)).await;
        let req = test::TestRequest::post()
            .uri("/echo")
            .insert_header(ContentType::plaintext())
            .set_payload("Hello world!")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().is_success(), true);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

        Ok(())
    }

    #[actix_web::test]
    async fn test_manual_hello_get() -> Result<(), Error> {
        let app = test::init_service(App::new().route("/hey", web::get().to(manual_hello))).await;
        let req = test::TestRequest::get().uri("/hey").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().is_success(), true);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hey there!"##);

        Ok(())
    }

    #[actix_web::test]
    async fn test_index_with_state() -> Result<(), Error> {
        let counter = web::Data::new(AppStateWithCounter {
            counter: Mutex::new(0),
        });
        let app = test::init_service(
            App::new()
                .app_data(counter.clone())
                .route("/index_with_state", web::get().to(index_with_state)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/index_with_state")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().is_success(), true);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Request number: 1"##);

        // 2回目のリクエスト
        let req = test::TestRequest::get()
            .uri("/index_with_state")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().is_success(), true);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Request number: 2"##);

        Ok(())
    }
}
