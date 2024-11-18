use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::jint;


#[no_mangle]
pub extern "system" fn Java_xyz_kpzip_Iron_rustTest<'local>(mut env: JNIEnv<'local>, class: JClass<'local>) -> jint {
    5
}

