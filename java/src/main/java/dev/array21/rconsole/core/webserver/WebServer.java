package dev.array21.rconsole.core.webserver;

import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Comparator;
import java.util.HashMap;
import java.util.function.BiConsumer;
import java.util.function.Consumer;
import java.util.regex.Pattern;

import org.yaml.snakeyaml.Yaml;
import org.yaml.snakeyaml.constructor.Constructor;
import org.yaml.snakeyaml.introspector.PropertyUtils;
import org.yaml.snakeyaml.representer.Representer;

import com.google.gson.Gson;

import net.lingala.zip4j.ZipFile;
import dev.array21.rconsole.core.RConsole;
import dev.array21.rconsole.core.annotations.Nullable;
import dev.array21.rconsole.core.util.Pair;
import dev.array21.rconsole.core.util.Util;

/**
 * Wrapper class around {@link Native}
 * @author Tobias de Bruijn
 *
 */
public class WebServer {

	private static boolean LIB_LOADED = false;
	private static RConsole rconsole;
	private static Consumer<String> consoleCommandExecutor;
	
	public WebServer(RConsole rconsole, Consumer<String> consoleCommandExecutor) {
		WebServer.rconsole = rconsole;
		WebServer.consoleCommandExecutor = consoleCommandExecutor;
	}
	
	static {
		saveLib: {
			String libName;
			
			String osName = System.getProperty("os.name").toLowerCase();
			if(osName.contains("linux")) {
				switch(System.getProperty("os.arch")) {
				case "amd64": libName = "/x86_64/linux/libskinfixer-focal.so"; break;
				case "arm": libName = "/armhf/linux/libskinfixer.so"; break;
				case "aarch64": libName = "/aarch64/linux/libskinfixer.so"; break;
				default:
					RConsole.logWarn(String.format("Your architecture is not supported. Please open a request here: https://github.com/TheDutchMC/rConsole/issues/new/choose. Your Arch is '%s' running on Linux, make sure you mention this in your request!", System.getProperty("os.arch")));
					break saveLib;
				}
				
			} else if(osName.contains("windows")) {
				switch(System.getProperty("os.arch")) {
				case "amd64": libName = "/x86_64/windows/libskinfixer.dll"; break;
				default:
					RConsole.logWarn(String.format("Your architecture is not supported. Please open a request here: https://github.com/TheDutchMC/rConsole/issues/new/choose. Your Arch is '%s' running on Windows, make sure you mention this in your request!", System.getProperty("os.arch")));
					break saveLib;
				}
				
			} else if(osName.contains("mac")) {
				switch(System.getProperty("os.arch")) {
				case "amd64": libName = "/x86_64/darwin/libskinfixer.dylib";; break;
				default:
					RConsole.logWarn(String.format("Your architecture is not supported. Please open a request here: https://github.com/TheDutchMC/rConsole/issues/new/choose. Your Arch is '%s' running on MacOS (Apple Darwin), make sure you mention this in your request!", System.getProperty("os.arch")));
					break saveLib;
				}			
			} else {
				RConsole.logWarn(String.format("Your operating system is not supported. Please open a request here: https://github.com/TheDutchMC/rConsole/issues/new/choose. Your OS is '%s', make sure you mention this in your request!", System.getProperty("os.name")));
				break saveLib;
			}
			
			Pair<File, File> pairedFile = saveLib(libName);
			if(pairedFile == null) {
				break saveLib;
			}
			
			File tmpDir = pairedFile.a();
			File libTmpFile = pairedFile.b();
			
			try {
				System.load(libTmpFile.getAbsolutePath());
			} catch(UnsatisfiedLinkError e) {
				if(osName.contains("linux")) {
					RConsole.logWarn("Failed to load libskinfixer Focal. Trying Bionic.");
					
					String loadBionicError = tryLoadAmd64Linux("bionic");
					if(loadBionicError != null) {
						RConsole.logWarn(loadBionicError);
						RConsole.logWarn("Failed to load libskinfixer Bionic. Trying Xenial.");					
						
						String loadXenialError = tryLoadAmd64Linux("xenial");
						if(loadXenialError != null) {
							RConsole.logWarn(loadXenialError);
							RConsole.logWarn("Failed to load libskinfixer Focal.");
							printLibDebugHelp();
						}
					}
				} else {
					RConsole.logWarn("Failed to load library: " + e.getMessage());
					libTmpFile.delete();
					tmpDir.delete();
					
					printLibDebugHelp();
					break saveLib;
				}
			}
			
			RConsole.logInfo("libskinfixer loaded.");
			LIB_LOADED = true;
		}
	}
	
