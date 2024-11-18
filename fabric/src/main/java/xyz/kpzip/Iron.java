package xyz.kpzip;

import java.io.FileWriter;
import java.io.IOException;
import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.nio.file.Files;
import java.nio.file.Path;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import net.fabricmc.api.ModInitializer;
import net.minecraft.block.Block;
import net.minecraft.registry.Registries;

public class Iron implements ModInitializer {
	
	
	public static final String MOD_ID = "iron";
	
	
	// This logger is used to write text to the console and the log file.
	// It is considered best practice to use your mod id as the logger's name.
	// That way, it's clear which mod wrote info, warnings, and errors.
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);

	@Override
	public void onInitialize() {
		// This code runs as soon as Minecraft is in a mod-load-ready state.
		// However, some things (like resources) may still be uninitialized.
		// Proceed with mild caution.
		
		
		//formatRustFields(MinecraftClient.class, true);
		FileWriter out = null;
        try {
        	out = new FileWriter("output.txt");
        	
        	
        	for (Block block : Registries.BLOCK) {
            	String str = block.getName().toString();
            	str = str.substring(33, str.length()-11);
            	out.write("\t\t\"minecraft:" + str + "\",\n");
            }
        	out.close();
        } catch (IOException e) {
			e.printStackTrace();
		} finally {
        	
        }

		LOGGER.info("Loading Native Libraries...");
		
		try {
			NativeLibraryHandler.loadJarDll("/META-INF/natives/windows/x86_64/test64.dll");
		} catch (IOException e) {
			e.printStackTrace();
		}
		System.out.println("From Rust: " + rustTest());
		
	}
	
	public static native int rustTest();
	
	private static void formatRustFields(Class<?> clazz, boolean file) {
		Field[] fs = clazz.getDeclaredFields();
		String s;
		StringBuilder str = new StringBuilder();
		for (Field f : fs) {
			if (Modifier.isStatic(f.getModifiers())) continue;
			if (f.getType() == boolean.class) {
				s = f.getName() + ": bool,\n";
			}
			else if (f.getType() == byte.class) {
				s = f.getName() + ": i8,\n";
			}
			else if (f.getType() == short.class) {
				s = f.getName() + ": i16,\n";
			}
			else if (f.getType() == int.class) {
				s = f.getName() + ": i32,\n";
			}
			else if (f.getType() == long.class) {
				s = f.getName() + ": i64,\n";
			}
			else if (f.getType() == char.class) {
				s = f.getName() + ": i16,\n";
			}
			else if (f.getType() == float.class) {
				s = f.getName() + ": f32,\n";
			}
			else if (f.getType() == double.class) {
				s = f.getName() + ": i64,\n";
			}
			else {
				s = f.getName() + ": JObject<'a>,\n";
			}
			LOGGER.debug(s);
			str.append(s);
			
		}
		if (file) {
			try {
				Files.writeString(Path.of("rust_formatter.txt"), str.toString());
			} catch (IOException e) {
				System.out.println("Error writing rust data to file");
				e.printStackTrace();
			}
		}
		LOGGER.debug("Formatted " + String.valueOf(fs.length) + " Fields.");
	}
}