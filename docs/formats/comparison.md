# Format-Vergleich

Ein umfassender Vergleich aller von ASP CLI unterstützten Datenformate.

## Übersichtstabelle

| Feature              | JSON       | YAML       | TOML       | CSV        |
| -------------------- | ---------- | ---------- | ---------- | ---------- |
| **Lesbarkeit**       | ⭐⭐⭐     | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐   | ⭐⭐⭐     |
| **Parsing-Speed**    | ⭐⭐⭐⭐⭐ | ⭐⭐⭐     | ⭐⭐⭐⭐   | ⭐⭐⭐⭐⭐ |
| **Kompaktheit**      | ⭐⭐⭐⭐   | ⭐⭐⭐     | ⭐⭐⭐     | ⭐⭐⭐⭐⭐ |
| **Verschachtelung**  | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐   | ❌         |
| **Typsystem**        | ⭐⭐⭐⭐   | ⭐⭐⭐⭐   | ⭐⭐⭐⭐⭐ | ❌         |
| **Kommentare**       | ❌         | ✅         | ✅         | ❌         |
| **Fehlerresistenz**  | ⭐⭐⭐⭐   | ⭐⭐       | ⭐⭐⭐     | ⭐⭐⭐     |
| **Standardisierung** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐     | ⭐⭐⭐⭐   | ⭐⭐       |
| **Tool-Support**     | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐     | ⭐⭐⭐⭐⭐ |

## Detaillierter Vergleich

### JSON (JavaScript Object Notation)

#### Stärken

- ✅ **Web-Standard**: Universelle Unterstützung in allen modernen Sprachen
- ✅ **Schnell zu parsen**: Native JavaScript-Unterstützung
- ✅ **Gut für APIs**: Kompaktes Format für Datentransfer
- ✅ **Typisiert**: Unterscheidet Numbers, Strings, Booleans, null

#### Schwächen

- ❌ **Keine Kommentare**: Keine Inline-Dokumentation möglich
- ❌ **Weniger lesbar**: Viele Klammern und Quotes
- ❌ **Strikte Syntax**: Trailing commas führen zu Fehlern

#### Beste Verwendung

- REST APIs und Web-Services
- Konfiguration für JavaScript/TypeScript-Projekte
- Datenübertragung zwischen Systemen
- NoSQL-Datenbanken (MongoDB, CouchDB)

**Beispiel:**

```json
{
  "app": {
    "name": "MyApp",
    "version": "1.0.0",
    "server": {
      "host": "localhost",
      "port": 8080
    }
  }
}
```

---

### YAML (YAML Ain't Markup Language)

#### Stärken

- ✅ **Sehr lesbar**: Minimale Syntax, intuitive Struktur
- ✅ **Kommentare**: Inline-Dokumentation mit `#`
- ✅ **Flexibel**: Mehrere Wege, Daten zu strukturieren
- ✅ **Anker & Aliase**: Wiederverwendbare Konfiguration

#### Schwächen

- ❌ **Indentierungs-sensitiv**: Spaces vs. Tabs können problematisch sein
- ❌ **Langsames Parsing**: Komplexer als JSON
- ❌ **Mehrdeutig**: Verschiedene Syntaxen für dasselbe Resultat
- ❌ **Boolean-Fallen**: `yes`, `no`, `on`, `off` werden zu Booleans

#### Beste Verwendung

- CI/CD Pipelines (GitHub Actions, GitLab CI)
- Docker Compose und Kubernetes
- Konfigurationsdateien für DevOps
- Ansible Playbooks

**Beispiel:**

```yaml
app:
  name: MyApp
  version: 1.0.0
  server:
    host: localhost
    port: 8080
```

---

### TOML (Tom's Obvious Minimal Language)

#### Stärken

- ✅ **Eindeutig**: Nur eine Art, Daten zu strukturieren
- ✅ **Typisiert**: Starkes Typsystem mit vielen Datentypen
- ✅ **Kommentare**: Inline-Dokumentation mit `#`
- ✅ **Lesbar**: Ähnlich wie INI-Dateien

#### Schwächen

- ❌ **Weniger verbreitet**: Nicht so populär wie JSON/YAML
- ❌ **Verbos bei Verschachtelung**: Tief verschachtelte Strukturen werden lang
- ❌ **Limitierte Tool-Unterstützung**: Weniger Editor-Plugins

#### Beste Verwendung

- Rust-Projekte (Cargo.toml)
- Python-Projekte (pyproject.toml)
- Einfache bis mittlere Konfigurationen
- Wenn Eindeutigkeit wichtig ist

**Beispiel:**

```toml
[app]
name = "MyApp"
version = "1.0.0"

[app.server]
host = "localhost"
port = 8080
```

---

### CSV (Comma-Separated Values)

#### Stärken

- ✅ **Universell**: Excel, Google Sheets, alle Datenbanken
- ✅ **Einfach**: Minimale Syntax
- ✅ **Kompakt**: Sehr kleine Dateigröße
- ✅ **Streaming**: Kann zeilenweise verarbeitet werden

#### Schwächen

- ❌ **Nur flache Strukturen**: Keine Verschachtelung
- ❌ **Keine Typen**: Alles ist ein String
- ❌ **Inkonsistent**: Viele verschiedene Dialekte
- ❌ **Escaping-Probleme**: Kommas und Quotes müssen escaped werden

#### Beste Verwendung