	private static String tryLoadAmd64Linux(String distro) {
		String name = String.format("/x86_64/linux/libskinfixer-%s.so", distro);
		Pair<File, File> pairedFile = saveLib(name);
		if(pairedFile == null) {
			return String.format("Failed to save library libskinfixer-%.so", distro);
		}
		
		File tmpFolder = pairedFile.a();
		File libTmpFile = pairedFile.b();
		
		try {
			System.load(libTmpFile.getAbsolutePath());
		} catch(UnsatisfiedLinkError e) {
			libTmpFile.delete();
			tmpFolder.delete();
			return String.format("Failed to load library libskinfixer-%.so: %s", distro, Util.getStackTrace(e));
		}
		
		return null;
	}
	
	private static void printLibDebugHelp() {
		RConsole.logWarn("Check that all required dependencies are installed.");
		RConsole.logWarn("You should make sure that you are using GLIBC >=2.23.");
		RConsole.logWarn("If you are using GLIBC =< 2.23, make sure that you have libssl 1.0.0 AND libcrypto 1.0.0 installed.");
		RConsole.logWarn("If you are using GLIBC >= 2.31, make sure that you have OpenSSL (libssl 1.1 AND libcrypto 1.1)installed.");
		RConsole.logWarn("For more help you can join Dutchy76's Discord: https://discord.com/invite/xE3FcGj");
		RConsole.logWarn("Alternatively, you can open an issue on GitHub: https://github.com/TheDutchMC/SkinFixer/issues/new/choose");
		RConsole.logWarn("In either case, please include your Operating System, OS version, architecture, Minecraft version and the version of SkinFixer you are using");
	}
	
	@Nullable
	private static Pair<File, File> saveLib(String libName) {
		URL libUrl = WebServer.class.getResource(libName);
		File tmpDir;
		try {
			tmpDir = Files.createTempDirectory("libskinfixer").toFile();
		} catch (IOException e) {
			RConsole.logWarn("Failed to create temporary directory: " + e);
			return null;
		}
		
		String[] libNameParts = libName.split(Pattern.quote("/"));
		File libTmpFile = new File(tmpDir, libNameParts[libNameParts.length -1]);
		
		try {
			InputStream is = libUrl.openStream();
			Files.copy(is, libTmpFile.toPath());
		} catch(IOException e) {
			tmpDir.delete();
			
			RConsole.logWarn("Failed to save dynamic library as temporay file: " + e);
			return null;
		}
		
		libTmpFile.deleteOnExit();
		tmpDir.deleteOnExit();
		
		return new Pair<File, File>(tmpDir, libTmpFile);
	}
	
