package nl.thedutchmc.rconsole.core;

import java.io.File;
import java.util.function.BiConsumer;
import java.util.function.Consumer;

public class CoreInitParams {
	private Consumer<Object> logDebugFn;
	private Consumer<Object> logInfoFn;
	private Consumer<Object> logWarnFn;
	private Consumer<Runnable> syncFn;
	private Consumer<String> execCommandFn;
	private BiConsumer<String, Boolean> saveResourceFn;
	private File dataFolder;
	private String semVer;
	
	private CoreInitParams() {}
	
	public static CoreInitParams newInitParams() {
		return new CoreInitParams();
	}
	
	/**
	 * The function to be used for [DEBUG] log items
	 * @param logDebugFn
	 * @return
	 */
	public CoreInitParams setLogDebugFn(Consumer<Object> logDebugFn) {
		this.logDebugFn = logDebugFn;
		return this;
	}
	
	/**
	 * The function to be used for [INFO] log items
	 * @param logInfoFn
	 * @return
	 */
	public CoreInitParams setLogInfoFn(Consumer<Object> logInfoFn) {
		this.logInfoFn = logInfoFn;
		return this;
	}
	
	/**
	 * The function to be used for [WARN] log items
	 * @param logWarnFn
	 * @return
	 */
	public CoreInitParams setLogWarnFn(Consumer<Object> logWarnFn) {
		this.logWarnFn = logWarnFn;
		return this;
	}
	
	/**
	 * The function to be used for executing code synchronously
	 * @param execSyncFn
	 * @return
	 */
	public CoreInitParams setExecSyncFn(Consumer<Runnable> execSyncFn) {
		this.syncFn = execSyncFn;
		return this;
	}
	
	/**
	 * The function to be used for executing commands as Console
	 * @param execCommandFn
	 * @return
	 */
	public CoreInitParams setExecCommandFn(Consumer<String> execCommandFn) {
		this.execCommandFn = execCommandFn;
		return this;
	}
	
	/**
	 * The function to be used for saving a resource from the jar.
	 * Where String is the name of the resource, and Boolean determines if an existing item should be overridden
	 * @param saveResourceFn
	 * @return
	 */
	public CoreInitParams setSaveResourceFn(BiConsumer<String, Boolean> saveResourceFn) {
		this.saveResourceFn =saveResourceFn;
		return this;
	}
	
	/**
	 * The data folder in which data for the plugin/mod should be stored
	 * @param dataFolder
	 * @return
	 */
	public CoreInitParams setDataFolder(File dataFolder) {
		this.dataFolder = dataFolder;
		return this;
	}
	
	/**
	 * The semantic version of this plugin. Ex. 0.6.0
	 * @param semVer
	 * @return
	 */
	public CoreInitParams semVer(String semVer) {
		this.semVer = semVer;
		return this;
	}
	
	protected Consumer<Object> getLogDebugFn() {
		return this.logDebugFn;
	}
	
	protected Consumer<Object> getLogInfoFn() {
		return this.logInfoFn;
	}
	
	protected Consumer<Object> getLogWarnFn() {
		return this.logWarnFn;
	}
	
	protected Consumer<Runnable> getExecSyncFn() {
		return this.syncFn;
	}
	
	protected Consumer<String> getExecCommandFn() {
		return this.execCommandFn;
	}
	
	protected BiConsumer<String, Boolean> getSaveResourceFn() {
		return this.saveResourceFn;
	}
	
	protected File getDataFolder() {
		return this.dataFolder;
	}
	
	protected String getSemVer() {
		return this.semVer;
	}
}
