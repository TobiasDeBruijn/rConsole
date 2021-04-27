package nl.thedutchmc.rconsole.update;

import com.google.gson.annotations.SerializedName;

public class GithubResponse {
	private String url;
	
	@SerializedName("tag_name")
	private String tagName;
	/**
	 * @return the url
	 */
	public String getUrl() {
		return url;
	}
	/**
	 * @return the tagName
	 */
	public String getTagName() {
		return tagName;
	}
}
