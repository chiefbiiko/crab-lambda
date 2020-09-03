use anyhow::Result as Rezult;
// use serde_json::json;

use core::intrinsics;
use core::panic::PanicInfo;

#[no_mangle]
pub fn lambda(event: String, context: String) -> Rezult<String> {
    println!("{}", &event);
    println!("{}", &context);

    // Ok(json!({ "fraud": "money" }).to_string())
    Ok("{\"fraud\":\"money\"}".to_string())
}

#[lang = "eh_personality"] extern fn "C" rust_eh_personality() {}
#[lang = "panic_impl"] extern fn "C" rust_begin_panic(info: &PanicInfo) -> ! { unsafe { intrinsics::abort() } }
