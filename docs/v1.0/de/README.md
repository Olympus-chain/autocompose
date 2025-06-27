# AutoCompose v1.0 Dokumentation

Willkommen bei AutoCompose v1.0! Dies ist die ursprüngliche Version, die grundlegende Funktionalität zum Generieren von Docker Compose-Dateien aus laufenden Containern bietet.

> **Hinweis:** Dies ist eine Legacy-Version. Für erweiterte Funktionen, bessere Leistung und verbesserte Container-Erkennung sollten Sie ein Upgrade auf [Version 1.5](../../v1.5/de/index.html) in Betracht ziehen.

## Was ist AutoCompose?

AutoCompose ist ein Kommandozeilen-Tool, das automatisch Docker Compose-Dateien aus Ihren laufenden Docker-Containern generiert. Es vereinfacht den Prozess der Erstellung von Compose-Dateien durch Extraktion der Konfiguration aus bestehenden Containern.

### Hauptfunktionen

- **Automatische Generierung:** Erstellt docker-compose.yml aus laufenden Containern
- **Basis-Konfiguration:** Extrahiert wesentliche Container-Einstellungen
- **Einfache CLI:** Benutzerfreundliche Kommandozeilen-Schnittstelle
- **YAML-Ausgabe:** Standard Docker Compose-Format

## Installation

### Anforderungen

- Linux oder macOS
- Docker Engine 19.03+
- Python 3.6+ oder vorkompilierte Binärdatei

### Binärdatei herunterladen

```bash
# v1.0 Release herunterladen
wget https://github.com/Olympus-chain/autocompose/releases/download/v1.0.0/autocompose
chmod +x autocompose
sudo mv autocompose /usr/local/bin/

# Installation überprüfen
autocompose --version
```

### Aus Quellcode installieren

```bash
# Repository klonen (v1.0 Tag)
git clone --branch v1.0.0 https://github.com/Olympus-chain/autocompose.git
cd autocompose-podman-docker

# Installieren
make install
```

## Schnellstart

### Einfaches Beispiel

Generieren Sie eine Compose-Datei aus allen laufenden Containern:

```bash
# docker-compose.yml generieren
autocompose

# Ausgabe in spezifische Datei
autocompose -o meine-compose.yml

# Ausgabe anzeigen ohne zu speichern
autocompose --stdout
```

### Was wird exportiert

AutoCompose v1.0 extrahiert folgende Informationen:

- Container-Image und Tag
- Container-Name
- Umgebungsvariablen
- Port-Mappings
- Volume-Mounts
- Neustart-Richtlinie
- Netzwerke (grundlegend)

## Grundlegende Verwendung

### Befehlssyntax

```bash
autocompose [OPTIONEN]

Wobei OPTIONEN sein können:
  -o, --output DATEI    Ausgabedatei (Standard: docker-compose.yml)
  --stdout              Auf stdout ausgeben statt in Datei
  -v, --version         Version anzeigen
  -h, --help            Hilfsnachricht anzeigen
```

### Einfacher Workflow

1. Starten Sie Ihre Container manuell mit docker run
2. Konfigurieren Sie sie nach Bedarf
3. Führen Sie autocompose aus, um die Compose-Datei zu generieren
4. Verwenden Sie die generierte Datei für zukünftige Deployments

```bash
# Beispiel: Container manuell ausführen
docker run -d --name web -p 80:80 nginx
docker run -d --name db -e MYSQL_ROOT_PASSWORD=geheim mysql:5.7

# Compose-Datei generieren
autocompose

# Ergebnis anzeigen
cat docker-compose.yml
```

## Befehlsoptionen

### Verfügbare Optionen

| Option | Beschreibung | Standard |
|--------|-------------|----------|
| `-o, --output` | Ausgabedateipfad | docker-compose.yml |
| `--stdout` | Auf Standardausgabe drucken | false |
| `-v, --version` | Versionsinformationen anzeigen | - |
| `-h, --help` | Hilfsnachricht anzeigen | - |

### Umgebungsvariablen

```bash
# Docker-Socket-Speicherort (falls nicht Standard)
export DOCKER_HOST=tcp://localhost:2375

# autocompose ausführen
autocompose
```

## Ausgabeformat

### Generierte Struktur

AutoCompose v1.0 generiert eine Standard Docker Compose v2 Datei:

```yaml
version: '2'
services:
  container_name:
    image: image:tag
    container_name: container_name
    environment:
      - ENV_VAR=wert
    ports:
      - "host:container"
    volumes:
      - /host/pfad:/container/pfad
    restart: richtlinie
```

