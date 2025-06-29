# Documentation AutoCompose v1.0

Bienvenue dans AutoCompose v1.0 ! Il s'agit de la version originale qui fournit les fonctionnalités de base pour générer des fichiers Docker Compose à partir de conteneurs en cours d'exécution.

> **Note :** Ceci est une version héritée. Pour des fonctionnalités améliorées, de meilleures performances et une détection de conteneurs améliorée, veuillez envisager de passer à la [version 1.5](../../v1.5/fr/index.html).

## Qu'est-ce qu'AutoCompose ?

AutoCompose est un outil en ligne de commande qui génère automatiquement des fichiers Docker Compose à partir de vos conteneurs Docker en cours d'exécution. Il simplifie le processus de création de fichiers compose en extrayant la configuration des conteneurs existants.

### Fonctionnalités Clés

- **Génération Automatique :** Crée docker-compose.yml depuis les conteneurs en cours
- **Configuration de Base :** Extrait les paramètres essentiels des conteneurs
- **CLI Simple :** Interface en ligne de commande facile à utiliser
- **Sortie YAML :** Format Docker Compose standard

## Installation

### Prérequis

- Linux ou macOS
- Docker Engine 19.03+
- Python 3.6+ ou binaire pré-compilé

### Télécharger le Binaire

```bash
# Télécharger la version v1.0
wget https://github.com/Olympus-chain/autocompose/releases/download/v1.0.0/autocompose
chmod +x autocompose
sudo mv autocompose /usr/local/bin/

# Vérifier l'installation
autocompose --version
```

### Installer depuis les Sources

```bash
# Cloner le dépôt (tag v1.0)
git clone --branch v1.0.0 https://github.com/Olympus-chain/autocompose.git
cd autocompose-podman-docker

# Installer
make install
```

## Démarrage Rapide

### Exemple de Base

Générer un fichier compose depuis tous les conteneurs en cours :

```bash
# Générer docker-compose.yml
autocompose

# Sortie vers un fichier spécifique
autocompose -o mon-compose.yml

# Afficher la sortie sans sauvegarder
autocompose --stdout
```

### Ce qui est Exporté

AutoCompose v1.0 extrait les informations suivantes :

- Image et tag du conteneur
- Nom du conteneur
- Variables d'environnement
- Mappages de ports
- Montages de volumes
- Politique de redémarrage
- Réseaux (basique)

## Utilisation de Base

### Syntaxe de Commande

```bash
autocompose [OPTIONS]

Où OPTIONS peut être :
  -o, --output FICHIER    Fichier de sortie (défaut : docker-compose.yml)
  --stdout                Afficher sur stdout au lieu d'un fichier
  -v, --version           Afficher la version
  -h, --help              Afficher le message d'aide
```

### Flux de Travail Simple

1. Démarrez vos conteneurs manuellement avec docker run
2. Configurez-les selon vos besoins
3. Exécutez autocompose pour générer le fichier compose
4. Utilisez le fichier généré pour les déploiements futurs

```bash
# Exemple : Exécuter des conteneurs manuellement
docker run -d --name web -p 80:80 nginx
docker run -d --name db -e MYSQL_ROOT_PASSWORD=secret mysql:5.7

# Générer le fichier compose
autocompose

# Voir le résultat
cat docker-compose.yml
```

## Options de Commande

### Options Disponibles

| Option | Description | Défaut |
|--------|-------------|---------|
| `-o, --output` | Chemin du fichier de sortie | docker-compose.yml |
| `--stdout` | Afficher sur la sortie standard | false |
| `-v, --version` | Afficher les informations de version | - |
| `-h, --help` | Afficher le message d'aide | - |

### Variables d'Environnement

```bash
# Emplacement du socket Docker (si non standard)
export DOCKER_HOST=tcp://localhost:2375

# Exécuter autocompose
autocompose
```

## Format de Sortie

### Structure Générée

AutoCompose v1.0 génère un fichier Docker Compose v2 standard :

