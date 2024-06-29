use midoku_bindings::exports::{Number, Value};
use miniserde::json;

/// Parse a json value into a midoku value
pub fn parse_value(value: json::Value) -> Result<Value, Box<dyn std::error::Error>> {
    match value {
        json::Value::Bool(value) => Ok(Value::Bool(value)),
        json::Value::Number(value) => match value {
            json::Number::I64(value) => Ok(Value::Number(Number::S64(value))),
            json::Number::U64(value) => Ok(Value::Number(Number::U64(value))),
            json::Number::F64(value) => Ok(Value::Number(Number::F64(value))),
        },
        json::Value::String(value) => Ok(Value::String(value)),
        json::Value::Array(value) => {
            let mut parsed_value: Vec<String> = Vec::new();
            for value in value {
                match value {
                    json::Value::String(value) => {
                        parsed_value.push(value);
                    }
                    _ => return Err("Invalid value type in array".into()),
                }
            }
            Ok(Value::Array(parsed_value))
        }
        json::Value::Object(value) => {
            let mut parsed_value: Vec<(String, String)> = Vec::new();
            for (key, value) in value {
                match value {
                    json::Value::String(value) => {
                        parsed_value.push((key, value));
                    }
                    _ => return Err("Invalid value type in object".into()),
                }
            }
            Ok(Value::Map(parsed_value))
        }
        _ => Err("Invalid value type".into()),
    }
}
