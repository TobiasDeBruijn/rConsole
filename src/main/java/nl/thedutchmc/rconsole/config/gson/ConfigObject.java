package nl.thedutchmc.rconsole.config.gson;

public class ConfigObject {
	private boolean debugMode;
	private int listenPort;
	private TokenObject[] tokens;
	/**
	 * @return the debugMode
	 */
	public boolean isDebugMode() {
		return debugMode;
	}
	/**
	 * @return the listenPort
	 */
	public int getListenPort() {
		return listenPort;
	}
	/**
	 * @return the tokens
	 */
	public TokenObject[] getTokens() {
		return tokens;
	}
	
}