- Tabulare Daten (Tabellen)
- Datenbank-Im-/Export
- Excel/Google Sheets-Integration
- Große Datasets

**Beispiel:**

```csv
name,version,host,port
MyApp,1.0.0,localhost,8080
```

## Anwendungsfälle

### API-Entwicklung

| Format | Eignung    | Begründung                   |
| ------ | ---------- | ---------------------------- |
| JSON   | ⭐⭐⭐⭐⭐ | Standard für REST APIs       |
| YAML   | ⭐⭐       | Zu verbose für Datentransfer |
| TOML   | ⭐⭐       | Nicht üblich in APIs         |
| CSV    | ⭐         | Nur für tabulare Daten       |

**Empfehlung:** JSON

---

### Konfigurationsdateien

| Format | Eignung    | Begründung                          |
| ------ | ---------- | ----------------------------------- |
| JSON   | ⭐⭐⭐     | Funktioniert, aber keine Kommentare |
| YAML   | ⭐⭐⭐⭐⭐ | Sehr lesbar, Kommentare möglich     |
| TOML   | ⭐⭐⭐⭐⭐ | Eindeutig, gut lesbar               |
| CSV    | ❌         | Nicht geeignet                      |

**Empfehlung:** YAML oder TOML (je nach Projekt)

---

### CI/CD Pipelines

| Format | Eignung    | Begründung                            |
| ------ | ---------- | ------------------------------------- |
| JSON   | ⭐⭐       | Zu unleserlich für komplexe Pipelines |
| YAML   | ⭐⭐⭐⭐⭐ | Industrie-Standard                    |
| TOML   | ⭐⭐       | Nicht üblich                          |
| CSV    | ❌         | Nicht geeignet                        |

**Empfehlung:** YAML

---

### Datenbank-Export

| Format | Eignung    | Begründung                            |
| ------ | ---------- | ------------------------------------- |
| JSON   | ⭐⭐⭐⭐   | Gut für NoSQL und strukturierte Daten |
| YAML   | ⭐⭐       | Zu verbose                            |
| TOML   | ⭐⭐       | Nicht üblich                          |
| CSV    | ⭐⭐⭐⭐⭐ | Perfekt für tabulare Daten            |

**Empfehlung:** CSV für Tabellen, JSON für Dokumente

---

### Rust-Projekte

| Format | Eignung    | Begründung                        |
| ------ | ---------- | --------------------------------- |
| JSON   | ⭐⭐⭐     | Möglich, aber nicht idiomatisch   |
| YAML   | ⭐⭐⭐     | Funktioniert, aber nicht Standard |
| TOML   | ⭐⭐⭐⭐⭐ | Cargo.toml ist der Standard       |
| CSV    | ⭐         | Nur für Daten                     |

**Empfehlung:** TOML

## Konvertierungs-Matrix

Diese Tabelle zeigt, wie gut sich Formate ineinander konvertieren lassen:

|            | → JSON | → YAML | → TOML | → CSV                |
| ---------- | ------ | ------ | ------ | -------------------- |
| **JSON →** | ✅     | ✅     | ✅     | ⚠️ Nur flache Arrays |
| **YAML →** | ✅     | ✅     | ✅     | ⚠️ Nur flache Arrays |
| **TOML →** | ✅     | ✅     | ✅     | ⚠️ Nur flache Arrays |
| **CSV →**  | ✅     | ✅     | ✅     | ✅                   |

**Legende:**

- ✅ = Vollständige Konvertierung ohne Datenverlust
- ⚠️ = Konvertierung mit Einschränkungen

## Performance-Vergleich

### Parsing-Geschwindigkeit (relativer Vergleich)

```
JSON:   ████████████████████ (schnellste)
CSV:    ███████████████████
TOML:   ████████████
YAML:   ████████ (langsamste)
```

### Dateigröße (relativer Vergleich, gleiche Daten)

```
CSV:    ████ (kleinste)
JSON:   ██████
TOML:   ████████
YAML:   ██████████ (größte)
```

## Entscheidungshilfe

### Wähle JSON, wenn:

- Du ein Web-API entwickelst
- Performance kritisch ist
- Du maximale Kompatibilität brauchst
- Du mit JavaScript/TypeScript arbeitest

### Wähle YAML, wenn:

- Du Konfigurationsdateien schreibst
- Lesbarkeit wichtiger als Performance ist
- Du Kommentare brauchst
- Du CI/CD Pipelines konfigurierst

### Wähle TOML, wenn:

- Du Rust- oder Python-Projekte entwickelst
- Du Eindeutigkeit willst
- Du ein starkes Typsystem brauchst
- Du mittlere Konfigurationsdateien schreibst

### Wähle CSV, wenn:

- Du tabulare Daten hast
- Du mit Excel/Google Sheets arbeitest
- Du Datenbank-Im-/Export machst
- Du sehr große Datasets hast

## Kombinierte Verwendung

In realen Projekten werden oft mehrere Formate kombiniert:

```
Project/
├── Cargo.toml           # TOML (Rust-Config)
├── config.yaml          # YAML (App-Config)
├── data/
│   ├── users.csv        # CSV (Daten)
│   └── cache.json       # JSON (Runtime-Daten)
└── .github/
    └── workflows/
        └── ci.yml       # YAML (CI/CD)
```

## Nächste Schritte

- [Praktische Beispiele →](/guide/examples)
- [Architektur verstehen →](/architecture/overview)
- [E-Learning starten →](/learning/introduction)
