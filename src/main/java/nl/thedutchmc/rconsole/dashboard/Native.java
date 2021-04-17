package nl.thedutchmc.rconsole.dashboard;

public class Native {
	private String[] logBuffer;
	private Native self;
	
	protected Native() {
		this.self = this;
	}
	
	protected native void nativeStartDashboardServer(String configFolder);
	protected native void nativeStopDashboardServer();
	protected native void appendConsoleLog(String log);

	protected void setLogBuffer(String[] logBuffer) {
		this.logBuffer = logBuffer;
	}
	
	protected String[] getLogBuffer() {
		return this.logBuffer;
	}
}
