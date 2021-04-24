use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::webserver::AppData;
use crate::endpoints::check_session_id;
use crate::jni::{JvmCommand, Method};
use jni::sys::jlong;

#[derive(Deserialize)]
pub struct GetMemRequest {
    session_id: String
}

#[derive(Serialize)]
pub struct GetMemResponse {
    status:     i16,
    total_mem:  Option<f64>,
    free_mem:   Option<f64>,
    max_mem:    Option<f64>
}

#[post("/stats/mem")]
pub fn post_get_mem(data: web::Data<AppData>, form: web::Form<GetMemRequest>) -> HttpResponse {
    let session_id_valid = check_session_id(&data, &form.session_id);
    if session_id_valid.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if !session_id_valid.unwrap() {
        return HttpResponse::Ok().json(GetMemResponse { status: 401, free_mem: None, total_mem: None, max_mem: None });
    }

    let tx = data.jvm_command_tx.lock().unwrap();

    let runtime_class_command = JvmCommand::get_class("java/lang/Runtime");
    tx.send(runtime_class_command.0).expect("An error occurred while sending the JvmCommand 'runtime_class_command'");
    let runtime_class = runtime_class_command.1.recv().unwrap();

    let get_runtime_method = Method::static_method(runtime_class, "getRuntime", "()Ljava/lang/Runtime;", Vec::new());
    let get_runtime_command = JvmCommand::exec_method(get_runtime_method);
    tx.send(get_runtime_command.0).expect("An error occurred while sending the JvmCommand 'get_runtime_command'");
    let runtime_object = get_runtime_command.1.recv().unwrap();

    let total_mem_method = Method::method(runtime_object, "totalMemory", "()J", Vec::new());
    let total_mem_command = JvmCommand::exec_method(total_mem_method);
    tx.send(total_mem_command.0).expect("An error occurred while sending the JvmCommand 'total_mem_command'");
    let total_mem_jobject = total_mem_command.1.recv().unwrap();

    let free_mem_method = Method::method(runtime_object, "freeMemory", "()J", Vec::new());
    let free_mem_command = JvmCommand::exec_method(free_mem_method);
    tx.send(free_mem_command.0).expect("An error occurred while sending the JvmCommand 'free_mem_command'");
    let free_mem_jobject = free_mem_command.1.recv().unwrap();

    let max_mem_method = Method::method(runtime_object, "maxMemory", "()J", Vec::new());
    let max_mem_command = JvmCommand::exec_method(max_mem_method);
    tx.send(max_mem_command.0).expect("An error occurred while sending the JvmCommand 'max_mem_command'");
    let max_mem_jobject = max_mem_command.1.recv().unwrap();

    //the objects are of type *mut _jobject, so is jlong, so we can cast them directly
    //since we know the methods we called return a java long, i.e i64.
    //We then cast them to f64, so we can floating point division when we convert the
    //values from Bytes to Megabytes
    let total_mem = total_mem_jobject as jlong as f64;
    let free_mem = free_mem_jobject as jlong as f64;
    let max_mem = max_mem_jobject as jlong as f64;

    const BYTE_TO_MB_FACTOR: f64 = 1_000_000f64;

    //Some more information about the memory metrics, see: https://stackoverflow.com/a/18375641/10765090
    let response = GetMemResponse {
        status: 200,
        total_mem: Some(total_mem / BYTE_TO_MB_FACTOR),
        free_mem: Some(free_mem / BYTE_TO_MB_FACTOR),
        max_mem: Some(max_mem / BYTE_TO_MB_FACTOR)
    };

    HttpResponse::Ok().json(response)
}