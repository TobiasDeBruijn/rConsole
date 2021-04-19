package nl.thedutchmc.rconsole.command.executor;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.bukkit.command.CommandSender;

import net.md_5.bungee.api.ChatColor;
import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.command.PluginCommand;

public class DelSessionExecutor implements PluginCommand {

	private RConsole plugin;
	public DelSessionExecutor(RConsole plugin) {
		this.plugin = plugin;
		
		plugin.commandLoader.registerSimple("delsession", "rconsole", null, this);
	}
	
	@Override
	public boolean onCommand(CommandSender sender, String[] args) {
		if(!sender.hasPermission("rconsole.delsession")) {
			sender.sendMessage(ChatColor.GOLD + "You do not have permission to use this command.");
			return true;
		}
		
		if(this.plugin.nativeWebServer == null) {
			sender.sendMessage(ChatColor.GOLD + "The native web server has not been enabled. To use this command, enable it in the config!");
			return true;
		}
		
		if(args.length != 1) {
			sender.sendMessage(ChatColor.GOLD + String.format("Invalid number of arguments. See %s/rconsole%s for help.", ChatColor.RED, ChatColor.GOLD));
			return true;
		}
		
		Boolean removeSuccessful = this.plugin.nativeWebServer.delSession(args[0]);
		if(removeSuccessful == null) {
			sender.sendMessage(ChatColor.GOLD + "Something went wrong. Please check console for more details.");
			return true;
		}
		
		if(removeSuccessful == false) {
			sender.sendMessage(ChatColor.GOLD + String.format("The session with ID %s%s%s does not exist.", ChatColor.RED, args[0], ChatColor.GOLD));
			return true;
		}
		
		sender.sendMessage(ChatColor.GOLD + String.format("The session with ID %s%s%s was deleted successfully.", ChatColor.RED, args[0], ChatColor.GOLD));
		
		return true;
	}

	@Override
	public List<String> onTabComplete(CommandSender sender, List<String> args) {
		if(!sender.hasPermission("rconsole.delsession")) {
			return null;
		}
		
		if(this.plugin.nativeWebServer == null) {
			return null;
		}
		
		if(args.size() != 1) {
			return null;
		}
		
		HashMap<String, String[]> userSessionsMap = this.plugin.nativeWebServer.getUserSessions();
		if(userSessionsMap == null) {
			return null;
		}
		
		List<String> sessions = new ArrayList<>();
		for(Map.Entry<String, String[]> entry : userSessionsMap.entrySet()) {
			sessions.addAll(Arrays.asList(entry.getValue()));
		}
		
		return sessions;
	}
}
