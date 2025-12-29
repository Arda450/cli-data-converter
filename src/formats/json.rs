// json to other formats



// use serde::{Deserialize, Serialize};
use std::fs;
use crate::error::FormatError;
use crate::formats::utils::json_to_toml_value;


// Konvertiert JSON zu formatiertem JSON
pub fn convert_json_to_json(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // Zuerst validieren
    let json_value = validate_json(input_path)?;

    // JSON formatiert ausgeben (pretty print mit 2 Leerzeichen Einrückung)
    let formatted_json = serde_json::to_string_pretty(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von JSON: {}", e)))?;

    // In Ausgabedatei schreiben
    fs::write(output_path, formatted_json)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}


// Konvertiert JSON zu TOML
// ruft mehrere helper funktionen auf
pub fn convert_json_to_toml(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // Zuerst input json datei validieren
    let json_value = validate_json(input_path)?;

    // TOML unterstützt kein Array als Root-Element
    // Wenn es ein Array ist, wrappen wir es in ein Objekt
    let toml_ready_value = match json_value {
        serde_json::Value::Array(arr) => {
            // Array in ein Objekt wrappen mit dem Key "items"
            serde_json::json!({ "items": arr })
        }
        _ => {
            // Für Objekte und andere Typen direkt verwenden
            json_value
        }
    };
    
    // JSON Value → TOML Value strukturell konvertieren
    let toml_value = json_to_toml_value(&toml_ready_value)?;
    
    // TOML Value → Pretty-Printed String
    let toml_string = toml::to_string_pretty(&toml_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Serialisieren von TOML: {}", e)))?;

    // In Ausgabedatei schreiben
    fs::write(output_path, toml_string)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}


/// Konvertiert JSON zu formatiertem YAML
pub fn convert_json_to_yaml(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // Zuerst validieren
    let json_value = validate_json(input_path)?;

    // Zu YAML konvertieren
    let formatted_yaml = serde_yaml::to_string(&json_value)
        .map_err(|e| FormatError::SerializationError(format!("Fehler beim Formatieren von YAML: {}", e)))?;

    // In Ausgabedatei schreiben
    fs::write(output_path, formatted_yaml)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben nach {}: {}", output_path, e)))?;

    Ok(())
}


/// Konvertiert JSON zu CSV
/// csv crate ist eine Rust-Bibliothek für CSV-Verarbeitung und ist bereits in der Cargo.toml hinzugefügt
/// Wichtig: CSV ist flach, JSON kann verschachtelt sein.
/// Diese Funktion behandelt:
/// - Arrays von Objekten → CSV Zeilen
/// - Einzelnes Objekt → Eine CSV Zeile
/// - Verschachtelte Objekte → Werden geflattened mit Unterstrich (z.B. contact_email)
pub fn convert_json_to_csv(
    input_path: &str,
    output_path: &str,
) -> Result<(), FormatError> {
    // Zuerst validieren
    let json_value = validate_json(input_path)?;

    // CSV Writer erstellen
    let mut writer = csv::Writer::from_path(output_path) // erzeugt einen writer, der in die output path schreibt
        .map_err(|e| FormatError::IoError(format!("Fehler beim Erstellen der CSV-Datei: {}", e)))?;

    match json_value { // match entscheidet, welche funktion aufgerufen werden soll, basierend auf dem json_value
       
        // Wenn es ein Array ist
        serde_json::Value::Array(arr) => {
            // wenn array leer, dann leere csv datei
            if arr.is_empty() {
                return Ok(());
            }

            // Alle Objekte flattenen
            let flattened: Vec<_> = arr.iter()
                .map(|v| flatten_json(v, ""))
                .collect();

            // Header aus allen flattened Objekten sammeln
            let mut all_headers = std::collections::BTreeSet::new();
            for obj in &flattened {
                for key in obj.keys() {
                    all_headers.insert(key.clone());
                }
            }
            let headers: Vec<String> = all_headers.into_iter().collect();

            // Header schreiben
            writer.write_record(&headers)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Header: {}", e)))?;

            // Datenzeilen schreiben
            for flat_obj in flattened {
                let row: Vec<String> = headers.iter()
                    .map(|h| flat_obj.get(h).cloned().unwrap_or_default())
                    .collect();
                writer.write_record(&row)
                    .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Zeile: {}", e)))?;
            }
        }

        // Wenn es ein einzelnes Objekt ist
        serde_json::Value::Object(_) => {
            let flattened = flatten_json(&json_value, "");
            
            // Header schreiben
            let headers: Vec<String> = flattened.keys().cloned().collect();
            writer.write_record(&headers)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Header: {}", e)))?;

            // Datenzeile schreiben
            let row: Vec<String> = flattened.values().cloned().collect();
            writer.write_record(&row)
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Zeile: {}", e)))?;
        }
        
        // Andere Typen (String, Number, etc.) → Eine Spalte
        _ => {
            writer.write_record(&["value"])
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Header: {}", e)))?;
            writer.write_record(&[json_value.to_string()])
                .map_err(|e| FormatError::SerializationError(format!("Fehler beim Schreiben der Zeile: {}", e)))?;
        }
    }

    writer.flush()
        .map_err(|e| FormatError::IoError(format!("Fehler beim Schreiben der CSV-Datei: {}", e)))?;

    Ok(())
}



/// Validiert eine JSON-Datei ohne sie zu schreiben
pub fn validate_json(input_path: &str) -> Result<serde_json::Value, FormatError> {
    let content = fs::read_to_string(input_path)
        .map_err(|e| FormatError::IoError(format!("Fehler beim Lesen von {}: {}", input_path, e)))?;

        // serde_json::from_str prüft ob syntax korrekt ist
        // prüft ob syntax korrekt ist
    let json_value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| FormatError::ParseError(format!("Fehler beim Parsen von JSON: {}", e)))?;

    Ok(json_value)
}


// __________________________________________________________________________________________________________________________
// Helper-Funktionen für CSV-Konvertierung

/// Konvertiert einen JSON-Wert zu einem String
fn value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        // Verschachtelte Strukturen als JSON-String
        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
            serde_json::to_string(value).unwrap_or_else(|_| String::new())
        }
    }
}

/// Flattened ein JSON-Objekt zu einer flachen Map mit Unterstrich-Trennzeichen
/// 
/// Beispiel:
/// Input:  {"contact": {"email": "test@test.com", "phone": "+49"}}
/// Output: {"contact_email": "test@test.com", "contact_phone": "+49"}
fn flatten_json(value: &serde_json::Value, prefix: &str) -> std::collections::HashMap<String, String> {
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
                    let nested = flatten_json(val, &new_key);
                    result.extend(nested);
                } else {
                    result.insert(new_key, value_to_string(val));
                }
            }
        }
        _ => {
            // Primitive Werte direkt einfügen
            if !prefix.is_empty() {
                result.insert(prefix.to_string(), value_to_string(value));
            }
        }
    }
    
    result
}