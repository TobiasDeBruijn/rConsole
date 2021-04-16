package nl.thedutchmc.rconsole.gson.in;

import java.util.Arrays;

import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.gson.Scope;

public class LoginPacketIn extends BasicPacketIn {
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
	
	public boolean equals(nl.thedutchmc.rconsole.config.gson.TokenObject tokenObject) {
		//Split out into multiple IF statements for readability's sake
		
		if(!tokenObject.getName().equals(this.name)) {
			RConsole.logDebug("Signin failed. 'name' failed.");
			return false;
		}
		
		if(!tokenObject.getToken().equals(this.token)) {
			RConsole.logDebug("Signin failed. 'token' failed.");
			return false;
		}
				
		java.util.List<Scope> scopesAsList = Arrays.asList(this.scopes);
		for(Scope scope : tokenObject.getScopes()) {
			if(!scopesAsList.contains(scope)) {
				RConsole.logDebug(String.format("Signin failed. 'scope' failed on scope '%s'", scope.toString()));
				return false;
			}
		}
		
		return true;		
	}
}
