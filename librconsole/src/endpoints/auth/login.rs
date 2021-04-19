use crate::Database;
use crate::webserver::AppData;
use crate::jni::logging::{log, LogLevel};

use rusqlite::{named_params, OptionalExtension};
use actix_web::{web, post, HttpResponse};
use serde::{Serialize, Deserialize};
use sha2::{Sha512Trunc256, Digest};
use std::path::PathBuf;
use rand::Rng;

#[derive(Deserialize)]
pub struct LoginRequest {
    username_base64:    String,
    password_base64:    String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    status:     i16,
    session_id: Option<String>
}

#[post("/auth/login")]
pub async fn post_login(data: web::Data<AppData>, form: web::Form<LoginRequest>) -> HttpResponse {
    let password = String::from_utf8(base64::decode(form.password_base64.as_bytes()).unwrap()).unwrap();
    let username = String::from_utf8(base64::decode(form.username_base64.as_bytes()).unwrap()).unwrap();

    let db = Database::new(PathBuf::from(data.db_path.clone())).unwrap();
    let sql_get_salt_result = db.connection.query_row("SELECT salt FROM users WHERE username = :username", named_params! {
        ":username": &username
    }, |row| row.get(0)).optional();

    if sql_get_salt_result.is_err() {
        let tx = data.log_tx.lock().unwrap();
        log(&tx, LogLevel::Warn, &format!("An error occurred while fetching a user's salt: {:?}", sql_get_salt_result.err().unwrap()));

        return HttpResponse::InternalServerError().finish();
    }

    let sql_get_salt_option = sql_get_salt_result.unwrap();
    if sql_get_salt_option.is_none() {
        return HttpResponse::Ok().json(LoginResponse {status: 401, session_id: None});
    }

    let salt: String = sql_get_salt_option.unwrap();

    let mut hasher = Sha512Trunc256::new();
    hasher.update(&password);
    hasher.update(&salt);
    hasher.update(&data.config.pepper);

    let password_hashed = base64::encode(hasher.finalize());
    let password_bcrypt = bcrypt::hash_with_salt(&password_hashed, 10, &salt.as_bytes()).unwrap();
    let password_finalized = password_bcrypt.format_for_version(bcrypt::Version::TwoY);

    let sql_verify_user = db.connection.query_row("SELECT user_id FROM users WHERE username = :username and hash = :hash", named_params! {
        ":username": &username,
        ":hash":     &password_finalized
    }, |row| row.get(0));

    if sql_verify_user.is_err() {
        let tx = data.log_tx.lock().unwrap();
        log(&tx, LogLevel::Warn, &format!("An error occurred while verifying a user's password: {:?}", sql_verify_user.err().unwrap()));

        return HttpResponse::InternalServerError().finish();
    }

    let session_id: String = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(16).map(char::from).collect();
    let expiry = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp();
    let user_id: String = sql_verify_user.unwrap();

    let sql_insert_session_id = db.connection.execute("INSERT INTO sessions (session_id, user_id, expiry) VALUES (:session_id, :user_id, :expiry)", named_params! {
        ":session_id": &session_id,
        ":user_id": user_id,
        ":expiry": expiry
    });

    if sql_insert_session_id.is_err() {
        let tx = data.log_tx.lock().unwrap();
        log(&tx, LogLevel::Warn, &format!("An error occurred while inserting a new session_id: {:?}", sql_insert_session_id.err().unwrap()));

        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().json(LoginResponse { status: 200, session_id: Some(session_id)})
}