package nl.thedutchmc.rconsole.dashboard;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.Files;
import java.util.regex.Pattern;

import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.Util;

public class DashboardServer {

	private static boolean LIB_LOADED = false;
	private Native nativeObj;
	
	static {
		saveNativeLib: {
			/*
			String osString = System.getProperty("os.name").toLowerCase();
			String nativeLibraryName;
			
			if(osString.contains("linux") ) {
				nativeLibraryName = "/x86_64/linux/librconsole.so";
			} else if(osString.contains("windows")) {
				nativeLibraryName = "/x86_64/windows/librconsole.dll";
			} else {
				RConsole.logWarn(String.format("You are running on an OS not supported by rConsole (%s). The build-in webserver for the web interface will not work.", osString));
				break saveNativeLib;
			}
		
			URL libUrl = DashboardServer.class.getResource(nativeLibraryName);
			RConsole.logDebug(libUrl.getPath());
			File tmpDir;
			try {
				tmpDir = Files.createTempDirectory("librconsole").toFile();
			} catch(IOException e) {
				RConsole.logWarn("An error occurred while creating a temporary directory for 'librconsole': " + e.getMessage());
				RConsole.logDebug(Util.getStackTrace(e));
				
				break saveNativeLib;
			}
			
			String[] nativeLibNameParts = nativeLibraryName.split(Pattern.quote("/"));
			String fileName = nativeLibNameParts[nativeLibNameParts.length-1];
			RConsole.logDebug("Filename: " + fileName);
			
			File libTmpFile = new File(tmpDir, fileName);
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
			*/
			try {
				System.load("/tmp/librconsole.so");
			} catch(Exception e) {
				e.printStackTrace();
			}
			LIB_LOADED = true;
		}
	}
	
	public void startDashboardServer(String configFolder) {
		if(!LIB_LOADED) {
			RConsole.logWarn("Unable to start Dashboard server because the native library 'librconsole' is not loaded");
			return;
		}
		
		File librconsoleConfigFolder = new File(configFolder + File.separator + "librconsole");
		if(!librconsoleConfigFolder.exists()) {
			librconsoleConfigFolder.mkdirs();
		}
		
		File librconsoleConfigFile = new File(librconsoleConfigFolder, "config.yml");
		this.nativeObj = new Native();
		this.nativeObj.nativeStartDashboardServer(librconsoleConfigFile.getAbsolutePath());
	}
	
	public void stopDashboardServer() {
		if(!LIB_LOADED) {
			return;
		}
		
		this.nativeObj.nativeStopDashboardServer();
	}
}
