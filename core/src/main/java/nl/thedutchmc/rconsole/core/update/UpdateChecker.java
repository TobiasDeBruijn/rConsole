package nl.thedutchmc.rconsole.core.update;

import java.io.IOException;
import java.util.HashMap;
import java.util.regex.Pattern;

import com.google.gson.Gson;

import nl.thedutchmc.httplib.Http;
import nl.thedutchmc.httplib.Http.RequestMethod;
import nl.thedutchmc.httplib.Http.ResponseObject;
import nl.thedutchmc.rconsole.core.RConsole;

/**
 * Check if updates are available for rConsole
 * @author Tobias de Bruijn
 *
 */
public class UpdateChecker {
	
	private RConsole rconsole;
	
	public UpdateChecker(RConsole rconsole) {
		this.rconsole = rconsole;
	}
	
	/**
	 * Check if an update of RConsole is available
	 */
	public void checkUpdate() {
		String[] currentVersion = this.rconsole.getSemanticVersion().split(Pattern.quote("."));
		int currentMajorVersion = Integer.parseInt(currentVersion[0]);
		int currentMinorVersion = Integer.parseInt(currentVersion[1]);
		int currentBuild = Integer.parseInt((currentVersion.length > 2) ? currentVersion[2] : "0");
		
		HashMap<String, String> headers = new HashMap<>();
		headers.put("User-Agent", "RConsole Plugin v" + this.rconsole.getSemanticVersion());
		
		ResponseObject response;
		try {
			response = new Http(RConsole.DEBUG).makeRequest(RequestMethod.GET, "https://api.github.com/repos/thedutchmc/rconsole/releases/latest", null, null, null, headers);
		} catch(IOException e) {
			e.printStackTrace();
			return;
		}
		
		if(response.getResponseCode() != 200) {
			return;
		}
		
		final Gson gson = new Gson();
		GithubResponse responseDeserialized = gson.fromJson(response.getMessage(), GithubResponse.class);
		
		String[] latestVersion = responseDeserialized.getTagName().split(Pattern.quote("."));
		int latestMajorVersion = Integer.parseInt(latestVersion[0]);
		int latestMinorVersion = Integer.parseInt(latestVersion[1]);
		int latestBuild = Integer.parseInt((latestVersion.length > 2) ? latestVersion[2] : "0");
	
		if(latestMajorVersion > currentMajorVersion) {
			updateAvailable(responseDeserialized.getUrl(), responseDeserialized.getTagName(), this.rconsole.getSemanticVersion());
			return;
		}
		
		if(latestMinorVersion > currentMinorVersion) {
			updateAvailable(responseDeserialized.getUrl(), responseDeserialized.getTagName(), this.rconsole.getSemanticVersion());
			return;
		}
		
		if(latestBuild > currentBuild) {
			updateAvailable(responseDeserialized.getUrl(), responseDeserialized.getTagName(), this.rconsole.getSemanticVersion());
			return;
		}
		
		this.rconsole.logInfo(String.format("Plugin is up-to-date. Running rConsole v%s", this.rconsole.getSemanticVersion()));
	}
	
	private void updateAvailable(String url, String latestVersion, String currentVersion) {
		this.rconsole.logWarn(String.format("An update is available. You are running version %s, the latest version is %s. You can download it here: %s", currentVersion, latestVersion, url));
	}
}
