package nl.thedutchmc.rconsole.features.readconsole;

import java.io.IOException;
import java.io.PrintWriter;
import java.net.Socket;
import java.util.ArrayList;
import java.util.List;

import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.core.Logger;

import com.google.gson.Gson;

import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.gson.out.ConsoleLogEventPacketOut;
import nl.thedutchmc.rconsole.tcp.TcpClient;
import nl.thedutchmc.rconsole.webserver.WebServer;

public class ReadConsole {

	private static final Logger rootLogger = (Logger) LogManager.getRootLogger();
	private static final Gson GSON = new Gson();
	private final WebServer nativeWebServer;
	private List<TcpClient> subscribedClients = new ArrayList<>();
	
	public ReadConsole(WebServer nativeWebServer) {
		rootLogger.addAppender(new ConsoleAppender(this));
		this.nativeWebServer = nativeWebServer;
	}
	
	/**
	 * Subscribe a TcpClient for Console output
	 * @param client
	 */
	public synchronized void subscribeClient(TcpClient client) {
		this.subscribedClients.add(client);
	}
	
	/**
	 * Unsubscribe a TcpClient from Console output
	 * @param client The TcpClient to unsubscribe
	 */
	public synchronized void unsubscribeClient(TcpClient client) {
		this.subscribedClients.remove(client);
	}
	
	/**
	 * Check if a TcpClient is subscribed to Console output
	 * @param client The client to check
	 * @return Returns true if the Client is subscribed, false if it is not
	 */
	public synchronized boolean isSubscribed(TcpClient client) {
		return this.subscribedClients.contains(client);
	}
	
	/**
	 * Send a log message to the librconsole<br>
	 * This will have no negative effects if librconsole is not loaded
	 * @param message The message to log
	 * @param timestamp The epoch timestamp of the log event
	 * @param level The level (WARN/INFO) of the log event
	 * @param thread The thread from which the log event originated
	 */
	public void sendToLibRconsole(String message, long timestamp, String level, String thread) {
		if(this.nativeWebServer != null) {
			this.nativeWebServer.log(message, timestamp, level, thread);
		}
	}
	
	/**
	 * Send a log message to all signed in clients
	 * @param message The message to log
	 * @param timestamp The epoch timestamp of the log event
	 * @param level The level (WARN/INFO) of the log event
	 * @param thread The thread from which the log event originated
	 */
	public void send(String message, long timestamp, String level, String thread) {
		for(TcpClient client : this.subscribedClients) {
			Socket socket = client.getSocket();
			if(socket.isClosed()) {
				continue;
			}
			
			ConsoleLogEventPacketOut packetOut = new ConsoleLogEventPacketOut(message, timestamp, level, thread);
			String packetSerialized = GSON.toJson(packetOut);
			
			try {
				PrintWriter pw = new PrintWriter(socket.getOutputStream());
				pw.println(packetSerialized);
				pw.flush();
			} catch(IOException e) {
				RConsole.logDebug(String.format("Failed to send console log event to client '%s' due to an IOException: '%s'", client.getName(), e.getMessage()));
				continue;
			}
		}
	}
}
