use jni::JNIEnv;
use jni::objects::{JClass, JObject};


#[no_mangle]
pub extern "system" fn Java_xyz_kpzip_RustTest_rustTest<'local>(mut env: JNIEnv<'local>, class: JClass<'local>) {
    println!("Hello From Rust!");
    
}

