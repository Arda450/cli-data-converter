# Architektur-Überblick

Eine technische Übersicht über die Architektur von ASP CLI.

## Projektstruktur

```
asp_cli/
├── src/
│   ├── main.rs          # Entry Point, CLI-Parsing
│   ├── lib.rs           # Library Root
│   ├── error.rs         # Fehlerbehandlung
│   ├── detect.rs        # Format-Erkennung
│   └── formats/
│       ├── mod.rs       # Modul-Deklarationen
│       ├── json.rs      # JSON-Konvertierung
│       ├── yaml.rs      # YAML-Konvertierung
│       ├── toml.rs      # TOML-Konvertierung
│       └── csv.rs       # CSV-Konvertierung
├── Cargo.toml           # Rust-Dependencies
└── docs/                # VitePress-Dokumentation
```

## Architektur-Diagramm

```
┌─────────────┐
│   main.rs   │  ← Entry Point
└──────┬──────┘
       │
       ├─→ clap::Parser  ← CLI-Argumente parsen
       │
       ├─→ detect.rs     ← Format erkennen
       │
       └─→ formats/      ← Konvertierung durchführen
           │
           ├─→ json.rs
           ├─→ yaml.rs
           ├─→ toml.rs
           └─→ csv.rs
```

## Komponenten

### 1. CLI-Interface (`main.rs`)

**Verantwortlichkeiten:**

- Kommandozeilen-Argumente parsen mit `clap`
- Konvertierungsfunktion basierend auf Dateierweiterung auswählen
- Fehlerbehandlung und Ausgabe

**Implementierung:**

```rust
#[derive(Parser)]
struct Cli {
    input: String,   // Eingabedatei
    output: String,  // Ausgabedatei
}

fn main() {
    let cli = Cli::parse();
    match convert_based_on_extension(&cli.input, &cli.output) {
        Ok(_) => println!("✓ Konvertierung erfolgreich"),
        Err(e) => eprintln!("✗ Fehler: {}", e),
    }
}
```

### 2. Format-Erkennung (`detect.rs`)

**Verantwortlichkeiten:**

- Eingabeformat automatisch erkennen
- Dateierweiterungen validieren
- Format-Metadaten bereitstellen

**Ablauf:**

```
Dateiname → Extension extrahieren → Format bestimmen
  "data.json"  → ".json"           → JSON
```

### 3. Konvertierungs-Module (`formats/`)

Jedes Format hat sein eigenes Modul mit Konvertierungsfunktionen.

#### JSON-Modul (`json.rs`)

- `convert_json_to_json()` - Pretty-Printing
- `convert_json_to_yaml()`
- `convert_json_to_toml()`
- `convert_json_to_csv()`

#### YAML-Modul (`yaml.rs`)

- `convert_yaml_to_json()`
- `convert_yaml_to_yaml()` - Pretty-Printing
- `convert_yaml_to_toml()`
- `convert_yaml_to_csv()`

#### TOML-Modul (`toml.rs`)

- `convert_toml_to_json()`
- `convert_toml_to_yaml()`
- `convert_toml_to_toml()` - Pretty-Printing
- `convert_toml_to_csv()`

#### CSV-Modul (`csv.rs`)

- `convert_csv_to_json()`
- `convert_csv_to_yaml()`
- `convert_csv_to_toml()`
- `convert_csv_to_csv()` - Pretty-Printing

### 4. Fehlerbehandlung (`error.rs`)

**Verantwortlichkeiten:**

- Zentrale Fehlertypen definieren
- Fehler-Konvertierung zwischen Libraries
- Benutzerfreundliche Fehlermeldungen

**Fehlertypen:**

```rust
pub enum FormatError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error),
    TomlError(toml::de::Error),
    CsvError(csv::Error),
}
```

## Datenfluss

### Konvertierungs-Pipeline

```
1. Eingabe
   ↓
2. Format-Erkennung
   ↓
3. Datei einlesen
   ↓
4. Parse (String → Datenstruktur)
   ↓
5. Serialize (Datenstruktur → String)
   ↓
6. Datei schreiben
   ↓
7. Erfolg/Fehler zurückmelden
```

### Beispiel: JSON zu YAML

```rust
// 1. Datei einlesen
let content = std::fs::read_to_string(input)?;

// 2. JSON parsen
let value: serde_json::Value = serde_json::from_str(&content)?;

// 3. YAML serialisieren
let yaml = serde_yaml::to_string(&value)?;

// 4. Datei schreiben
std::fs::write(output, yaml)?;
```

