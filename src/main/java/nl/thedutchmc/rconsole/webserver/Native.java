package nl.thedutchmc.rconsole.webserver;

import java.util.HashMap;
import nl.thedutchmc.rconsole.annotations.Nullable;

/**
 * Native methods calling into librconsole. To use any of these methods, librconsole should be loaded.<br>
 * The methods should preferably be accessed through a wrapper class which checks if the library is loaded etc
 * @author Tobias de Bruijn
 *
 */
public class Native {
	
	/**
	 * Start the web server
	 * @param configFilePath The absolute path to where librconsole can find it's yaml configuration file
	 * @param databaseFilePath The absolute path to where librconsole can find the SQLite db3 file
	 * @param staticFilePath The absolute path to where the website can find it's json configuration file
	 */
	protected native static void startWebServer(String configFilePath, String databaseFilePath, String staticFilePath);
	
	/**
	 * Append a log entry
	 * @param log The message of the log entry
	 * @param timestamp The timestamp of when the log occurred
	 * @param level The level at which the log occurred, INFO or WARN
	 * @param thread The thread from which the log originated
	 */
	protected native static void appendConsoleLog(String log, long timestamp, String level, String thread);
	
	/**
	 * Add a user
	 * @param username The username of the user
	 * @param password The password of the user
	 */
	//TODO this should return a Boolean indicating if the creation was successful
	protected native static void addUser(String username, String password);
	
	/**
	 * Delete a user
	 * @param username The username of the user to delete
	 * @return returns true if the deletion was successful, false if the user did not exist or null if an error occurred
	 */
	@Nullable
	protected native static Boolean delUser(String username);
	
	/**
	 * Get a list of existing usernames
	 * @return Returns the usernames as a String[], or null if an error occurred
	 */
	@Nullable
	protected native static String[] listUsers();
	
	/**
	 * Get a HashMap with the username and sessions associated with that username
	 * @return Returns the HashMap, or null if an error occurred
	 */
	@Nullable
	protected native static HashMap<String, String[]> getUserSessions();
	
	/**
	 * Delete a session
	 * @param sessionId The ID of the session to delete
	 * @return returns true if the session was deleted successfully, false if the session did not exist or null if an error occurred
	 */
	@Nullable
	protected native static Boolean delSession(String sessionId);
	
	/**
	 * Start the command listening Thread<br>
	 * This is a blocking method
	 */
	protected native static void startCommandListenThread();
}
