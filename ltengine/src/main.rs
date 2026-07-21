use actix_multipart::form::{MultipartForm, text::Text as MPText};
use actix_web::{
    App, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder, get, http::header, post,
    web,
};
use actix_web_static_files::ResourceFiles;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

use actix_multipart::form::tempfile::TempFile;

mod banner;
mod cli;
mod error_response;
mod languages;
mod llm;
mod models;
mod prompt;
mod translation;

use banner::print_banner;
use cli::{Args, Command};
use error_response::ErrorResponse;
use languages::{LANGUAGES, detect_lang};
use models::load_model;
use translation::TranslationRequest;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// Maximum file size: 10MB
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

/// File TTL: 1 hour
const FILE_TTL_SECS: u64 = 3600;

/// Stored file metadata
struct StoredFile {
    filename: String,
    content: Vec<u8>,
    created: Instant,
}

/// In-memory file storage with TTL cleanup
struct FileStore {
    files: HashMap<Uuid, StoredFile>,
}

impl FileStore {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    fn insert(&mut self, filename: String, content: Vec<u8>) -> Uuid {
        let id = Uuid::new_v4();
        self.files.insert(
            id,
            StoredFile {
                filename,
                content,
                created: Instant::now(),
            },
        );
        id
    }

    fn get(&self, id: &Uuid) -> Option<&StoredFile> {
        self.files.get(id)
    }

    fn remove(&mut self, id: &Uuid) -> Option<StoredFile> {
        self.files.remove(id)
    }

