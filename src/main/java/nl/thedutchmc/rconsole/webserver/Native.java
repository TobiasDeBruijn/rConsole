package nl.thedutchmc.rconsole.webserver;

public class Native {
	protected native static void startWebServer(String configFilePath, String databaseFilePath);
	protected native static void appendConsoleLog(String log, long timestamp, String level, String thread);
	protected native static void addUser(String username, String password);
	protected native static boolean delUser(String username);
	protected native static String[] listUsers();
}
