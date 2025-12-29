---
layout: home

hero:
  name: "ASP CLI"
  text: "Datenkonvertierungs-Tool"
  tagline: Bidirektionale Konvertierung zwischen JSON, YAML, TOML und CSV
  actions:
    - theme: brand
      text: Erste Schritte
      link: /guide/getting-started
    - theme: alt
      text: Formate entdecken
      link: /formats/comparison
    - theme: alt
      text: E-Learning
      link: /learning/introduction

features:
  - title: 🔄 Flexible Konvertierung
    details: Unterstützt bidirektionale Konvertierung zwischen JSON, YAML, TOML und CSV mit automatischer Formaterkennung
  - title: 🚀 Performant & Sicher
    details: Entwickelt in Rust für maximale Performance und Speichersicherheit bei der Verarbeitung großer Dateien
  - title: 💻 Benutzerfreundlich
    details: Intuitive CLI mit klarem Feedback, Pretty-Printing und ausführlichen Fehlerberichten
  - title: 📚 Gut dokumentiert
    details: Umfassende Dokumentation mit Beispielen, technischen Details und E-Learning-Inhalten
  - title: 🎯 Format-Validierung
    details: Automatische Validierung der Eingabedateien mit detaillierten Fehlermeldungen
  - title: ⚙️ Konfigurierbar
    details: Verschiedene Optionen für Output-Format, Pretty-Printing und mehr
---

## Quick Start

```bash
# JSON zu YAML konvertieren
asp_cli convert input.json --to yaml -o output.yaml

# TOML zu JSON konvertieren
asp_cli convert config.toml --to json -o config.json

# CSV zu JSON konvertieren
asp_cli convert data.csv --to json -o data.json
```

## Über dieses Projekt

Dieses CLI-Tool wurde als **Advanced Specialised Project (ASP)** entwickelt und demonstriert die praktische Anwendung von Rust in der Datenverarbeitung. Es verbindet systemnahe Programmierung mit effizienter Datenkonvertierung und bietet eine solide Grundlage für die Arbeit mit verschiedenen Datenformaten.

**Entwickelt von:** Arda Karadavut  
**Fachrichtung:** Webdesign & Development  
**Spezialisierung:** Datenkonvertierung mit CLI-Tools in Rust
