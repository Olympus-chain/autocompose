# AutoCompose v1.5 Dokumentation

Willkommen bei AutoCompose v1.5! Diese verbesserte Version bietet erweiterte Docker- und Podman-Unterstützung, fortschrittliche Filteroptionen und Leistungsoptimierungen.

> **Highlights der Version 1.5:** Verbesserte Container-Erkennung, optimierte Netzwerkkonfigurationsextraktion, besseres Volume-Mapping und optimierte Leistung für große Deployments.

## Was ist AutoCompose?

AutoCompose ist ein leistungsstarkes Kommandozeilen-Tool, das automatisch Docker Compose-Dateien aus Ihren laufenden Containern generiert. Es vereinfacht den Prozess der Konvertierung bestehender Container-Deployments in reproduzierbare, versionskontrollierte Konfigurationen.

### Hauptvorteile

- **Automatisierung:** Kein manuelles YAML-Schreiben - automatische Extraktion von Konfigurationen
- **Genauigkeit:** Erfasst exakte Container-Konfigurationen einschließlich Netzwerke, Volumes und Umgebungsvariablen
- **Flexibilität:** Unterstützung für Docker- und Podman-Umgebungen
- **Intelligenz:** Intelligente Filterung und Validierung für saubere, optimierte Ausgabe

## Installation

### Systemanforderungen

- Linux, macOS oder Windows (mit WSL2)
- Docker Engine 20.10+ oder Podman 3.0+
- Rust 1.65+ (zum Kompilieren aus dem Quellcode)

### Vorkompilierte Binärdateien

Laden Sie die neueste Version für Ihre Plattform herunter:

```bash
# Linux/macOS
curl -L https://github.com/Olympus-chain/autocompose/releases/latest/download/autocompose-linux-amd64 -o autocompose
chmod +x autocompose
sudo mv autocompose /usr/local/bin/

# Installation überprüfen
autocompose --version
```

### Aus dem Quellcode kompilieren

```bash
# Repository klonen
git clone https://github.com/Olympus-chain/autocompose.git
cd autocompose-podman-docker

# Mit Cargo kompilieren
cargo build --release

# Installieren
sudo cp target/release/autocompose /usr/local/bin/
```

### Docker-Installation

Sie können AutoCompose auch mit Docker ausführen:

```bash
# Alias für einfache Verwendung erstellen
alias autocompose='docker run --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/drasrax/autocompose:latest'

# Tool verwenden
autocompose --help
```

## Schnellstart

### Basis-Export

Generieren Sie eine Docker Compose-Datei aus allen laufenden Containern:

```bash
# Alle laufenden Container exportieren
autocompose

# In eine bestimmte Datei exportieren
autocompose -o mein-stack.yml

# Vorschau ohne Speichern
autocompose --dry-run
```

### Gefilterter Export

Exportieren Sie spezifische Container basierend auf Kriterien:

```bash
# Container exportieren, die einem Muster entsprechen
autocompose --filter "web-*"

# Nur laufende Container exportieren
autocompose --running-only

# System-Container ausschließen
autocompose --exclude-system
```

### Interaktiver Modus

Container interaktiv auswählen:

```bash
# Interaktive Auswahl starten
autocompose --interactive

# Mit Vorschau
autocompose --interactive --preview
```

## Funktionen

### Container-Erkennung

AutoCompose v1.5 bietet erweiterte Container-Erkennungsfunktionen:

- Automatische Erkennung von Docker- und Podman-Containern
- Unterstützung für rootless Podman-Deployments
- Erkennung von Container-Abhängigkeiten und -Verknüpfungen
- Intelligente Gruppierung verwandter Container

### Konfigurationsextraktion

Umfassende Extraktion von Container-Konfigurationen:

