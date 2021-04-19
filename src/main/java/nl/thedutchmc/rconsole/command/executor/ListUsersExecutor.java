package nl.thedutchmc.rconsole.command.executor;

import java.util.List;

import org.bukkit.command.CommandSender;

import net.md_5.bungee.api.ChatColor;
import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.command.PluginCommand;

public class ListUsersExecutor implements PluginCommand {

	private RConsole plugin;
	public ListUsersExecutor(RConsole plugin) {
		this.plugin = plugin;
		
		plugin.commandLoader.registerSimple("listusers", "rconsole", null, this);
	}
	
	@Override
	public boolean onCommand(CommandSender sender, String[] args) {
		if(!sender.hasPermission("rconsole.listuser")) {
			sender.sendMessage(ChatColor.GOLD + "You do not have permission to use this command.");
			return true;
		}
		
		if(this.plugin.nativeWebServer == null) {
			sender.sendMessage(ChatColor.GOLD + "The native web server has not been enabled. To use this command, enable it in the config!");
			return true;
		}
		
		String[] users = this.plugin.nativeWebServer.listUsers();
		
		if(users.length == 0) {
			sender.sendMessage(ChatColor.GOLD + "There are no users.");
			return true;
		}
		
		sender.sendMessage(ChatColor.GOLD + "The following users exist:");
		for(String user : users) {
			sender.sendMessage(ChatColor.GOLD + "- " + ChatColor.RED + user);
		}
		
		return true;
	}

	@Override
	public List<String> onTabComplete(CommandSender sender, List<String> args) {
		return null;
	}
}
