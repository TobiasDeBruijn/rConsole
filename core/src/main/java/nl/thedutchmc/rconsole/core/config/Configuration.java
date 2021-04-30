package nl.thedutchmc.rconsole.core.config;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.function.BiConsumer;

import org.yaml.snakeyaml.Yaml;

import com.google.gson.Gson;

import nl.thedutchmc.rconsole.core.config.gson.ConfigObject;

public class Configuration {

	private ConfigObject configObj;
	
	public Configuration(File dataFolder, BiConsumer<String, Boolean> saveResourceFunction) {
		loadConfig(dataFolder, saveResourceFunction);
	}
	
	private void loadConfig(File dataFolder, BiConsumer<String, Boolean> saveResourceFunction) {
		File configFile = new File(dataFolder, "config.yml");
		
		if(!configFile.exists()) {
			configFile.getParentFile().mkdirs();
			saveResourceFunction.accept("config.yml", false);
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