## Verwendete Technologien

### Rust Ecosystem

| Crate        | Version | Verwendung                      |
| ------------ | ------- | ------------------------------- |
| `clap`       | 4.5     | CLI-Argument-Parsing            |
| `serde`      | 1.0     | Serialisierung/Deserialisierung |
| `serde_json` | 1.0     | JSON-Parsing                    |
| `serde_yaml` | 0.9     | YAML-Parsing                    |
| `toml`       | 0.8     | TOML-Parsing                    |
| `csv`        | 1.3     | CSV-Parsing                     |

### Warum diese Crates?

**clap:**

- Industry-Standard für Rust CLI-Apps
- Derive-Macros für einfache Definitionen
- Automatische Help-Messages

**serde:**

- Universeller Serialisierungs-Standard in Rust
- Format-agnostische Datenstrukturen
- Zero-Cost Abstractions

**serde_json / serde_yaml / toml:**

- Nahtlose Integration mit `serde`
- Hohe Performance
- Gute Fehlerbehandlung

**csv:**

- RFC 4180 konform
- Streaming-Support
- Serde-Integration

## Design-Prinzipien

### 1. Modularität

Jedes Format ist ein separates Modul:

- Einfache Wartung
- Klare Verantwortlichkeiten
- Testbarkeit

### 2. Fehlerbehandlung

Alle Fehler werden zentral behandelt:

- Keine `panic!()` im Produktionscode
- `Result<T, E>` für alle I/O-Operationen
- Benutzerfreundliche Fehlermeldungen

### 3. Typsicherheit

Rust's Type-System verhindert viele Fehler:

- Keine null-Pointer
- Keine Buffer-Overflows
- Ownership-System garantiert Memory-Safety

### 4. Performance

Effiziente Implementierung:

- Streaming wo möglich
- Minimale Allokationen
- Zero-Copy-Optimierungen

## Erweiterbarkeit

### Neues Format hinzufügen

Das modulare Design ermöglicht einfaches Hinzufügen neuer Formate in vier Schritten:

1. **Neues Modul erstellen:**

```rust
// src/formats/newformat.rs
use crate::error::FormatError;

pub fn convert_newformat_to_json(input: &str, output: &str) -> Result<(), FormatError> {
    // Implementierung
}
```

2. **Modul registrieren:**

```rust
// src/formats/mod.rs
pub mod newformat;
```

3. **Format-Erkennung erweitern:**

```rust
// src/detect.rs
match extension {
    "newformat" => Format::NewFormat,
    // ...
}
```

4. **CLI erweitern:**

```rust
// src/main.rs
match output_ext {
    "newformat" => convert_to_newformat(input, output),
    // ...
}
```

## Testing-Strategie

### Unit-Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_yaml() {
        let result = convert_json_to_yaml("test.json", "test.yaml");
        assert!(result.is_ok());
    }
}
```

### Integration-Tests

```bash
# Erstelle Test-Dateien
echo '{"name": "test"}' > test.json

# Führe Konvertierung aus
cargo run -- convert test.json test.yaml

# Validiere Ausgabe
diff expected.yaml test.yaml
```

## Performance-Optimierungen

### 1. Lazy Loading

Dateien werden nur bei Bedarf gelesen.

### 2. Streaming für CSV

```rust
let mut reader = csv::Reader::from_path(input)?;
for result in reader.deserialize() {
    // Prozessiere Zeile für Zeile
}
```

### 3. Release-Build mit Optimierungen

```toml
[profile.release]
opt-level = 3
lto = true
```

## Sicherheit

### 1. Input-Validierung

Alle Eingabedateien werden vor der Verarbeitung validiert.

### 2. Memory-Safety

Rust garantiert Memory-Safety zur Compile-Zeit.

### 3. Error-Handling

Keine unbehandelten Fehler im Produktionscode.

## Deployment

### Binary-Größe

```bash
# Release-Build
cargo build --release

# Größe prüfen
ls -lh target/release/asp_cli
# ~5-10 MB (abhängig von Dependencies)
```

### Cross-Compilation

```bash
# Für Windows
cargo build --target x86_64-pc-windows-gnu

# Für Linux
cargo build --target x86_64-unknown-linux-gnu

# Für macOS
cargo build --target x86_64-apple-darwin
```

## Nächste Schritte

- [Projektstruktur im Detail →](/architecture/structure)
- [Design-Entscheidungen →](/architecture/decisions)
- [Verwendete Libraries →](/architecture/libraries)
