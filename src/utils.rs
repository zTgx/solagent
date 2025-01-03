#[macro_export]
macro_rules! json_schema {
    ($($name:ident: $type:tt),* $(,)?) => {{
        let mut properties = serde_json::Map::new();
        $(
            let property = match stringify!($type) {
                "String" => json!({
                    "type": "string"
                }),
                "i32" | "i64" | "u64" | "u32" => json!({
                    "type": "number"
                }),
                "bool" => json!({
                    "type": "boolean"
                }),
                _ => panic!("Unsupported type"),
            };
            properties.insert(stringify!($name).to_string(), property);
        )*
        json!({
            "type": "object",
            "properties": properties,
        })
    }};
}
