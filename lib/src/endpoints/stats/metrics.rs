use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::metrics::MetricsCollection;
use crate::webserver::AppData;

#[derive(Deserialize)]
pub struct GetMetricsRequest {
    session_id: String,
}

#[derive(Serialize)]
pub struct GetMetricsResponse {
    status:     i16,
    metrics:    Option<Vec<MetricsEntry>>
}

#[derive(Serialize)]
pub struct MetricsEntry {
    epoch:  i64,
    #[serde(flatten)]
    metric: MetricsCollection
}

#[post("/stats/metrics")]
pub async fn get_metrics(data: web::Data<AppData>, form: web::Form<GetMetricsRequest>) -> HttpResponse {
    let session_id_check = crate::endpoints::check_session_id(&data, &form.session_id);
    if session_id_check.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if !session_id_check.unwrap() {
        return HttpResponse::Ok().json(GetMetricsResponse { status: 401, metrics: None });
    }

    let mut metrics: Vec<MetricsEntry> = Vec::new();
    for (k, v) in crate::metrics::METRICS.pin().iter() {
        metrics.push(MetricsEntry { epoch: *k, metric: v.clone()})
    }

    HttpResponse::Ok().json(GetMetricsResponse { status: 200, metrics: Some(metrics)})
}