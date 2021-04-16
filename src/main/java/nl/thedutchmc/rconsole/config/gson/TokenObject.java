package nl.thedutchmc.rconsole.config.gson;

import nl.thedutchmc.rconsole.gson.Scope;

public class TokenObject {
	private String name, token;
	private Scope[] scopes;
	
	/**
	 * @return the name
	 */
	public String getName() {
		return name;
	}
	/**
	 * @return the token
	 */
	public String getToken() {
		return token;
	}
	/**
	 * @return the scopes
	 */
	public Scope[] getScopes() {
		return scopes;
	}
}