	/**
	 * Start the web server<br>
	 * This is a blocking method.
	 */
	public void startWebServer(BiConsumer<String, Boolean> saveResource, File dataFolder) {
		if(!LIB_LOADED) {
			RConsole.logWarn("Unable to start native webserver because the native library 'librconsole' is not loaded");
			return;
		}
		
		//Create the configuration folder for librconsole if it doesn't exist.
		File librconsoleConfigFolder = new File(dataFolder + File.separator + "librconsole");
		if(!librconsoleConfigFolder.exists()) {
			librconsoleConfigFolder.mkdirs();
		}
		
		//Class describing the configuration for librconsole
		//It's values come from the plugin's main configuration
		final class LibrconsoleConfig {
			//The variables in here aren't actually unused, they're serialized by Yaml
			
			@SuppressWarnings("unused")
			public int port = WebServer.rconsole.getConfig().getPort();
			
			@SuppressWarnings("unused")
			public String pepper = WebServer.rconsole.getConfig().getPepper();
		}
		
		//Serialize the LibrconsoleConfig object to a Yaml String
		PropertyUtils propUtils = new PropertyUtils();
		propUtils.setAllowReadOnlyProperties(true);
		Representer repr = new Representer();
		repr.setPropertyUtils(propUtils);
		
		final Yaml yaml = new Yaml(new Constructor(), repr);
		String librConsoleConfigSerialized = yaml.dump(new LibrconsoleConfig());
		
		//Write the String to $server/plugins/rConsole/librconsole/config.yml
		File librconsoleConfigFile = new File(librconsoleConfigFolder, "config.yml");
		try {
			librconsoleConfigFile.delete();
			BufferedWriter bw = new BufferedWriter(new FileWriter(librconsoleConfigFile));
			bw.write(librConsoleConfigSerialized);
			bw.close();
		} catch(IOException e) {
			e.printStackTrace();
		}

		//Get the 'static' folder, in which the website lives.
		//$server/plugins/rConsole/librconsole/static/
		File staticFilesFolder = new File(librconsoleConfigFolder + File.separator + "static");
		
		//Delete and recreate the static folder.
		//We want to do this because the files are likely to change if the 
		//plugin is updated. This way we make sure the files are
		//always up-to-date.
		if(staticFilesFolder.exists()) {
			try {
				Files.walk(staticFilesFolder.toPath())
				.sorted(Comparator.reverseOrder())
				.map(Path::toFile)
				.forEach(File::delete);
			} catch (IOException e1) {
				e1.printStackTrace();
			}
		}

		//Recreate the folder
		staticFilesFolder.mkdirs();
				
		//Serialize the website config to a JSON String
		//The WebConfig class is defined below this method.
		final Gson gson = new Gson();
		String webConfigSerialized = gson.toJson(new WebConfig());
		
		//Write the JSON String to $server/plugins/rConsole/librconsole/static/rconsole_web_config.json
		try {
			File webConfigFile = new File(staticFilesFolder, "rconsole_web_config.json");
			if(webConfigFile.exists()) {
				webConfigFile.delete();
			}
			
			BufferedWriter bw = new BufferedWriter(new FileWriter(webConfigFile));
			bw.write(webConfigSerialized);
			bw.close();
		} catch(IOException e) {
			e.printStackTrace();
		}
		
		//We always want to re-extract the zip file, because if there is an update to the plugin
		//the static web files also need to be updated. By deleting it every time,
		//we can guarantee that the files are always up to date.
		saveResource.accept("web.zip", true);
		try {
			File finalDistZipFile = new File(staticFilesFolder, "web.zip");
			Files.move(new File(dataFolder, "web.zip").toPath(), finalDistZipFile.toPath());
			new ZipFile(finalDistZipFile).extractAll(staticFilesFolder.getAbsolutePath());
			
			finalDistZipFile.delete();
		} catch (IOException e) {
			e.printStackTrace();
		}
		
		//Database file
		//$server/plugins/rConsole/librconsole/librconsole.db3
		File librconsoleDatabaseFile = new File(librconsoleConfigFolder, "librconsole.db3");
		
		//Finally, start the webserver in librconsole
		//This is a blocking method call.
		Native.startWebServer(librconsoleConfigFile.getAbsolutePath(), librconsoleDatabaseFile.getAbsolutePath(), staticFilesFolder.getAbsolutePath());
	}
	
