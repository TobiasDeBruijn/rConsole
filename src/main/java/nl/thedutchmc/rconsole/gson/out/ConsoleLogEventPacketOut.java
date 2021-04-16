package nl.thedutchmc.rconsole.gson.out;

import nl.thedutchmc.rconsole.gson.Intent;

@SuppressWarnings("unused")
public class ConsoleLogEventPacketOut {
	private Intent intent;
	private String message, level, thread;
	private long timestamp;
	
	public ConsoleLogEventPacketOut(String message, long timestamp, String level, String thread) {
		this.intent = Intent.CONSOLE_LOG_EVENT;
		this.message = message;
		this.timestamp = timestamp;
		this.level = level;
		this.thread = thread;
		
	}
}
