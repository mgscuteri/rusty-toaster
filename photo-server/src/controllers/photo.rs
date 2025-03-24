use std::fs;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

// Define a constant for the photos directory
const PHOTOS_DIR: &str = "../photos";

use tokio::io::AsyncReadExt;

#[get("/albums/{album}/{photo}")]
async fn serve_photo(params: web::Path<(String, String)>) -> impl Responder {
    let (album, photo) = params.into_inner();
    let file_path = format!("{}/{}/{}", PHOTOS_DIR, album, photo);

    println!("Attempting to serve file: {:?}", file_path);

    match tokio::fs::File::open(&file_path).await { // Borrow file_path
        Ok(file) => {
            let stream = Box::pin(futures_util::stream::unfold((file, Vec::new()), move |(mut file, mut buffer)| async move {
                buffer.resize(8192, 0); // Allocate or resize the buffer to 8 KB

                match file.read(&mut buffer).await {
                    Ok(0) => None, // End of file
                    Ok(n) => {
                        buffer.truncate(n); // Keep only the valid portion
                        Some((Ok(web::Bytes::copy_from_slice(&buffer)), (file, Vec::new())))
                    }
                    Err(e) => Some((Err(e), (file, Vec::new()))),
                }
            }));

            HttpResponse::Ok()
                .content_type(mime_guess::from_path(&file_path).first_or_octet_stream().to_string()) // Borrow file_path
                .streaming(stream)
        }
        Err(e) => {
            println!("Error reading file: {:?}", e);
            HttpResponse::NotFound().body(format!("File not found: {:?}", file_path)) // Clone file_path here
        }
    }
}



#[get("/albums")]
async fn list_albums() -> impl Responder {
    println!("Listing albums in directory: {:?}", PHOTOS_DIR);

    match std::fs::read_dir(PHOTOS_DIR) {
        Ok(entries) => {
            let albums: Vec<Album> = entries
                .filter_map(|entry| entry.ok()) // Filter out errors
                .filter_map(|entry| {
                    let album_name = entry
                        .file_name()
                        .into_string()
                        .unwrap_or_else(|_| "Invalid UTF-8 folder name".to_string());

                    let album_path = entry.path();
                
                    // Reuse the helper function to get photos in the album directory
                    let photos = get_photos_in_album(album_path.to_str().unwrap_or(""));

                    // Use the first photo as the thumbnail
                    let thumb_nail = photos.get(0).cloned().unwrap_or_else(|| "".to_string());

                    Some(Album {
                        albumName: album_name,
                        thumbNail: thumb_nail,
                    })
                    
                })
                .collect();

            println!("Final list of albums: {:?}", albums);
            HttpResponse::Ok().json(albums) // Return JSON array of Album objects
        }
        Err(e) => {
            println!("Error accessing directory {:?}: {:?}", PHOTOS_DIR, e);
            HttpResponse::InternalServerError().body(format!("Error accessing photos directory: {:?}", e))
        }
    }
}

#[get("/albums/{album}")]
async fn list_photos(album: web::Path<String>) -> impl Responder {
    let album_path = format!("{}/{}", PHOTOS_DIR, album);

    println!("Constructed album path: {:?}", album_path);

    match std::fs::read_dir(&album_path) {
        Ok(entries) => {
            println!("Successfully accessed directory: {:?}", album_path);
            let photos: Vec<String> = entries
                .filter_map(|entry| {
                    match entry {
                        Ok(e) => {
                            let file = e.file_name();
                            let file_name = file.to_string_lossy();

                            // Debugging file filtering
                            println!("Evaluating entry: {:?}", file_name);

                            // Exclude files starting with "." or ending with ".Identifier"
                            if file_name.starts_with('.') || file_name.ends_with(".Identifier") {
                                println!("Excluding file: {:?}", file_name);
                                None
                            } else {
                                println!("Including file: {:?}", file_name);
                                Some(e)
                            }
                        }
                        Err(err) => {
                            println!("Error reading entry: {:?}", err);
                            None
                        }
                    }
                })
                .map(|entry| {
                    entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8 filename".to_string())
                })
                .collect();

            println!("Final list of files: {:?}", photos);
            HttpResponse::Ok().json(photos) // Return the list of photo filenames as JSON
        }
        Err(e) => {
            println!("Error accessing directory: {:?} - {:?}", album_path, e);
            HttpResponse::NotFound().body(format!("Album not found: {}", album_path))
        }
    }
}


fn get_photos_in_album(album_path: &str) -> Vec<String> {
    match fs::read_dir(album_path) {
        Ok(entries) => {
            entries
                .filter_map(|entry| {
                    match entry {
                        Ok(e) => {
                            let file = e.file_name();
                            let file_name = file.to_string_lossy();

                            // Debugging file filtering
                            println!("Evaluating entry: {:?}", file_name);

                            if !file_name.starts_with("._") {
                                println!("Including file: {:?}", file_name);
                                Some(e)
                            } else {
                                println!("Excluding file: {:?}", file_name);
                                None
                            }
                        }
                        Err(err) => {
                            println!("Error reading entry: {:?}", err);
                            None
                        }
                    }
                })
                .map(|entry| {
                    entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8 filename".to_string())
                })
                .collect()
        }
        Err(err) => {
            println!("Error accessing directory: {:?} - {:?}", album_path, err);
            vec![] // Return an empty vector on error
        }
    }
}


#[derive(Serialize, Debug)]
struct Album {
    albumName: String,
    thumbNail: String,
}