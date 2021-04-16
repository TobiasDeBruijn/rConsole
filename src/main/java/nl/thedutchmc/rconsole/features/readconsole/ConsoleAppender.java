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
		
		this.readConsole.send(
				eventImmutable.getMessage().getFormattedMessage(),
				eventImmutable.getTimeMillis() / 1000,
				eventImmutable.getLevel().name(),
				eventImmutable.getThreadName());
	}	
}
