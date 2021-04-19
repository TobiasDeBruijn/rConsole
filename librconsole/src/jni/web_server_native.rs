use crate::jni::logging::{log_warn, log_debug, log_info, ConsoleLogItem, logging_rec};
use crate::config::Config;
use crate::LogEntry;

use jni::JNIEnv;
use jni::objects::{JString, JClass, JValue, JObject};
use jni::sys::{jlong, jboolean, jobjectArray, jobject};
use std::path::PathBuf;
use std::sync::mpsc::{Sender, Receiver};
use rand::Rng;
use sha2::{Sha512Trunc256, Digest};
use rusqlite::{named_params, OptionalExtension, Connection};
use std::collections::HashMap;

/**
 * Class:     nl.thedutchmc.rconsole.webserver.Native
 * Method:    startWebServer(String configFolder) -> void
 * Signature: (Ljava/lang/String;)V
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_startWebServer(env: JNIEnv, _class: JClass, config_file_path_jstring: JString, database_file_path_jstring: JString) {
    log_info(&env, "Loading library librconsole");

    let config_file_path: String = env.get_string(config_file_path_jstring).expect("Unable to get String from JString 'config_folder_jstring'").into();
    let database_file_path: String = env.get_string(database_file_path_jstring).expect("Unable to get String from JString 'database_file_path_jstring'").into();

    //Load the configuration file
    let config_wrapped = Config::load(PathBuf::from(config_file_path));
    if config_wrapped.is_err() {
        log_warn(&env, "Unable to load configuration file.");
        log_debug(&env, &config_wrapped.err().unwrap());
        return;
    }
    let config = config_wrapped.unwrap();
    log_info(&env, "Configuration loaded.");

    //Load the database
    let database_wrapped = crate::database::Database::new(PathBuf::from(database_file_path.clone()));
    if database_wrapped.is_err() {
        log_warn(&env, &format!("An error occurred while loading the database: {:?}", database_wrapped.err().unwrap()));
        return;
    }

    let database = database_wrapped.unwrap();
    log_info(&env, "Database loaded.");

    log_info(&env, &format!("Loaded librconsole configuration. Listening on 0.0.0.0:{port} and [::]:{port}", port = config.port));

    {
        let config_pinned = crate::CONFIG.lock().unwrap();
        config_pinned.set(Some(config.clone()));
    }

    //Create a Channel for logging purposes
    let (tx , rx): (Sender<ConsoleLogItem>, Receiver<ConsoleLogItem>) = std::sync::mpsc::channel();

    //Create the 'users' table if it doesn't exist in the database
    let sql_create_user_table = database.connection.execute("CREATE TABLE IF NOT EXISTS users (user_id TEXT PRIMARY KEY, username TEXT NOT NULL, hash TEXT NOT NULL, salt TEXT NOT NULL)", named_params! {});
    if sql_create_user_table.is_err() {
        log_warn(&env, &format!("An error occurred while creating the 'users' table: {:?}", sql_create_user_table.err().unwrap()));
        return;
    }

    let sql_create_session_table = database.connection.execute("CREATE TABLE IF NOT EXISTS sessions (session_id TEXT PRIMARY KEY, user_id TEXT NOT NULL, expiry INTEGER NOT NULL)", named_params! {});
    if sql_create_session_table.is_err() {
        log_warn(&env, &format!("An error occurred while creating the 'sessions' table: {:?}", sql_create_session_table.err().unwrap()));
        return;
    }

    //Set the database path
    //We have to do this in a separate scope, because logging_rec() is blocking (end of function),
    //If we wouldn't do this in a separate scope, we'd never release the lock.
    {
        let database_pinned = crate::DATABASE.lock().unwrap();
        database_pinned.set(Some(database));
    }

    //Start the HTTP server
    std::thread::spawn(move || {
        let _ = crate::webserver::start(config, tx, database_file_path);
    });

    //Start listening for logging 'packets' on the created Receiver Channel
    logging_rec(env, rx);
}

/**
 * Class:     nl.thedutchmc.rconsole.webserver.Native
 * Method:    appendConsoleLog(String log, long timestamp, String level, String thread) -> void
 * Signature: (Ljava/lang/String;JLjava/lang/String;Ljava/lang/String;)V
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_appendConsoleLog(env: JNIEnv, _class: JClass, log_message_jstring: JString, log_timestamp_jlong: jlong, log_level_jstring: JString, log_thread_jstring: JString) {
    let log_message: String = env.get_string(log_message_jstring).expect("Unable to get String from JString 'log_jstring'").into();
    let log_timestamp: i64 = log_timestamp_jlong;
    let log_level: String = env.get_string(log_level_jstring).expect("Unable to get String from JString 'log_level_jstring").into();
    let log_thread: String = env.get_string(log_thread_jstring).expect("Unable to get String from JString 'log_thread_jstring'").into();

    let log_entry = LogEntry {
        message: log_message,
        timestamp: log_timestamp,
        level: log_level,
        thread: log_thread
    };

    let buffer_pinned = crate::LOG_BUFFER.pin();
    buffer_pinned.insert(buffer_pinned.len() as u32, log_entry);
}

/**
 * Class:     nl.thedutchmc.rconsole.webserver.Native
 * Method:    addUser(String username, String password) -> void
 * Signature: (Ljava/lang/String;Ljava/lang/String;)V
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_addUser(env: JNIEnv, _class: JClass, username_jstring: JString, password_jstring: JString) {
    let username: String = env.get_string(username_jstring).expect("Unable to get String from JString 'username_jstring'").into();
    let password: String = env.get_string(password_jstring).expect("Unable to get String from JString 'password_jstring'").into();

    //Create a salt
    let salt: String = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(16).map(char::from).collect();

    //Hash the password
    //We do this in a separate scope due to the config lock
    //We want the lock to be gone as fast as possible
    let password_hash = {
        let mut hasher = Sha512Trunc256::new();
        hasher.update(&password);

        let config_lock = crate::CONFIG.lock().unwrap();
        let config = config_lock.take().unwrap();

        hasher.update(&salt);
        hasher.update(&config.pepper);

        config_lock.set(Some(config));

        base64::encode(hasher.finalize())
    };

    //Run the bcrypt algorithm over the password
    let password_bcrypt = bcrypt::hash_with_salt(&password_hash, 10, &salt.as_bytes()).unwrap();
    let password_finalized = password_bcrypt.format_for_version(bcrypt::Version::TwoY);

    //Generate a user ID
    let user_id: String = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(8).map(char::from).collect();

    let database_locked = crate::DATABASE.lock().unwrap();
    let database = database_locked.take().unwrap();

    let sql_insert_result = database.connection.execute("INSERT INTO users (user_id, username, hash, salt) VALUES (:user_id, :username, :hash, :salt)", named_params! {
        ":user_id": user_id,
        ":username": username,
        ":hash": password_finalized,
        ":salt": salt
    });

    if sql_insert_result.is_err() {
        log_warn(&env, &format!("An error occurred while inserting a new user into the database: {:?}", sql_insert_result.err().unwrap()));
        database_locked.set(Some(database));
        return;
    }

    //Set the database back, since take() removes the value
    database_locked.set(Some(database));
}

/**
 * Class:     nl.thedutchmc.rconsole.webserver.Native
 * Method:    delUser(String username) -> boolean
 * Signature: (Ljava/lang/String;)Z
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_delUser(env: JNIEnv, _class: JClass, username_jstring: JString) -> jboolean {
    let username: String = env.get_string(username_jstring).expect("Unable to get String from JString 'username_jstring'").into();

    let database_lock = crate::DATABASE.lock().unwrap();
    let database = database_lock.take().unwrap();

    let sql_get_users_wrapped: rusqlite::Result<Option<String>> = database.connection.query_row("SELECT user_id FROM users WHERE username = :username", named_params! {
        ":username": &username
    }, |row| row.get(0)).optional();

    if sql_get_users_wrapped.is_err() {
        log_warn(&env, &format!("An error occurred while retrieving users from the 'users' table: {:?}", sql_get_users_wrapped.err().unwrap()));
        database_lock.set(Some(database));
        return jboolean::from(false);
    }

    let sql_get_users = sql_get_users_wrapped.unwrap();
    if sql_get_users.is_none() {
        log_warn(&env, &format!("User tried to delete user '{}', this user does not exist.", &username));
        database_lock.set(Some(database));
        return jboolean::from(false);
    }

    let user_id = sql_get_users.unwrap();

    let sql_drop_user = database.connection.execute("DELETE FROM users WHERE user_id = :user_id", named_params! {
        ":user_id": user_id
    });

    if sql_drop_user.is_err() {
        log_warn(&env, &format!("An error occurred while deleting user '{}': {:?}", &username, sql_drop_user.err().unwrap()));
        database_lock.set(Some(database));
        return jboolean::from(false);
    }

    //Put the database back
    database_lock.set(Some(database));

    jboolean::from(true)
}

/**
 * Class:     nl_thedutchmc_rconsole_webserver_Native
 * Method:    listUsers() -> String[]
 * Signature: ()[Ljava/lang/String;
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_listUsers(env: JNIEnv, _class: JClass) -> jobjectArray {
    let database_lock = crate::DATABASE.lock().unwrap();
    let database = database_lock.take().unwrap();

    let result = {
        let mut stmt = database.connection.prepare("SELECT username FROM users").unwrap();
        let sql_get_users = stmt.query_map([], |row| row.get(0));

        let jstring_class = env.find_class("java/lang/String").unwrap();

        if sql_get_users.is_err() {
            log_warn(&env, &format!("An error occurred while getting all users: {:?}", sql_get_users.err().unwrap()));
            drop(stmt);
            database_lock.set(Some(database));
            return env.new_object_array(0, jstring_class, env.new_string("").unwrap()).unwrap();
        }

        let mut usernames: Vec<String> = Vec::new();
        for row in sql_get_users.unwrap() {
            usernames.push(row.unwrap());
        }

        let result_string_arr = env.new_object_array(usernames.len() as i32, jstring_class, env.new_string("").unwrap()).unwrap();
        for i in 0..usernames.len() {
            let _ = env.set_object_array_element(result_string_arr, i as i32, env.new_string(&usernames.get(i).unwrap()).unwrap());
        }

        result_string_arr
    };

    //Put the database back
    database_lock.set(Some(database));

    result
}

/**
 * Class:     nl_thedutchmc_rconsole_webserver_Native
 * Method:    getUserSessions() -> HashMap<String, String[]>
 * Signature: ()Ljava/util/HashMap;
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_getUserSessions(env: JNIEnv, _class: JClass) -> jobject {

    let mut sessions_users_map: HashMap<String, Vec<String>> = HashMap::new();
    {
        let database_lock = crate::DATABASE.lock().unwrap();
        let database = database_lock.take().unwrap();

        let sql_get_all_users_result_fn = |db: &Connection| -> Result<HashMap<String, String>, ()> {
            let mut stmt_get_all_users = db.prepare("SELECT user_id, username FROM users").unwrap();
            let sql_get_all_users_wrapped = stmt_get_all_users.query(named_params! {});


            if sql_get_all_users_wrapped.is_err() {
                log_warn(&env, &format!("An error occurred retrieving users from the database: {:?}", sql_get_all_users_wrapped.err().unwrap()));
                return Err(());
            }

            let mut users_map: HashMap<String, String> = HashMap::new();

            let mut sql_get_all_users = sql_get_all_users_wrapped.unwrap();
            while let Some(row) = sql_get_all_users.next().unwrap() {
                users_map.insert(row.get(0).unwrap(), row.get(1).unwrap());
            }

            Ok(users_map)
        };

        let sql_get_all_users_result = sql_get_all_users_result_fn(&database.connection);
        if sql_get_all_users_result.is_err() {
            database_lock.set(Some(database));
            return *JObject::null();
        }

        let sql_get_user_sessions_fn = |db: &Connection, user_id: &String| -> Result<Vec<String>, ()> {
            let mut stmt_get_user_sessions = db.prepare("SELECT session_id FROM sessions WHERE user_id = :user_id").unwrap();
            let sql_get_user_sessions_wrapped = stmt_get_user_sessions.query(named_params! {
                ":user_id": &user_id
            });

            if sql_get_user_sessions_wrapped.is_err() {
                log_warn(&env, &format!("An error occurred retrieving sessions for the user '{}': {:?}", &user_id, sql_get_user_sessions_wrapped.err().unwrap()));
                return Err(())
            }

            let mut sessions_vec: Vec<String> = vec![];
            let mut sql_get_users_sessions = sql_get_user_sessions_wrapped.unwrap();
            while let Some(row) = sql_get_users_sessions.next().unwrap() {
                sessions_vec.push(row.get(0).unwrap());
            }

            Ok(sessions_vec)
        };

        //Fetch all sessions for each user
        let users_map = sql_get_all_users_result.unwrap();
        for (user_id, username) in &users_map {
            let sql_get_user_sessions = sql_get_user_sessions_fn(&database.connection, &user_id);
            if sql_get_user_sessions.is_err() {
                database_lock.set(Some(database));
                return *JObject::null();

            }

            sessions_users_map.insert(username.clone(), sql_get_user_sessions.unwrap());
        }

        database_lock.set(Some(database));
    }

    //Convert the HashMap<String, Vec<String>> to a jobject
    let hashmap_jclass = env.find_class("java/util/HashMap").unwrap();
    let jstring_class = env.find_class("java/lang/String").unwrap();

    let hashmap_jobject = env.new_object(hashmap_jclass, "(I)V", &[JValue::Int(sessions_users_map.len() as i32)]).unwrap();

    for (k, v) in sessions_users_map {
        let sessions_string_arr_jobject = env.new_object_array(v.len() as i32, jstring_class, env.new_string("").unwrap()).unwrap();

        for i in 0..v.len() {
            let _ = env.set_object_array_element(sessions_string_arr_jobject, i as i32, env.new_string(v.get(i).unwrap()).unwrap());
        }

        let _ = env.call_method(hashmap_jobject, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", &[JValue::from(env.new_string(&k).unwrap()), JValue::from(sessions_string_arr_jobject)]);
    }

    *hashmap_jobject
}

/**
 * Class:     nl_thedutchmc_rconsole_webserver_Native
 * Method:    delSession(String sessionId) -> void
 * Signature: (Ljava/lang/String;)Ljava/lang/Boolean;
 */
