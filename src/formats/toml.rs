// TOML zu anderen Formaten konvertieren

use std::fs;
use crate::error::FormatError;

/// Konvertiert TOML zu JSON
/// Pipeline: TOML → toml::Value (IR) → serde_json::Value → JSON String
pub fn convert_toml_to_json(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. TOML-Datei validieren und parsen
    let toml_value = validate_toml(input_path)?;
    
    // 2. toml::Value → serde_json::Value (IR)
    let json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;

    // 3. JSON Value → Pretty-Printed String
    let json_string = serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von JSON: {}", e)))?;
    
    // 4. String in Datei schreiben
    fs::write(output_path, json_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert TOML zu YAML
/// Pipeline: TOML → toml::Value (IR) → serde_yaml::Value → YAML String
pub fn convert_toml_to_yaml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. TOML-Datei validieren und parsen
    let toml_value = validate_toml(input_path)?;
    
    // 2. toml::Value → serde_json::Value (als Zwischenschritt, da YAML und JSON ähnlich sind)
    let json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
    // 3. JSON Value → YAML String
    let yaml_string = serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von YAML: {}", e)))?;
    
    // 4. String in Datei schreiben
    fs::write(output_path, yaml_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert TOML zu TOML (Pretty-Printing / Formatierung)
/// Pipeline: TOML → toml::Value (IR) → TOML String (neu formatiert)
pub fn convert_toml_to_toml(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. TOML-Datei validieren und parsen
    let toml_value = validate_toml(input_path)?;
    
    // 2. toml::Value → Pretty-Printed TOML String
    let toml_string = toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von TOML: {}", e)))?;
    
    // 3. String in Datei schreiben
    fs::write(output_path, toml_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}

/// Konvertiert TOML zu CSV
/// Verschachtelte Strukturen werden geflattened mit Unterstrich (z.B. contact_email)
/// Pipeline: TOML → toml::Value (IR) → serde_json::Value → Flattened → CSV
pub fn convert_toml_to_csv(
    input_path: &str,
    output_path: &str
) -> Result<(), FormatError> {
    // 1. TOML-Datei validieren und parsen
    let toml_value = validate_toml(input_path)?;
    
    // 2. toml::Value → serde_json::Value (CSV-Crate arbeitet besser mit JSON)
    let mut json_value = serde_json::to_value(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Konvertieren: {}", e)))?;
    
    // 3. Wenn es ein Objekt mit "data" Key ist (von CSV → TOML), extrahiere das Array
    if let serde_json::Value::Object(ref obj) = json_value {
        if let Some(data_value) = obj.get("data") {
            json_value = data_value.clone();
        }
    }
    
    // 4. Prüfen ob es ein Array ist (CSV braucht Array von Objekten)
    let array = match json_value {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(_) => {
            // Wenn es ein Objekt ist, packen wir es in ein Array
            vec![json_value]
        },
        _ => {
            return Err(FormatError::SerializationError(
                "TOML muss ein Array oder Objekt für CSV-Konvertierung sein".to_string()
            ));
        }
    };
    
    // 5. Alle Objekte flattenen
    let flattened: Vec<_> = array.iter()
        .map(|v| flatten_json_value(v, ""))
        .collect();

    // 6. Header aus allen flattened Objekten sammeln
    let mut all_headers = std::collections::BTreeSet::new();
    for obj in &flattened {
        for key in obj.keys() {
            all_headers.insert(key.clone());
        }
    }
    let headers: Vec<String> = all_headers.into_iter().collect();

    // 7. CSV-Writer erstellen
    let mut writer = csv::Writer::from_path(output_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Erstellen von CSV: {}", e)))?;
    
    // 8. Header schreiben
    writer.write_record(&headers)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Header: {}", e)))?;
    
    // 9. Daten schreiben
    for flat_obj in flattened {
        let row: Vec<String> = headers.iter()
            .map(|h| flat_obj.get(h).cloned().unwrap_or_default())
            .collect();
        writer.write_record(&row)
            .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der CSV-Daten: {}", e)))?;
    }
    
    // 10. Writer flushen (sicherstellen dass alles geschrieben wurde)
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

/// Validiert eine TOML-Datei ohne sie zu schreiben
/// Ähnlich wie validate_json, aber für TOML
/// 
/// Diese Funktion ist zentral für alle TOML-Konvertierungen:
/// Sie liest die Datei, parst sie und gibt ein toml::Value zurück
pub fn validate_toml(input_path: &str) -> Result<toml::Value, FormatError> {
    // 1. Datei lesen - content enthält den TOML-Text als String
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;

    // 2. TOML parsen und validieren
    // toml::from_str wandelt den String in eine toml::Value Struktur um
    // Dies ist die "Intermediate Representation" (IR)
    let toml_value: toml::Value = toml::from_str(&content)
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Parsen von TOML: {}", e)))?;

    // 3. Validiertes toml::Value zurückgeben
    Ok(toml_value)
}