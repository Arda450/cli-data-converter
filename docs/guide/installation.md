# Installation

Diese Anleitung führt Sie durch die Installation von ASP CLI auf Ihrem System.

## Voraussetzungen

Um ASP CLI zu nutzen oder selbst zu kompilieren, benötigen Sie:

- **Rust** (mindestens Version 1.70.0)
- **Cargo** (wird mit Rust installiert)

::: tip Rust noch nicht installiert?
Besuchen Sie [rustup.rs](https://rustup.rs/) für die offizielle Installation von Rust und Cargo.
:::

## Installation von Rust

Falls Sie Rust noch nicht installiert haben:

### Windows

```bash
# PowerShell oder CMD ausführen
# Laden Sie den Rust-Installer herunter und führen Sie ihn aus
# https://rustup.rs/
```

### macOS / Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Nach der Installation starten Sie ein neues Terminal und überprüfen Sie die Installation:

```bash
rustc --version
cargo --version
```

## ASP CLI installieren

### Option 1: Aus dem Source-Code kompilieren (empfohlen)

1. **Repository klonen:**

```bash
git clone https://github.com/Arda450/cli-data-converter.git
cd asp_cli
```

2. **Projekt kompilieren:**

```bash
# Debug-Build (schneller zu kompilieren, langsamer in der Ausführung)
cargo build

# Release-Build (optimiert, empfohlen für produktive Nutzung)
cargo build --release
```

3. **Ausführbare Datei finden:**

Nach dem Build finden Sie die ausführbare Datei unter:

- **Debug**: `target/debug/asp_cli` (oder `asp_cli.exe` auf Windows)
- **Release**: `target/release/asp_cli` (oder `asp_cli.exe` auf Windows)

### Option 2: Direkt mit Cargo ausführen

Sie können das Tool auch direkt mit Cargo ausführen, ohne es zu installieren:

```bash
cargo run -- convert input.json --to yaml -o output.yaml
```

::: warning Hinweis
Bei dieser Methode muss `--` verwendet werden, um die Argumente an das Programm zu übergeben.
:::

## Installation im System-PATH

Um `asp_cli` von überall ausführen zu können, fügen Sie es zu Ihrem System-PATH hinzu:

### Windows

```powershell
# Kopieren Sie die .exe in ein Verzeichnis im PATH, z.B.:
copy target\release\asp_cli.exe C:\Windows\System32\
```

Oder fügen Sie den Ordner `target/release` zu Ihrem PATH hinzu:

1. Systemsteuerung → System → Erweiterte Systemeinstellungen
2. Umgebungsvariablen → PATH → Bearbeiten
3. Pfad zu `target/release` hinzufügen

### macOS / Linux

```bash
# Kopieren Sie die Binary nach /usr/local/bin
sudo cp target/release/asp_cli /usr/local/bin/

# Oder erstellen Sie einen Symlink
sudo ln -s $(pwd)/target/release/asp_cli /usr/local/bin/asp_cli
```

## Installation verifizieren

Nach der Installation (oder dem Build) testen Sie die Funktionalität:

```bash
# Wenn im PATH installiert:
asp_cli --version

# Oder direkt aus dem Projektverzeichnis:
./target/release/asp_cli --version
```

## Dependencies

ASP CLI verwendet folgende Rust-Crates (werden automatisch von Cargo installiert):

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
toml = "0.8"
csv = "1.3"
```

Diese werden automatisch beim Build-Prozess heruntergeladen und kompiliert.

## Troubleshooting

### Build-Fehler

::: danger Fehler: "linker 'cc' not found"
Auf Linux-Systemen benötigen Sie einen C-Compiler:

```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora/RHEL
sudo dnf install gcc
```

:::

::: danger Fehler: Dependencies können nicht heruntergeladen werden
Überprüfen Sie Ihre Internetverbindung und versuchen Sie:

```bash
cargo clean
cargo build --release
```

:::

### Ausführungsprobleme

::: warning "Permission denied" auf Linux/macOS
Machen Sie die Binary ausführbar:

```bash
chmod +x target/release/asp_cli
```

:::

## Nächste Schritte

Nach erfolgreicher Installation können Sie mit der [Verwendung](/guide/usage) beginnen oder sich die [Beispiele](/guide/examples) ansehen.
