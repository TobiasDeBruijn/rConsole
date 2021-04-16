package nl.thedutchmc.rconsole.tcp;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.net.ServerSocket;
import java.net.SocketException;
import java.util.Arrays;

import org.bukkit.Bukkit;
import org.bukkit.scheduler.BukkitRunnable;

import com.google.gson.Gson;
import com.google.gson.JsonSyntaxException;

import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.config.gson.TokenObject;
import nl.thedutchmc.rconsole.features.readconsole.ReadConsole;
import nl.thedutchmc.rconsole.gson.Scope;
import nl.thedutchmc.rconsole.gson.in.BasicPacketIn;
import nl.thedutchmc.rconsole.gson.in.LoginPacketIn;
import nl.thedutchmc.rconsole.gson.in.SendCommandPacketIn;
import nl.thedutchmc.rconsole.gson.in.SubscribePacketIn;
import nl.thedutchmc.rconsole.gson.out.BasicPacketOut;
import nl.thedutchmc.rconsole.gson.out.MessagedPacketOut;

public class TcpListener implements Runnable {

	private final ServerSocket serverSocket;
	private final TokenObject[] validTokens;
	private final TcpServer tcpServer;
	private final ReadConsole readConsoleFeature;
	private final RConsole plugin;
	
	private static final Gson GSON = new Gson();
	
	public TcpListener(ServerSocket serverSocket, TokenObject[] validTokens, TcpServer tcpServer, ReadConsole readConsoleFeature, RConsole plugin) {
		this.serverSocket = serverSocket;
		this.validTokens = validTokens;
		this.tcpServer = tcpServer;
		this.readConsoleFeature = readConsoleFeature;
		this.plugin = plugin;
	}
	
	@Override
	public void run() {
		RConsole.logDebug(String.format("Spawned worker thread '%s'", Thread.currentThread().getName()));

		while(RConsole.getIsRunning()) {
			try {
				TcpClient client = new TcpClient(this.serverSocket.accept());
				RConsole.logDebug(String.format("Client %s connected", client.getSocket().getInetAddress().toString()));
				
				new Thread(new MessageListener(client), Thread.currentThread().getName() + "-client-listener-" + client.getSocket().getPort()).start();
				
			} catch (IOException e) {
				e.printStackTrace();
			}
		}
	}
	
	private class MessageListener implements Runnable {

		private TcpClient client;
		
		public MessageListener(TcpClient client) {
			this.client = client;
		}
		
