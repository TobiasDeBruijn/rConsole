package dev.array21.rconsole.spigot.command;

import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;

public class PluginCommandExecutor implements CommandExecutor {
	private PluginCommand command;
	
	public PluginCommandExecutor(PluginCommand command) {
		this.command = command;
	}
	
	@Override
	public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
		return this.command.onCommand(sender, args);
	}
}
