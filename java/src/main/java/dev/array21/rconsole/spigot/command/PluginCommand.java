package dev.array21.rconsole.spigot.command;

import java.util.List;

import org.bukkit.command.CommandSender;

public interface PluginCommand {
	public boolean onCommand(CommandSender sender, String[] args);
	public List<String> onTabComplete(CommandSender sender, List<String> args);
}
