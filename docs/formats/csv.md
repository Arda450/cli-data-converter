# CSV (Comma-Separated Values)

CSV ist ein einfaches, textbasiertes Format zum Speichern tabularer Daten.

## Überblick

**Entwickelt:** Frühe 1970er Jahre  
**Dateiendung:** `.csv`  
**MIME-Type:** `text/csv`  
**Standard:** [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180)

## Eigenschaften

### ✅ Vorteile

- **Universell**: Wird von allen Tabellenkalkulationen und Datenbanken unterstützt
- **Einfach**: Sehr simples, menschenlesbares Format
- **Kompakt**: Minimal Overhead, kleine Dateigröße
- **Streaming**: Kann zeilenweise verarbeitet werden
- **Legacy-Support**: Existiert seit den 1970ern

### ❌ Nachteile

- **Nur flache Strukturen**: Keine verschachtelten Objekte oder Arrays
- **Keine Typen**: Alles wird als String gespeichert
- **Inkonsistent**: Keine einheitliche Spezifikation (viele Dialekte)
- **Escaping-Probleme**: Kommas und Anführungszeichen müssen escaped werden
- **Keine Metadaten**: Keine Informationen über Datentypen oder Struktur

## Syntax

### Grundstruktur

```csv
header1,header2,header3
value1,value2,value3
value4,value5,value6
```

### Mit Header-Zeile

```csv
id,name,email,age
1,Alice,alice@example.com,30
2,Bob,bob@example.com,25
3,Charlie,charlie@example.com,35
```

### Escaping von Sonderzeichen

```csv
id,name,description,price
1,"Product A","Simple product",19.99
2,"Product B","Contains, comma",29.99
3,"Product C","Contains ""quotes""",39.99
```

**Regeln:**

- Felder mit Kommas müssen in Anführungszeichen
- Anführungszeichen werden als `""` escaped
- Zeilenumbrüche in Feldern erfordern Anführungszeichen

### Verschiedene Trennzeichen

```csv
# Semikolon (häufig in Europa)
id;name;email
1;Alice;alice@example.com

# Tab-getrennt (TSV)
id	name	email
1	Alice	alice@example.com
```

## Verwendungszwecke

### 1. Datenbank-Export

```csv
id,user_id,product_id,quantity,price,created_at
1001,42,101,2,39.98,2025-12-18 10:30:00
1002,43,102,1,19.99,2025-12-18 10:35:00
1003,42,103,5,99.95,2025-12-18 10:40:00
```

### 2. Datenimport

Excel/Google Sheets-Daten für Webanwendungen:

```csv
username,email,role,active
john_doe,john@example.com,admin,true
jane_smith,jane@example.com,user,true
bob_jones,bob@example.com,user,false
```

### 3. Log-Daten

```csv
timestamp,level,message,user_id
2025-12-18T10:30:00Z,INFO,User logged in,42
2025-12-18T10:30:15Z,WARNING,Failed login attempt,43
2025-12-18T10:30:30Z,ERROR,Database connection lost,NULL
```

## Konvertierung mit ASP CLI

### CSV zu anderen Formaten

```bash
# CSV zu JSON
asp_cli convert users.csv --to json -o users.json

# CSV zu YAML
asp_cli convert data.csv --to yaml -o data.yaml

# CSV zu TOML
asp_cli convert products.csv --to toml -o products.toml
```

**Beispiel-Konvertierung:**

```csv
# users.csv
id,name,email
1,Alice,alice@example.com
2,Bob,bob@example.com
```

```json
// users.json
[
  {
    "id": "1",
    "name": "Alice",
    "email": "alice@example.com"
  },
  {
    "id": "2",
    "name": "Bob",
    "email": "bob@example.com"
  }
]
```

### Andere Formate zu CSV

```bash
# JSON zu CSV
asp_cli convert users.json --to csv -o users.csv

# YAML zu CSV
asp_cli convert data.yaml --to csv -o data.csv

# TOML zu CSV
asp_cli convert config.toml --to csv -o config.csv
```

::: warning Limitierung: Nur flache Arrays
CSV unterstützt nur flache Strukturen. Verschachtelte Objekte können nicht direkt konvertiert werden.

```json
// ✅ Funktioniert
[
  { "id": 1, "name": "Alice" },
  { "id": 2, "name": "Bob" }
]

// ❌ Problematisch
[
  {
    "id": 1,
    "user": {
      "name": "Alice",
      "address": { "city": "Berlin" }
    }
  }
]
```

:::

## Best Practices

::: tip Immer Header-Zeile verwenden
Eine Header-Zeile macht CSV-Dateien selbstdokumentierend:

