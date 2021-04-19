use crate::jni::logging::{log_warn, log_debug, log_info, ConsoleLogItem, logging_rec};
use crate::config::Config;
use crate::LogEntry;

use jni::JNIEnv;
use jni::objects::{JString, JClass};
use jni::sys::{jlong, jboolean, jobjectArray};
use std::path::PathBuf;
use std::sync::mpsc::{Sender, Receiver};
use rand::Rng;
use sha2::{Sha512Trunc256, Digest};
use rusqlite::{named_params, OptionalExtension};

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
        return jboolean::from(false);
    }

    let sql_get_users = sql_get_users_wrapped.unwrap();
    if sql_get_users.is_none() {
        log_warn(&env, &format!("User tried to delete user '{}', this user does not exist.", &username));
        return jboolean::from(false);
    }

    let user_id = sql_get_users.unwrap();

    let sql_drop_user = database.connection.execute("DELETE FROM users WHERE user_id = :user_id", named_params! {
        ":user_id": user_id
    });

    if sql_drop_user.is_err() {
        log_warn(&env, &format!("An error occurred while deleting user '{}': {:?}", &username, sql_drop_user.err().unwrap()));
        return jboolean::from(false);
    }

    //Put the database back
    database_lock.set(Some(database));

    jboolean::from(true)
}

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