| Kategorie | Extrahierte Felder | v1.5 Verbesserungen |
|-----------|-------------------|---------------------|
| Basisinfo | Image, Name, Befehl, Arbeitsverzeichnis | Verbesserte Tag-Auflösung |
| Netzwerk | Ports, Netzwerke, Hostname, DNS | IPv6-Unterstützung, benutzerdefinierte Treiber |
| Speicher | Volumes, Bind Mounts, tmpfs | Volume-Treiber-Optionen |
| Laufzeit | Umgebung, Labels, Neustart-Richtlinie | Healthcheck-Konfigurationen |
| Sicherheit | Capabilities, Sicherheitsoptionen | SELinux-Kontexte, AppArmor |

### Ausgabeformate

Mehrere Ausgabeformate für verschiedene Anwendungsfälle:

```bash
# Standard YAML (Standard)
autocompose -o docker-compose.yml

# JSON-Format
autocompose --format json -o compose.json

# YAML mit spezifischer Version
autocompose --version 3.8 -o compose-v3.8.yml

# Kompakte Ausgabe
autocompose --compact -o minimal.yml
```

## Grundlegende Verwendung

### Befehlsstruktur

```bash
autocompose [OPTIONEN] [CONTAINER...]

OPTIONEN:
    -o, --output <DATEI>          Ausgabedatei (Standard: docker-compose.yml)
    -f, --filter <MUSTER>         Container nach Muster filtern (unterstützt * und ?)
    -v, --version <VER>           Compose-Dateiversion (Standard: 3.9)
    -i, --interactive             Interaktive Container-Auswahl
    --format <FORMAT>             Ausgabeformat: yaml, json, toml (Standard: yaml)
    --dry-run                     Vorschau ohne Datei zu schreiben
    --preview                     Vorschau anzeigen ohne zu schreiben
    --exclude <MUSTER>            Container nach Muster ausschließen
    --exclude-system              System-Container ausschließen
    --docker-host <URL>           Mit spezifischem Docker-Host verbinden
    --context <NAME>              Spezifischen Docker-Kontext verwenden
    --include-networks            Netzwerkdefinitionen einschließen
    --include-volumes             Volume-Definitionen einschließen
    --compact                     Kompakte Ausgabe generieren
    --debug                       Debug-Ausgabe aktivieren
    --verbose                     Verbosität erhöhen (-vv, -vvv)
    --strict                      Strikter Validierungsmodus
    --help                        Hilfeinformationen anzeigen
```

### Gängige Workflows

#### Entwicklungsumgebung

Exportieren Sie Ihren Entwicklungs-Stack:

```bash
# Dev-Container exportieren
autocompose --filter "dev-*" -o dev-compose.yml

# Nur spezifische Dienste einschließen
autocompose dev-web dev-db dev-redis -o dev-stack.yml

# Mit benutzerdefiniertem Netzwerk
autocompose --network dev-network -o dev-compose.yml
```

#### Produktionsmigration

Container für Produktions-Deployment vorbereiten:

```bash
# Export mit Produktionseinstellungen
autocompose --running-only \
  --exclude-system \
  --remove-caps \
  -o production-compose.yml

# Ausgabe validieren
autocompose validate production-compose.yml
```

## Docker-Befehle

### Docker-spezifische Optionen

```bash
# Mit Remote-Docker-Daemon verbinden
autocompose --docker-host tcp://remote:2375

# Spezifischen Docker-Kontext verwenden
autocompose --context production

# Docker-Labels einschließen
autocompose --include-labels

# Container-IDs beibehalten
autocompose --preserve-ids
```

### Netzwerkkonfiguration

Erweiterte Netzwerkextraktion für Docker:

```bash
# Benutzerdefinierte Netzwerke einschließen
autocompose --include-networks

# Netzwerk-Aliase zuordnen
autocompose --preserve-aliases

# Netzwerktreiber-Optionen einschließen
autocompose --network-details
```

### Volume-Verwaltung

```bash
# Benannte Volumes einschließen
autocompose --include-volumes

# Bind Mounts in Volumes konvertieren
autocompose --convert-mounts

# Volume-Treiber-Optionen einschließen
autocompose --volume-details
```

