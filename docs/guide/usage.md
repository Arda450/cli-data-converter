# Verwendung

Diese Seite erklärt die grundlegende Verwendung von ASP CLI.

## Grundlegende Syntax

```bash
asp_cli convert <INPUT_FILE> --to <FORMAT> -o <OUTPUT_FILE>
```

### Parameter

| Parameter             | Beschreibung                       | Erforderlich |
| --------------------- | ---------------------------------- | ------------ |
| `<INPUT_FILE>`        | Pfad zur Eingabedatei              | Ja           |
| `--to <FORMAT>`       | Zielformat (json, yaml, toml, csv) | Ja           |
| `-o, --output <FILE>` | Pfad zur Ausgabedatei              | Ja           |

## Unterstützte Formate

Das Tool unterstützt folgende Formate für Ein- und Ausgabe:

- `json` - JavaScript Object Notation
- `yaml` - YAML Ain't Markup Language
- `toml` - Tom's Obvious Minimal Language
- `csv` - Comma-Separated Values

## Beispiele

### JSON zu YAML

```bash
asp_cli convert data.json --to yaml -o data.yaml
```

**Eingabe (data.json):**

```json
{
  "name": "ASP CLI",
  "version": "1.0.0",
  "author": "Arda Karadavut"
}
```

**Ausgabe (data.yaml):**

```yaml
name: ASP CLI
version: 1.0.0
author: Arda Karadavut
```

### YAML zu JSON

```bash
asp_cli convert config.yaml --to json -o config.json
```

**Eingabe (config.yaml):**

```yaml
server:
  host: localhost
  port: 8080
database:
  url: postgresql://localhost/mydb
```

**Ausgabe (config.json):**

```json
{
  "server": {
    "host": "localhost",
    "port": 8080
  },
  "database": {
    "url": "postgresql://localhost/mydb"
  }
}
```

### TOML zu JSON

```bash
asp_cli convert Cargo.toml --to json -o package.json
```

**Eingabe (Cargo.toml):**

```toml
[package]
name = "asp_cli"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = "4.5"
serde = "1.0"
```

**Ausgabe (package.json):**

```json
{
  "package": {
    "name": "asp_cli",
    "version": "0.1.0",
    "edition": "2024"
  },
  "dependencies": {
    "clap": "4.5",
    "serde": "1.0"
  }
}
```

### CSV zu JSON

```bash
asp_cli convert users.csv --to json -o users.json
```

**Eingabe (users.csv):**

```csv
id,name,email
1,Alice,alice@example.com
2,Bob,bob@example.com
```

**Ausgabe (users.json):**

```json
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

## Formatererkennung

ASP CLI erkennt das Eingabeformat automatisch anhand der Dateiendung:

- `.json` → JSON
- `.yaml` oder `.yml` → YAML
- `.toml` → TOML
- `.csv` → CSV

::: tip Automatische Erkennung
Sie müssen das Eingabeformat nicht explizit angeben. Das Tool erkennt es automatisch und validiert die Syntax.
:::

## Fehlerbehandlung

Das Tool bietet detaillierte Fehlermeldungen bei Problemen:

### Ungültige Syntax

```bash
$ asp_cli convert invalid.json --to yaml -o output.yaml
```

**Ausgabe:**

```
Fehler beim Parsen der JSON-Datei:
Zeile 3, Spalte 15: Erwartetes Komma oder schließende Klammer
```

### Datei nicht gefunden

```bash
$ asp_cli convert missing.json --to yaml -o output.yaml
```

**Ausgabe:**

```
Fehler: Eingabedatei 'missing.json' nicht gefunden
```

### Ungültiges Format

```bash
$ asp_cli convert data.json --to xml -o output.xml
```

**Ausgabe:**

```
Fehler: Ungültiges Ausgabeformat 'xml'
Unterstützte Formate: json, yaml, toml, csv
```

## Status-Ausgabe

Bei erfolgreicher Konvertierung zeigt das Tool folgende Informationen:

```bash
$ asp_cli convert config.yaml --to json -o config.json
```

**Ausgabe:**

```
✓ Eingabedatei: config.yaml (YAML)
✓ Ausgabedatei: config.json (JSON)
✓ Konvertierung erfolgreich abgeschlossen!
```

## Tipps und Best Practices

::: tip Pretty-Printing
Das Tool formatiert die Ausgabe automatisch für bessere Lesbarkeit. JSON wird mit Einrückung ausgegeben, YAML und TOML werden strukturiert formatiert.
:::

::: warning Große Dateien
Bei sehr großen Dateien (> 100 MB) kann die Konvertierung einige Sekunden dauern. Das Tool lädt die gesamte Datei in den Speicher.
:::

::: danger Datenverlust
Beachten Sie, dass nicht alle Formate die gleichen Datentypen unterstützen. CSV unterstützt beispielsweise nur flache Strukturen und Strings.
:::

## Arbeiten mit Pipes

Sie können ASP CLI auch mit Unix-Pipes kombinieren:

```bash
# Ausgabe direkt anzeigen
cat config.yaml | asp_cli convert - --to json -o -

# Mit anderen Tools kombinieren
asp_cli convert data.json --to yaml -o - | grep "version"
```

::: info Hinweis
Die Pipe-Funktionalität ist in der aktuellen Version noch nicht vollständig implementiert.
:::

## Nächste Schritte

- Sehen Sie sich weitere [Beispiele](/guide/examples) an
- Erfahren Sie mehr über die einzelnen [Formate](/formats/comparison)
- Verstehen Sie die [Architektur](/architecture/overview) des Tools
