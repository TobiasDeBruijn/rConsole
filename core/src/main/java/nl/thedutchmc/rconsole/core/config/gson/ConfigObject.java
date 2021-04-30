package nl.thedutchmc.rconsole.core.config.gson;

public class ConfigObject {
	private boolean debugMode, useWebServer;
	
	private int librconsolePort;
	private String pepper;
	
	private String baseUrl;
	
	public boolean isDebugMode() {
		return debugMode;
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
