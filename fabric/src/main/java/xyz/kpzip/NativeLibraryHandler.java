/**
 * 
 */
package xyz.kpzip;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;

/**
 * 
 * @author kpzip
 * 
 */
public final class NativeLibraryHandler {

	/**
	 * 
	 */
	private NativeLibraryHandler() {}
	
	
	public static void loadJarDll(String name) throws IOException {
	    InputStream in = NativeLibraryHandler.class.getResourceAsStream(name);
	    byte[] buffer = new byte[1024];
	    int read = -1;
	    File temp = File.createTempFile(name, "");
	    FileOutputStream fos = new FileOutputStream(temp);

	    while((read = in.read(buffer)) != -1) {
	        fos.write(buffer, 0, read);
	    }
	    fos.close();
	    in.close();

	    System.load(temp.getAbsolutePath());
	}

}
