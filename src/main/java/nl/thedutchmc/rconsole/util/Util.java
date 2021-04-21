package nl.thedutchmc.rconsole.util;

import java.io.PrintWriter;
import java.io.StringWriter;

/**
 * Utility methods
 * @author Tobias de Bruijn
 *
 */
public class Util {
	
	/**
	 * Extract the Stacktrace as a String from a Throwable
	 * @param e The Throwable
	 * @return The stacktrace
	 */
	public static String getStackTrace(Throwable e) {
		StringWriter sw = new StringWriter();
		PrintWriter pw = new PrintWriter(sw, true);
		
		e.printStackTrace(pw);
		return sw.getBuffer().toString();
	}
}
