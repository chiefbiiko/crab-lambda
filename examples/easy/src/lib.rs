use anyhow::Result as Rezult;
use serde_json::json;

pub fn lambda(event: String, context: String) -> Rezult<String> {
    println!("{}", event);
    println!("{}", context);

    Ok(json!({ "🦀🐑": "made by chiefbiiko" }).to_string())
}