```yaml
version: '2'
services:
  nom_conteneur:
    image: image:tag
    container_name: nom_conteneur
    environment:
      - ENV_VAR=valeur
    ports:
      - "hote:conteneur"
    volumes:
      - /chemin/hote:/chemin/conteneur
    restart: politique
```

### Exemple de Sortie

Pour une application web simple :

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
      - MYSQL_ROOT_PASSWORD=motdepasse
      - MYSQL_DATABASE=monapp
    volumes:
      - /var/lib/mysql:/var/lib/mysql
    restart: always
```

## Exemples

### Serveur Web Simple

```bash
# Exécuter un conteneur nginx
docker run -d \
  --name serveurweb \
  -p 8080:80 \
  -v ~/site:/usr/share/nginx/html:ro \
  nginx:alpine

# Générer le fichier compose
autocompose -o serveurweb-compose.yml

# Résultat :
version: '2'
services:
  serveurweb:
    image: nginx:alpine
    container_name: serveurweb
    ports:
      - "8080:80"
    volumes:
      - ~/site:/usr/share/nginx/html:ro
```

### Conteneur de Base de Données

```bash
# Exécuter PostgreSQL
docker run -d \
  --name postgres-db \
  -e POSTGRES_PASSWORD=monmotdepasse \
  -e POSTGRES_DB=madb \
  -v postgres-data:/var/lib/postgresql/data \
  -p 5432:5432 \
  postgres:12

# Générer le fichier compose
autocompose

# Le résultat inclut les variables d'environnement et les volumes
```

### Configuration Multi-Conteneurs

```bash
# Exécuter plusieurs conteneurs
docker run -d --name frontend -p 3000:3000 mon-app:frontend
docker run -d --name backend -p 5000:5000 --link frontend mon-app:backend
docker run -d --name cache redis:alpine

# Générer le fichier compose complet
autocompose -o stack-complet.yml

# Crée un fichier compose avec les trois services
```

## Dépannage

### Problèmes Courants

#### Aucun conteneur trouvé

Assurez-vous que le daemon Docker est en cours d'exécution et que les conteneurs sont actifs :

```bash
# Vérifier le statut Docker
docker info

# Lister les conteneurs en cours
docker ps

# Si aucun conteneur n'est en cours, en démarrer d'abord
docker run -d nginx
```

#### Permission refusée

Ajoutez votre utilisateur au groupe docker ou utilisez sudo :

```bash
# Ajouter l'utilisateur au groupe docker
sudo usermod -aG docker $USER

# Ou exécuter avec sudo
sudo autocompose
```

#### Impossible de se connecter à Docker

Vérifiez le socket Docker :

```bash
# Emplacement du socket par défaut
ls -la /var/run/docker.sock

# Si utilisation d'un socket personnalisé
export DOCKER_HOST=unix:///chemin/vers/docker.sock
autocompose
```

### Obtenir de l'Aide

```bash
# Afficher l'aide
autocompose --help

# Vérifier la version
autocompose --version

# Signaler des problèmes
# https://github.com/Olympus-chain/autocompose/issues
```

## Limitations

### Limitations Connues dans v1.0

- **Docker Uniquement :** Pas de support Podman
- **Extraction de Base :** Options de configuration limitées extraites
- **Pas de Filtrage :** Exporte tous les conteneurs en cours
- **Réseaux :** Support réseau basique uniquement
- **Version Compose :** Supporte uniquement le format version 2
- **Pas de Validation :** Les fichiers générés ne sont pas validés

### Fonctionnalités Non Incluses

- Health checks
- Limites de ressources
- Configurations réseau personnalisées
- Options de sécurité
- Labels
- Configuration de journalisation

> **Recommandation de Mise à Niveau :** Pour ces fonctionnalités et plus encore, veuillez passer à [AutoCompose v1.5](../../v1.5/fr/index.html) qui inclut une détection complète des conteneurs, un filtrage avancé, la validation et le support pour Docker et Podman.

### Ajustements Manuels

Après génération, vous devrez peut-être éditer manuellement le fichier compose pour :

- Ajouter des options de configuration manquantes
- Définir des réseaux personnalisés
- Définir des contraintes de ressources
- Ajouter des health checks
- Configurer la journalisation