## Podman-Befehle

### Podman-spezifische Funktionen

AutoCompose v1.5 enthält erweiterte Podman-Unterstützung:

```bash
# Rootless Podman
autocompose --podman-rootless

# Pod-Konfigurationen einschließen
autocompose --include-pods

# SystemD-Integration
autocompose --systemd-compatible

# SELinux-Labels
autocompose --preserve-selinux
```

### Pod-Verwaltung

```bash
# Ganze Pods exportieren
autocompose --pod meine-app-pod

# Nach Pods gruppieren
autocompose --group-by-pod

# Infra-Container einschließen
autocompose --include-infra
```

## Filteroptionen

### Namensbasierte Filterung

```bash
# Wildcard-Muster
autocompose --filter "app-*"

# Reguläre Ausdrücke
autocompose --filter-regex "^(web|api)-.*"

# Mehrere Filter
autocompose --filter "web-*" --filter "api-*"

# Ausschlussmuster
autocompose --exclude "test-*" --exclude "*-temp"
```

### Zustandsbasierte Filterung

```bash
# Nur laufende Container
autocompose --running-only

# Gestoppte Container einschließen
autocompose --all

# Nach Container-Zustand
autocompose --state running,paused

# Nach Gesundheitsstatus
autocompose --health healthy
```

### Label-basierte Filterung

```bash
# Nach Label filtern
autocompose --label-filter "environment=production"

# Mehrere Labels (UND)
autocompose --label-filter "app=myapp" --label-filter "tier=frontend"

# Label existiert
autocompose --has-label "backup"

# Label-Muster
autocompose --label-regex "version=2\.*"
```

## Konfiguration

### Konfigurationsdatei

AutoCompose unterstützt Konfigurationsdateien für persistente Einstellungen:

```bash
# Standardkonfiguration erstellen
autocompose config init

# Speicherort: ~/.autocompose/config.yml
# Mit bevorzugten Einstellungen bearbeiten
```

### Konfigurationsoptionen

```yaml
# config.yml Beispiel
defaults:
  output: docker-compose.yml
  format: yaml
  compose_version: "3.9"
  
filters:
  exclude_system: true
  exclude_patterns:
    - "k8s_*"
    - "*_test"
  
docker:
  socket: /var/run/docker.sock
  timeout: 30
  
podman:
  socket: /run/user/1000/podman/podman.sock
  rootless: true
  
output:
  compact: false
  sort_services: true
  include_timestamps: false
```

### Umgebungsvariablen

```bash
# Konfigurationsdatei überschreiben
export AUTOCOMPOSE_CONFIG=/pfad/zu/config.yml

# Docker-Socket
export DOCKER_HOST=tcp://localhost:2375

# Ausgabeverzeichnis
export AUTOCOMPOSE_OUTPUT_DIR=/compose-files

# Log-Level
export AUTOCOMPOSE_LOG_LEVEL=debug
```

## Validierung

### Integrierte Validierung

AutoCompose v1.5 enthält umfassende Validierung:

```bash
# Generierte Datei validieren
autocompose validate docker-compose.yml

# Mit spezifischer Version validieren
autocompose validate --version 3.8 docker-compose.yml

# Strenge Validierung
autocompose validate --strict docker-compose.yml

# Auf Sicherheitsprobleme prüfen
autocompose validate --security docker-compose.yml
```

### Validierungsprüfungen

- **Syntax:** YAML/JSON-Syntaxvalidierung
- **Schema:** Compose-Datei-Schema-Konformität
- **Referenzen:** Netzwerk-, Volume- und Service-Referenzen
- **Sicherheit:** Privilegierte Container, Capabilities, Bind Mounts
- **Best Practices:** Ressourcenlimits, Health Checks, Neustart-Richtlinien

### Validierungsausgabe

