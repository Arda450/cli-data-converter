# Erste Schritte

Willkommen bei ASP CLI – Ihrem Tool für die Konvertierung zwischen verschiedenen Datenformaten!

## Was ist ASP CLI?

ASP CLI ist ein Kommandozeilen-Tool, das in Rust entwickelt wurde und die bidirektionale Konvertierung zwischen folgenden Datenformaten ermöglicht:

- **JSON** (JavaScript Object Notation)
- **YAML** (YAML Ain't Markup Language)
- **TOML** (Tom's Obvious Minimal Language)
- **CSV** (Comma-Separated Values)

## Hauptfunktionen

### 🔄 Bidirektionale Konvertierung

Konvertieren Sie beliebige unterstützte Formate ineinander. Das Tool erkennt automatisch das Eingabeformat und konvertiert es in das gewünschte Zielformat.

### ✅ Automatische Validierung

Jede Eingabedatei wird vor der Konvertierung validiert. Bei Syntaxfehlern erhalten Sie detaillierte Fehlermeldungen mit Zeilennummern und Hinweisen.

### 🎨 Pretty-Printing

Die Ausgabe kann mit Pretty-Printing formatiert werden, um die Lesbarkeit zu verbessern – ideal für Konfigurationsdateien und Debugging.

### 💬 Klares Feedback

Das Tool zeigt während der Konvertierung klare Statusmeldungen an:

- Eingabedatei und erkanntes Format
- Ausgabedatei und Zielformat
- Erfolgs- oder Fehlermeldungen

## Anwendungsfälle

### Konfigurationsmanagement

Konvertieren Sie Konfigurationsdateien zwischen verschiedenen Formaten, z.B. wenn Sie von YAML auf TOML umsteigen möchten.

```bash
asp_cli convert docker-compose.yml --to toml -o config.toml
```

### API-Entwicklung

Konvertieren Sie Testdaten zwischen JSON und anderen Formaten für verschiedene Test-Szenarien.

```bash
asp_cli convert test-data.json --to yaml -o test-data.yaml
```

### Datenverarbeitung

Konvertieren Sie CSV-Daten in strukturierte Formate wie JSON für die Weiterverarbeitung.

```bash
asp_cli convert data.csv --to json -o data.json
```

## Warum Rust?

Dieses Tool wurde in Rust entwickelt, weil:

- **Performance**: Rust bietet native Performance ohne Garbage Collection
- **Sicherheit**: Der Rust-Compiler verhindert Memory-Leaks und Race Conditions
- **Zuverlässigkeit**: Das Type-System garantiert zur Compile-Zeit viele Fehlerfreiheiten
- **Ecosystem**: Rust hat exzellente Libraries für Parsing und CLI-Entwicklung

## Nächste Schritte

::: tip Bereit zum Starten?
Folgen Sie der [Installation](/guide/installation), um ASP CLI auf Ihrem System einzurichten.
:::

Nach der Installation können Sie direkt mit der [Verwendung](/guide/usage) beginnen oder sich die [Beispiele](/guide/examples) ansehen.
