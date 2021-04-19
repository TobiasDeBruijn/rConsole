package nl.thedutchmc.rconsole.command.executor;

import java.util.Arrays;
import java.util.List;

import org.bukkit.command.CommandSender;

import net.md_5.bungee.api.ChatColor;
import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.command.PluginCommand;

public class AddUserExecutor implements PluginCommand {

	private RConsole plugin;
	public AddUserExecutor(RConsole plugin) {
		this.plugin = plugin;
		
		plugin.commandLoader.registerSimple("adduser", "rconsole", null, this);
	}
	
	@Override
	public boolean onCommand(CommandSender sender, String[] args) {
		if(!sender.hasPermission("rconsole.adduser")) {
			sender.sendMessage(ChatColor.GOLD + "You do not have permission to use this command.");
			return true;
		}
		
		if(this.plugin.nativeWebServer == null) {
			sender.sendMessage(ChatColor.GOLD + "The native web server has not been enabled. To use this command, enable it in the config!");
			return true;
		}
		
		if(args.length != 2) {
			sender.sendMessage(ChatColor.GOLD + "Invalid number of arguments. See " + ChatColor.RED + "/rconsole" + ChatColor.GOLD + " for help.");
			return true;
		}
		
		this.plugin.nativeWebServer.addUser(args[0], args[1]);
		sender.sendMessage(String.format("%sAdded user %s%s%s to the database.", ChatColor.GOLD, ChatColor.RED, args[0], ChatColor.GOLD));
		return true;
	}

	@Override
	public List<String> onTabComplete(CommandSender sender, List<String> args) {
		if(!sender.hasPermission("rconsole.adduser")) {
			return null;
		}
		
		if(args.size() == 1) {
			return Arrays.asList(new String[] {"<username>"});
		}
		
		if(args.size() == 2) {
			return Arrays.asList(new String[] {"<password>"});
		}
		
		return null;
	}

}
