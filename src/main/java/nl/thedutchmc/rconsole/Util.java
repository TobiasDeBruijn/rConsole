package nl.thedutchmc.rconsole;

import java.io.PrintWriter;
import java.io.StringWriter;

public class Util {
	
	public static String getStackTrace(Throwable e) {
		StringWriter sw = new StringWriter();
		PrintWriter pw = new PrintWriter(sw, true);
		
		e.printStackTrace(pw);
		return sw.getBuffer().toString();
	}
}
