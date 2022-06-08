use actix_web::{App,HttpServer,get,post,web,HttpResponse};
use askama::Template;
use serde::Deserialize;
use std::io::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate{
    resolt: isize,
}

//struct IndexTemplate;

//#[get("/index.html")]
//async fn index() -> HttpResponse {
   // let html =  IndexTemplate {};
    //let response_body = html.render().unwrap();
    //HttpResponse::Ok()
    //.content_type("text/html")
    //.body(response_body)
//}

#[derive(Deserialize)]
pub struct Number1 {
    pub number1: isize,
}

#[derive(Deserialize)]
pub struct Number2 {
    pub number2: isize,
}

#[post("/add_method")]
async fn add_method(
    first: web::Form<Number1>,
    second: web::Form<Number2>,
) -> Result<HttpResponse> {
    let first_num: isize = first.parse::<isize>().unwrap();
    let second_num: isize = second.parse::<isize>().unwrap();
    let resolt = first_num + second_num;
    let html = IndexTemplate{resolt};
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
    .content_type("text/html")
    .body(response_body))
}

#[actix_web::main]
async fn main() -> Result<()> {
  // Webサーバを起動
  HttpServer::new(move ||
    App::new()
    .service(add_method)
  )
  .bind("0.0.0.0:8080")?
  .run()
  .await?;

  Ok(())
}