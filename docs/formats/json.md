# JSON (JavaScript Object Notation)

JSON ist ein leichtgewichtiges Datenformat, das häufig für den Datenaustausch zwischen Client und Server verwendet wird.

## Überblick

**Entwickelt von:** Douglas Crockford (2001)  
**Dateiendung:** `.json`  
**MIME-Type:** `application/json`  
**Website:** [json.org](https://www.json.org/)

## Eigenschaften

### ✅ Vorteile

- **Weit verbreitet**: Standard für Web-APIs und moderne Anwendungen
- **Einfach zu parsen**: Native Unterstützung in JavaScript und den meisten Programmiersprachen
- **Kompakt**: Relativ kleine Dateigröße
- **Strukturiert**: Unterstützt verschachtelte Objekte und Arrays
- **Typisiert**: Unterscheidet zwischen Strings, Numbers, Booleans, null

### ❌ Nachteile

- **Nicht für Menschen**: Weniger lesbar als YAML, besonders bei komplexen Strukturen
- **Keine Kommentare**: JSON unterstützt keine Kommentare im Code
- **Strikte Syntax**: Trailing Commas und andere kleine Fehler führen zu Parse-Errors
- **Keine Referenzen**: Daten können nicht referenziert werden (Wiederholung nötig)

## Syntax

### Grundlegende Datentypen

```json
{
  "string": "Text mit \"Escape-Zeichen\"",
  "number": 42,
  "float": 3.14,
  "boolean": true,
  "null": null,
  "array": [1, 2, 3, 4, 5],
  "object": {
    "nested": "value"
  }
}
```

### Objekte (Key-Value)

```json
{
  "name": "ASP CLI",
  "version": "1.0.0",
  "author": {
    "name": "Arda Karadavut",
    "email": "arda@example.com"
  }
}
```

### Arrays

```json
{
  "users": [
    { "id": 1, "name": "Alice" },
    { "id": 2, "name": "Bob" }
  ],
  "tags": ["rust", "cli", "converter"]
}
```

## Verwendungszwecke

### 1. Web-APIs

JSON ist der Standard für RESTful APIs:

```json
{
  "status": "success",
  "data": {
    "id": 123,
    "username": "john_doe",
    "email": "john@example.com"
  }
}
```

### 2. Konfigurationsdateien

Viele Tools verwenden JSON für Konfiguration:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "strict": true
  }
}
```

**Beispiele:**

- `package.json` (Node.js/npm)
- `tsconfig.json` (TypeScript)
- `.vscode/settings.json` (Visual Studio Code)

### 3. Datenspeicherung

Ideal für strukturierte Daten:

```json
[
  {
    "id": "001",
    "product": "Laptop",
    "price": 999.99,
    "in_stock": true
  }
]
```

## Konvertierung mit ASP CLI

### JSON zu anderen Formaten

```bash
# JSON zu YAML
asp_cli convert data.json --to yaml -o data.yaml

# JSON zu TOML
asp_cli convert config.json --to toml -o config.toml

# JSON zu CSV (nur flache Arrays)
asp_cli convert users.json --to csv -o users.csv
```

### Andere Formate zu JSON

```bash
# YAML zu JSON
asp_cli convert config.yaml --to json -o config.json

# TOML zu JSON
asp_cli convert Cargo.toml --to json -o cargo.json

# CSV zu JSON
asp_cli convert data.csv --to json -o data.json
```

## Best Practices

::: tip Pretty-Printing
Verwenden Sie Einrückung für bessere Lesbarkeit:

```json
// Gut formatiert
{
  "name": "value",
  "nested": {
    "key": "value"
  }
}

// Schlecht: Alles in einer Zeile
{"name":"value","nested":{"key":"value"}}
```

:::

::: warning Keys immer in Anführungszeichen
JSON erfordert doppelte Anführungszeichen für Keys:

```json
// ✅ Korrekt
{ "name": "value" }

// ❌ Falsch
{ name: "value" }
{ 'name': 'value' }
```

:::

::: danger Keine Trailing Commas
JSON erlaubt keine Kommas nach dem letzten Element:

```json
// ✅ Korrekt
{
  "a": 1,
  "b": 2
}

// ❌ Falsch
{
  "a": 1,
  "b": 2,
}
```

:::

## Häufige Fehler

### Syntax-Fehler

```json
// ❌ Fehler: Fehlende schließende Klammer
{
  "name": "value"

// ❌ Fehler: Fehlende Anführungszeichen
{
  name: "value"
}

// ❌ Fehler: Trailing Comma
{
  "a": 1,
  "b": 2,
}

// ❌ Fehler: Kommentare nicht erlaubt
{
  // Dies ist ein Kommentar
  "name": "value"
}
```

## JSON in Rust (Serde)

ASP CLI verwendet die `serde_json` Library zum Parsen:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    version: String,
}

// Deserialisieren
let data: Config = serde_json::from_str(json_string)?;

// Serialisieren
let json = serde_json::to_string_pretty(&data)?;
```

## Vergleich mit anderen Formaten

| Feature       | JSON       | YAML       | TOML       |
| ------------- | ---------- | ---------- | ---------- |
| Lesbarkeit    | ⭐⭐⭐     | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐   |
| Kompaktheit   | ⭐⭐⭐⭐   | ⭐⭐⭐     | ⭐⭐⭐     |
| Parsing-Speed | ⭐⭐⭐⭐⭐ | ⭐⭐⭐     | ⭐⭐⭐⭐   |
| Typsystem     | ⭐⭐⭐⭐   | ⭐⭐⭐⭐   | ⭐⭐⭐⭐⭐ |
| Kommentare    | ❌         | ✅         | ✅         |

## Ressourcen

- [JSON Specification](https://www.json.org/)
- [JSON Schema](https://json-schema.org/)
- [serde_json Documentation](https://docs.rs/serde_json/)
- [RFC 8259 - The JSON Data Interchange Format](https://datatracker.ietf.org/doc/html/rfc8259)

## Nächste Schritte

- [YAML Format →](/formats/yaml)
- [Formatvergleich →](/formats/comparison)
- [Beispiele →](/guide/examples)