	/**Class describing the configuration for the website<br>
	 * It's values come from the plugin's main configuration<br>
	 * <br>
	 * This is not a local class to {@link WebServer#startWebServer()} because of limitations in Gson
	 * 
	 * @see <a href="https://github.com/google/gson/issues/1595"> Gson issue 1595 on GitHub</a>
	 */
	final class WebConfig {
		//The variable isn't actually unused, it's serialized by Gson
		
		//Private constructor so the class cannot be instatiated in a different class than WebServer
		private WebConfig() {}
		
		@SuppressWarnings("unused")
		private String uri = WebServer.rconsole.getConfig().getBaseUrl();
	}
	
	/**
	 * Append a log entry<br>
	 * This will spawn a new Thread
	 * @param log The message of the log entry
	 * @param timestamp The timestamp of when the log occurred
	 * @param level The level at which the log occurred, INFO or WARN
	 * @param thread The thread from which the log originated
	 */
	public void log(String log, long timestamp, String level, String thread) {
		if(!LIB_LOADED) {
			return;
		}
		
		//We put this on a new thread, since it is possible that the lock of the Database object on the rust side isn't free,
		//Then this call will block, and because logging happens quite frequently, and we don't want to hold it up,
		//we put it on a different thread.
		new Thread(new Runnable() {
			@Override
			public void run() {
				Native.appendConsoleLog(log, timestamp, level, thread);
			}
		}).start();
	}
	
	/**
	 * Add a user
	 * @param username The username of the user
	 * @param password The password of the user
	 */
	//TODO this should return a Boolean indicating if the creation was successful
	public void addUser(String username, String password) {
		if(!LIB_LOADED) {
			return;
		}
		
		Native.addUser(username, password);
	}
	
	/**
	 * Delete a user
	 * @param username The username of the user to delete
	 * @return returns true if the deletion was successful, false if the user did not exist or null if an error occurred
	 */
	@Nullable
	public Boolean delUser(String username) {
		if(!LIB_LOADED) {
			return null;
		}
		
		return Native.delUser(username);
	}
	
	/**
	 * Get a list of existing usernames
	 * @return Returns the usernames as a String[], or null if an error occurred
	 */
	@Nullable
	public String[] listUsers() {
		if(!LIB_LOADED) {
			return null;
		}
		
		return Native.listUsers();
	}
	
	/**
	 * Get a HashMap with the username and sessions associated with that username
	 * @return Returns the HashMap, or null if an error occurred
	 */
	@Nullable
	public HashMap<String, String[]> getUserSessions() {
		if(!LIB_LOADED) {
			return null;
		}
		
		return Native.getUserSessions();
	}
	
	/**
	 * Delete a session
	 * @param sessionId The ID of the session to delete
	 * @return returns true if the session was deleted successfully, false if the session did not exist or null if an error occurred
	 */
	@Nullable
	public Boolean delSession(String sessionId) {
		if(!LIB_LOADED) {
			return null;
		}
		
		return Native.delSession(sessionId);
	}
	
	/**
	 * Execute a command<br>
	 * This is a wrapper method around {@link Bukkit#dispatchCommand(org.bukkit.command.CommandSender, String)}, this is because we need to sync with the main server thread.<br>
	 * This method is for native methods to call.
	 * @param cmd The command to execute
	 */
	//Private method because you shouldn't call it from java, and native code does not care about visibility modifiers.
	//The function is used, but again, from native code
	//
	//In theory we don't need this method, but reimplementing this with JNI is a massive pain
	@SuppressWarnings("unused")
	private static void execCommand(String cmd) {
		//We're doing a try/catch on all Exceptions, because if an exception occurs and it is not caught in Java code,
		//but is caught in native code instead, it could crash the JVM
		try {
			Runnable sync = new Runnable() {
				
				@Override
				public void run() {
					consoleCommandExecutor.accept(cmd);
				}
			};
			
			WebServer.rconsole.execSync(sync);
		} catch(Exception e) {
			e.printStackTrace();
		}
	}
}
