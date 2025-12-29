// CSV zu anderen Formaten konvertieren

use std::fs;
use crate::error::FormatError;
use crate::formats::utils::json_to_toml_value;
use csv::ReaderBuilder;
use serde_json::Value as JsonValue;

/// Konvertiert CSV zu JSON (flache Struktur)
/// Pipeline: CSV → Vec<HashMap> → serde_json::Value (Array von Objekten) → JSON String
/// Beispiel: "contact.email" → {"contact.email": "..."} (kein Verschachteln)
pub fn convert_csv_to_json(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. CSV-Datei lesen und parsen (flach, keine Dot-Notation)
    let records = read_csv_to_json_flat(input_path)?;
    
    // 2. Vec → JSON Array
    let json_value = JsonValue::Array(records);
    
    // 3. JSON Value → Pretty-Printed String
    let json_string = serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;
    
    // 4. String in Datei schreiben
    fs::write(output_path, json_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert CSV zu YAML (flache Struktur)
/// Pipeline: CSV → Vec<HashMap> → serde_json::Value → YAML String
/// Beispiel: "contact.email" → {"contact.email": "..."} (kein Verschachteln)
pub fn convert_csv_to_yaml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. CSV-Datei lesen und parsen (flach, keine Dot-Notation)
    let records = read_csv_to_json_flat(input_path)?;
    
    // 2. Vec → JSON Array (als Zwischenschritt)
    let json_value = JsonValue::Array(records);
    
    // 3. JSON Value → YAML String
    let yaml_string = serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von YAML: {}", e)))?;
    
    // 4. String in Datei schreiben
    fs::write(output_path, yaml_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert CSV zu TOML (flache Struktur)
/// Pipeline: CSV → Vec<HashMap> → serde_json::Value → toml::Value → TOML String
/// ACHTUNG: TOML unterstützt keine Arrays als Root, daher wird das Array unter "data" gespeichert
/// Beispiel: "contact.email" → {"contact.email": "..."} (kein Verschachteln)
pub fn convert_csv_to_toml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. CSV-Datei lesen und parsen (flach, keine Dot-Notation)
    let records = read_csv_to_json_flat(input_path)?;
    
    // 2. Vec → JSON Array
    let json_array = JsonValue::Array(records);
    
    // 3. TOML braucht ein Objekt als Root, nicht ein Array
    // Daher packen wir das Array in ein Objekt unter dem Key "data"
    let mut root_object = serde_json::Map::new();
    root_object.insert("data".to_string(), json_array);
    let json_value = JsonValue::Object(root_object);
    
    // 4. JSON Value → TOML Value konvertieren
    let toml_value = json_to_toml_value(&json_value)?;
    
    // 5. TOML Value → Pretty-Printed String
    let toml_string = toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von TOML: {}", e)))?;
    
    // 6. String in Datei schreiben
    fs::write(output_path, toml_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert CSV zu CSV (Formatierung / Cleanup)
/// Pipeline: CSV → Vec<HashMap> → CSV (neu formatiert)
pub fn convert_csv_to_csv(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. CSV-Datei lesen
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von CSV {}: {}", input_path, e)))?;
    
    // 2. CSV-Writer erstellen
    let mut writer = csv::Writer::from_path(output_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Erstellen von CSV: {}", e)))?;
    
    // 3. Header schreiben
    let headers = reader.headers()
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen der CSV-Header: {}", e)))?
        .clone();
    
    writer.write_record(&headers)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Header: {}", e)))?;
    
    // 4. Alle Records schreiben
    for result in reader.records() {
        let record = result
            .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen eines CSV-Records: {}", e)))?;
        
        writer.write_record(&record)
            .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben eines CSV-Records: {}", e)))?;
    }
    
    // 5. Writer flushen
    writer.flush()
        .map_err(|e| FormatError::IoError(format!("Fehler beim Abschliessen von CSV: {}", e)))?;

    Ok(())
}

/// Hilfsfunktion: Versucht den Typ eines String-Wertes zu erkennen
/// 
/// Reihenfolge:
/// 1. Boolean (true/false)
/// 2. Integer
/// 3. Float
/// 4. String (fallback)
fn infer_type(value: &str) -> JsonValue {
    // Leerer String → null
    if value.is_empty() {
        return JsonValue::Null;
    }
    
    // Boolean
    match value.to_lowercase().as_str() {
        "true" => return JsonValue::Bool(true),
        "false" => return JsonValue::Bool(false),
        _ => {}
    }
    
    // Integer
    if let Ok(num) = value.parse::<i64>() {
        return JsonValue::Number(num.into());
    }
    
    // Float
    if let Ok(num) = value.parse::<f64>() {
        if let Some(json_num) = serde_json::Number::from_f64(num) {
            return JsonValue::Number(json_num);
        }
    }
    
    // Fallback: String
    JsonValue::String(value.to_string())
}

/// Konvertiert CSV mit Dot-Notation zu JSON (verschachtelt)
/// Beispiel: "contact.email" wird zu {"contact": {"email": "..."}}
pub fn convert_csv_to_json_nested(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. CSV-Datei lesen und parsen (mit Dot-Notation Support)
    let records = read_csv_to_json_nested(input_path)?;
    
    // 2. Vec → JSON Array
    let json_value = JsonValue::Array(records);
    
    // 3. JSON Value → Pretty-Printed String
    let json_string = serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;
    
    // 4. String in Datei schreiben
    fs::write(output_path, json_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Hilfsfunktion: Liest CSV und erstellt flache JSON-Objekte (KEINE Dot-Notation)
/// 
/// Beispiel:
/// CSV: name,contact.email,contact.phone
/// JSON: {"name": "...", "contact.email": "...", "contact.phone": "..."}
fn read_csv_to_json_flat(input_path: &str) -> Result<Vec<JsonValue>, FormatError> {
    // 1. CSV-Reader erstellen
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von CSV {}: {}", input_path, e)))?;
    
    // 2. Header lesen
    let headers = reader.headers()
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen der CSV-Header: {}", e)))?
        .clone();
    
    let header_vec: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    
    // 3. Records lesen und in flache JSON-Objekte konvertieren
    let mut records = Vec::new();
    
    for result in reader.records() {
        let record = result
            .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen eines CSV-Records: {}", e)))?;
        
        // Jedes Record wird ein flaches JSON-Objekt (KEINE Verschachtelung)
        let mut obj = serde_json::Map::new();
        
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = header_vec.get(i) {
                // Typ-Inferenz
                let value = infer_type(field);
                
                // Direkt als flacher Key einfügen (keine Dot-Notation-Verarbeitung)
                obj.insert(header.clone(), value);
            }
        }
        
        records.push(JsonValue::Object(obj));
    }
    
    Ok(records)
}

/// Hilfsfunktion: Liest CSV mit Dot-Notation und erstellt verschachtelte Strukturen
/// 
/// Diese Funktion wird für die separate `convert_csv_to_json_nested()` Funktion verwendet.
/// Beispiel:
/// CSV: name,contact.email,contact.phone
/// JSON: {"name": "...", "contact": {"email": "...", "phone": "..."}}
fn read_csv_to_json_nested(input_path: &str) -> Result<Vec<JsonValue>, FormatError> {
    // 1. CSV-Reader erstellen
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von CSV {}: {}", input_path, e)))?;
    
    // 2. Header lesen
    let headers = reader.headers()
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen der CSV-Header: {}", e)))?
        .clone();
    
    let header_vec: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    
    // 3. Records lesen und in verschachtelte JSON-Objekte konvertieren
    let mut records = Vec::new();
    
    for result in reader.records() {
        let record = result
            .map_err(|e| FormatError::ParseError(format!("Fehler beim Lesen eines CSV-Records: {}", e)))?;
        
        // Jedes Record wird ein JSON-Objekt (möglicherweise verschachtelt)
        let mut root = serde_json::Map::new();
        
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = header_vec.get(i) {
                // Typ-Inferenz wie gewohnt
                let value = infer_type(field);
                
                // Verschachtelung durch Dot-Notation erstellen
                insert_nested_value(&mut root, header, value);
            }
        }
        
        records.push(JsonValue::Object(root));
    }
    
    Ok(records)
}

/// Hilfsfunktion: Fügt einen Wert in ein verschachteltes Objekt ein
/// 
/// Beispiel: insert_nested_value(obj, "contact.email", "test@test.com")
/// Erstellt: {"contact": {"email": "test@test.com"}}
fn insert_nested_value(
    obj: &mut serde_json::Map<String, JsonValue>,
    key: &str,
    value: JsonValue
) {
    let parts: Vec<&str> = key.split('.').collect();
    
    if parts.len() == 1 {
        // Einfacher Key ohne Verschachtelung
        obj.insert(key.to_string(), value);
    } else {
        // Verschachtelter Key: z.B. "contact.email"
        let first = parts[0];
        let rest = parts[1..].join(".");
        
        // Hole oder erstelle das verschachtelte Objekt
        let nested = obj.entry(first.to_string())
            .or_insert_with(|| JsonValue::Object(serde_json::Map::new()));
        
        // Stelle sicher, dass es ein Objekt ist
        if let JsonValue::Object(nested_map) = nested {
            // Rekursiv für tiefere Verschachtelung
            insert_nested_value(nested_map, &rest, value);
        }
    }
}

