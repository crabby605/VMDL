use std::collections::HashMap;
use std::fmt;
use serde::Serialize;
///
/// this parser is hereby released under the MIT license.
/// copyright (c) 2025, Vihaan Pingalkar.
/// 
#[derive(Debug, Clone, Serialize)]
pub enum Value {
    String(String),
    Object(HashMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Object(_) => write!(f, "<Object>"),
        }
    }
}

impl Value {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(map) => map.get(key),
            _ => None,
        }
    }

    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.get(key).and_then(|v| v.as_string())
    }

    pub fn get_object(&self, key: &str) -> Option<&HashMap<String, Value>> {
        self.get(key).and_then(|v| v.as_object())
    }
}

pub fn parse(content: &str) -> Result<Value, String> {
    // fisrt pass: convert the content into a flat key-value structure
    let mut flat_map = HashMap::new();
    let mut context_stack: Vec<String> = Vec::new();
    let mut current_context = String::new();
    
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        
        // skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        // check if this line defines a key-value pair
        if let Some(separator_pos) = trimmed.find(|c| c == '=' || c == ':') {
            let key = trimmed[..separator_pos].trim().to_string();
            let value_str = trimmed[separator_pos+1..].trim();
            let separator = trimmed.chars().nth(separator_pos).unwrap();
            
            // determine the full key path based on context
            let full_key = if current_context.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", current_context, key)
            };
            
            // store the key-value pair in our flat map
            flat_map.insert(full_key.clone(), value_str.to_string());
            
            // if this is an object declaration (ends with colon and empty value)
            if separator == ':' && value_str.is_empty() {
                // update context for subsequent lines
                context_stack.push(current_context.clone());
                current_context = full_key;
            }
        } else {
            // if we reach here and the line is not empty, its a malformed line
            if !trimmed.is_empty() {
                return Err(format!("Invalid line format at line {}: {}", line_num + 1, line));
            }
        }
    }
    
    // second pass convert flat map to hierarchical structure
    let root = build_hierarchy_from_flat(&flat_map)?;
    Ok(root)
}

fn build_hierarchy_from_flat(flat_map: &HashMap<String, String>) -> Result<Value, String> {
    let mut root = HashMap::new();
    
    for (key_path, value) in flat_map {
        let parts: Vec<&str> = key_path.split('.').collect();
        let mut current = &mut root;
        
        //process all but the last part as object contexts
        for (i, &part) in parts.iter().enumerate().take(parts.len() - 1) {
            if !current.contains_key(part) {
                current.insert(part.to_string(), Value::Object(HashMap::new()));
            }
            
            if let Some(Value::Object(ref mut obj)) = current.get_mut(part) {
                current = obj;
            } else {
                return Err(format!("Key conflict: '{}' is both a value and an object", 
                                  parts[..=i].join(".")));
            }
        }
        
        // process the last part as a value
        let last_part = parts.last().unwrap();
        
        // skip empty oject declarations (already created above)
        if !value.is_empty() || !current.contains_key(*last_part) {
            current.insert(last_part.to_string(), Value::String(value.clone()));
        }
    }
    
    Ok(Value::Object(root))
}

fn count_indent(line: &str) -> usize {
    line.chars().take_while(|&c| c == ' ').count()
}

pub fn to_json(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| "Error converting to JSON".to_string())
}

pub fn to_text(value: &Value, indent: usize) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Object(map) => {
            let indent_str = " ".repeat(indent);
            let mut result = String::new();
            
            for (key, val) in map.iter() {
                match val {
                    Value::String(s) => {
                        result.push_str(&format!("{}{} = {}\n", indent_str, key, s));
                    },
                    Value::Object(_) => {
                        result.push_str(&format!("{}{}:\n", indent_str, key));
                        result.push_str(&to_text(val, indent + 4));
                    }
                }
            }
            
            result
        }
    }
}