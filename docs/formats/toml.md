# TOML (Tom's Obvious Minimal Language)

TOML ist ein minimalistisches Konfigurationsformat, das darauf ausgelegt ist, eindeutig und leicht zu lesen zu sein.

## Überblick

**Entwickelt von:** Tom Preston-Werner (2013)  
**Dateiendung:** `.toml`  
**MIME-Type:** `application/toml`  
**Website:** [toml.io](https://toml.io/)

## Eigenschaften

### ✅ Vorteile

- **Eindeutig**: Nur eine Art, Daten zu strukturieren
- **Lesbar**: Ähnlich wie INI-Dateien, aber mit mehr Features
- **Typisiert**: Klare Unterscheidung zwischen Datentypen
- **Kommentare**: Unterstützt `#` für Kommentare
- **Fehlerresistent**: Weniger anfällig für Syntax-Fehler als YAML

### ❌ Nachteile

- **Weniger verbreitet**: Nicht so populär wie JSON oder YAML
- **Verbos bei Verschachtelung**: Tief verschachtelte Strukturen werden lang
- **Limitierte Array-Syntax**: Gemischte Arrays sind kompliziert
- **Weniger flexibel**: Strengere Regeln als YAML

## Syntax

### Grundlegende Datentypen

```toml
# Strings
name = "ASP CLI"
description = 'Single quotes auch möglich'

# Multi-line String
long_text = """
Dies ist ein
mehrzeiliger String
"""

# Numbers
integer = 42
float = 3.14
hex = 0xDEADBEEF
octal = 0o755
binary = 0b11010110

# Booleans
enabled = true
debug = false

# Datetime
date = 2025-12-18
datetime = 2025-12-18T14:30:00Z

# Arrays
tags = ["rust", "cli", "converter"]
numbers = [1, 2, 3, 4, 5]

# Multi-line Array
dependencies = [
    "serde",
    "clap",
    "toml",
]
```

### Tables (Objekte)

```toml
# Table Definition
[server]
host = "localhost"
port = 8080
enabled = true

# Verschachtelte Tables
[server.ssl]
enabled = true
cert = "/path/to/cert.pem"
key = "/path/to/key.pem"

# Oder mit Dot-Notation
[database]
primary.host = "db.example.com"
primary.port = 5432
replica.host = "db-replica.example.com"
replica.port = 5432
```

### Array of Tables

```toml
# Array von Objekten mit [[table]]
[[users]]
name = "Alice"
email = "alice@example.com"
role = "admin"

[[users]]
name = "Bob"
email = "bob@example.com"
role = "user"
```

### Kommentare

```toml
# Dies ist ein Kommentar
name = "value" # Inline-Kommentar

# Mehrzeilige Kommentare
# müssen mit # pro Zeile
# geschrieben werden
```

## Verwendungszwecke

### 1. Rust-Projekte (Cargo)

TOML ist der Standard für Rust-Konfiguration:

```toml
# Cargo.toml
[package]
name = "asp_cli"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
toml = "0.8"
csv = "1.3"

[profile.release]
opt-level = 3
lto = true
```

### 2. Applikations-Konfiguration

```toml
# config.toml
[app]
name = "MyApp"
version = "1.0.0"

[app.server]
host = "0.0.0.0"
port = 8080
workers = 4

[app.database]
url = "postgresql://localhost/mydb"
pool_size = 10
timeout = 30
```

### 3. Tool-Konfiguration

Viele moderne Tools nutzen TOML:

```toml
# pyproject.toml (Python)
[project]
name = "my-package"
version = "1.0.0"
dependencies = [
    "requests>=2.28.0",
    "pydantic>=2.0.0",
]

[tool.ruff]
line-length = 88
target-version = "py311"
```

**Beispiele:**

- Rust (`Cargo.toml`)
- Poetry (`pyproject.toml`)
- Hugo (`config.toml`)

## Konvertierung mit ASP CLI

### TOML zu anderen Formaten

```bash
# TOML zu JSON
asp_cli convert Cargo.toml --to json -o cargo.json

# TOML zu YAML
asp_cli convert config.toml --to yaml -o config.yaml

# TOML zu CSV (nur flache Strukturen)
asp_cli convert data.toml --to csv -o data.csv
```

### Andere Formate zu TOML

```bash
# JSON zu TOML
asp_cli convert package.json --to toml -o package.toml

# YAML zu TOML
asp_cli convert docker-compose.yml --to toml -o config.toml

# CSV zu TOML
asp_cli convert users.csv --to toml -o users.toml
```

## Best Practices

::: tip Keys immer zuerst
Definieren Sie Keys vor Subtables:

```toml
# ✅ Gut
[server]
host = "localhost"
port = 8080

[server.ssl]
enabled = true

# ❌ Falsch
[server.ssl]
enabled = true

[server]  # Fehler: server bereits als parent table definiert
host = "localhost"
```

:::

::: tip Inline Tables für kleine Objekte
Verwenden Sie Inline Tables für kompakte Darstellung:

```toml
# Inline Table
server = { host = "localhost", port = 8080 }

# Entspricht:
[server]
host = "localhost"
port = 8080
```

:::

::: warning Quotes bei Sonderzeichen
Keys mit Sonderzeichen müssen in Quotes:

```toml
# ✅ Korrekt
"special-key" = "value"
"127.0.0.1" = "localhost"

# ❌ Fehler ohne Quotes
special-key = "value"
127.0.0.1 = "localhost"
```

:::

## Häufige Fehler

### Table-Reihenfolge

```toml
# ❌ Fehler: Kann [server] nicht neu definieren
[server]
host = "localhost"

[server.ssl]
enabled = true

[server]  # <-- Fehler!
port = 8080
```

### Gemischte Arrays

```toml
# ❌ Fehler: Arrays müssen homogen sein
mixed = [1, "string", true]

# ✅ Korrekt: Arrays of Tables
[[items]]
type = "number"
value = 1

[[items]]
type = "string"
value = "text"
```

### Trailing Commas

```toml
# ❌ Fehler bei inline tables
server = { host = "localhost", port = 8080, }

# ✅ Korrekt: Kein trailing comma
server = { host = "localhost", port = 8080 }

# ✅ Aber OK in Arrays
deps = [
    "serde",
    "clap",
]  # <-- Trailing comma erlaubt
```

## TOML in Rust

ASP CLI verwendet die `toml` Library:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    server: Server,
}

