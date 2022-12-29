use actix_files::NamedFile;
use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./index.html").await.unwrap()
}

fn fibonacci(n: i32) -> i128 {
    let mut previous = 0;
    let mut current = 1;
    if n == 0 {
        current = 0
    } else {
        for _ in 1..n {
            let next = previous + current;
            previous = current;
            current = next;
        }
    }
    current
}

#[get("/fibonacci/{n}")]
async fn get_number(n: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(fibonacci((*n).into()).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(get_number))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
