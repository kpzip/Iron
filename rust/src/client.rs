use std::sync::Mutex;
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::jint;

static mut INSTANCE: Option<Mutex<MinecraftClient>> = None;

static GL_ERROR_DIALOGUE: &'static str = "Please make sure you have up-to-date drivers (see aka.ms/mcdriver for instructions).";

pub struct MinecraftClient {

}