use std::time::Instant;

use serde_json::{json, Map, Value};

pub fn filter_json(json_data: &Value, excluded_keys: &[&str]) -> Value {
      let check_hashing = Instant::now();
      if let Value::Object(obj) = json_data {
            let filtered_obj: Map<String, Value> = obj
                  .iter()
                  .filter(|(key, _)| !excluded_keys.contains(&key.as_str()))
                  .map(|(key, value)| (key.clone(), value.clone()))
                  .collect();
            println!("Check filter json: {:?}", check_hashing.elapsed());
            return Value::Object(filtered_obj);
      } else {
            json!({})
      }
}
