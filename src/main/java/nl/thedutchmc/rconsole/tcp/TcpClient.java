package nl.thedutchmc.rconsole.tcp;

import java.net.Socket;

import nl.thedutchmc.rconsole.gson.Scope;

public class TcpClient {

	private String name;
	private Socket socket;
	private boolean healthy;
	private Scope[] scopes;
	
	public TcpClient(Socket socket) {
		this.socket = socket;
	}

	/**
	 * @return the name
	 */
	public String getName() {
		return name;
	}

	/**
	 * @param name the name to set
	 */
	public void setName(String name) {
		this.name = name;
	}

	/**
	 * @return the socket
	 */
	public Socket getSocket() {
		return socket;
	}

	/**
	 * @return the healthy
	 */
	public boolean isHealthy() {
		return healthy;
	}

	/**
	 * @param healthy the healthy to set
	 */
	public void setHealthy(boolean healthy) {
		this.healthy = healthy;
	}

	/**
	 * @return the scopes
	 */
	public Scope[] getScopes() {
		return scopes;
	}

	/**
	 * @param scopes the scopes to set
	 */
	public void setScopes(Scope[] scopes) {
		this.scopes = scopes;
	}
	
	
}