		@Override
		public void run() {
			try {
				PrintWriter out = new PrintWriter(this.client.getSocket().getOutputStream());
				BufferedReader in = new BufferedReader(new InputStreamReader(this.client.getSocket().getInputStream()));
				String inputLine;
				while(!this.client.getSocket().isClosed() && (inputLine = in.readLine()) != null) {					
					try {
						RConsole.logDebug(String.format("Received message '%s' from client '%s'", inputLine, this.client.getSocket().getInetAddress().toString()));
						
						BasicPacketIn basicPacket = GSON.fromJson(inputLine, BasicPacketIn.class);
						if(basicPacket.getPath() == null) {
							MessagedPacketOut invalidPacketOut = new MessagedPacketOut(400, "Required field `path` is missing");
							out.println(GSON.toJson(invalidPacketOut));
							out.flush();
							continue;
						}
						
						switch(basicPacket.getPath()) {
						case "/login":
							LoginPacketIn loginPacket = GSON.fromJson(inputLine, LoginPacketIn.class);
							if(loginPacket.getName() == null) {
								MessagedPacketOut invalidPacketOut = new MessagedPacketOut(400, "Required field `name` is missing");
								out.println(GSON.toJson(invalidPacketOut));
								out.flush();
								continue;
							}
							
							if(loginPacket.getToken() == null) {
								MessagedPacketOut invalidPacketOut = new MessagedPacketOut(400, "Required field `token` is missing");
								out.println(GSON.toJson(invalidPacketOut));
								out.flush();
								continue;
							}
							
							if(loginPacket.getScopes() == null) {
								MessagedPacketOut invalidPacketOut = new MessagedPacketOut(400, "Required field `scopes` is missing");
								out.println(GSON.toJson(invalidPacketOut));
								out.flush();
								continue;
							}
							
							if(loginPacket.getScopes().length == 0) {
								MessagedPacketOut invalidPacketOut = new MessagedPacketOut(400, "Required field `scopes` may not be empty");
								out.println(GSON.toJson(invalidPacketOut));
								out.flush();
								continue;
							}
							
							//Check if the token is correct
							boolean anyMatch = false;
							for(TokenObject tokenObj : TcpListener.this.validTokens) {
								if(loginPacket.equals(tokenObj)) {
									anyMatch = true;
								}
							}
							
							if(!anyMatch) {
								MessagedPacketOut messagedPacketOut = new MessagedPacketOut(401, "Invalid Token, Name and Scopes combination.");
								out.println(GSON.toJson(messagedPacketOut));
								out.flush();
								continue;
							}
							
							this.client.setName(loginPacket.getName());
							this.client.setScopes(loginPacket.getScopes());
							
							{
								TcpListener.this.tcpServer.addSignedInClient(this.client);
								BasicPacketOut loginSuccessPacketOut = new BasicPacketOut(200);
								out.println(GSON.toJson(loginSuccessPacketOut));
								out.flush();
								continue;
							}
						
						case "/subscribe":
							SubscribePacketIn subscribePacketIn = GSON.fromJson(inputLine, SubscribePacketIn.class);
							
							//Check if the client is signed in
							if(!TcpListener.this.tcpServer.isClientSignedIn(this.client)) {
								MessagedPacketOut messagedPacketOut = new MessagedPacketOut(401, "Client is not signed in");
								out.println(GSON.toJson(messagedPacketOut));
								out.flush();
								continue;
							}
							
							if(subscribePacketIn.getSubscribeType() == null) {
								MessagedPacketOut invalidPacketOut = new MessagedPacketOut(400, "Required field 'subscribeType' is missing");
								out.println(GSON.toJson(invalidPacketOut));
								out.flush();
								continue;
							}
							
							switch(subscribePacketIn.getSubscribeType()) {
							case CONSOLE_OUTPUT:
								if(!Arrays.asList(this.client.getScopes()).contains(Scope.READ_CONSOLE)) {
									MessagedPacketOut messagedPacketOut = new MessagedPacketOut(401, "Missing scope 'READ_CONSOLE'");
									out.println(GSON.toJson(messagedPacketOut));
									out.flush();
									continue;
								}
								
								if(TcpListener.this.readConsoleFeature.isSubscribed(this.client)) {
									MessagedPacketOut messagedPacketOut = new MessagedPacketOut(400, "Client is already subscribed");
									out.println(GSON.toJson(messagedPacketOut));
									out.flush();
									continue;
								}
								
								{
									TcpListener.this.readConsoleFeature.subscribeClient(this.client);
									BasicPacketOut loginSuccessPacketOut = new BasicPacketOut(200);
									out.println(GSON.toJson(loginSuccessPacketOut));
									out.flush();
									continue;
								}
							}
						
							break;
						case "/command":
							SendCommandPacketIn commandPacketIn = GSON.fromJson(inputLine, SendCommandPacketIn.class);
							
							//Check if the client is signed in
							if(!TcpListener.this.tcpServer.isClientSignedIn(client)) {
								MessagedPacketOut messagedPacketOut = new MessagedPacketOut(401, "Client is not signed in");
								out.println(GSON.toJson(messagedPacketOut));
								out.flush();
								continue;
							}
							
							if(!Arrays.asList(this.client.getScopes()).contains(Scope.SEND_COMMAND)) {
								MessagedPacketOut messagedPacketOut = new MessagedPacketOut(401, "Missing scope 'SEND_COMMAND'");
								out.println(GSON.toJson(messagedPacketOut));
								out.flush();
								continue;
							}
							
							if(commandPacketIn.getCommand() == null) {
								MessagedPacketOut invalidPacketOut = new MessagedPacketOut(400, "Required field 'command' is missing");
								out.println(GSON.toJson(invalidPacketOut));
								out.flush();
								continue;
							}
							
							new BukkitRunnable() {
								
								@Override
								public void run() {
									Bukkit.dispatchCommand(Bukkit.getConsoleSender(), commandPacketIn.getCommand());
								}
							}.runTask(TcpListener.this.plugin);
							
							BasicPacketOut loginSuccessPacketOut = new BasicPacketOut(200);
							out.println(GSON.toJson(loginSuccessPacketOut));
							out.flush();
							continue;
						}						
					} catch(JsonSyntaxException e) {
						MessagedPacketOut invalidPacketOut = new MessagedPacketOut(400, e.getMessage());
						out.println(GSON.toJson(invalidPacketOut));
						out.flush();
						continue;
					}
				}
			} catch(SocketException e) {
				//TODO probably shouldn't be just suppressing all socket exceptions.
			} catch(IOException e) {
				e.printStackTrace();
			}
		}
	}
}
