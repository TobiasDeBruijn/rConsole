package nl.thedutchmc.rconsole.spigot;

import org.bukkit.Bukkit;
import org.bukkit.plugin.java.JavaPlugin;

import nl.thedutchmc.rconsole.spigot.command.CommandLoader;
import nl.thedutchmc.rconsole.spigot.command.executor.*;
import nl.thedutchmc.rconsole.core.CoreInitParams;
import nl.thedutchmc.rconsole.core.RConsole;

public class RConsoleSpigot extends JavaPlugin {

	private static RConsoleSpigot INSTANCE;	
	private RConsole rconsole;
	
	public CommandLoader commandLoader;
	
	@Override
	public void onEnable() {
		RConsoleSpigot.INSTANCE = this;
		
		CoreInitParams initParams = CoreInitParams.newInitParams()
				.semVer(this.getDescription().getVersion())
				.setDataFolder(this.getDataFolder())
				.setLogDebugFn(RConsoleSpigot::logDebug)
				.setLogInfoFn(RConsoleSpigot::logInfo)
				.setLogWarnFn(RConsoleSpigot::logWarn)
				.setSaveResourceFn(this::saveResource)
				.setExecCommandFn(RConsoleSpigot::execCommand)
				.setExecSyncFn(RConsoleSpigot::execSync);
				
		this.rconsole = new RConsole(initParams);
		rconsole.checkUpdates();
		
		//Register all commands
		commandLoader = new CommandLoader(this);
		new AddUserExecutor(this);
		new DelUserExecutor(this);
		new ListUsersExecutor(this);
		new ListSessionsExecutor(this);
		new DelSessionExecutor(this);
				
		//Start the console appender
		new ConsoleAppender(rconsole);
	}
	
	public static void execSync(Runnable r) {
		Bukkit.getScheduler().scheduleSyncDelayedTask(INSTANCE, r);
	}
	
	public static void execCommand(String cmd) {
		Bukkit.dispatchCommand(Bukkit.getConsoleSender(), cmd);
	}
	
	public static void logInfo(Object log) {
		RConsoleSpigot.INSTANCE.getLogger().info(log.toString());
	}
	
	public static void logWarn(Object log) {
		RConsoleSpigot.INSTANCE.getLogger().warning(log.toString());
	}
	
	public static void logDebug(Object log) {		
		RConsoleSpigot.INSTANCE.getLogger().info("[DEBUG] " + log.toString());
	}
	
	public RConsole getCore() {
		return this.rconsole;
	}
}