```
# Beispiel-Validierungsausgabe
✓ Syntax gültig
✓ Schema-konform (Version 3.8)
⚠ Warnung: Service 'web' verwendet Tag 'latest'
⚠ Warnung: Service 'db' fehlt Health Check
✗ Fehler: Netzwerk 'frontend' referenziert aber nicht definiert
✗ Fehler: Volume 'data' hat ungültige Treiber-Optionen

Zusammenfassung: 2 Fehler, 2 Warnungen
```

## Best Practices

### Sicherheitsempfehlungen

- Generierte Dateien immer vor dem Deployment überprüfen
- Unnötige Privilegien und Capabilities entfernen
- Spezifische Image-Tags statt 'latest' verwenden
- Ordnungsgemäßes Secret-Management implementieren
- Angemessene Ressourcenlimits festlegen

### Performance-Tipps

- `--running-only` für schnellere Verarbeitung verwenden
- Container filtern, um Verarbeitungszeit zu reduzieren
- Caching für wiederholte Exporte aktivieren
- Kompaktmodus für kleinere Dateien verwenden

### Wartung

- Compose-Dateien versionskontrollieren
- Benutzerdefinierte Änderungen dokumentieren
- Regelmäßige Validierung von Compose-Dateien
- AutoCompose aktuell halten

## Fehlerbehebung

### Häufige Probleme

#### Verbindungsfehler

```bash
# Docker-Daemon prüfen
docker info

# Socket-Berechtigungen prüfen
ls -la /var/run/docker.sock

# Bei Bedarf sudo verwenden
sudo autocompose

# Socket explizit angeben
autocompose --docker-socket /var/run/docker.sock
```

#### Zugriff verweigert

```bash
# Benutzer zur Docker-Gruppe hinzufügen
sudo usermod -aG docker $USER

# Abmelden und erneut anmelden
# Oder newgrp verwenden
newgrp docker
```

#### Leere Ausgabe

```bash
# Prüfen, ob Container laufen
docker ps

# Alle Container einschließen
autocompose --all

# Filter prüfen
autocompose --no-filters

# Debug-Logs aktivieren
AUTOCOMPOSE_LOG_LEVEL=debug autocompose
```

### Debug-Modus

```bash
# Debug-Ausgabe aktivieren
autocompose --debug

# Ausführliche Protokollierung
autocompose -vvv

# Dry Run mit Debug
autocompose --dry-run --debug

# Debug-Informationen exportieren
autocompose debug-info > debug.txt
```

## CLI-Referenz

### Globale Optionen

| Option | Kurz | Beschreibung | Standard |
|--------|------|--------------|----------|
| `--output` | `-o` | Ausgabedateipfad | docker-compose.yml |
| `--filter` | `-f` | Nach Muster filtern (unterstützt * und ?) | |
| `--version` | `-v` | Compose-Dateiversion | 3.9 |
| `--interactive` | `-i` | Interaktive Auswahl | false |
| `--format` | | Ausgabeformat (yaml/json/toml) | yaml |
| `--dry-run` | | Vorschau ohne Schreiben | false |
| `--preview` | | Vorschau anzeigen ohne zu schreiben | false |
| `--compact` | | Kompakte Ausgabe | false |
| `--debug` | | Debug-Modus | false |
| `--verbose` | | Verbosität (-v, -vv, -vvv) | 0 |
| `--help` | `-h` | Hilfenachricht anzeigen | |
| `--version` | | Tool-Version anzeigen | |

### Filteroptionen

| Option | Beschreibung | Beispiel |
|--------|--------------|----------|
| `--filter` | Nach Namensmuster filtern | `--filter "web-*"` |
| `--exclude` | Nach Muster ausschließen | `--exclude "*-test"` |
| `--exclude-system` | System-Container ausschließen | `--exclude-system` |
| `--running-only` | Nur laufende Container | `--running-only` |
| `--all` | Gestoppte Container einschließen | `--all` |
| `--label-filter` | Nach Label filtern | `--label-filter "env=prod"` |
| `--has-label` | Container mit Label | `--has-label "backup"` |
| `--state` | Nach Status filtern | `--state running` |

