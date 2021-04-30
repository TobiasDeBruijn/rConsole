package nl.thedutchmc.rconsole.core.update;

import com.google.gson.annotations.SerializedName;

public class GithubResponse {
	private String url;
	
	@SerializedName("tag_name")
	private String tagName;
	
	/**
	 * @return The URL of the new GitHub Release
	 */
	public String getUrl() {
		return url;
	}
	/**
	 * @return The name of the Release
	 */
	public String getTagName() {
		return tagName;
	}
}
