use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let obj = MyObj{
        name: String::from("John")
    };
    let res = serde_json::to_string(&obj).unwrap();
    HttpResponse::Ok().body(res)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
