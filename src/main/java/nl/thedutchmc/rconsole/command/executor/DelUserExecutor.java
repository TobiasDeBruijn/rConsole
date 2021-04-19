package nl.thedutchmc.rconsole.command.executor;

import java.util.Arrays;
import java.util.List;

import org.bukkit.command.CommandSender;

import net.md_5.bungee.api.ChatColor;
import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.command.PluginCommand;

public class DelUserExecutor implements PluginCommand {

	private RConsole plugin; 
	public DelUserExecutor(RConsole plugin) {
		this.plugin = plugin;
		
		this.plugin.commandLoader.registerSimple("deluser", "rconsole", null, this);
	}
	
	@Override
	public boolean onCommand(CommandSender sender, String[] args) {
		if(!sender.hasPermission("rconsole.deluser")) {
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
		
		
		if(this.plugin.nativeWebServer.delUser(args[0])) {
			sender.sendMessage(ChatColor.GOLD + String.format("User %s%s%s was deleted successfully.", ChatColor.RED, args[0], ChatColor.GOLD));
		} else {
			sender.sendMessage(ChatColor.GOLD + String.format("Deleting user %s%s%s unsuccessful. See console for more details.", ChatColor.RED, args[0], ChatColor.GOLD));
		}
		
		return true;
	}

	@Override
	public List<String> onTabComplete(CommandSender sender, List<String> args) {
		if(!sender.hasPermission("rconsole.deluser")) {
			return null;
		}
		
		if(args.size() == 1) {
			String[] users = this.plugin.nativeWebServer.listUsers();
			return Arrays.asList(users);
		}
		
		return null;
	}

}
