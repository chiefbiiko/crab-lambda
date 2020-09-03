#![no_main]
#![no_std]

use anyhow::Result as Rezult;
// use serde_json::json;
#[cfg(not(debug_assertions))]
use panic_abort as _;

#[no_mangle]
pub fn lambda(event: String, context: String) -> Rezult<String> {
    println!("{}", &event);
    println!("{}", &context);

    // Ok(json!({ "fraud": "money" }).to_string())
    Ok("{\"fraud\":\"money\"}".to_string())
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
