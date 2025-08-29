use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use errors::AppError;
use rand::Rng;
use rand_distr::Alphanumeric;
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use url::Url;

mod errors;

//API request struct
#[derive(Debug, Deserialize)]
struct Request {
    original_url: String,
}

//API response struct
#[derive(Debug, Serialize)]
struct Response {
    short_url: String,
}

//short code generation function
fn generate_short_code() -> String {
    return rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
}

//async function for handling URL creation and Http Response
#[post("/shorten")]
async fn shorten_url(
    pool: web::Data<SqlitePool>,
    req: web::Json<Request>
) -> Result<impl Responder, AppError> {

    //url validation
    if Url::parse(&req.original_url).is_err() {
        return Err(AppError::InvalidUrl);
    }

    //check if url already has a short match in database
    let query_row = sqlx::query!("SELECT short_code FROM mappings WHERE original_url = ?", req.original_url)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(AppError::SqlxError)?;
    
    if let Some(row) = query_row {
        if let Some(short_code) = row.short_code {
            let short_url = format!("http://localhost:8080/{}", short_code);
            return Ok(HttpResponse::Ok().json(Response {short_url}));
        }
        
        else {
            return Err(AppError::SqlxError(sqlx::Error::RowNotFound));
        }
    }

    //generate short url and check if unique in database
    let mut short_code;
    loop {
        short_code = generate_short_code();

        let query_result = sqlx::query!("INSERT INTO mappings (short_code, original_url) VALUES (?, ?)",
            short_code,
            req.original_url
        )
        .execute(pool.get_ref())
        .await;

        if query_result.is_ok() {
            break;
        }

        if let Err(sqlx::Error::Database(db_error)) = &query_result {
            if db_error.is_unique_violation() {
                continue;
            }
        }

        query_result.map_err(AppError::SqlxError)?;
    }

    let short_url = format!("http://localhost:8080/{}", short_code);
    Ok(HttpResponse::Ok().json(Response {short_url}))
}

//async function for redirect handling
#[get("/{short_code}")]
async fn redirect(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>
) -> Result<impl Responder, AppError> {

    let short_code = path.into_inner();

    //find short code url pair
    let query_row = sqlx::query!("SELECT original_url FROM mappings WHERE short_code = ?", short_code)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(AppError::SqlxError)?;

    if let Some(row) = query_row {
        Ok(HttpResponse::Found()
            .insert_header(("Location", row.original_url))
            .finish())
    }

    else {
        Err(AppError::NotFound)
    }
}

//async main function
#[actix_web::main]
async fn main() -> std::io::Result<()>{

    //create pool connection for table connection
    let pool = SqlitePool::connect("sqlite:url_mappings.db")
        .await
        .expect("Pool creation failed.");

    sqlx::query(
            "CREATE TABLE IF NOT EXISTS mappings (
                short_code TEXT PRIMARY KEY,
                original_url TEXT NOT NULL UNIQUE
            )",
        )
        .execute(&pool)
        .await
        .expect("Table creation failed.");

    //create server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(shorten_url)
            .service(redirect)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}