#[derive(Serialize, Deserialize)]
struct Server {
    host: String,
    port: u16,
}

// Deserialisieren
let config: Config = toml::from_str(toml_string)?;

// Serialisieren
let toml_string = toml::to_string_pretty(&config)?;
```

## TOML Versionen

- **TOML 0.5.0** (2018) - Stabile Version
- **TOML 1.0.0** (2021) - Aktuelle Version

::: info ASP CLI unterstützt TOML 1.0.0
Die neueste stabile Version mit allen Features.
:::

## Vergleich mit anderen Formaten

| Feature         | TOML       | JSON       | YAML       |
| --------------- | ---------- | ---------- | ---------- |
| Lesbarkeit      | ⭐⭐⭐⭐   | ⭐⭐⭐     | ⭐⭐⭐⭐⭐ |
| Eindeutigkeit   | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐       |
| Parsing-Speed   | ⭐⭐⭐⭐   | ⭐⭐⭐⭐⭐ | ⭐⭐⭐     |
| Typsystem       | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐   | ⭐⭐⭐⭐   |
| Verschachtelung | ⭐⭐⭐     | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Kommentare      | ✅         | ❌         | ✅         |

## Wann TOML verwenden?

### ✅ Ideal für:

- Rust-Projekte (Cargo.toml)
- Einfache bis mittlere Konfigurationen
- Wenn Eindeutigkeit wichtig ist
- Konfigurationsdateien für Menschen

### ❌ Weniger geeignet für:

- Sehr tief verschachtelte Strukturen
- Datenübertragung zwischen Systemen (→ JSON)
- CI/CD Pipelines (→ YAML)
- Große komplexe Datenstrukturen

## Ressourcen

- [TOML Official Website](https://toml.io/)
- [TOML Specification](https://toml.io/en/v1.0.0)
- [toml-rs Documentation](https://docs.rs/toml/)
- [TOML Lint - Online Validator](https://www.toml-lint.com/)

## Nächste Schritte

- [CSV Format →](/formats/csv)
- [Formatvergleich →](/formats/comparison)
- [Beispiele →](/guide/examples)
