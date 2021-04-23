use jni::JNIEnv;
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::jni::util::str_to_jvalue;
use jni::sys::{_jobject, jclass, jobject};
use crate::jni::logging::{ConsoleLogItem, LogLevel, log_info, log_warn, log_debug};
use jni::objects::{JValue, JObject, JClass};

pub mod logging;
pub mod util;

mod web_server_native;

/**
Describes an action to execute on the JVM
*/
pub struct JvmCommand {
    intent:         Intent,
    response_tx:    Option<Sender<*mut _jobject>>,
}

unsafe impl Sync for JvmCommand {}
unsafe impl Send for JvmCommand {}

impl JvmCommand {
    /**
    Log a ConsoleLogItem to the console
    */
    pub fn log(log: ConsoleLogItem) -> JvmCommand {
        let intent = Intent::Log(log);

        JvmCommand {
            intent,
            response_tx: None
        }
    }

    /**
    Get a Java class
    */
    pub fn get_class(class_name: &str) -> (JvmCommand, Receiver<*mut _jobject>) {
        let (tx, rx): (Sender<*mut _jobject>, Receiver<*mut _jobject>) = channel();
        let intent = Intent::GetClass(class_name.to_string());

        let command = JvmCommand {
            intent,
            response_tx: Some(tx)
        };

        (command, rx)
    }

    /**
    Execute a Method where receiving the return value is desired
    */
    #[allow(dead_code)]
    pub fn exec_method(method: Method) -> (JvmCommand, Receiver<*mut _jobject>) {
        let (tx, rx): (Sender<*mut _jobject>, Receiver<*mut _jobject>) = channel();
        let intent = Intent::ExecMethod(method);

        let command = JvmCommand {
            intent,
            response_tx: Some(tx)
        };

        (command, rx)
    }

    /**
    Execute a Method where receiving a return value isn't desired, or not necessary
    */
    pub fn exec_method_no_return(method: Method) -> JvmCommand {
        let intent = Intent::ExecMethod(method);
        JvmCommand {
            intent,
            response_tx: None
        }
    }
}

/**
Describes the intention of the JvmCommand
*/
pub enum Intent {
    /// Describes a logging intention
    Log(ConsoleLogItem),

    ///Describes the intention to get a class by it's name
    GetClass(String),

    ///Describes the intention to execute (i.e call) a method
    ExecMethod(Method)
}

/**
Describes a Java Method
*/
pub struct Method {
    ///The class in which the method lives. This only has to be provided for static method calls
    class:  Option<jclass>,

    ///The object on which to call the method
    obj:    Option<jobject>,

    ///The name of the method
    name:   String,

    ///The method signature. See: [https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/types.html](Oracle docs: Type Signatures)
    sig:    String,

    ///The Arguments to be passed to the method. The order should be the same as provided in the method's signature. If no arguments are needed, an empty Vec should be provided
    args:   Vec<Argument>
}

impl Method {
    /**
    Get a Method for a static method

    ## Parameters
        class: The class in which the method lives
        name: The name of the method
        sig: The signature of the method. See: [https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/types.html](Oracle docs: Type Signatures)
        args: The Arguments to be passed to the method. The order should be the same as provided in the method's signature. If no arguments are needed, an empty Vec should be provided
    */
    pub fn static_method(class: jclass, name: &str, sig: &str, args: Vec<Argument>) -> Method {
        Method {
            class: Some(class),
            obj: None,
            name: name.to_string(),
            sig: sig.to_string(),
            args
        }
    }

    /**
    Get a Method for a non-static method

    ## Parameters
        obj: The object on which to invoke the method
        name: The name of the method
        sig: The signature of the method. See: [https://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/types.html](Oracle docs: Type Signatures)
        args: The Arguments to be passed to the method. The order should be the same as provided in the method's signature. If no arguments are needed, an empty Vec should be provided
    */
    #[allow(dead_code)]
    pub fn method(obj: jobject, name: String, sig: String, args: Vec<Argument>) -> Method {
        Method {
            class: None,
            obj: Some(obj),
            name,
            sig,
            args
        }
    }
}

///An enum describing a Method's arguments
#[allow(dead_code)]
pub enum Argument {
    ///A java.lang.String type argument
    String(String),

    ///A type extending java.lang.Object (this is most things)
    JObject(jobject),

    //TODO primitives
}

pub fn jvm_command_exec(env: JNIEnv, rx: Receiver<JvmCommand>) {
    loop {
        let received_cmd = rx.recv().unwrap();
        match received_cmd.intent {
            Intent::Log(log_item) => {
                match log_item.level {
                    LogLevel::Info => {
                        log_info(&env, &log_item.log);
                    },
                    LogLevel::Warn => {
                        log_warn(&env, &log_item.log);
                    },
                    LogLevel::Debug => {
                        log_debug(&env, &log_item.log);
                    }
                }
            },
            Intent::GetClass(class_name) => {
                let class_jclass = env.find_class(&class_name).expect(&format!("Unable to find class '{}'", &class_name));
                received_cmd.response_tx.unwrap().send(**class_jclass).expect("An issue occurred while sending the response of a JvmCommand");
            },
            Intent::ExecMethod(method) => {
                let mut args_as_jvalues: Vec<JValue> = Vec::new();

                for arg in method.args {
                    match arg {
                        Argument::String(string) => {
                            let string_as_jvalue = str_to_jvalue(&env, &string);
                            args_as_jvalues.push(string_as_jvalue);
                        },
                        Argument::JObject(jobject) => {
                            let object_as_jobject = JObject::from(jobject);
                            let jobject_as_jvalue = JValue::Object(object_as_jobject);

                            args_as_jvalues.push(jobject_as_jvalue);
                        }
                    }
                }

                //Object is given, meaning this is not a static method
                let return_val = if method.obj.is_some() {
                    env.call_method(method.obj.unwrap(), &method.name, method.sig, args_as_jvalues.as_slice()).expect(&format!("An error occurred while executing the method '{}'", &method.name))
                } else {
                    let class = JClass::from(method.class.unwrap());
                    env.call_static_method(class, &method.name, method.sig, args_as_jvalues.as_slice()).expect(&format!("An error occurred while executing the method '{}'", &method.name))
                };

                if received_cmd.response_tx.is_some() {
                    received_cmd.response_tx.unwrap().send(unsafe { return_val.to_jni().l }).expect("An issue occurred while sending the response of a JvmCommand");
                }
            }
        }
    }
}