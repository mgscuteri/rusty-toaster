use std::env;

use actix_web::{get, post, HttpResponse, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    let cwd = env::current_dir().unwrap().display().to_string();
    
    HttpResponse::Ok().body(cwd)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/test")]
async fn test() -> impl Responder {
    println!("Current working directory: {:?}", env::current_dir());
    println!("Serving static files from: {:?}", std::fs::canonicalize("../photos"));

    match std::fs::read("../photos/Aquairium/DSCF0429.JPG") {
        Ok(contents) => {
            println!("File read successfully: {} bytes", contents.len());
            HttpResponse::Ok()
                .content_type("image/jpeg")
                .body(contents)
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            HttpResponse::InternalServerError().body(format!("Error reading file: {}", e))
        }
    }
}
