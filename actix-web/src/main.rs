use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// This struct represents state
struct AppState {
    app_name: String,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(hello)
            .service(index)
            // .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
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
}
