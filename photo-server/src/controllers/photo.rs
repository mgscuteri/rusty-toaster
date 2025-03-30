// use std::{fs, num::NonZeroUsize};

// use actix_web::{get, web, HttpResponse, Responder};
// use serde::Serialize;

// // Define a constant for the photos directory
// const PHOTOS_DIR: &str = "../photos";


// use std::path::Path;

// use dashmap::DashMap;
// use std::sync::Arc;
// use mime_guess;

// // Define a type alias for the file buffer cache
// pub type FileBufferCache = Arc<DashMap<String, Arc<Vec<u8>>>>;


// use bytes::Bytes;

// use tokio::sync::Mutex;
// use std::collections::HashMap;
// use lazy_static::lazy_static;

// lazy_static! {
//     static ref FILE_LOCKS: Mutex<HashMap<String, Mutex<()>>> = Mutex::new(HashMap::new());
// }

// async fn read_file_with_lock(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
//     let mut locks = FILE_LOCKS.lock().await;
//     let file_lock = locks.entry(file_path.to_string()).or_insert_with(|| Mutex::new(()));
//     let _guard = file_lock.lock().await;

//     tokio::fs::read(file_path).await
// }



// #[get("/albums/{album}/{photo}")]
// async fn serve_photo(
//     params: web::Path<(String, String)>,
//     cache: web::Data<FileBufferCache>, // Inject shared cache
// ) -> impl Responder {
//     let (album, photo) = params.into_inner();
//     let file_path = Path::new(PHOTOS_DIR).join(album).join(photo);

//     // Validate the file path
//     if !file_path.starts_with(PHOTOS_DIR) {
//         return HttpResponse::BadRequest().body("Invalid file path.");
//     }

//     let file_key = file_path.to_string_lossy().to_string();

//     // Check if the file is cached
//     if let Some(buffer) = cache.get(&file_key) {
//         return HttpResponse::Ok()
//             .content_type(mime_guess::from_path(&file_path).first_or_octet_stream().to_string())
//             .body(Bytes::from(buffer.as_ref().clone()));
//     }

//     // If the file is not cached, read it with a lock
//     let file_data = match read_file_with_lock(&file_key).await {
//         Ok(data) => Arc::new(data), // Wrap the file data in Arc for caching
//         Err(err) => {
//             eprintln!("Error reading file: {:?}", err);
//             return HttpResponse::NotFound().body("File not found.");
//         }
//     };

//     // Cache the file data
//     cache.insert(file_key.clone(), file_data.clone());

//     // Serve the file
//     HttpResponse::Ok()
//         .content_type(mime_guess::from_path(&file_path).first_or_octet_stream().to_string())
//         .body(Bytes::from(file_data.as_ref().clone()))
// }




// #[get("/albums")]
// async fn list_albums() -> impl Responder {
//     println!("Listing albums in directory: {:?}", PHOTOS_DIR);

//     match std::fs::read_dir(PHOTOS_DIR) {
//         Ok(entries) => {
//             let albums: Vec<Album> = entries
//                 .filter_map(|entry| entry.ok()) // Filter out errors
//                 .filter_map(|entry| {
//                     let album_name = entry
//                         .file_name()
//                         .into_string()
//                         .unwrap_or_else(|_| "Invalid UTF-8 folder name".to_string());

//                     let album_path = entry.path();
                
//                     // Reuse the helper function to get photos in the album directory
//                     let photos = get_photos_in_album(album_path.to_str().unwrap_or(""));

//                     // Use the first photo as the thumbnail
//                     let thumb_nail = photos.get(0).cloned().unwrap_or_else(|| "".to_string());

//                     Some(Album {
//                         albumName: album_name,
//                         thumbNail: thumb_nail,
//                     })
                    
//                 })
//                 .collect();

//             println!("Final list of albums: {:?}", albums);
//             HttpResponse::Ok().json(albums) // Return JSON array of Album objects
//         }
//         Err(e) => {
//             println!("Error accessing directory {:?}: {:?}", PHOTOS_DIR, e);
//             HttpResponse::InternalServerError().body(format!("Error accessing photos directory: {:?}", e))
//         }
//     }
// }

// #[get("/albums/{album}")]
// async fn list_photos(album: web::Path<String>) -> impl Responder {
//     let album_path = format!("{}/{}", PHOTOS_DIR, album);

//     println!("Constructed album path: {:?}", album_path);

//     match std::fs::read_dir(&album_path) {
//         Ok(entries) => {
//             println!("Successfully accessed directory: {:?}", album_path);
//             let photos: Vec<String> = entries
//                 .filter_map(|entry| {
//                     match entry {
//                         Ok(e) => {
//                             let file = e.file_name();
//                             let file_name = file.to_string_lossy();

//                             // Exclude files starting with "." or ending with ".Identifier"
//                             if file_name.starts_with('.') || file_name.ends_with(".Identifier") {
//                                 None
//                             } else {
//                                 Some(e)
//                             }
//                         }
//                         Err(err) => {
//                             println!("Error reading entry: {:?}", err);
//                             None
//                         }
//                     }
//                 })
//                 .map(|entry| {
//                     entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8 filename".to_string())
//                 })
//                 .collect();

//             println!("Final list of files: {:?}", photos);
//             HttpResponse::Ok().json(photos) // Return the list of photo filenames as JSON
//         }
//         Err(e) => {
//             println!("Error accessing directory: {:?} - {:?}", album_path, e);
//             HttpResponse::NotFound().body(format!("Album not found: {}", album_path))
//         }
//     }
// }


// fn get_photos_in_album(album_path: &str) -> Vec<String> {
//     match fs::read_dir(album_path) {
//         Ok(entries) => {
//             entries
//                 .filter_map(|entry| {
//                     match entry {
//                         Ok(e) => {
//                             let file = e.file_name();
//                             let file_name = file.to_string_lossy();

//                             if !file_name.starts_with("._") {
//                                 Some(e)
//                             } else {
//                                 None
//                             }
//                         }
//                         Err(err) => {
//                             println!("Error reading entry: {:?}", err);
//                             None
//                         }
//                     }
//                 })
//                 .map(|entry| {
//                     entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8 filename".to_string())
//                 })
//                 .collect()
//         }
//         Err(err) => {
//             println!("Error accessing directory: {:?} - {:?}", album_path, err);
//             vec![] // Return an empty vector on error
//         }
//     }
// }


// #[derive(Serialize, Debug)]
// struct Album {
//     albumName: String,
//     thumbNail: String,
// }