package nl.thedutchmc.rconsole.webserver;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.Files;
import java.util.HashMap;
import java.util.regex.Pattern;

import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.annotations.Nullable;
import nl.thedutchmc.rconsole.util.Util;

public class WebServer {

	private static boolean LIB_LOADED = false;
	private RConsole plugin;
	
	public WebServer(RConsole plugin) {
		this.plugin = plugin;
	}
	
	static {
		saveNativeLib: {
			String osString = System.getProperty("os.name").toLowerCase();
			String nativeLibraryName;
			
			if(osString.contains("linux") ) {
				nativeLibraryName = "/x86_64/linux/librconsole.so";
			} else if(osString.contains("windows")) {
				nativeLibraryName = "/x86_64/windows/librconsole.dll";
			} else {
				RConsole.logWarn(String.format("You are running on an OS not supported by rConsole (%s). The built-in webserver will not work.", osString));
				break saveNativeLib;
			}
		
			URL libUrl = WebServer.class.getResource(nativeLibraryName);
			File tmpDir;
			try {
				tmpDir = Files.createTempDirectory("librconsole").toFile();
			} catch(IOException e) {
				RConsole.logWarn("An error occurred while creating a temporary directory for 'librconsole': " + e.getMessage());
				RConsole.logDebug(Util.getStackTrace(e));
				
				break saveNativeLib;
			}
			
			String[] nativeLibNameParts = nativeLibraryName.split(Pattern.quote("/"));
			File libTmpFile = new File(tmpDir, nativeLibNameParts[nativeLibNameParts.length-1]);
			libTmpFile.deleteOnExit();
			
			try {
				InputStream in = libUrl.openStream();
				Files.copy(in, libTmpFile.toPath());
			} catch(IOException e) {
				RConsole.logWarn("An error occurred while saving 'librconsole': " + e.getMessage());
				RConsole.logDebug(Util.getStackTrace(e));
				tmpDir.delete();
				break saveNativeLib;
			}
			
			try {
				System.load(libTmpFile.getAbsolutePath());
			} catch(UnsatisfiedLinkError e) {
				RConsole.logWarn("An error occurred while loading 'librconsole': " + e.getMessage());
				RConsole.logDebug(Util.getStackTrace(e));
				tmpDir.delete();
				break saveNativeLib;
			}

			
			LIB_LOADED = true;
		}
	}
	
	public void startWebServer(String configFolder) {
		if(!LIB_LOADED) {
			RConsole.logWarn("Unable to start Dashboard server because the native library 'librconsole' is not loaded");
			return;
		}
		
		File librconsoleConfigFolder = new File(configFolder + File.separator + "librconsole");
		if(!librconsoleConfigFolder.exists()) {
			librconsoleConfigFolder.mkdirs();
		}
		
		File librconsoleConfigFile = new File(librconsoleConfigFolder, "config.yml");
		File librconsoleDatabaseFile = new File(librconsoleConfigFolder, "librconsole.db3");
		
		File staticFilesFolder = new File(librconsoleConfigFolder + File.separator + "static");
		if(!staticFilesFolder.exists()) {
			staticFilesFolder.mkdirs();
			
		}
		
		Native.startWebServer(librconsoleConfigFile.getAbsolutePath(), librconsoleDatabaseFile.getAbsolutePath(), staticFilesFolder.getAbsolutePath());
	}
	
	public void log(String log, long timestamp, String level, String thread) {
		if(!LIB_LOADED) {
			return;
		}
		
		Native.appendConsoleLog(log, timestamp, level, thread);
	}
	
	public void addUser(String username, String password) {
		if(!LIB_LOADED) {
			return;
		}
		
		Native.addUser(username, password);
	}
	
	@Nullable
	public Boolean delUser(String username) {
		if(!LIB_LOADED) {
			return null;
		}
		
		return Native.delUser(username);
	}
	
	@Nullable
	public String[] listUsers() {
		if(!LIB_LOADED) {
			return null;
		}
		
		return Native.listUsers();
	}
	
	@Nullable
	public HashMap<String, String[]> getUserSessions() {
		if(!LIB_LOADED) {
			return null;
		}
		
		return Native.getUserSessions();
	}
	
	@Nullable
	public Boolean delSession(String sessionId) {
		if(!LIB_LOADED) {
			return null;
		}
		
		return Native.delSession(sessionId);
	}
}
