use anyhow::Result as Rezult;
// use serde_json::json;
use core::panic;
// https://blog-os.netlify.app/freestanding-rust-binary/

#[no_mangle]
pub fn lambda(event: String, context: String) -> Rezult<String> {
    println!("{}", &event);
    println!("{}", &context);

    // Ok(json!({ "fraud": "money" }).to_string())
    Ok("{\"fraud\":\"money\"}".to_string())
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    panic!(_info);
}