### Docker-Optionen

| Option | Beschreibung | Beispiel |
|--------|--------------|----------|
| `--docker-host` | Docker-Host-URL | `--docker-host tcp://remote:2375` |
| `--context` | Docker-Kontext | `--context production` |
| `--include-networks` | Netzwerkdefinitionen einschließen | `--include-networks` |
| `--include-volumes` | Volume-Definitionen einschließen | `--include-volumes` |

## API-Referenz

### Bibliotheksnutzung

AutoCompose kann als Bibliothek in Rust-Projekten verwendet werden:

```rust
// Cargo.toml
[dependencies]
autocompose = "1.5"

// main.rs
use autocompose::{AutoCompose, Config, FilterOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let filter = FilterOptions::new()
        .running_only(true)
        .exclude_pattern("test-*");
    
    let compose = AutoCompose::new(config)
        .with_filter(filter)
        .generate()?;
    
    println!("{}", compose.to_yaml()?);
    Ok(())
}
```

### Haupttypen

```rust
// Konfiguration
pub struct Config {
    pub docker_socket: String,
    pub compose_version: String,
    pub output_format: Format,
}

// Filteroptionen
pub struct FilterOptions {
    pub patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub running_only: bool,
    pub labels: HashMap<String, String>,
}

// Generierte Compose-Datei
pub struct ComposeFile {
    pub version: String,
    pub services: HashMap<String, Service>,
    pub networks: Option<HashMap<String, Network>>,
    pub volumes: Option<HashMap<String, Volume>>,
}
```

## Beispiele

### Multi-Service-Anwendung

```bash
# Laufende Container:
# - webapp (nginx)
# - api (node:14)
# - database (postgres:13)
# - cache (redis:6)

# Vollständigen Stack exportieren
autocompose -o fullstack.yml

# Ergebnis:
```

```yaml
version: '3.9'
services:
  webapp:
    image: nginx:latest
    ports:
      - "80:80"
    volumes:
      - ./html:/usr/share/nginx/html
    networks:
      - frontend
    
  api:
    image: node:14
    working_dir: /app
    command: npm start
    environment:
      NODE_ENV: production
      DB_HOST: database
    ports:
      - "3000:3000"
    networks:
      - frontend
      - backend
    
  database:
    image: postgres:13
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: user
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - db-data:/var/lib/postgresql/data
    networks:
      - backend
    
  cache:
    image: redis:6
    command: redis-server --appendonly yes
    volumes:
      - cache-data:/data
    networks:
      - backend

networks:
  frontend:
  backend:

volumes:
  db-data:
  cache-data:
```

### Entwicklungsumgebung

```bash
# Export mit Entwicklungs-Overrides
autocompose \
  --filter "dev-*" \
  --include-labels \
  --preserve-mounts \
  -o docker-compose.dev.yml

# Ergebnis enthält:
# - Source-Code Bind Mounts
# - Entwicklungs-Umgebungsvariablen
# - Freigegebene Debug-Ports
# - Keine Neustart-Richtlinien
```

### Produktions-Deployment

```bash
# Strikter Produktions-Export
autocompose \
  --running-only \
  --exclude-system \
  --remove-caps \
  --add-healthchecks \
  --resource-limits \
  -o docker-compose.prod.yml

# Enthält:
# - Health Checks für alle Services
# - Ressourcenlimits (CPU/Memory)
# - Neustart-Richtlinien
# - Keine privilegierten Container
# - Spezifische Image-Tags
```

## Änderungsprotokoll

### Version 1.5.0 (Aktuell)

