package nl.thedutchmc.rconsole.spigot;

import org.apache.logging.log4j.core.LogEvent;
import org.apache.logging.log4j.core.appender.AbstractAppender;
import org.apache.logging.log4j.core.layout.PatternLayout;

import nl.thedutchmc.rconsole.core.RConsole;

public class ConsoleAppender extends AbstractAppender {
	
	private RConsole rconsole;
	
	protected ConsoleAppender(RConsole rconsole) {
		super("rconsole-appender", null, PatternLayout.createDefaultLayout());		
		this.rconsole = rconsole;
		super.start();		
	}

	@Override
	public void append(LogEvent event) {
		LogEvent eventImmutable = event.toImmutable();
		
		final String message = eventImmutable.getMessage().getFormattedMessage();
		final long timestamp = eventImmutable.getTimeMillis() / 1000;
		final String level = eventImmutable.getLevel().name();
		final String thread = eventImmutable.getThreadName();
		this.rconsole.getWebServer().log(message, timestamp, level, thread);
	}	
}
