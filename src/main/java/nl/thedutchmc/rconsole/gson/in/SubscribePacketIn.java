package nl.thedutchmc.rconsole.gson.in;

public class SubscribePacketIn {
	private SubscribeType subscribeType;
	
	public enum SubscribeType {
		CONSOLE_OUTPUT;
	}

	/**
	 * @return the subscribeType
	 */
	public SubscribeType getSubscribeType() {
		return subscribeType;
	}
}
