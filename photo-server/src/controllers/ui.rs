// use actix_web::{get, web, HttpResponse, Responder};
// use std::path::PathBuf;

// pub async fn serve_static_file(filename: PathBuf) -> impl Responder {
//     let base_path = PathBuf::from("../ui/dist/photo-lib/browser");
//     let file_path = base_path.join(&filename);

//     println!("Attempting to serve file: {:?}", file_path);

//     if tokio::fs::try_exists(&file_path).await.unwrap_or(false) {
//         match tokio::fs::read(&file_path).await {
//             Ok(contents) => {
//                 let content_type = mime_guess::from_path(&file_path).first_or_octet_stream();
//                 println!("Serving file: {:?} with MIME type: {}", file_path, content_type);
//                 return HttpResponse::Ok()
//                     .content_type(content_type.to_string())
//                     .body(contents);
//             }
//             Err(e) => {
//                 println!("Error reading file: {:?}, error: {:?}", file_path, e);
//                 return HttpResponse::InternalServerError().body("Error reading file");
//             }
//         }
//     }

//     // Fallback to index.html
//     let fallback_path = base_path.join("index.html");
//     println!("File not found, falling back to index.html: {:?}", fallback_path);

//     match tokio::fs::read(&fallback_path).await {
//         Ok(contents) => {
//             println!("Fallback file (index.html) served successfully: {:?}", fallback_path);
//             HttpResponse::Ok()
//                 .content_type("text/html")
//                 .body(contents)
//         }
//         Err(e) => {
//             println!("Error reading fallback file (index.html): {:?}", fallback_path);
//             HttpResponse::InternalServerError().body(format!("Error reading fallback file: {:?}", e))
//         }
//     }
// }


// #[get("/static/{filename:.*}")]
// async fn serve_static(filename: web::Path<String>) -> impl Responder {
//     serve_static_file(PathBuf::from(filename.into_inner())).await
// }