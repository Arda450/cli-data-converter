// YAML zu anderen Formaten konvertieren

use std::fs;
use crate::error::FormatError;

/// Konvertiert YAML zu JSON
/// Pipeline: YAML → serde_yaml::Value (IR) → serde_json::Value → JSON String
pub fn convert_yaml_to_json(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. YAML-Datei validieren und parsen
    let yaml_value = validate_yaml(input_path)?;
    
    // 2. serde_yaml::Value → serde_json::Value (IR)
    // YAML und JSON sind strukturell sehr ähnlich
    let json_value = serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;

    // 3. JSON Value → Pretty-Printed String
    let json_string = serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;
    
    // 4. String in Datei schreiben
    fs::write(output_path, json_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert YAML zu YAML (Pretty-Printing / Formatierung)
/// Pipeline: YAML → serde_yaml::Value (IR) → YAML String (neu formatiert)
pub fn convert_yaml_to_yaml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. YAML-Datei validieren und parsen
    let yaml_value = validate_yaml(input_path)?;
    
    // 2. serde_yaml::Value → YAML String
    let yaml_string = serde_yaml::to_string(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von YAML: {}", e)))?;
    
    // 3. String in Datei schreiben
    fs::write(output_path, yaml_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert YAML zu TOML
/// Pipeline: YAML → serde_yaml::Value → serde_json::Value (IR) → toml::Value → TOML String
pub fn convert_yaml_to_toml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. YAML-Datei validieren und parsen
    let yaml_value = validate_yaml(input_path)?;
    
    // 2. serde_yaml::Value → serde_json::Value (Zwischenschritt)
    let json_value = serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
    // 3. Prüfen ob TOML-kompatibel (TOML braucht ein Object als Root, kein Array)
    if json_value.is_array() {
        return Err(FormatError::SerializationError(
            "TOML unterstützt keine Arrays als Root-Element. YAML muss ein Objekt enthalten.".to_string()
        ));
    }
    
    // 4. JSON Value → TOML String über Serde
    // Wir gehen den Weg: JSON Value → JSON String → TOML Value → TOML String
    let json_string = serde_json::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;
    
    let toml_value: toml::Value = toml::from_str(&json_string)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren zu TOML: {}", e)))?;
    
    // 5. TOML Value → Pretty-Printed String
    let toml_string = toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von TOML: {}", e)))?;
    
    // 6. String in Datei schreiben
    fs::write(output_path, toml_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert YAML zu CSV
/// Verschachtelte Strukturen werden geflattened mit Unterstrich (z.B. contact_email)
/// Pipeline: YAML → serde_yaml::Value → serde_json::Value (IR) → Flattened → CSV
pub fn convert_yaml_to_csv(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. YAML-Datei validieren und parsen
    let yaml_value = validate_yaml(input_path)?;
    
    // 2. serde_yaml::Value → serde_json::Value (CSV-Crate arbeitet besser mit JSON)
    let json_value = serde_json::to_value(&yaml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
    // 3. Prüfen ob es ein Array ist (CSV braucht Array von Objekten)
    let array = match json_value {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => {
            // Wenn es ein Objekt ist, packen wir es in ein Array
            vec![json_value]
        },
        _ => {
            return Err(FormatError::SerializationError(
                "YAML muss ein Array oder Objekt für CSV-Konvertierung sein".to_string()
            ));
        }
    };
    
    // 4. Alle Objekte flattenen
    let flattened: Vec<_> = array.iter()
        .map(|v| flatten_json_value(v, ""))
        .collect();

    // 5. Header aus allen flattened Objekten sammeln
    let mut all_headers = std::collections::BTreeSet::new();
    for obj in &flattened {
        for key in obj.keys() {
            all_headers.insert(key.clone());
        }
    }
    let headers: Vec<String> = all_headers.into_iter().collect();

    // 6. CSV-Writer erstellen
    let mut writer = csv::Writer::from_path(output_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Erstellen von CSV: {}", e)))?;
    
    // 7. Header schreiben
    writer.write_record(&headers)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Header: {}", e)))?;
    
    // 8. Daten schreiben
    for flat_obj in flattened {
        let row: Vec<String> = headers.iter()
            .map(|h| flat_obj.get(h).cloned().unwrap_or_default())
            .collect();
        writer.write_record(&row)
            .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Daten: {}", e)))?;
    }
    
    // 9. Writer flushen (sicherstellen dass alles geschrieben wurde)
    writer.flush()
        .map_err(|e| FormatError::IoError(format!("Fehler beim Abschliessen von CSV: {}", e)))?;

    Ok(())
}

/// Flattened ein JSON-Objekt zu einer flachen Map mit Unterstrich-Trennzeichen
/// 
/// Beispiel:
/// Input:  {"contact": {"email": "test@test.com", "phone": "+49"}}
/// Output: {"contact_email": "test@test.com", "contact_phone": "+49"}
fn flatten_json_value(value: &serde_json::Value, prefix: &str) -> std::collections::HashMap<String, String> {
    use std::collections::HashMap;
    
    let mut result = HashMap::new();
    
    match value {
        serde_json::Value::Object(obj) => {
            for (key, val) in obj {
                let new_key = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}_{}", prefix, key)
                };
                
                // Rekursiv für verschachtelte Objekte
                if val.is_object() {
                    let nested = flatten_json_value(val, &new_key);
                    result.extend(nested);
                } else {
                    result.insert(new_key, json_value_to_string(val));
                }
            }
        }
        _ => {
            // Primitive Werte direkt einfügen
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), json_value_to_string(value));
            }
        }
    }
    
    result
}

/// Konvertiert einen JSON-Wert zu einem String
fn json_value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}

/// Validiert eine YAML-Datei ohne sie zu schreiben
/// Ähnlich wie validate_json und validate_toml, aber für YAML
/// 
/// Diese Funktion ist zentral für alle YAML-Konvertierungen:
/// Sie liest die Datei, parst sie und gibt ein serde_yaml::Value zurück
pub fn validate_yaml(input_path: &str) -> Result<serde_yaml::Value, FormatError> {
    // 1. Datei lesen - content enthält den YAML-Text als String
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;

    // 2. YAML parsen und validieren
    // serde_yaml::from_str wandelt den String in eine serde_yaml::Value Struktur um
    // Dies ist die "Intermediate Representation" (IR)
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Parsen von YAML: {}", e)))?;

    // 3. Validiertes serde_yaml::Value zurückgeben
    Ok(yaml_value)
}