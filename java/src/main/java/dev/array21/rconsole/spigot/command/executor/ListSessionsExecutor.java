
package dev.array21.rconsole.spigot.command.executor;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.bukkit.command.CommandSender;

import net.md_5.bungee.api.ChatColor;
import dev.array21.rconsole.spigot.RConsoleSpigot;
import dev.array21.rconsole.spigot.command.PluginCommand;

public class ListSessionsExecutor implements PluginCommand{

	private RConsoleSpigot plugin; 
	public ListSessionsExecutor(RConsoleSpigot plugin) {
		this.plugin = plugin;
		
		this.plugin.commandLoader.registerSimple("listsessions", "rconsole", null, this);
	}
	
	@Override
	public boolean onCommand(CommandSender sender, String[] args) {
		if(!sender.hasPermission("rconsole.listsessions")) {
			sender.sendMessage(ChatColor.GOLD + "You do not have permission to use this command.");
			return true;
		}
		
		HashMap<String, String[]> sessions = this.plugin.getCore().getWebServer().getUserSessions();
		if(sessions == null) {
			sender.sendMessage(ChatColor.GOLD + "Something went wrong. See console for more details.");
			return true;
		}
		
		for(Map.Entry<String, String[]> entry : sessions.entrySet()) {
			sender.sendMessage(ChatColor.GOLD + String.format("All session IDs for user %s%s%s:", ChatColor.RED, entry.getKey(), ChatColor.GOLD));
			for(String session_id: entry.getValue()) {
				sender.sendMessage(ChatColor.GOLD + "- " + ChatColor.RED + session_id);
			}
		}
		
		return true;
	}

	@Override
	public List<String> onTabComplete(CommandSender sender, List<String> args) {
		return null;
	}

}
