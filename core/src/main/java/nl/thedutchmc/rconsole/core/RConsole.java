package nl.thedutchmc.rconsole.core;

import java.io.File;
import java.util.function.BiConsumer;
import java.util.function.Consumer;

import nl.thedutchmc.rconsole.core.annotations.Nullable;
import nl.thedutchmc.rconsole.core.config.Configuration;
import nl.thedutchmc.rconsole.core.config.gson.ConfigObject;
import nl.thedutchmc.rconsole.core.update.UpdateChecker;
import nl.thedutchmc.rconsole.core.webserver.WebServer;

public class RConsole {
	private static Consumer<Object> logDebug;
	private static Consumer<Object> logInfo;
	private static Consumer<Object> logWarn;
	private Consumer<Runnable> execSyncFunction;
	
	private Configuration config;
	private WebServer webServer;
	private String semanticVersion;
	
	public static volatile boolean DEBUG = false;
	
	public RConsole(CoreInitParams initParams) {
		RConsole.logInfo = initParams.getLogInfoFn();
		RConsole.logWarn = initParams.getLogWarnFn();
		RConsole.logDebug = initParams.getLogDebugFn();
		this.semanticVersion =initParams.getSemVer();
		
		this.config = new Configuration(initParams.getDataFolder(), initParams.getSaveResourceFn());
		this.startWebServer(initParams.getSaveResourceFn(), initParams.getExecCommandFn(), initParams.getDataFolder());
	}
	
	public static void logInfo(Object log) {
		RConsole.logInfo.accept(log);
	}
	
	public static void logWarn(Object log) {
		RConsole.logWarn.accept(log);
	}
	
	public static void logDebug(Object log) {
		if(DEBUG) {
			RConsole.logDebug.accept(log);
		}
	}
	
	public void execSync(Runnable r) {
		this.execSyncFunction.accept(r);
	}
	
	public ConfigObject getConfig() {
		return this.config.getConfig();
	}
	
	public String getSemanticVersion() {
		return this.semanticVersion;
	}
	
	@Nullable
	public WebServer getWebServer() {
		return this.webServer;
	}
	
	public void checkUpdates() {
		UpdateChecker checker = new UpdateChecker(this);
		checker.checkUpdate();
	}
	
	private void startWebServer(BiConsumer<String, Boolean> saveResource, Consumer<String> consoleCommandExecutor, File dataFolder) {
		this.webServer = new WebServer(this, consoleCommandExecutor);
		new Thread(new Runnable() {
			
			@Override
			public void run() {
				RConsole.this.webServer.startWebServer(saveResource, dataFolder);
				
			}
		}, "rConsole-librconsole-server-Thread").start();
	}
}