#[no_mangle]
pub extern "system" fn Java_nl_thedutchmc_rconsole_webserver_Native_delSession(env: JNIEnv, _class: JClass, session_id_jstring: JString) -> jobject {
    let session_id: String = env.get_string(session_id_jstring).expect("Unable to get String from JString 'session_id_jstring'").into();

    let database_lock = crate::DATABASE.lock().unwrap();
    let database = database_lock.take().unwrap();

    let sql_session_exists: rusqlite::Result<Option<String>> = database.connection.query_row("SELECT user_id FROM sessions WHERE session_id = :session_id", named_params! {
        ":session_id": &session_id
    }, |row| row.get(0)).optional();

    if sql_session_exists.is_err() {
        log_warn(&env, &format!("An error occurred while checking if a session_id exists: {:?}", sql_session_exists.err().unwrap()));
        database_lock.set(Some(database));
        return *JObject::null();
    }

    if sql_session_exists.unwrap().is_none() {
        log_warn(&env, &format!("The user tried to delete a session_id which doesn't exist! (session_id: '{}')", &session_id));
        database_lock.set(Some(database));
        return *bool_to_java_Boolean(&env, false).l().unwrap();
    }

    let sql_delete_session_id = database.connection.execute("DELETE FROM sessions WHERE session_id = :session_id", named_params! {
        ":session_id": &session_id
    });

    if sql_delete_session_id.is_err() {
        log_warn(&env, &format!("An error occurred while deleting a session_id from the sessions table: {:?}", sql_delete_session_id.err().unwrap()));
        database_lock.set(Some(database));
        return *JObject::null();
    }

    database_lock.set(Some(database));

    *bool_to_java_Boolean(&env, true).l().unwrap()

}

/**
Convert a bool to a java.lang.Boolean
*/
#[allow(non_snake_case)]
fn bool_to_java_Boolean<'a>(env: &'a JNIEnv, v: bool) -> JValue<'a> {
    let boolean_class = env.find_class("java/lang/Boolean").unwrap();
    env.call_static_method(boolean_class, "valueOf", "(Z)Ljava/lang/Boolean;", &[JValue::from(v)]).expect("Unable to convert bool to java.lang.Boolean")
}