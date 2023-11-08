use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod get;

// This struct represents state
struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index_with_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1; // <- Lock counter

    // format!("Request number: {}", counter) // <- Response with counter
    format!("Request number: {counter}") // <- response with count
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    log::info!("starting HTTP server at http://localhost:9999");

    HttpServer::new(move || {
        App::new()
            .configure(get::get_config)
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(counter.clone())
            .service(index)
            // .service(hello)
            .service(echo)
            .route("index_with_state", web::get().to(index_with_state))
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, http::header::ContentType, test, App, Error};

    use super::*;

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
