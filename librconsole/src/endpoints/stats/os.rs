use actix_web::{web, post, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::webserver::AppData;

#[derive(Deserialize)]
pub struct OsRequest {
    session_id: String
}

#[derive(Serialize)]
pub struct OsResponse {
    status: i16,
    os:     Option<String>
}

#[post("/stats/os")]
pub async fn post_get_os(data: web::Data<AppData>, form: web::Form<OsRequest>) -> HttpResponse {
    let session_id_check = crate::endpoints::check_session_id(&data, &form.session_id);
    if session_id_check.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if !session_id_check.unwrap() {
        return HttpResponse::Ok().json(OsResponse { status: 401, os: None });
    }

    //We need to allow this because we must assign a value to the String
    //It might be overwritten, but it might not be.
    #[allow(unused_assignments)]
    let mut os: &str = "";

    #[cfg(windows)]
    {
        os = "Windows";
    }

    #[cfg(linux)]
    {
        os = "Linux";
    }

    #[cfg(macos)]
    {
        os = "MacOS";
    }

    #[cfg(any(freebsd, openbsd, netbsd))]
    {
        os = "BSD";
    }
    if os.is_empty() {

        return HttpResponse::Ok().json(OsResponse { status: 200, os: Some("Unknown".to_string())});
    }

    HttpResponse::Ok().json(OsResponse { status: 200, os: Some(os.to_string()) })
}
