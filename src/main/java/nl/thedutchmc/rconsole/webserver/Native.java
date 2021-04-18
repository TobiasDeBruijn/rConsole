package nl.thedutchmc.rconsole.webserver;

public class Native {
	protected native static void startWebServer(String configFolder);
	protected native static void appendConsoleLog(String log, long timestamp, String level, String thread);
}