```csv
# ✅ Gut: Mit Header
id,name,email
1,Alice,alice@example.com

# ❌ Schlecht: Ohne Header
1,Alice,alice@example.com
```

:::

::: tip Konsistentes Trennzeichen
Bleiben Sie bei einem Trennzeichen pro Datei:

```csv
# ✅ Konsistent: Immer Komma
id,name,email

# ❌ Inkonsistent: Gemischt
id,name;email
```

:::

::: warning Vorsicht bei Excel
Excel hat eigene CSV-Konventionen (z.B. Semikolon in Europa):

```csv
# Excel Europa: Semikolon
id;name;email
1;Alice;alice@example.com
```

:::

## Häufige Probleme

### 1. Kommas in Daten

```csv
# ❌ Problem: Komma in Daten
name,description,price
Product A,Contains, extra features,19.99

# ✅ Lösung: Anführungszeichen
name,description,price
Product A,"Contains, extra features",19.99
```

### 2. Anführungszeichen in Daten

```csv
# ❌ Problem: Quote in Daten
name,description
Product,"20" Monitor

# ✅ Lösung: Doppelte Quotes
name,description
Product,"20"" Monitor"
```

### 3. Zeilenumbrüche in Feldern

```csv
# ❌ Problem: Zeilenumbruch
description
This is line 1
This is line 2

# ✅ Lösung: Quotes um gesamtes Feld
description
"This is line 1
This is line 2"
```

### 4. Fehlende Werte

```csv
# Verschiedene Arten, NULL darzustellen
id,name,email
1,Alice,alice@example.com
2,Bob,
3,Charlie,NULL
4,David,N/A
```

## CSV in Rust

ASP CLI verwendet die `csv` Library:

```rust
use csv::Reader;
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    id: u32,
    name: String,
    email: String,
}

// CSV lesen
let mut reader = csv::Reader::from_path("data.csv")?;
for result in reader.deserialize() {
    let record: Record = result?;
    println!("{:?}", record);
}

// CSV schreiben
let mut writer = csv::Writer::from_path("output.csv")?;
writer.write_record(&["id", "name", "email"])?;
writer.write_record(&["1", "Alice", "alice@example.com"])?;
writer.flush()?;
```

## CSV-Dialekte

### Standard CSV (RFC 4180)

```csv
id,name,email
1,Alice,alice@example.com
```

- **Trennzeichen:** Komma (`,`)
- **Quote:** Doppelte Anführungszeichen (`"`)
- **Escape:** Doppelte Quotes (`""`)

### Excel CSV (Europa)

```csv
id;name;email
1;Alice;alice@example.com
```

- **Trennzeichen:** Semikolon (`;`)
- **Dezimaltrennzeichen:** Komma (`,`)

### TSV (Tab-Separated Values)

```tsv
id	name	email
1	Alice	alice@example.com
```

- **Trennzeichen:** Tab (`\t`)
- **Vorteil:** Tabs kommen selten in Daten vor

## Vergleich mit anderen Formaten

| Feature         | CSV        | JSON       | YAML       | TOML       |
| --------------- | ---------- | ---------- | ---------- | ---------- |
| Tabulare Daten  | ⭐⭐⭐⭐⭐ | ⭐⭐⭐     | ⭐⭐       | ⭐⭐       |
| Verschachtelung | ❌         | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐   |
| Typsystem       | ❌         | ⭐⭐⭐⭐   | ⭐⭐⭐⭐   | ⭐⭐⭐⭐⭐ |
| Dateigröße      | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐   | ⭐⭐⭐     | ⭐⭐⭐     |
| Excel-Support   | ⭐⭐⭐⭐⭐ | ⭐⭐       | ⭐         | ⭐         |

## Wann CSV verwenden?

### ✅ Ideal für:

- Tabulare Daten (Tabellen)
- Datenbank-Im-/Export
- Excel/Google Sheets-Integration
- Große Datasets (Streaming möglich)
- Legacy-Systeme

### ❌ Weniger geeignet für:

- Verschachtelte Strukturen
- Komplexe Datentypen
- Konfigurationsdateien
- API-Kommunikation

## Tools und Ressourcen

- [RFC 4180 - CSV Standard](https://datatracker.ietf.org/doc/html/rfc4180)
- [csv-rs Documentation](https://docs.rs/csv/)
- [CSV Lint - Online Validator](https://csvlint.io/)
- [Papa Parse - JavaScript CSV Parser](https://www.papaparse.com/)

## Nächste Schritte

- [Formatvergleich →](/formats/comparison)
- [Beispiele →](/guide/examples)
- [Architektur →](/architecture/overview)
