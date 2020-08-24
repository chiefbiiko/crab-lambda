use anyhow::Result as Rezult;
use serde_json::json;

#[no_mangle]
pub fn lambda(event: String, context: String) -> Rezult<String> {
    println!("{}", event);
    println!("{}", context);

    Ok(json!({ "fraud": "money" }).to_string())
}
