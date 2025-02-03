// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_export]
macro_rules! parameters {
    ($($name:ident: $type:ty),* $(,)?) => {{
        use serde_json::json;

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

                s if s.starts_with("Vec<") && s.ends_with(">") => {

                    let inner_type_str = &s[4..s.len() - 1];
                    let inner_type = match inner_type_str {
                        "String" => json!({
                            "type": "string"
                        }),
                        "i32" | "i64" | "u64" | "u32" => json!({
                            "type": "number"
                        }),
                        "bool" => json!({
                            "type": "boolean"
                        }),
                        _ => json!({
                            "type": "object"
                        }),
                    };
                    json!({
                        "type": "array",
                        "items": inner_type
                    })
                }
                _ => {
                    json!({
                        "type": "object"
                    })
                }
            };
            properties.insert(stringify!($name).to_string(), property);
        )*
        json!({
            "type": "object",
            "properties": properties,
        })
    }};
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_parameters_with_option_string() {
        let schema = parameters!(name: Option<String>);
        let expected_schema = json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "object" // Since Option<T> is treated as an object
                }
            }
        });
        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_parameters_with_string() {
        let schema = parameters!(name: String);
        let expected_schema = json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                }
            }
        });
        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_parameters_with_i32() {
        let schema = parameters!(age: i32);
        let expected_schema = json!({
            "type": "object",
            "properties": {
                "age": {
                    "type": "number"
                }
            }
        });
        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_parameters_with_vec_string() {
        let schema = parameters!(tags: Vec<String>);
        let expected_schema = json!({
            "type": "object",
            "properties": {
                "tags": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            }
        });
        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_parameters_with_option_i32() {
        let schema = parameters!(score: Option<i32>);
        let expected_schema = json!({
            "type": "object",
            "properties": {
                "score": {
                    "type": "object" // Since Option<T> is treated as an object
                }
            }
        });
        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_parameters_with_vec_option_bool() {
        let schema = parameters!(flags: Vec<Option<bool>>);
        let expected_schema = json!({
            "type": "object",
            "properties": {
                "flags": {
                    "type": "array",
                    "items": {
                        "type": "object" // Since Option<T> is treated as an object
                    }
                }
            }
        });
        assert_eq!(schema, expected_schema);
    }
}