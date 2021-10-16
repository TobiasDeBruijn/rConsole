use crate::jni::{JvmCommand, Method};
use std::sync::mpsc::Sender;
use std::io;
use jni::sys::{_jobject, jlong};

const BYTE_TO_MB_FACTOR: f64 = 1_000_000f64;
static mut RUNTIME: Option<*mut _jobject> = None;

pub fn get_mem_usage(tx: &Sender<JvmCommand>) -> io::Result<(f64, f64, f64)> {

    //SAFETY: This is safe because we know that this will never be access from multiple threads, so we can do this
    if unsafe { RUNTIME.is_none() } {
        let runtime_class_command = JvmCommand::get_class("java/lang/Runtime");
        let tx_send = tx.send(runtime_class_command.0);
        if tx_send.is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "An error occurred while getting the java.lang.Runtime class"))
        }
        let runtime_class = runtime_class_command.1.recv().unwrap();

        let get_runtime_method = Method::static_method(runtime_class, "getRuntime", "()Ljava/lang/Runtime;", Vec::new());
        let get_runtime_command = JvmCommand::exec_method(get_runtime_method);
        let tx_send = tx.send(get_runtime_command.0);
        if tx_send.is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "An error occurred while executing the static method getRuntime() on java.lang.Runtime"))
        }
        let runtime_object = get_runtime_command.1.recv().unwrap();

        //SAFETY: This is safe because we know that this will never be access from multiple threads, so we can do this
        unsafe { RUNTIME = Some(runtime_object); }
    }

    //SAFETY: This is safe because we know that this will never be access from multiple threads, so we can do this
    let runtime = unsafe { RUNTIME.unwrap() };

    let total_mem_method = Method::method(runtime, "totalMemory", "()J", Vec::new());
    let total_mem_command = JvmCommand::exec_method(total_mem_method);
    let tx_send = tx.send(total_mem_command.0);
    if tx_send.is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "An error occurred while executing the method totalMemory() on java.lang.Runtime"))
    }
    let total_mem_jobject = total_mem_command.1.recv().unwrap();

    let free_mem_method = Method::method(runtime, "freeMemory", "()J", Vec::new());
    let free_mem_command = JvmCommand::exec_method(free_mem_method);
    let tx_send = tx.send(free_mem_command.0);
    if tx_send.is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "An error occurred while executing the method freeMemory() on java.lang.Runtime"))
    }
    let free_mem_jobject = free_mem_command.1.recv().unwrap();

    let max_mem_method = Method::method(runtime, "maxMemory", "()J", Vec::new());
    let max_mem_command = JvmCommand::exec_method(max_mem_method);
    let tx_send = tx.send(max_mem_command.0);
    if tx_send.is_err() {
        return Err(io::Error::new(io::ErrorKind::Other, "An error occurred while executing the method maxMemory() on java.lang.Runtime"))
    }
    let max_mem_jobject = max_mem_command.1.recv().unwrap();

    //the objects are of type *mut _jobject, so is jlong, so we can cast them directly
    //since we know the methods we called return a java long, i.e i64.
    //We then cast them to f64, so we can floating point division when we convert the
    //values from Bytes to Megabytes
    let total_mem = total_mem_jobject as jlong as f64;
    let free_mem = free_mem_jobject as jlong as f64;
    let max_mem = max_mem_jobject as jlong as f64;

    Ok((
        total_mem / BYTE_TO_MB_FACTOR,
        free_mem / BYTE_TO_MB_FACTOR,
        max_mem / BYTE_TO_MB_FACTOR
    ))
}