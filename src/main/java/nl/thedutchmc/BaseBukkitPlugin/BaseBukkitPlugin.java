package nl.thedutchmc.BaseBukkitPlugin;

import org.bukkit.plugin.java.JavaPlugin;

public class BaseBukkitPlugin extends JavaPlugin {

	public static BaseBukkitPlugin INSTANCE;
	
	@Override
	public void onEnable() {
		INSTANCE = this;
		
		ConfigurationHandler configHandler = new ConfigurationHandler();
		configHandler.loadConfig();
	
	}
	
	@Override
	public void onDisable() {
		
	}
	
	public static void logInfo(String log) {
		INSTANCE.getLogger().info(log);
	}
	
	public static void logWarn(String log) {
		INSTANCE.getLogger().warning(log);
	}
}
