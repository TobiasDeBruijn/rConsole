package dev.array21.rconsole.spigot.command.executor;

import java.util.Arrays;
import java.util.List;

import org.bukkit.command.CommandSender;

import net.md_5.bungee.api.ChatColor;
import dev.array21.rconsole.spigot.RConsoleSpigot;
import dev.array21.rconsole.spigot.command.PluginCommand;

public class DelUserExecutor implements PluginCommand {

	private RConsoleSpigot plugin; 
	public DelUserExecutor(RConsoleSpigot plugin) {
		this.plugin = plugin;
		
		this.plugin.commandLoader.registerSimple("deluser", "rconsole", null, this);
	}
	
	@Override
	public boolean onCommand(CommandSender sender, String[] args) {
		if(!sender.hasPermission("rconsole.deluser")) {
			sender.sendMessage(ChatColor.GOLD + "You do not have permission to use this command.");
			return true;
		}
		
		if(args.length != 1) {
			sender.sendMessage(ChatColor.GOLD + String.format("Invalid number of arguments. See %s/rconsole%s for help.", ChatColor.RED, ChatColor.GOLD));
			return true;
		}
		
		Boolean successful = this.plugin.getCore().getWebServer().delUser(args[0]);
		if(successful == null) {
			sender.sendMessage(ChatColor.GOLD + String.format("An error occurred while deleting user %s%s%.", ChatColor.RED, args[0], ChatColor.GOLD));
		}
		
		if(successful) {
			sender.sendMessage(ChatColor.GOLD + String.format("User %s%s%s was deleted successfully.", ChatColor.RED, args[0], ChatColor.GOLD));
		} else {
			sender.sendMessage(ChatColor.GOLD + String.format("The user %s%s%s does not exist.", ChatColor.RED, args[0], ChatColor.GOLD));
		}
		
		return true;
	}

	@Override
	public List<String> onTabComplete(CommandSender sender, List<String> args) {
		if(!sender.hasPermission("rconsole.deluser")) {
			return null;
		}
		
		if(args.size() == 1) {
			String[] users = this.plugin.getCore().getWebServer().listUsers();
			return Arrays.asList(users);
		}
		
		return null;
	}

}
