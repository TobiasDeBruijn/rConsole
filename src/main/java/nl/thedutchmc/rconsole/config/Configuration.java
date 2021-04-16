package nl.thedutchmc.rconsole.config;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import org.yaml.snakeyaml.Yaml;

import com.google.gson.Gson;

import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.config.gson.ConfigObject;

public class Configuration {

	private ConfigObject configObj;
	
	public Configuration(RConsole plugin) {
		
		loadConfig(plugin);
	}
	
	private void loadConfig(RConsole plugin) {
		File configFile = new File(plugin.getDataFolder(), "config.yml");
		
		if(!configFile.exists()) {
			configFile.getParentFile().mkdirs();
			plugin.saveResource("config.yml", false);
		}
		
		final Yaml yaml = new Yaml();
		final Gson gson = new Gson();

		Object loadedYaml;
		try {
			loadedYaml = yaml.load(new FileReader(configFile));
		} catch(FileNotFoundException e) {
			return;
		}

		String json = gson.toJson(loadedYaml, java.util.LinkedHashMap.class);		
		this.configObj = gson.fromJson(json, ConfigObject.class);
	}
	
	public ConfigObject getConfig() {
		return this.configObj;
	}
}
