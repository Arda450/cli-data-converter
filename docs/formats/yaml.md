# YAML (YAML Ain't Markup Language)

YAML ist ein menschenfreundliches Datenformat, das häufig für Konfigurationsdateien verwendet wird.

## Überblick

**Entwickelt von:** Clark Evans, Ingy döt Net, Oren Ben-Kiki (2001)  
**Dateiendung:** `.yaml` oder `.yml`  
**MIME-Type:** `application/x-yaml`  
**Website:** [yaml.org](https://yaml.org/)

## Eigenschaften

### ✅ Vorteile

- **Sehr lesbar**: Minimale Syntax, natürliche Struktur
- **Kommentare**: Unterstützt `#` für Inline-Kommentare
- **Flexibel**: Mehrere Wege, Daten zu strukturieren
- **Kompakt**: Keine geschweiften Klammern oder Kommas nötig
- **Mehrere Dokumente**: Ein File kann mehrere YAML-Dokumente enthalten

### ❌ Nachteile

- **Indentierung-sensitiv**: Leerzeichen und Tabs können Probleme verursachen
- **Langsames Parsing**: Komplexer zu parsen als JSON
- **Mehrdeutig**: Verschiedene Wege, dasselbe auszudrücken
- **Anfällig für Fehler**: Kleine Indentierungs-Fehler brechen das gesamte File

## Syntax

### Grundlegende Datentypen

```yaml
# Strings
name: ASP CLI
description: "Mit Anführungszeichen"
multiline: |
  Dies ist ein
  mehrzeiliger String

# Numbers
port: 8080
version: 1.0

# Booleans
enabled: true
debug: false

# Null
value: null
empty: ~

# Arrays
tags:
  - rust
  - cli
  - converter

# Inline Array
colors: [red, green, blue]
```

### Objekte und Verschachtelung

```yaml
app:
  name: ASP CLI
  version: 1.0.0
  author:
    name: Arda Karadavut
    email: arda@example.com
  dependencies:
    - serde
    - clap
    - toml
```

### Kommentare

```yaml
# Dies ist ein Kommentar
name: value # Inline-Kommentar

# Mehrzeilige Kommentare
# müssen mit # pro Zeile
# geschrieben werden
```

### Mehrzeilige Strings

```yaml
# Literal Style (behält Zeilenumbrüche)
description: |
  Dies ist Zeile 1
  Dies ist Zeile 2
  Alle Zeilenumbrüche bleiben erhalten

# Folded Style (faltet Zeilenumbrüche)
text: >
  Dies ist ein langer Text
  der über mehrere Zeilen geht
  aber als eine Zeile behandelt wird
```

### Anker und Aliase (Referenzen)

```yaml
# Definition eines Ankers
defaults: &defaults
  timeout: 30
  retries: 3

# Verwendung des Ankers
development:
  <<: *defaults
  host: localhost

production:
  <<: *defaults
  host: prod.example.com
```

## Verwendungszwecke

### 1. CI/CD Konfiguration

YAML ist der Standard für CI/CD-Pipelines:

```yaml
# GitHub Actions
name: CI
on:
  push:
    branches: [main]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run tests
        run: cargo test
```

**Beispiele:**

- GitHub Actions (`.github/workflows/*.yml`)
- GitLab CI (`.gitlab-ci.yml`)
- CircleCI (`.circleci/config.yml`)

### 2. Container-Orchestrierung

```yaml
# Docker Compose
version: "3.8"
services:
  web:
    image: nginx:latest
    ports:
      - "80:80"
    environment:
      - DEBUG=true
```

**Beispiele:**

- Docker Compose (`docker-compose.yml`)
- Kubernetes (`deployment.yml`, `service.yml`)

### 3. Applikations-Konfiguration

```yaml
# Applikations-Config
server:
  host: 0.0.0.0
  port: 8080
  ssl:
    enabled: true
    cert: /path/to/cert.pem

database:
  url: postgresql://localhost/mydb
  pool_size: 10
```

## Konvertierung mit ASP CLI

### YAML zu anderen Formaten

```bash
# YAML zu JSON
asp_cli convert config.yaml --to json -o config.json

# YAML zu TOML
asp_cli convert docker-compose.yml --to toml -o config.toml

# YAML zu CSV (nur flache Arrays)
asp_cli convert data.yaml --to csv -o data.csv
```

### Andere Formate zu YAML

```bash
# JSON zu YAML
asp_cli convert data.json --to yaml -o data.yaml

# TOML zu YAML
asp_cli convert Cargo.toml --to yaml -o cargo.yaml

# CSV zu YAML
asp_cli convert users.csv --to yaml -o users.yaml
```

## Best Practices

::: tip Konsistente Indentierung
Verwenden Sie immer 2 Leerzeichen für Indentierung:

```yaml
# ✅ Gut: 2 Leerzeichen
server:
  host: localhost
  port: 8080

# ❌ Schlecht: Gemischt
server:
    host: localhost
  port: 8080
```

:::

::: warning Keine Tabs verwenden
YAML erlaubt keine Tabs für Indentierung:

```yaml
# ❌ Fehler: Tab statt Leerzeichen
server:
	host: localhost  # <-- Dies ist ein Tab!
```

:::

::: tip Anführungszeichen bei Sonderzeichen
Verwenden Sie Anführungszeichen bei speziellen Werten:

```yaml
# Ohne Anführungszeichen wird "yes" zu boolean true
answer: "yes"
version: "1.0" # String, nicht Number
```

:::

## Häufige Fehler

### Indentierungs-Fehler

```yaml
# ❌ Fehler: Inkonsistente Indentierung
server:
  host: localhost
   port: 8080  # <-- 3 Leerzeichen statt 2

# ❌ Fehler: Fehlende Indentierung
server:
host: localhost  # <-- Muss eingerückt sein
```

### Boolean-Fallen

```yaml
# Diese Werte werden als Booleans interpretiert:
value1: yes # true
value2: no # false
value3: on # true
value4: off # false

# Lösung: Anführungszeichen verwenden
value5: "yes" # String "yes"
```

### Mehrdeutige Syntax

```yaml
# ❌ Mehrdeutig
description: This is a sentence: with colons

# ✅ Besser
description: "This is a sentence: with colons"

# Oder literal style
description: |
  This is a sentence: with colons
```

## YAML in Rust (Serde)

ASP CLI verwendet die `serde_yaml` Library:

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
let config: Config = serde_yaml::from_str(yaml_string)?;

// Serialisieren
let yaml = serde_yaml::to_string(&config)?;
```

## YAML Versionen

Es gibt drei YAML-Versionen:

- **YAML 1.0** (2004) - Erste Version
- **YAML 1.1** (2005) - Am weitesten verbreitet
- **YAML 1.2** (2009) - Kompatibel mit JSON

::: info ASP CLI unterstützt YAML 1.2
Dies bedeutet, dass jedes gültige JSON auch gültiges YAML ist.
:::

## Vergleich mit anderen Formaten

| Feature            | YAML       | JSON       | TOML     |
| ------------------ | ---------- | ---------- | -------- |
| Lesbarkeit         | ⭐⭐⭐⭐⭐ | ⭐⭐⭐     | ⭐⭐⭐⭐ |
| Parsing-Speed      | ⭐⭐⭐     | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| Fehleranfälligkeit | ⭐⭐       | ⭐⭐⭐⭐   | ⭐⭐⭐   |
| Kommentare         | ✅         | ❌         | ✅       |
| Referenzen         | ✅         | ❌         | ❌       |

## Ressourcen

- [YAML Official Website](https://yaml.org/)
- [YAML Specification](https://yaml.org/spec/)
- [serde_yaml Documentation](https://docs.rs/serde_yaml/)
- [YAML Lint - Online Validator](https://www.yamllint.com/)

## Nächste Schritte

- [TOML Format →](/formats/toml)
- [Formatvergleich →](/formats/comparison)
- [Beispiele →](/guide/examples)