    fn cleanup(&mut self) {
        let now = Instant::now();
        self.files.retain(|_, file| {
            now.duration_since(file.created) < Duration::from_secs(FILE_TTL_SECS)
        });
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TranslateRequest {
    q: Option<String>,
    source: Option<String>,
    target: Option<String>,
    format: Option<String>,
    api_key: Option<String>,
    alternatives: Option<u32>,
}

#[derive(MultipartForm)]
struct MPTranslateRequest {
    q: Option<MPText<String>>,
    source: Option<MPText<String>>,
    target: Option<MPText<String>>,
    format: Option<MPText<String>>,
    api_key: Option<MPText<String>>,
    alternatives: Option<MPText<u32>>,
}
impl MPTranslateRequest {
    fn into_translate_request(self) -> TranslateRequest {
        TranslateRequest {
            q: self.q.map(|v| v.into_inner()),
            source: self.source.map(|v| v.into_inner()),
            target: self.target.map(|v| v.into_inner()),
            format: self.format.map(|v| v.into_inner()),
            api_key: self.api_key.map(|v| v.into_inner()),
            alternatives: self.alternatives.map(|v| v.into_inner()),
        }
    }
}

/// Multipart form for file translation
#[derive(MultipartForm)]
struct FileTranslateForm {
    file: TempFile,
    source: Option<MPText<String>>,
    target: Option<MPText<String>>,
    format: Option<MPText<String>>,
}

async fn parse_payload(
    req: HttpRequest,
    payload: web::Payload,
) -> Result<TranslateRequest, ErrorResponse> {
    let content_type = req
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|h| h.to_str().unwrap_or(""))
        .unwrap_or("");
    let body: TranslateRequest;

    if content_type.starts_with("application/json") {
        let json =
            actix_web::web::Json::<TranslateRequest>::from_request(&req, &mut payload.into_inner())
                .await?;
        body = json.into_inner()
    } else if content_type.starts_with("application/x-www-form-urlencoded") {
        let form =
            actix_web::web::Form::<TranslateRequest>::from_request(&req, &mut payload.into_inner())
                .await?;
        body = form.into_inner()
    } else if content_type.starts_with("multipart/form-data") {
        let form =
            MultipartForm::<MPTranslateRequest>::from_request(&req, &mut payload.into_inner())
                .await?;
        body = form.into_inner().into_translate_request();
    } else {
        return Err(ErrorResponse {
            error: "Unsupported content-type".to_string(),
            status: 400,
        });
    }

    Ok(body)
}

fn check_params(
    body: &TranslateRequest,
    args: &Args,
    required_params: &[(&str, &Option<String>)],
) -> Result<bool, ErrorResponse> {
    // Validate required params
    for (key, value) in required_params {
        if value.as_ref().is_none_or(|v| v.trim().is_empty()) {
            return Err(ErrorResponse {
                error: format!("Invalid request: missing {} parameter", key),
                status: 400,
            });
        }
    }

    // Check key
    if !args.api_key.is_empty() && body.api_key.as_ref().is_none_or(|key| *key != args.api_key) {
        return Err(ErrorResponse {
            error: "Invalid API key".to_string(),
            status: 403,
        });
    }

    let q = body.q.as_ref().unwrap();
    if q.len() > args.char_limit {
        return Err(ErrorResponse {
            error: format!(
                "Invalid request: request ({}) exceeds text limit ({})",
                q.len(),
                args.char_limit
            ),
            status: 400,
        });
    }

    Ok(true)
}

#[post("/detect")]
async fn detect(
    req: HttpRequest,
    payload: web::Payload,
    args: web::Data<Arc<Args>>,
) -> Result<HttpResponse, ErrorResponse> {
    let body = parse_payload(req, payload).await?;
    check_params(&body, &args, &[("q", &body.q)])?;

    let q = body.q.unwrap();
    let d = detect_lang(&q);

    Ok(HttpResponse::Ok().json(serde_json::json!([{
        "language": d.language.code,
        "confidence": d.confidence
    }])))
}

#[post("/translate")]
async fn translate(
    req: HttpRequest,
    payload: web::Payload,
    args: web::Data<Arc<Args>>,
    llm: actix_web::web::Data<Arc<llm::LLM>>,
) -> Result<HttpResponse, ErrorResponse> {
    let body = parse_payload(req, payload).await?;
    check_params(
        &body,
        &args,
        &[
            ("q", &body.q),
            ("source", &body.source),
            ("target", &body.target),
        ],
    )?;

    let q = body.q.unwrap();
    let source = body.source.unwrap();
    let target = body.target.unwrap();
    let format = body.format.unwrap_or("text".to_string());
    let result = translation::translate(
        llm.get_ref().as_ref(),
        TranslationRequest {
            text: &q,
            source: &source,
            target: &target,
            format: &format,
        },
    )
    .map_err(ErrorResponse::from)?;

    let mut response = serde_json::json!({"translatedText": result.text});

    // TODO: we just add this for compatibility for now
    // we should allow multiple alternatives to be generated
    if body.alternatives.is_some_and(|v| v > 0) {
        response["alternatives"] = serde_json::json!([]);
    }

    if let Some(detected) = result.detected_language {
        response["detectedLanguage"] = serde_json::json!({
            "language": detected.code,
            "confidence": detected.confidence
        });
    }

    Ok(HttpResponse::Ok().json(response))
}

#[post("/translate_file")]
async fn translate_file(
    MultipartForm(form): MultipartForm<FileTranslateForm>,
    args: web::Data<Arc<Args>>,
    llm: actix_web::web::Data<Arc<llm::LLM>>,
    file_store: web::Data<Arc<Mutex<FileStore>>>,
) -> Result<HttpResponse, ErrorResponse> {
    // Validate required parameters
    let source = form.source.as_ref().ok_or_else(|| ErrorResponse {
        error: "Invalid request: missing source parameter".to_string(),
        status: 400,
    })?;
    let source = source.as_str().trim();

    let target = form.target.as_ref().ok_or_else(|| ErrorResponse {
        error: "Invalid request: missing target parameter".to_string(),
        status: 400,
    })?;
    let target = target.as_str().trim();

    let format = form
        .format
        .as_ref()
        .map(|f| f.as_str().trim())
        .unwrap_or("text");

    // Get file content - read from the temp file
    use std::io::Read;
    let mut file = std::fs::File::open(form.file.file.path()).map_err(|_| ErrorResponse {
        error: "Failed to open uploaded file".to_string(),
        status: 500,
    })?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)
        .map_err(|_| ErrorResponse {
            error: "Failed to read uploaded file".to_string(),
            status: 500,
        })?;

    if file_content.len() > MAX_FILE_SIZE {
        return Err(ErrorResponse {
            error: format!("File too large. Maximum size is {} bytes", MAX_FILE_SIZE),
            status: 400,
        });
    }

    // Get filename and validate extension
    let filename = form
        .file
        .file_name
        .unwrap_or_else(|| "file.txt".to_string());
    let extension = filename.rsplit('.').next().unwrap_or("").to_lowercase();

