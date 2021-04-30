package nl.thedutchmc.rconsole.spigot.command.executor;

import java.util.List;

import org.bukkit.command.CommandSender;

import net.md_5.bungee.api.ChatColor;
import nl.thedutchmc.rconsole.spigot.RConsoleSpigot;
import nl.thedutchmc.rconsole.spigot.command.PluginCommand;

public class ListUsersExecutor implements PluginCommand {

	private RConsoleSpigot plugin;
	public ListUsersExecutor(RConsoleSpigot plugin) {
		this.plugin = plugin;
		
		plugin.commandLoader.registerSimple("listusers", "rconsole", null, this);
	}
	
	@Override
	public boolean onCommand(CommandSender sender, String[] args) {
		if(!sender.hasPermission("rconsole.listuser")) {
			sender.sendMessage(ChatColor.GOLD + "You do not have permission to use this command.");
			return true;
		}
		
		String[] users = this.plugin.getCore().getWebServer().listUsers();
		
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
