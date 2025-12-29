// CLI-Interface hier implementieren



use asp_cli::formats::json::{convert_json_to_json, convert_json_to_toml, convert_json_to_yaml, convert_json_to_csv};
use asp_cli::formats::toml::{convert_toml_to_json, convert_toml_to_yaml, convert_toml_to_toml, convert_toml_to_csv};
use asp_cli::formats::yaml::{convert_yaml_to_json, convert_yaml_to_yaml, convert_yaml_to_toml, convert_yaml_to_csv};
use asp_cli::formats::csv::{convert_csv_to_json, convert_csv_to_yaml, convert_csv_to_toml, convert_csv_to_csv};
use asp_cli::error::FormatError;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
#[command(name = "asp")]
#[command(about = "Konvertiert zwischen verschiedenen Datenformaten (JSON, YAML, TOML, CSV)")]
#[command(version = "0.1.0")]
struct Cli {
    /// Eingabedatei (unterstützt: .json, .yaml, .yml, .toml, .csv)
    input: String,
    
    /// Ausgabedatei (unterstützt: .json, .yaml, .yml, .toml, .csv)
    output: String,
}

/// Entscheidet basierend auf Dateierweiterung, welche Konvertierungsfunktion aufgerufen werden soll
fn convert_based_on_extension(
    input: &str,
    output: &str,
) -> Result<(), FormatError> {
    // Prüft BEIDE Dateierweiterungen: Input UND Output
    let input_ext = Path::new(input)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("json");
    
    let output_ext = Path::new(output)
        .extension() // gibt die dateierweiterung der datei zurück, also z.B. "json" oder "toml"
        .and_then(|ext| ext.to_str()) // konvertiert die extension zu einem string, weil rust-os funktionen nicht utf-8 sein können
        .unwrap_or("json");
    
    // Nested match: Erst Eingabeformat prüfen, dann Ausgabeformat
    match input_ext.to_lowercase().as_str() {
        // JSON als Eingabeformat
        "json" => match output_ext.to_lowercase().as_str() {
            "json" => convert_json_to_json(input, output),
            "toml" => convert_json_to_toml(input, output),
            "yaml" | "yml" => convert_json_to_yaml(input, output),
            "csv" => convert_json_to_csv(input, output),
        _ => {
            // Fallback: Standard ist JSON
            convert_json_to_json(input, output)
        }
        },
        
        // TOML als Eingabeformat
        "toml" => match output_ext.to_lowercase().as_str() {
            "json" => convert_toml_to_json(input, output),
            "yaml" | "yml" => convert_toml_to_yaml(input, output),
            "toml" => convert_toml_to_toml(input, output),
            "csv" => convert_toml_to_csv(input, output),
            _ => {
                // Fallback: Konvertiere zu JSON
                convert_toml_to_json(input, output)
            }
        },
        
        // YAML als Eingabeformat
        "yaml" | "yml" => match output_ext.to_lowercase().as_str() {
            "json" => convert_yaml_to_json(input, output),
            "yaml" | "yml" => convert_yaml_to_yaml(input, output),
            "toml" => convert_yaml_to_toml(input, output),
            "csv" => convert_yaml_to_csv(input, output),
            _ => {
                // Fallback: Konvertiere zu JSON
                convert_yaml_to_json(input, output)
            }
        },
        
        // CSV als Eingabeformat
        "csv" => match output_ext.to_lowercase().as_str() {
            "json" => convert_csv_to_json(input, output),
            "yaml" | "yml" => convert_csv_to_yaml(input, output),
            "toml" => convert_csv_to_toml(input, output),
            "csv" => convert_csv_to_csv(input, output),
            _ => {
                // Fallback: Konvertiere zu JSON
                convert_csv_to_json(input, output)
            }
        },
        
        // Fallback für unbekannte Eingabeformate
        _ => Err(FormatError::SerializationError(format!(
            "Ungültiges Eingabeformat: .{}. Unterstützte Formate: json, yaml, yml, toml, csv",
            input_ext
        ))),
    }
}

 // hier fängts an
fn main() {
    // clap parst automatisch die Argumente
    let cli = Cli::parse();
    
    match convert_based_on_extension(&cli.input, &cli.output) {
        Ok(_) => println!("✓ Konvertierung erfolgreich: {} -> {}", cli.input, cli.output),
        Err(e) => {
            eprintln!("✗ Fehler: {}", e);
            std::process::exit(1);
        }
    }
}
