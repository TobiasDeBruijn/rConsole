package nl.thedutchmc.rconsole.webserver;

import java.util.HashMap;

public class Native {
	protected native static void startWebServer(String configFilePath, String databaseFilePath, String staticFilePath);
	protected native static void appendConsoleLog(String log, long timestamp, String level, String thread);
	protected native static void addUser(String username, String password);
	protected native static boolean delUser(String username);
	protected native static String[] listUsers();
	protected native static HashMap<String, String[]> getUserSessions();
	protected native static Boolean delSession(String sessionId);
}
