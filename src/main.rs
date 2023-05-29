use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_string = std::fs::read_to_string("sample.json")?;
    let json_object: serde_json::Value = serde_json::from_str(&json_string)?;
    println!("Loaded JSON object.");

    let mut value = String::new();
    eprint!("What value are you searching for: ");
    std::io::stdin().read_line(&mut value)?;
    value = value.trim().to_string();

    let mut map: HashMap<String, String> = HashMap::new();

    if let Some(array) = json_object.as_array() {
        search_array(String::new(), array, &value, &mut map);
    }
    else if let Some(object) = json_object.as_object() {
        search_object(String::new(), object, &value, &mut map);
    }
    else {
        panic!("Cannot iterate through this JSON data because it is not an array or a map.");
    }

    std::fs::write("found_paths.json",  serde_json::to_string_pretty(&map)?)?;
    println!("Written all paths to 'found_paths.json'.");

    return Ok(());
}

fn search_array(path: String, data: &Vec<serde_json::Value>, query: &str, map: &mut HashMap<String, String>) {
    for (index, value) in data.iter().enumerate() {
        if let Some(object) = value.as_object() {
            search_object(format!("{}[{}]", path, index), object, query, map);
        }
        else if let Some(array) = value.as_array() {
            search_array(format!("{}[{}]", path, index), array, query, map);
        }
        else if value.to_string().contains(query) {
            let value_type: &'static str = find_type(value);
            map.insert(format!("{}[{}]", path, index), value_type.to_string());
        }
    }
}

fn search_object(path: String, data: &serde_json::Map<String, serde_json::Value>, query: &str, map: &mut HashMap<String, String>) {
    for (key, value) in data.iter() {
        if let Some(object) = value.as_object() {
            search_object(format!("{}['{}']", path, key), object, query, map);
        }
        else if let Some(array) = value.as_array() {
            search_array(format!("{}['{}']", path, key), array, query, map);
        }
        else if value.to_string().contains(query) {
            let value_type: &'static str = find_type(value);
            map.insert(format!("{}['{}']", path, key), value_type.to_string());
        }
    }
}

fn find_type(val: &serde_json::Value) -> &'static str {
    if val.is_boolean() {
        return "boolean";
    }
    if val.is_f64() {
        return "float64";
    }
    if val.is_i64() {
        return "int64";
    }
    if val.is_string() {
        return "string";
    }
    if val.is_object() {
        return "map<string, object>";
    }
    if val.is_array() {
        return "array<object>"
    }
    return "undefined";
}