    // Only support .txt files initially
    if extension != "txt" {
        return Err(ErrorResponse {
            error: "Unsupported file format. Only .txt files are supported".to_string(),
            status: 400,
        });
    }

    // Convert file content to string
    let text_content = String::from_utf8(file_content.to_vec()).map_err(|_| ErrorResponse {
        error: "Invalid file encoding. Expected UTF-8 text file".to_string(),
        status: 400,
    })?;

    // Check text limit
    if text_content.len() > args.char_limit {
        return Err(ErrorResponse {
            error: format!(
                "File content ({}) exceeds text limit ({})",
                text_content.len(),
                args.char_limit
            ),
            status: 400,
        });
    }

    let result = translation::translate(
        llm.get_ref().as_ref(),
        TranslationRequest {
            text: &text_content,
            source,
            target,
            format,
        },
    )
    .map_err(ErrorResponse::from)?;

    // Store translated file
    let mut store = file_store.lock().map_err(|_| ErrorResponse {
        error: "Internal server error".to_string(),
        status: 500,
    })?;

    let translated_filename = filename.replace(".txt", "_translated.txt");
    let file_id = store.insert(translated_filename, result.text.into_bytes());

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "translatedFileUrl": format!("/download/{}", file_id)
    })))
}

#[get("/download/{id}")]
async fn download_file(
    path: web::Path<Uuid>,
    file_store: web::Data<Arc<Mutex<FileStore>>>,
) -> Result<HttpResponse, ErrorResponse> {
    let id = path.into_inner();
    let store = file_store.lock().map_err(|_| ErrorResponse {
        error: "Internal server error".to_string(),
        status: 500,
    })?;

    let file = store.get(&id).ok_or_else(|| ErrorResponse {
        error: "File not found or expired".to_string(),
        status: 404,
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file.filename),
        )
        .body(file.content.clone()))
}

#[post("/suggest")]
async fn suggest() -> Result<HttpResponse, ErrorResponse> {
    Err(ErrorResponse {
        error: "Not implemented".to_string(),
        status: 501,
    })
}

#[get("/languages")]
async fn get_languages() -> impl Responder {
    HttpResponse::Ok().json(&*LANGUAGES)
}

#[get("/frontend/settings")]
async fn get_frontend_settings(args: web::Data<Arc<Args>>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "apiKeys": false,
        "charLimit": args.char_limit,
        "filesTranslation": true,
        "frontendTimeout": 1000,
        "keyRequired": false,
        "language": {
            "source": {
                "code": "auto",
                "name": "Auto Detect"
            },
            "target": {
                "code": "en",
                "name": "English"
            }
        },
        "suggestions": false,
        "supportedFilesFormat": ["txt"]
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let model_path = load_model(&args.model, &args.model_file).unwrap_or_else(|err| {
        eprintln!("Failed to load model: {}", err);
        std::process::exit(1);
    });

    eprintln!("Loading model: {}", model_path.display());

    let llm = Arc::new(
        llm::LLM::new(model_path, args.cpu, args.verbose).unwrap_or_else(|err| {
            eprintln!("Failed to initialize LLM: {}", err);
            std::process::exit(1);
        }),
    );

    if let Some(Command::Translate(command)) = &args.command {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        cli::run_translate(command, llm.as_ref(), stdin.lock(), stdout.lock()).unwrap_or_else(
            |error| {
                eprintln!("Error: {error:#}");
                std::process::exit(1);
            },
        );
        return Ok(());
    }

    let args = Arc::new(args);
    let host = args.host.clone();
    let port = args.port;

    print_banner();

    let file_store = Arc::new(Mutex::new(FileStore::new()));

    let server = HttpServer::new(move || {
        let generated = generate();

        App::new()
            .app_data(web::Data::new(llm.clone()))
            .app_data(web::Data::new(args.clone()))
            .app_data(web::Data::new(file_store.clone()))
            .service(get_languages)
            .service(get_frontend_settings)
            .service(translate)
            .service(translate_file)
            .service(download_file)
            .service(detect)
            .service(suggest)
            .service(ResourceFiles::new("/", generated))
    })
    .bind((host.clone(), port))?
    .run();

    println!("Running on: http://{}:{}", host, port);

    return server.await;
}