- **Verbesserte Erkennung:** Bessere Erkennung von Container-Beziehungen und Abhängigkeiten
- **Erweiterte Filterung:** Neue Regex-Filter, Label-basierte Filterung und Zustandsfilterung
- **Podman-Verbesserungen:** Bessere Rootless-Unterstützung, Pod-Konfigurationen, SystemD-Integration
- **Netzwerk-Verbesserungen:** IPv6-Unterstützung, benutzerdefinierte Netzwerktreiber, Alias-Erhaltung
- **Leistung:** 3x schneller für große Deployments, parallele Container-Inspektion
- **Validierung:** Umfassende Validierung mit Sicherheitsprüfungen und Best-Practice-Empfehlungen
- **Ausgabeformate:** JSON-Ausgabe hinzugefügt, Kompaktmodus, sortierte Services

### Version 1.0.0

- Erstveröffentlichung
- Grundlegende Docker-Unterstützung
- YAML-Ausgabe
- Einfache Filterung

## Roadmap

Unsere Vision für AutoCompose ist es, das umfassendste und benutzerfreundlichste Tool für die Container-zu-Compose-Konvertierung zu werden. Hier ist, was als Nächstes kommt:

### Version 1.6 - Q3 2025

#### Kernverbesserungen

**🔗 Container-Abhängigkeiten**
- Automatische Erkennung von `depends_on`-Beziehungen
- Health-Check-basierte Abhängigkeiten
- Startreihenfolge-Analyse

**💾 Volume-Verwaltung**
- Ordnungsgemäße Volume-Definitionen
- Volume-Treiber-Optionen
- Backup-Empfehlungen

**🔍 Erweiterte Filterung**
- Label-basierte Filterung
- Zeitbasierte Filterung
- Ressourcenbasierte Filterung

**🌐 Netzwerk-Verbesserungen**
- Unterstützung externer Netzwerke
- Erweiterte Treiber-Optionen
- IPv6-Verbesserungen

### Version 1.7 - Q4 2025

#### Erweiterte Funktionen

**☸️ Kubernetes-Integration**
- Pod-zu-Compose-Konvertierung
- ConfigMaps & Secrets-Unterstützung
- Basis-Helm-Chart-Konvertierung

**📋 Mehrstufiges Compose**
- Umgebungsspezifische Dateien
- Override-Verwaltung
- Datei-Zusammenführungsfunktionen

**🔨 Build-Kontext-Unterstützung**
- Dockerfile-Erkennung
- Build-Argumente
- Mehrstufige Builds

**🖥️ Web-Oberfläche**
- Interaktive Web-UI
- Visueller Abhängigkeits-Editor
- Echtzeit-Vorschau

### Version 2.0 - Q1 2026

#### Enterprise-Funktionen

**🐝 Swarm-Modus**
- Stack-Datei-Generierung
- Platzierungsbeschränkungen
- Service-Replikate

**🔄 Bidirektionale Synchronisation**
- Compose zu Containern
- Live-Diff-Erkennung
- Update-Weitergabe

**🚀 CI/CD-Integration**
- GitHub Actions
- GitLab CI-Vorlagen
- Jenkins-Plugins

**📊 Überwachung**
- Prometheus-Labels
- Überwachungs-Overlays
- Alert-Generierung

### Version 2.1+ - 2026+

#### Zukunftsvision

**🤖 KI-gestützt**
- ML-Optimierung
- Prädiktive Skalierung
- Anomalieerkennung

**☁️ Multi-Cloud**
- AWS ECS-Unterstützung
- Azure Container Instances
- Google Cloud Run

**🔌 Plugin-System**
- Drittanbieter-Erweiterungen
- Plugin-Marktplatz
- Benutzerdefinierte Prozessoren

**🛠️ Entwickler-Tools**
- IDE-Integrationen
- Echtzeit-Linting
- Intelligente Vervollständigung

### Möchten Sie beitragen?

Wir begrüßen Beiträge! Prioritätsbereiche umfassen:

- Container-Abhängigkeitserkennungsalgorithmen
- Cloud-Anbieter-Integrationen
- Dokumentation und Beispiele
- Leistungsoptimierungen

[🤝 Beitragsleitfaden](https://github.com/Olympus-chain/autocompose/blob/main/CONTRIBUTING.md)