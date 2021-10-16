package dev.array21.rconsole.core.config.gson;

public class ConfigObject {
	private boolean debugMode;
	private int port;
	private String pepper;
	
	private String baseUrl;
	
	public int getPort() {
		return this.port;
	}
	
	public boolean isDebugMode() {
		return debugMode;
	}

	public String getPepper() {
		return pepper;
	}

	public String getBaseUrl() {
		return baseUrl;
	}
	
}
