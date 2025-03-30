use rocket::{
    config::{Config, TlsConfig}, data::{ByteUnit, Limits, ToByteUnit}, fs::NamedFile, get, http::{ContentType, MediaType, Status}, launch, routes, serde::{json::Json, Serialize}, Build, Rocket, State
};
use dashmap::DashMap;
use std::{collections::HashMap, path::{Path, PathBuf}, sync::Arc, fs::OpenOptions, io::Write};
use lazy_static::lazy_static;
use tokio::fs;
use tokio::sync::Mutex;
use mime_guess::{self, Mime};
use bytes::Bytes;
use num_cpus;

// --------------------------
// Global Constants
// --------------------------
const PHOTOS_DIR: &str = "../photos";
const UI_BASE_DIR: &str = "../ui/dist/photo-lib/browser";

// --------------------------
// Global File Cache
// --------------------------
pub type FileBufferCache = Arc<DashMap<String, Arc<Vec<u8>>>>;

#[derive(Clone)]
struct FileCache {
    map: FileBufferCache,
}

impl FileCache {
    fn new() -> Self {
        Self { map: Arc::new(DashMap::new()) }
    }
}

// --------------------------
// File Locking for Synchronized Reads
// --------------------------
lazy_static! {
    static ref FILE_LOCKS: Mutex<HashMap<String, Mutex<()>>> = Mutex::new(HashMap::new());
}

/// Reads a file using a per‑file asynchronous lock.
async fn read_file_with_lock(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut locks = FILE_LOCKS.lock().await;
    let file_lock = locks.entry(file_path.to_string()).or_insert_with(|| Mutex::new(()));
    let _guard = file_lock.lock().await;
    fs::read(file_path).await
}

// --------------------------
// Helper: Convert mime::Mime to Rocket ContentType
// --------------------------
fn content_type_from_mime(mime: &Mime) -> ContentType {
    let mt: MediaType = mime
        .to_string()
        .parse()
        .unwrap_or_else(|_| MediaType::new("application", "octet-stream"));
    ContentType::from(mt)
}

// --------------------------
// Endpoints
// --------------------------
#[get("/albums/<album>/<photo>")]
async fn serve_photo(album: &str, photo: &str, cache: &State<FileCache>) -> Result<(ContentType, Vec<u8>), Status> {
    let file_path = Path::new(PHOTOS_DIR).join(album).join(photo);

    // Validate the file path to guard against directory traversal.
    if !file_path.starts_with(PHOTOS_DIR) {
        return Err(Status::BadRequest);
    }

    // Canonicalize the file path.
    let canonical_file = fs::canonicalize(&file_path)
        .await
        .map_err(|_| Status::NotFound)?;
    let key = canonical_file.to_string_lossy().to_string();

    // Check if the file is in the cache.
    if let Some(cached) = cache.map.get(&key) {
        let mime = mime_guess::from_path(&canonical_file).first_or_octet_stream();
        let ct = content_type_from_mime(&mime);
        return Ok((ct, (**cached.value()).clone()));
    }

    // Not cached: read the file with a lock.
    let data = read_file_with_lock(&key).await.map_err(|_| Status::NotFound)?;
    let arc_data = Arc::new(data.clone());
    cache.map.insert(key, arc_data);

    let mime = mime_guess::from_path(&canonical_file).first_or_octet_stream();
    let ct = content_type_from_mime(&mime);
    Ok((ct, data))
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Album {
    albumName: String,
    thumbNail: String,
}

#[get("/albums")]
fn list_albums() -> Json<Vec<Album>> {
    let mut albums = Vec::new();
    if let Ok(entries) = std::fs::read_dir(PHOTOS_DIR) {
        for entry in entries.filter_map(Result::ok) {
            let album_name = entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8".into());
            let album_path = entry.path();
            let photos = get_photos_in_album(album_path.to_str().unwrap_or(""));
            let thumb = photos.get(0).cloned().unwrap_or_default();
            albums.push(Album { albumName: album_name, thumbNail: thumb });
        }
    }
    Json(albums)
}

#[get("/albums/<album>")]
fn list_photos(album: &str) -> Json<Vec<String>> {
    let album_path = format!("{}/{}", PHOTOS_DIR, album);
    let photos: Vec<String> = std::fs::read_dir(&album_path)
        .map(|iter| {
            iter.filter_map(|e| {
                if let Ok(entry) = e {
                    let s = entry.file_name().into_string().ok()?;
                    if s.starts_with('.') || s.ends_with(".Identifier") {
                        None
                    } else {
                        Some(s)
                    }
                } else {
                    None
                }
            }).collect()
        })
        .unwrap_or_default();
    Json(photos)
}

fn get_photos_in_album(album_path: &str) -> Vec<String> {
    std::fs::read_dir(album_path)
        .map(|iter| {
            iter.filter_map(|e| {
                if let Ok(entry) = e {
                    let s = entry.file_name().into_string().ok()?;
                    if !s.starts_with("._") {
                        Some(s)
                    } else {
                        None
                    }
                } else { None }
            }).collect()
        })
        .unwrap_or_default()
}

#[get("/static/<path..>")]
async fn serve_static(path: PathBuf) -> Option<NamedFile> {
    let base = PathBuf::from(UI_BASE_DIR);
    NamedFile::open(base.join(path)).await.ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(PathBuf::from(UI_BASE_DIR).join("index.html"))
        .await
        .ok()
}

// --------------------------
// Main: Launch Rocket Application
// --------------------------
#[launch]
fn rocket() -> Rocket<Build> {
    // Create the shared file cache.
    let file_cache = FileCache::new();

    // Build Rocket configuration using Figment.
    let config = Config {
        address: "0.0.0.0".parse().unwrap(),
        port: 443,
        workers: num_cpus::get(),        
        // TLS configuration using existing PEM certificates (no new cert required).
        tls: Some(TlsConfig::from_paths("thetoaster_ddns_net.pem", "server.key")),
        ..Config::default()
    };

    rocket::custom(config)
        .mount("/", routes![index, serve_static, serve_photo, list_albums, list_photos])
        .manage(file_cache)
}
