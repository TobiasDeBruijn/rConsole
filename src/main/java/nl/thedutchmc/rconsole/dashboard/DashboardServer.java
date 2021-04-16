package nl.thedutchmc.rconsole.dashboard;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.Files;

import nl.thedutchmc.rconsole.RConsole;
import nl.thedutchmc.rconsole.Util;

public class DashboardServer {

	private static boolean LIB_LOADED = false;
	
	static {
		saveNativeLib: {
			String osString = System.getProperty("os.name").toLowerCase();
			String nativeLibraryName;
			
			if(osString.contains("linux") ) {
				nativeLibraryName = "/x86_64/linux/librconsole-dashboard.so";
			} else if(osString.contains("windows")) {
				nativeLibraryName = "/x86_64/windows/librconsole-dashboard.dll";
			} else {
				RConsole.logWarn("You are running on an OS not supported by rConsole. The build-in webserver for the web interface will not work.");
				break saveNativeLib;
			}
		
			URL libUrl = DashboardServer.class.getResource(nativeLibraryName);
			File tmpDir;
			try {
				tmpDir = Files.createTempDirectory("librconsole-dashboard").toFile();
			} catch(IOException e) {
				RConsole.logWarn("An error occurred while creating a temporary directory for 'librconsole-dashboard': " + e.getMessage());
				RConsole.logDebug(Util.getStackTrace(e));
				
				break saveNativeLib;
			}
			
			File libTmpFile = new File(tmpDir, nativeLibraryName);
			libTmpFile.deleteOnExit();
			
			try {
				InputStream in = libUrl.openStream();
				Files.copy(in, libTmpFile.toPath());
			} catch(IOException e) {
				RConsole.logWarn("An error occurred while saving 'librconsole-dashboard': " + e.getMessage());
				RConsole.logDebug(Util.getStackTrace(e));
				
				break saveNativeLib;
			}
			
			try {
				System.load(libTmpFile.getAbsolutePath());
			} catch(UnsatisfiedLinkError e) {
				RConsole.logWarn("An error occurred while loading 'librconsole-dashboard': " + e.getMessage());
				RConsole.logDebug(Util.getStackTrace(e));
				
				break saveNativeLib;
			}
			
			LIB_LOADED = true;
		}
	}
	
	public void startDashboardServer(String configFolder) {
		if(!LIB_LOADED) {
			RConsole.logWarn("Unable to start Dashboard server because the native library 'librconsole-dashboard' is not loaded");
			return;
		}
		
		nativeStartDashboardServer(configFolder);
	}
	
	public void stopDashboardServer() {
		if(!LIB_LOADED) {
			return;
		}
		
		nativeStopDashboardServer();
	}
	
	private native void nativeStartDashboardServer(String configFolder);
	
	private native void nativeStopDashboardServer();
}
