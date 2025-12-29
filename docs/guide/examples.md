# Beispiele

Diese Seite bietet praktische Beispiele für verschiedene Konvertierungs-Szenarien.

## Konfigurationsdateien

### Docker Compose zu TOML

Konvertieren Sie eine Docker Compose YAML-Datei in ein TOML-Format:

```bash
asp_cli convert docker-compose.yml --to toml -o docker-compose.toml
```

::: code-group

```yaml [docker-compose.yml]
version: "3.8"
services:
  web:
    image: nginx:latest
    ports:
      - "80:80"
  db:
    image: postgres:13
    environment:
      POSTGRES_PASSWORD: secret
```

```toml [docker-compose.toml]
version = "3.8"

[services.web]
image = "nginx:latest"
ports = ["80:80"]

[services.db]
image = "postgres:13"

[services.db.environment]
POSTGRES_PASSWORD = "secret"
```

:::

### Package.json zu YAML

Konvertieren Sie eine npm `package.json` in YAML:

```bash
asp_cli convert package.json --to yaml -o package.yaml
```

::: code-group

```json [package.json]
{
  "name": "my-app",
  "version": "1.0.0",
  "scripts": {
    "dev": "vite",
    "build": "vite build"
  },
  "dependencies": {
    "vue": "^3.4.0"
  }
}
```

```yaml [package.yaml]
name: my-app
version: 1.0.0
scripts:
  dev: vite
  build: vite build
dependencies:
  vue: ^3.4.0
```

:::

## Datenverarbeitung

### CSV zu JSON für API

Konvertieren Sie CSV-Daten in JSON für API-Requests:

```bash
asp_cli convert users.csv --to json -o users.json
```

::: code-group

```csv [users.csv]
id,username,email,role
1,alice,alice@example.com,admin
2,bob,bob@example.com,user
3,charlie,charlie@example.com,user
```

```json [users.json]
[
  {
    "id": "1",
    "username": "alice",
    "email": "alice@example.com",
    "role": "admin"
  },
  {
    "id": "2",
    "username": "bob",
    "email": "bob@example.com",
    "role": "user"
  },
  {
    "id": "3",
    "username": "charlie",
    "email": "charlie@example.com",
    "role": "user"
  }
]
```

:::

### JSON zu CSV Export

Exportieren Sie strukturierte JSON-Daten nach CSV:

```bash
asp_cli convert products.json --to csv -o products.csv
```

::: code-group

```json [products.json]
[
  {
    "id": "001",
    "name": "Laptop",
    "price": 999.99,
    "stock": 15
  },
  {
    "id": "002",
    "name": "Mouse",
    "price": 29.99,
    "stock": 50
  }
]
```

```csv [products.csv]
id,name,price,stock
001,Laptop,999.99,15
002,Mouse,29.99,50
```

:::

## CI/CD Pipeline

### GitHub Actions zu TOML

Konvertieren Sie GitHub Actions YAML zu TOML:

```bash
asp_cli convert .github/workflows/ci.yml --to toml -o ci.toml
```

::: code-group

```yaml [ci.yml]
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

```toml [ci.toml]
name = "CI"

[on.push]
branches = ["main"]

[jobs.test]
runs-on = "ubuntu-latest"

[[jobs.test.steps]]
uses = "actions/checkout@v2"

[[jobs.test.steps]]
name = "Run tests"
run = "cargo test"
```

:::

## Komplexe Strukturen

### Verschachtelte JSON zu YAML

Arbeiten Sie mit tief verschachtelten Strukturen:

```bash
asp_cli convert config.json --to yaml -o config.yaml
```

::: code-group

```json [config.json]
{
  "app": {
    "name": "MyApp",
    "version": "2.1.0",
    "server": {
      "host": "0.0.0.0",
      "port": 8080,
      "ssl": {
        "enabled": true,
        "cert": "/path/to/cert.pem",
        "key": "/path/to/key.pem"
      }
    },
    "database": {
      "connections": {
        "primary": {
          "host": "db.example.com",
          "port": 5432,
          "name": "myapp"
        },
        "replica": {
          "host": "db-replica.example.com",
          "port": 5432,
          "name": "myapp"
        }
      }
    }
  }
}
```

```yaml [config.yaml]
app:
  name: MyApp
  version: 2.1.0
  server:
    host: 0.0.0.0
    port: 8080
    ssl:
      enabled: true
      cert: /path/to/cert.pem
      key: /path/to/key.pem
  database:
    connections:
      primary:
        host: db.example.com
        port: 5432
        name: myapp
      replica:
        host: db-replica.example.com
        port: 5432
        name: myapp
```

:::

## Batch-Konvertierung

### Mehrere Dateien konvertieren

Verwenden Sie Shell-Scripting für Batch-Konvertierungen:

```bash
# Alle JSON-Dateien zu YAML konvertieren
for file in *.json; do
  asp_cli convert "$file" --to yaml -o "${file%.json}.yaml"
done
```

```bash
# Alle YAML-Dateien zu JSON konvertieren
for file in *.yaml; do
  asp_cli convert "$file" --to json -o "${file%.yaml}.json"
done
```

## Migration von Projekten

### Rust Cargo.toml zu JSON

Für Tools, die JSON benötigen:

```bash
asp_cli convert Cargo.toml --to json -o cargo-config.json
```

### Python requirements zu JSON

Simulieren Sie eine requirements.txt Konvertierung (mit manueller CSV-Zwischenstufe):

```bash
# 1. Erstellen Sie eine CSV aus requirements.txt
echo "package,version" > requirements.csv
sed 's/==/, /g' requirements.txt >> requirements.csv

# 2. Konvertieren Sie zu JSON
asp_cli convert requirements.csv --to json -o requirements.json
```

## Debugging und Validierung

### Format-Validierung

Nutzen Sie das Tool zum Validieren von Dateien:

```bash
# Konvertieren Sie zurück ins gleiche Format zum Validieren
asp_cli convert suspicious.json --to json -o validated.json
```

Wenn die Datei ungültig ist, erhalten Sie detaillierte Fehlermeldungen:

```
Fehler beim Parsen der JSON-Datei:
Zeile 15, Spalte 8: Erwartetes schließendes Anführungszeichen
```

### Pretty-Print für Debugging

Nutzen Sie die Konvertierung für Pretty-Printing:

```bash
# Kompakte JSON lesbar machen
asp_cli convert minified.json --to json -o readable.json

# YAML neu formatieren
asp_cli convert messy.yaml --to yaml -o clean.yaml
```

## Realworld-Szenarien

### API-Response speichern und konvertieren

```bash
# API-Response zu Datei
curl https://api.example.com/data > response.json

# Zu YAML konvertieren für bessere Lesbarkeit
asp_cli convert response.json --to yaml -o response.yaml
```

### Konfiguration zwischen Systemen migrieren

```bash
# Docker Compose zu Kubernetes ConfigMap
asp_cli convert docker-compose.yml --to json -o config.json
```

### Datenexport aus Datenbanken

```bash
# CSV-Export aus Datenbank
psql -d mydb -c "COPY users TO STDOUT CSV HEADER" > users.csv

# Zu JSON konvertieren
asp_cli convert users.csv --to json -o users.json
```

## Nächste Schritte

- Lernen Sie mehr über die einzelnen [Formate](/formats/comparison)
- Verstehen Sie die [Architektur](/architecture/overview)
- Entdecken Sie die [technischen Details](/learning/parsing)
