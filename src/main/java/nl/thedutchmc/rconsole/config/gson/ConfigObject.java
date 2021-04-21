package nl.thedutchmc.rconsole.config.gson;

public class ConfigObject {
	private boolean debugMode, useWebServer;
	private int listenPort;
	private TokenObject[] tokens;
	
	private int librconsolePort;
	private String pepper;
	
	private String baseUrl;
	
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

	public int getLibrconsolePort() {
		return librconsolePort;
	}

	public String getPepper() {
		return pepper;
	}

	public String getBaseUrl() {
		return baseUrl;
	}
	
}
