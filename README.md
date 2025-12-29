# ASP CLI - Data Converter

Ein leistungsstarkes Command-Line-Interface Tool für bidirektionale Konvertierung zwischen verschiedenen Datenformaten (JSON, YAML, TOML, CSV).

## ✨ Features

- 🔄 Bidirektionale Konvertierung zwischen JSON, YAML, TOML und CSV
- ⚡ Schnelle Verarbeitung mit Rust
- 🛡️ Robuste Fehlerbehandlung
- 🎯 Einfache CLI-Bedienung mit `clap`
- 📦 Modulare Architektur

## 📋 Voraussetzungen

- Rust (mindestens Version 1.70.0)
- Cargo

## 🚀 Installation

### Option 1: Lokale Installation (Entwicklung)

```bash
# Repository klonen
git clone https://github.com/IhrUsername/asp_cli.git
cd asp_cli

# Projekt bauen
cargo build --release

# Tool testen
cargo run -- input.json output.yaml
```

### Option 2: Globale Installation (Empfohlen)

```bash
# Repository klonen
git clone https://github.com/IhrUsername/asp_cli.git
cd asp_cli

# Global installieren (Binary wird in ~/.cargo/bin/ installiert)
cargo install --path .

# Jetzt können Sie 'asp' von überall aufrufen:
asp input.json output.toml
```

### Option 3: Direkt von crates.io (Zukünftig)

```bash
# Sobald auf crates.io veröffentlicht:
cargo install asp_cli
```

### ⚠️ PATH-Konfiguration

Nach der Installation muss `~/.cargo/bin/` in Ihrem PATH sein:

**Linux/macOS:**

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Fügen Sie diese Zeile zu `~/.bashrc` oder `~/.zshrc` hinzu.

**Windows:**

- `~/.cargo/bin` ist normalerweise automatisch im PATH
- Falls nicht: Systemsteuerung → System → Erweiterte Systemeinstellungen → Umgebungsvariablen → PATH bearbeiten

## 📖 Verwendung

### Grundlegende Syntax

```bash
asp <input-datei> <output-datei>
```

### Beispiele

```bash
# JSON zu YAML
asp data.json data.yaml

# JSON zu TOML
asp config.json config.toml

# YAML zu JSON
asp data.yaml data.json

# JSON zu CSV
asp users.json users.csv

# TOML zu YAML
asp config.toml config.yaml
```

### Unterstützte Formate

| Von → Nach | JSON | YAML | TOML | CSV |
| ---------- | ---- | ---- | ---- | --- |
| **JSON**   | ✅   | ✅   | ✅   | ✅  |
| **YAML**   | 🔄   | 🔄   | 🔄   | 🔄  |
| **TOML**   | ✅   | ✅   | ✅   | ✅  |
| **CSV**    | 🔄   | 🔄   | 🔄   | 🔄  |

✅ = Implementiert | 🔄 = Geplant

## 🛠️ Entwicklung

```bash
# Projekt bauen
cargo build

# Tests ausführen
cargo test

# Code formatieren
cargo fmt

# Linting
cargo clippy

# Release-Build erstellen
cargo build --release
```

### Binary-Location nach Build

- **Debug:** `target/debug/asp` oder `target/debug/asp.exe` (Windows)
- **Release:** `target/release/asp` oder `target/release/asp.exe` (Windows)

## 📚 Dokumentation

Vollständige Dokumentation verfügbar unter:

```bash
# Dokumentation lokal starten
cd docs
npm install
npm run dev
```

Besuchen Sie dann: http://localhost:5173

## 🤝 Beitragen

1. Fork das Repository
2. Erstellen Sie einen Feature-Branch (`git checkout -b feature/AmazingFeature`)
3. Commit Ihre Änderungen (`git commit -m 'Add some AmazingFeature'`)
4. Push zum Branch (`git push origin feature/AmazingFeature`)
5. Öffnen Sie einen Pull Request

## 📄 Lizenz

Dieses Projekt ist lizenziert unter MIT oder Apache-2.0 - siehe [LICENSE](LICENSE) Datei für Details.

## 👤 Autor

Ihr Name - [@IhrGitHubUsername](https://github.com/IhrUsername)

## 🙏 Danksagungen

- [Serde](https://serde.rs/) für Serialisierung/Deserialisierung
- [Clap](https://docs.rs/clap/) für CLI-Parsing
- Rust Community
