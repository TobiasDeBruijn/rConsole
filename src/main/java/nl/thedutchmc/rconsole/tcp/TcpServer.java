package nl.thedutchmc.rconsole.tcp;

import java.io.IOException;
import java.net.ServerSocket;
import java.util.ArrayList;
import java.util.List;

import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.config.gson.TokenObject;
import nl.thedutchmc.rconsole.features.readconsole.ReadConsole;

public class TcpServer implements Runnable {

	private static final int LISTENERS = 2;	
	
	private final int port;
	private final TokenObject[] validTokens;
	private final RConsole plugin;
	private final ReadConsole readConsoleFeature = new ReadConsole();
	private final List<TcpClient> signedInClients = new ArrayList<>();
		
	private ServerSocket serverSocket;

	public TcpServer(int port, TokenObject[] validTokens, RConsole plugin) {
		this.port = port;
		this.validTokens = validTokens;
		this.plugin = plugin;
	}
	
	@Override
	public void run() {
		try {
			this.serverSocket = new ServerSocket(this.port);
			RConsole.logDebug("Started TCP Server on port " + this.port);
			
			for(int i = 0; i < LISTENERS; i++) {
				new Thread(new TcpListener(this.serverSocket, this.validTokens, this, this.readConsoleFeature, this.plugin), "rConsole-TcpListener-Worker-" + i).start();
			}
		} catch(IOException e) {
			
		}
	}
	
	public synchronized void addSignedInClient(TcpClient client) {
		this.signedInClients.add(client);
	}
	
	public synchronized void removeSignedInClient(TcpClient client) {
		this.signedInClients.remove(client);
	}
	
	public synchronized TcpClient[] getSignedInClients() {
		return this.signedInClients.toArray(new TcpClient[0]);
	}
	
	public synchronized boolean isClientSignedIn(TcpClient client) {
		return this.signedInClients.contains(client);
	}
}