### Beispielausgabe

Für eine einfache Webanwendung:

```yaml
version: '2'
services:
  webapp:
    image: nginx:latest
    container_name: webapp
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/www/html:/usr/share/nginx/html:ro
    restart: unless-stopped
    
  database:
    image: mysql:5.7
    container_name: database
    environment:
      - MYSQL_ROOT_PASSWORD=geheimpass
      - MYSQL_DATABASE=meineapp
    volumes:
      - /var/lib/mysql:/var/lib/mysql
    restart: always
```

## Beispiele

### Einfacher Webserver

```bash
# Nginx-Container ausführen
docker run -d \
  --name webserver \
  -p 8080:80 \
  -v ~/website:/usr/share/nginx/html:ro \
  nginx:alpine

# Compose-Datei generieren
autocompose -o webserver-compose.yml

# Ergebnis:
version: '2'
services:
  webserver:
    image: nginx:alpine
    container_name: webserver
    ports:
      - "8080:80"
    volumes:
      - ~/website:/usr/share/nginx/html:ro
```

### Datenbank-Container

```bash
# PostgreSQL ausführen
docker run -d \
  --name postgres-db \
  -e POSTGRES_PASSWORD=meinpasswort \
  -e POSTGRES_DB=meinedb \
  -v postgres-data:/var/lib/postgresql/data \
  -p 5432:5432 \
  postgres:12

# Compose-Datei generieren
autocompose

# Ergebnis enthält Umgebungsvariablen und Volumes
```

### Multi-Container-Setup

```bash
# Mehrere Container ausführen
docker run -d --name frontend -p 3000:3000 meine-app:frontend
docker run -d --name backend -p 5000:5000 --link frontend meine-app:backend
docker run -d --name cache redis:alpine

# Vollständige Compose-Datei generieren
autocompose -o vollstaendiger-stack.yml

# Erstellt Compose-Datei mit allen drei Services
```

## Fehlerbehebung

### Häufige Probleme

#### Keine Container gefunden

Stellen Sie sicher, dass der Docker-Daemon läuft und Container aktiv sind:

```bash
# Docker-Status prüfen
docker info

# Laufende Container auflisten
docker ps

# Falls keine Container laufen, zuerst welche starten
docker run -d nginx
```

#### Zugriff verweigert

Fügen Sie Ihren Benutzer zur Docker-Gruppe hinzu oder verwenden Sie sudo:

```bash
# Benutzer zur Docker-Gruppe hinzufügen
sudo usermod -aG docker $USER

# Oder mit sudo ausführen
sudo autocompose
```

#### Kann nicht mit Docker verbinden

Docker-Socket überprüfen:

```bash
# Standard-Socket-Speicherort
ls -la /var/run/docker.sock

# Bei Verwendung eines benutzerdefinierten Sockets
export DOCKER_HOST=unix:///pfad/zu/docker.sock
autocompose
```

### Hilfe erhalten

```bash
# Hilfe anzeigen
autocompose --help

# Version prüfen
autocompose --version

# Probleme melden
# https://github.com/Olympus-chain/autocompose/issues
```

## Einschränkungen

### Bekannte Einschränkungen in v1.0

- **Nur Docker:** Keine Podman-Unterstützung
- **Basis-Extraktion:** Begrenzte extrahierte Konfigurationsoptionen
- **Keine Filterung:** Exportiert alle laufenden Container
- **Netzwerke:** Nur grundlegende Netzwerkunterstützung
- **Compose-Version:** Unterstützt nur Version 2 Format
- **Keine Validierung:** Generierte Dateien werden nicht validiert

### Nicht enthaltene Funktionen

- Health Checks
- Ressourcenlimits
- Benutzerdefinierte Netzwerkkonfigurationen
- Sicherheitsoptionen
- Labels
- Logging-Konfiguration

> **Upgrade-Empfehlung:** Für diese Funktionen und mehr upgraden Sie bitte auf [AutoCompose v1.5](../../v1.5/de/index.html), das umfassende Container-Erkennung, erweiterte Filterung, Validierung und Unterstützung für Docker und Podman enthält.

### Manuelle Anpassungen

Nach der Generierung müssen Sie möglicherweise die Compose-Datei manuell bearbeiten, um:

- Fehlende Konfigurationsoptionen hinzuzufügen
- Benutzerdefinierte Netzwerke zu definieren
- Ressourcenbeschränkungen festzulegen
- Health Checks hinzuzufügen
- Logging zu konfigurieren