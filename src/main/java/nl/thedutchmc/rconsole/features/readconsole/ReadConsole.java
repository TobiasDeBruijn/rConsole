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

public class ReadConsole {

	private static final Logger rootLogger = (Logger) LogManager.getRootLogger();
	private static final Gson GSON = new Gson();
	private List<TcpClient> subscribedClients = new ArrayList<>();
	
	public ReadConsole() {
		rootLogger.addAppender(new ConsoleAppender(this));
	}
	
	public synchronized void subscribeClient(TcpClient client) {
		this.subscribedClients.add(client);
	}
	
	public synchronized void unsubscribeClient(TcpClient client) {
		this.subscribedClients.remove(client);
	}
	
	public synchronized boolean isSubscribed(TcpClient client) {
		return this.subscribedClients.contains(client);
	}
	
	public void send(String message, long timestamp, String level, String thread) {
		List<TcpClient> closedClients = new ArrayList<>();
		for(TcpClient client : this.subscribedClients) {
			Socket socket = client.getSocket();
			if(socket.isClosed()) {
				closedClients.add(client);
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
