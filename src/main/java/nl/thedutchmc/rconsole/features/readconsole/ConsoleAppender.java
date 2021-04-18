package nl.thedutchmc.rconsole.features.readconsole;

import org.apache.logging.log4j.core.LogEvent;
import org.apache.logging.log4j.core.appender.AbstractAppender;
import org.apache.logging.log4j.core.layout.PatternLayout;

public class ConsoleAppender extends AbstractAppender {
	
	private ReadConsole readConsole;
	
	protected ConsoleAppender(ReadConsole readConsole) {
		super("rconsole-appender", null, PatternLayout.createDefaultLayout());		
		this.readConsole = readConsole;
		super.start();		
	}

	@Override
	public void append(LogEvent event) {
		LogEvent eventImmutable = event.toImmutable();
		
		final String message = eventImmutable.getMessage().getFormattedMessage();
		final long timestamp = eventImmutable.getTimeMillis() / 1000;
		final String level = eventImmutable.getLevel().name();
		final String thread = eventImmutable.getThreadName();
		this.readConsole.sendToLibRconsole(message, timestamp, level, thread);
		this.readConsole.send(message, timestamp, level, thread);
	}	
}
