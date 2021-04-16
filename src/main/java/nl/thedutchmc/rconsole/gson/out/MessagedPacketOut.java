package nl.thedutchmc.rconsole.gson.out;

public class MessagedPacketOut extends BasicPacketOut {

	@SuppressWarnings("unused")
	private String message;
	
	public MessagedPacketOut(int status, String message) {
		super(status);
		
		this.message = message;
	}
	
}
