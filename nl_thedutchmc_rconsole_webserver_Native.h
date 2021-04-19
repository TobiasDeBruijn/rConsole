/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class nl_thedutchmc_rconsole_webserver_Native */

#ifndef _Included_nl_thedutchmc_rconsole_webserver_Native
#define _Included_nl_thedutchmc_rconsole_webserver_Native
#ifdef __cplusplus
extern "C" {
#endif
/*
 * Class:     nl_thedutchmc_rconsole_webserver_Native
 * Method:    startWebServer
 * Signature: (Ljava/lang/String;Ljava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_nl_thedutchmc_rconsole_webserver_Native_startWebServer
  (JNIEnv *, jclass, jstring, jstring);

/*
 * Class:     nl_thedutchmc_rconsole_webserver_Native
 * Method:    appendConsoleLog
 * Signature: (Ljava/lang/String;JLjava/lang/String;Ljava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_nl_thedutchmc_rconsole_webserver_Native_appendConsoleLog
  (JNIEnv *, jclass, jstring, jlong, jstring, jstring);

/*
 * Class:     nl_thedutchmc_rconsole_webserver_Native
 * Method:    addUser
 * Signature: (Ljava/lang/String;Ljava/lang/String;)V
 */
JNIEXPORT void JNICALL Java_nl_thedutchmc_rconsole_webserver_Native_addUser
  (JNIEnv *, jclass, jstring, jstring);

/*
 * Class:     nl_thedutchmc_rconsole_webserver_Native
 * Method:    delUser
 * Signature: (Ljava/lang/String;)Z
 */
JNIEXPORT jboolean JNICALL Java_nl_thedutchmc_rconsole_webserver_Native_delUser
  (JNIEnv *, jclass, jstring);

/*
 * Class:     nl_thedutchmc_rconsole_webserver_Native
 * Method:    listUsers
 * Signature: ()[Ljava/lang/String;
 */
JNIEXPORT jobjectArray JNICALL Java_nl_thedutchmc_rconsole_webserver_Native_listUsers
  (JNIEnv *, jclass);

#ifdef __cplusplus
}
#endif
#endif
