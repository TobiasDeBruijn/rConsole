package nl.thedutchmc.rconsole.config.gson;

public class ConfigObject {
	private boolean debugMode, useWebServer;
	private int listenPort;
	private TokenObject[] tokens;
	
	public boolean isDebugMode() {
		return debugMode;
	}

	public int getListenPort() {
		return listenPort;
	}

	public TokenObject[] getTokens() {
		return tokens;
	}

	public boolean isUseWebServer() {
		return useWebServer;
	}
	
}
