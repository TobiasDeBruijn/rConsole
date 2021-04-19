package nl.thedutchmc.rconsole.command;

import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.InvocationTargetException;
import java.util.HashMap;

import org.bukkit.Bukkit;
import org.bukkit.command.Command;
import org.bukkit.command.CommandMap;
import org.bukkit.permissions.Permission;
import org.bukkit.permissions.PermissionDefault;
import org.bukkit.plugin.Plugin;
import org.bukkit.plugin.SimplePluginManager;

import nl.thedutchmc.rconsole.RConsole;

public class CommandLoader {

	private static Constructor<?> pluginCommandConstructor;
	private static CommandMap commandMap;
	private static SimplePluginManager pluginManager;
	static {
		Field commandMapField;
		try {
			commandMapField = Bukkit.getServer().getClass().getDeclaredField("commandMap");
			commandMapField.setAccessible(true);
			
			commandMap = (CommandMap) commandMapField.get(Bukkit.getServer());
			
			pluginCommandConstructor = org.bukkit.command.PluginCommand.class.getDeclaredConstructor(String.class, Plugin.class);
			pluginCommandConstructor.setAccessible(true);
			
			Field simplePluginManagerField = Bukkit.getServer().getClass().getDeclaredField("pluginManager");
			simplePluginManagerField.setAccessible(true);
			
			pluginManager = (SimplePluginManager) simplePluginManagerField.get(Bukkit.getServer());
		} catch (NoSuchFieldException | SecurityException e) {
			e.printStackTrace();
		} catch (NoSuchMethodException e) {
			e.printStackTrace();
		} catch (IllegalArgumentException e) {
			e.printStackTrace();
		} catch (IllegalAccessException e) {
			e.printStackTrace();
		}
	}
	
	private RConsole plugin;
	public CommandLoader(RConsole plugin) {
		this.plugin = plugin;
	}
	
	public void registerSimple(String name, String namespace, PermissionDefault permDefault, PluginCommand executor) {
		this.registerCommandExecutor(name, executor, namespace);
		this.registerTabCompleter(name, executor);
		
		this.registerPermissionNode(name, permDefault, String.format("Allows the usage of /%s from %s", name, namespace), null);
	}
	
	public void registerCommandExecutor(String name, PluginCommand executor, String namespace) {
		try {
			org.bukkit.command.PluginCommand pluginCmd = (org.bukkit.command.PluginCommand) pluginCommandConstructor.newInstance(name, this.plugin);
			pluginCmd.setExecutor(new PluginCommandExecutor(executor));
			commandMap.register(namespace, pluginCmd);

		} catch (InstantiationException | IllegalAccessException | IllegalArgumentException | InvocationTargetException e) {
			e.printStackTrace();
		}
	}
	
	public void registerTabCompleter(String commandName, PluginCommand tabCompleter) {
		Command cmd = commandMap.getCommand(commandName);
		if(cmd == null) {
			throw new IllegalStateException("Attempted to set tab completer before executor was set");
		}
		
		org.bukkit.command.PluginCommand pluginCmd = (org.bukkit.command.PluginCommand) cmd;
		pluginCmd.setTabCompleter(new PluginTabCompleter(tabCompleter));
	}
	
	public void registerPermissionNode(String name, PermissionDefault permissionDefault, String description, HashMap<String, Boolean> children) {
		if(description == null) {
			description = "";
		}
		
		if(permissionDefault == null) {
			permissionDefault = PermissionDefault.OP;
		}
		
		Permission permNode = null;
		if(children == null) {
			permNode = new Permission(name, description, permissionDefault);
		} else {
			permNode = new Permission(name, description, permissionDefault, children);
		}
		
		pluginManager.addPermission(permNode);
	}
}
