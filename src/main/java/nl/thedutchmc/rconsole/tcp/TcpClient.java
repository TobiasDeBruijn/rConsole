package nl.thedutchmc.rconsole.tcp;

import java.net.Socket;

public class TcpClient {

	private String name;
	private Socket socket;
	private boolean healthy;
	
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
	
	
}
