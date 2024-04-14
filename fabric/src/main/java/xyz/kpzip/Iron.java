package xyz.kpzip;

import java.io.IOException;
import java.lang.reflect.Field;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import net.fabricmc.api.ModInitializer;
import net.minecraft.client.MinecraftClient;

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
		
		
		formatRustFields(MinecraftClient.class);

		LOGGER.info("Loading Native Libraries...");
		
		try {
			NativeLibraryHandler.loadJarDll("/META-INF/natives/windows/x86_64/test64.dll");
		} catch (IOException e) {
			e.printStackTrace();
		}
		System.out.println("From Rust: " + rustTest());
		
	}
	
	public static native int rustTest();
	
	private static void formatRustFields(Class<?> clazz) {
		Field[] fs = clazz.getDeclaredFields();
		LOGGER.info(String.valueOf(fs.length));
		for (Field f : fs) {
			if (f.getType() == boolean.class) {
				LOGGER.info(f.getName() + ": bool,");
			}
			else if (f.getType() == byte.class) {
				LOGGER.info(f.getName() + ": i8,");
			}
			else if (f.getType() == short.class) {
				LOGGER.info(f.getName() + ": i16,");
			}
			else if (f.getType() == int.class) {
				LOGGER.info(f.getName() + ": i32,");
			}
			else if (f.getType() == long.class) {
				LOGGER.info(f.getName() + ": i64,");
			}
			else if (f.getType() == char.class) {
				LOGGER.info(f.getName() + ": i16,");
			}
			else if (f.getType() == float.class) {
				LOGGER.info(f.getName() + ": f32,");
			}
			else if (f.getType() == double.class) {
				LOGGER.info(f.getName() + ": i64,");
			}
			else {
				LOGGER.info(f.getName() + ": jobject,");
			}
		}
	}
}