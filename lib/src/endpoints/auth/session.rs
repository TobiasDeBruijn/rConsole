use crate::endpoints::check_session_id;
use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::webserver::AppData;

#[derive(Deserialize)]
pub struct CheckSessionRequest {
    session_id: String
}

#[derive(Serialize)]
pub struct CheckSessionResponse {
    status: i16
}

#[post("/auth/session")]
pub async fn post_check_session(data: web::Data<AppData>, form: web::Form<CheckSessionRequest>) -> HttpResponse {
    let check_session_id_wrapped = check_session_id(&data, &form.session_id);
    if check_session_id_wrapped.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let response = CheckSessionResponse {
        status: if check_session_id_wrapped.unwrap() {
            200
        } else {
            401
        }
    };

    HttpResponse::Ok().json(&response)
}