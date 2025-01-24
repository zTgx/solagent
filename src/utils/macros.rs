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

use serde_json::{json, Value};

pub fn get_json_type(ty: &syn::Type) -> Value {
    match ty {
        syn::Type::Path(type_path) => {
            let segment = &type_path.path.segments[0];
            let type_name = segment.ident.to_string();

            match type_name.as_str() {
                "Vec" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let syn::GenericArgument::Type(inner_type) = &args.args[0] {
                            let inner_json_type = get_json_type(inner_type);
                            return json!({
                                "type": "array",
                                "items": inner_json_type
                            });
                        }
                    }
                    json!({ "type": "object" })
                }
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64" => {
                    json!({ "type": "number" })
                }
                "String" | "str" => {
                    json!({ "type": "string" })
                }
                "bool" => {
                    json!({ "type": "boolean" })
                }
                _ => {
                    json!({ "type": "object" })
                }
            }
        }
        _ => json!({ "type": "object" }),
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! parameters_json_schema {
    ($($name:ident: $type:ty),* $(,)?) => {{
        let mut properties = serde_json::Map::new();
        $(
            let property = $crate::utils::macros::get_json_type(&syn::parse_quote!(<$type>));
            properties.insert(stringify!($name).to_string(), property);
        )*
        json!({
            "type": "object",
            "properties": properties,
        })
    }};
}
