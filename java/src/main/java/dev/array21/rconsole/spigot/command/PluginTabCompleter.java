package dev.array21.rconsole.spigot.command;

import java.util.Arrays;
import java.util.List;

import org.bukkit.command.Command;
import org.bukkit.command.CommandSender;
import org.bukkit.command.TabCompleter;

public class PluginTabCompleter implements TabCompleter {

	PluginCommand pluginCmd;
	public PluginTabCompleter(PluginCommand pluginCmd) {
		this.pluginCmd = pluginCmd;
	}
	
	@Override
	public List<String> onTabComplete(CommandSender sender, Command command, String alias, String[] args) {
		return this.pluginCmd.onTabComplete(sender, Arrays.asList(args));
	}
}
