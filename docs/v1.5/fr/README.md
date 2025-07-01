# Documentation AutoCompose v1.5

Bienvenue dans AutoCompose v1.5 ! Cette version améliorée apporte un meilleur support Docker et Podman, des options de filtrage avancées et des optimisations de performance.

> **Points forts de la version 1.5 :** Détection améliorée des conteneurs, extraction optimisée de la configuration réseau, meilleur mapping des volumes, et performances optimisées pour les déploiements importants.

## Qu'est-ce qu'AutoCompose ?

AutoCompose est un outil en ligne de commande puissant qui génère automatiquement des fichiers Docker Compose à partir de vos conteneurs en cours d'exécution. Il simplifie le processus de conversion des déploiements de conteneurs existants en configurations reproductibles et versionnées.

### Avantages Clés

- **Automatisation :** Pas d'écriture manuelle de YAML - extraction automatique des configurations
- **Précision :** Capture les configurations exactes des conteneurs incluant réseaux, volumes et variables d'environnement
- **Flexibilité :** Support pour les environnements Docker et Podman
- **Intelligence :** Filtrage intelligent et validation pour garantir une sortie propre et optimisée

## Installation

### Configuration Requise

- Linux, macOS, ou Windows (avec WSL2)
- Docker Engine 20.10+ ou Podman 3.0+
- Rust 1.65+ (pour compiler depuis les sources)

### Binaires Pré-compilés

Téléchargez la dernière version pour votre plateforme :

```bash
# Linux/macOS
curl -L https://github.com/Olympus-chain/autocompose/releases/latest/download/autocompose-linux-amd64 -o autocompose
chmod +x autocompose
sudo mv autocompose /usr/local/bin/

# Vérifier l'installation
autocompose --version
```

### Compilation depuis les Sources

```bash
# Cloner le dépôt
git clone https://github.com/Olympus-chain/autocompose.git
cd autocompose-podman-docker

# Compiler avec Cargo
cargo build --release

# Installer
sudo cp target/release/autocompose /usr/local/bin/
```

### Installation Docker

Vous pouvez également exécuter AutoCompose avec Docker :

```bash
# Créer un alias pour une utilisation facile
alias autocompose='docker run --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/drasrax/autocompose:latest'

# Utiliser l'outil
autocompose --help
```

## Démarrage Rapide

### Export de Base

Générer un fichier Docker Compose depuis tous les conteneurs en cours :

```bash
# Exporter tous les conteneurs en cours
autocompose

# Exporter vers un fichier spécifique
autocompose -o ma-stack.yml

# Prévisualiser sans sauvegarder
autocompose --dry-run
```

### Export Filtré

Exporter des conteneurs spécifiques selon des critères :

```bash
# Exporter les conteneurs correspondant à un motif
autocompose --filter "web-*"

# Exporter uniquement les conteneurs en cours
autocompose --running-only

# Exclure les conteneurs système
autocompose --exclude-system
```

### Mode Interactif

Sélectionner les conteneurs de manière interactive :

```bash
# Lancer la sélection interactive
autocompose --interactive

# Avec prévisualisation
autocompose --interactive --preview
```

## Fonctionnalités

### Détection des Conteneurs

AutoCompose v1.5 dispose de capacités de détection améliorées :

- Détection automatique des conteneurs Docker et Podman
- Support des déploiements Podman rootless
- Détection des dépendances et liens entre conteneurs
- Regroupement intelligent des conteneurs liés

### Extraction de Configuration

Extraction complète des configurations de conteneurs :

| Catégorie | Champs Extraits | Améliorations v1.5 |
|-----------|-----------------|-------------------|
| Infos de Base | Image, nom, commande, répertoire de travail | Résolution améliorée des tags |
| Réseau | Ports, réseaux, hostname, DNS | Support IPv6, drivers personnalisés |
| Stockage | Volumes, bind mounts, tmpfs | Options de driver de volume |
| Runtime | Environnement, labels, politique de redémarrage | Configurations de healthcheck |
| Sécurité | Capacités, options de sécurité | Contextes SELinux, AppArmor |

### Formats de Sortie

Multiples formats de sortie pour différents cas d'usage :

```bash
# YAML standard (par défaut)
autocompose -o docker-compose.yml

# Format JSON
autocompose --format json -o compose.json

# YAML avec version spécifique
autocompose --version 3.8 -o compose-v3.8.yml

# Sortie compacte
autocompose --compact -o minimal.yml
```

## Utilisation de Base

### Structure de Commande

```bash
autocompose [OPTIONS] [CONTENEURS...]

OPTIONS:
    -o, --output <FICHIER>        Fichier de sortie (défaut: docker-compose.yml)
    -f, --filter <MOTIF>          Filtrer conteneurs par motif (supporte * et ?)
    -v, --version <VER>           Version du fichier compose (défaut: 3.9)
    -i, --interactive             Sélection interactive des conteneurs
    --format <FORMAT>             Format de sortie: yaml, json, toml (défaut: yaml)
    --dry-run                     Prévisualiser sans écrire le fichier
    --preview                     Afficher l'aperçu sans écrire
    --exclude <MOTIF>             Exclure conteneurs par motif
    --exclude-system              Exclure les conteneurs système
    --docker-host <URL>           Se connecter à un hôte Docker spécifique
    --context <NOM>               Utiliser le contexte Docker spécifique
    --include-networks            Inclure les définitions réseau
    --include-volumes             Inclure les définitions de volume
    --compact                     Générer une sortie compacte
    --debug                       Activer la sortie de débogage
    --verbose                     Augmenter la verbosité (-vv, -vvv)
    --strict                      Mode de validation strict
    --help                        Afficher l'aide
```

### Workflows Courants

#### Environnement de Développement

Exporter votre stack de développement :

```bash
# Exporter les conteneurs de dev
autocompose --filter "dev-*" -o dev-compose.yml

# Inclure seulement des services spécifiques
autocompose dev-web dev-db dev-redis -o dev-stack.yml

# Avec réseau personnalisé
autocompose --network dev-network -o dev-compose.yml
```

#### Migration Production

Préparer les conteneurs pour un déploiement en production :

```bash
# Exporter avec paramètres de production
autocompose --running-only \
  --exclude-system \
  --remove-caps \
  -o production-compose.yml

# Valider la sortie
autocompose validate production-compose.yml
```

## Commandes Docker

### Options Spécifiques Docker

```bash
# Se connecter à un daemon Docker distant
autocompose --docker-host tcp://remote:2375

# Utiliser un contexte Docker spécifique
autocompose --context production

# Inclure les labels Docker
autocompose --include-labels

# Préserver les IDs de conteneurs
autocompose --preserve-ids
```

### Configuration Réseau

Extraction réseau avancée pour Docker :

```bash
# Inclure les réseaux personnalisés
autocompose --include-networks

# Mapper les alias réseau
autocompose --preserve-aliases

# Inclure les options de driver réseau
autocompose --network-details
```

### Gestion des Volumes

```bash
# Inclure les volumes nommés
autocompose --include-volumes

# Convertir les bind mounts en volumes
autocompose --convert-mounts

# Inclure les options de driver de volume
autocompose --volume-details
```

## Commandes Podman

### Fonctionnalités Spécifiques Podman

AutoCompose v1.5 inclut un support Podman amélioré :

```bash
# Podman rootless
autocompose --podman-rootless

# Inclure les configurations de pods
autocompose --include-pods

# Intégration SystemD
autocompose --systemd-compatible

# Labels SELinux
autocompose --preserve-selinux
```

### Gestion des Pods

```bash
# Exporter des pods entiers
autocompose --pod mon-app-pod

# Grouper par pods
autocompose --group-by-pod

# Inclure les conteneurs infra
autocompose --include-infra
```

## Options de Filtrage

### Filtrage par Nom

```bash
# Motifs avec jokers
autocompose --filter "app-*"

# Expressions régulières
autocompose --filter-regex "^(web|api)-.*"

# Filtres multiples
autocompose --filter "web-*" --filter "api-*"

# Motifs d'exclusion
autocompose --exclude "test-*" --exclude "*-temp"
```

### Filtrage par État

```bash
# Seulement les conteneurs en cours
autocompose --running-only

# Inclure les conteneurs arrêtés
autocompose --all

# Par état du conteneur
autocompose --state running,paused

# Par statut de santé
autocompose --health healthy
```

### Filtrage par Label

```bash
# Filtrer par label
autocompose --label-filter "environment=production"

# Labels multiples (ET)
autocompose --label-filter "app=myapp" --label-filter "tier=frontend"

# Label existe
autocompose --has-label "backup"

# Motif de label
autocompose --label-regex "version=2\.*"
```

## Configuration

### Fichier de Configuration

AutoCompose supporte les fichiers de configuration pour des paramètres persistants :

```bash
# Créer la configuration par défaut
autocompose config init

# Emplacement : ~/.autocompose/config.yml
# Éditer avec vos paramètres préférés
```

### Options de Configuration

```yaml
# Exemple config.yml
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

### Variables d'Environnement

```bash
# Surcharger le fichier de config
export AUTOCOMPOSE_CONFIG=/chemin/vers/config.yml

# Socket Docker
export DOCKER_HOST=tcp://localhost:2375

# Répertoire de sortie
export AUTOCOMPOSE_OUTPUT_DIR=/compose-files

# Niveau de log
export AUTOCOMPOSE_LOG_LEVEL=debug
```

## Validation

### Validation Intégrée

AutoCompose v1.5 inclut une validation complète :

```bash
# Valider le fichier généré
autocompose validate docker-compose.yml

# Valider avec version spécifique
autocompose validate --version 3.8 docker-compose.yml

# Validation stricte
autocompose validate --strict docker-compose.yml

# Vérifier les problèmes de sécurité
autocompose validate --security docker-compose.yml
```

### Vérifications de Validation

- **Syntaxe :** Validation de syntaxe YAML/JSON
- **Schéma :** Conformité au schéma du fichier compose
- **Références :** Références réseau, volume et service
- **Sécurité :** Conteneurs privilégiés, capacités, bind mounts
- **Bonnes Pratiques :** Limites de ressources, health checks, politiques de redémarrage

### Sortie de Validation

```
# Exemple de sortie de validation
✓ Syntaxe valide
✓ Conforme au schéma (version 3.9)
⚠ Avertissement : Le service 'web' utilise le tag 'latest'
⚠ Avertissement : Le service 'db' n'a pas de health check
✗ Erreur : Le réseau 'frontend' est référencé mais non défini
✗ Erreur : Le volume 'data' a des options de driver invalides

Résumé : 2 erreurs, 2 avertissements
```

## Bonnes Pratiques

### Recommandations de Sécurité

- Toujours vérifier les fichiers générés avant le déploiement
- Supprimer les privilèges et capacités inutiles
- Utiliser des tags d'image spécifiques au lieu de 'latest'
- Implémenter une gestion appropriée des secrets
- Définir des limites de ressources appropriées

### Conseils de Performance

- Utiliser `--running-only` pour un traitement plus rapide
- Filtrer les conteneurs pour réduire le temps de traitement
- Activer le cache pour les exports répétés
- Utiliser le mode compact pour des fichiers plus petits

### Maintenance

- Versionner vos fichiers compose
- Documenter les modifications personnalisées
- Validation régulière des fichiers compose
- Maintenir AutoCompose à jour

## Dépannage

### Problèmes Courants

#### Erreurs de Connexion

```bash
# Vérifier le daemon Docker
docker info

# Vérifier les permissions du socket
ls -la /var/run/docker.sock

# Utiliser sudo si nécessaire
sudo autocompose

# Spécifier le socket explicitement
autocompose --docker-socket /var/run/docker.sock
```

#### Permission Refusée

```bash
# Ajouter l'utilisateur au groupe docker
sudo usermod -aG docker $USER

# Se déconnecter et se reconnecter
# Ou utiliser newgrp
newgrp docker
```

#### Sortie Vide

```bash
# Vérifier si des conteneurs sont en cours
docker ps

# Inclure tous les conteneurs
autocompose --all

# Vérifier les filtres
autocompose --no-filters

# Activer les logs de debug
AUTOCOMPOSE_LOG_LEVEL=debug autocompose
```

### Mode Debug

```bash
# Activer la sortie debug
autocompose --debug

# Logs verbeux
autocompose -vvv

# Dry run avec debug
autocompose --dry-run --debug

# Exporter les informations de debug
autocompose debug-info > debug.txt
```

## Référence CLI

### Options Globales

| Option | Court | Description | Défaut |
|--------|-------|-------------|--------|
| `--output` | `-o` | Chemin du fichier de sortie | docker-compose.yml |
| `--filter` | `-f` | Filtrer par motif (supporte * et ?) | |
| `--version` | `-v` | Version du fichier compose | 3.9 |
| `--interactive` | `-i` | Sélection interactive | false |
| `--format` | | Format de sortie (yaml/json/toml) | yaml |
| `--dry-run` | | Prévisualiser sans écrire | false |
| `--preview` | | Afficher aperçu sans écrire | false |
| `--compact` | | Sortie compacte | false |
| `--debug` | | Mode débogage | false |
| `--verbose` | | Verbosité (-v, -vv, -vvv) | 0 |
| `--help` | `-h` | Afficher l'aide | |
| `--version` | | Afficher la version de l'outil | |

### Options de Filtrage

| Option | Description | Exemple |
|--------|-------------|---------|
| `--filter` | Filtrer par motif de nom | `--filter "web-*"` |
| `--exclude` | Exclure par motif | `--exclude "*-test"` |
| `--exclude-system` | Exclure conteneurs système | `--exclude-system` |
| `--running-only` | Seulement conteneurs en cours | `--running-only` |
| `--all` | Inclure conteneurs arrêtés | `--all` |
| `--label-filter` | Filtrer par label | `--label-filter "env=prod"` |
| `--has-label` | Conteneur ayant un label | `--has-label "backup"` |
| `--state` | Filtrer par état | `--state running` |

### Options Docker

| Option | Description | Exemple |
|--------|-------------|---------|
| `--docker-host` | URL hôte Docker | `--docker-host tcp://remote:2375` |
| `--context` | Contexte Docker | `--context production` |
| `--include-networks` | Inclure définitions réseau | `--include-networks` |
| `--include-volumes` | Inclure définitions volume | `--include-volumes` |

## Référence API

### Utilisation en Bibliothèque

AutoCompose peut être utilisé comme bibliothèque dans des projets Rust :

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

### Types Principaux

```rust
// Configuration
pub struct Config {
    pub docker_socket: String,
    pub compose_version: String,
    pub output_format: Format,
}

// Options de filtrage
pub struct FilterOptions {
    pub patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub running_only: bool,
    pub labels: HashMap<String, String>,
}

// Fichier compose généré
pub struct ComposeFile {
    pub version: String,
    pub services: HashMap<String, Service>,
    pub networks: Option<HashMap<String, Network>>,
    pub volumes: Option<HashMap<String, Volume>>,
}
```

## Exemples

### Application Multi-Services

```yaml
# Conteneurs en cours :
# - webapp (nginx)
# - api (node:14)
# - database (postgres:13)
# - cache (redis:6)

# Exporter la stack complète
autocompose -o fullstack.yml

# Résultat :
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

### Environnement de Développement

```bash
# Exporter avec surcharges de développement
autocompose \
  --filter "dev-*" \
  --include-labels \
  --preserve-mounts \
  -o docker-compose.dev.yml

# Le résultat inclut :
# - Bind mounts du code source
# - Variables d'environnement de développement
# - Ports de debug exposés
# - Pas de politiques de redémarrage
```

### Déploiement Production

```bash
# Export strict pour la production
autocompose \
  --running-only \
  --exclude-system \
  --remove-caps \
  --add-healthchecks \
  --resource-limits \
  -o docker-compose.prod.yml

# Inclut :
# - Health checks pour tous les services
# - Limites de ressources (CPU/Mémoire)
# - Politiques de redémarrage
# - Pas de conteneurs privilégiés
# - Tags d'image spécifiques
```

## Historique des Versions

### Version 1.5.0 (Actuelle)

- **Détection Améliorée :** Meilleure détection des relations et dépendances entre conteneurs
- **Filtrage Avancé :** Nouveaux filtres regex, filtrage par label, et filtrage par état
- **Améliorations Podman :** Meilleur support rootless, configurations de pods, intégration SystemD
- **Améliorations Réseau :** Support IPv6, drivers réseau personnalisés, préservation des alias
- **Performance :** 3x plus rapide pour les déploiements importants, inspection parallèle des conteneurs
- **Validation :** Validation complète avec vérifications de sécurité et recommandations de bonnes pratiques
- **Formats de Sortie :** Ajout sortie JSON, mode compact, services triés

### Version 1.0.0

- Version initiale
- Support Docker de base
- Sortie YAML
- Filtrage simple

## Feuille de route

Notre vision pour AutoCompose est de devenir l'outil le plus complet et convivial pour la conversion de conteneurs en compose. Voici ce qui arrive ensuite :

### Version 1.6 - T3 2025

#### Améliorations Principales

**🔗 Dépendances des Conteneurs**
- Détection automatique des relations `depends_on`
- Dépendances basées sur les health checks
- Analyse de l'ordre de démarrage

**💾 Gestion des Volumes**
- Définitions de volumes appropriées
- Options de driver de volume
- Recommandations de sauvegarde

**🔍 Filtrage Avancé**
- Filtrage basé sur les labels
- Filtrage temporel
- Filtrage basé sur les ressources

**🌐 Améliorations Réseau**
- Support des réseaux externes
- Options de driver avancées
- Améliorations IPv6

### Version 1.7 - T4 2025

#### Fonctionnalités Avancées

**☸️ Intégration Kubernetes**
- Conversion Pod vers Compose
- Support ConfigMaps et Secrets
- Conversion basique de charts Helm

**📋 Compose Multi-Étapes**
- Fichiers spécifiques à l'environnement
- Gestion des overrides
- Capacités de fusion de fichiers

**🔨 Support du Contexte de Build**
- Détection de Dockerfile
- Arguments de build
- Builds multi-étapes

**🖥️ Interface Web**
- UI web interactive
- Éditeur visuel de dépendances
- Aperçu en temps réel

### Version 2.0 - T1 2026

#### Fonctionnalités Entreprise

**🐝 Mode Swarm**
- Génération de fichiers stack
- Contraintes de placement
- Répliques de service

**🔄 Synchronisation Bidirectionnelle**
- Compose vers conteneurs
- Détection de diff en direct
- Propagation des mises à jour

**🚀 Intégration CI/CD**
- GitHub Actions
- Templates GitLab CI
- Plugins Jenkins

**📊 Surveillance**
- Labels Prometheus
- Overlays de surveillance
- Génération d'alertes

### Version 2.1+ - 2026+

#### Vision Future

**🤖 Alimenté par l'IA**
- Optimisation ML
- Mise à l'échelle prédictive
- Détection d'anomalies

**☁️ Multi-Cloud**
- Support AWS ECS
- Azure Container Instances
- Google Cloud Run

**🔌 Système de Plugins**
- Extensions tierces
- Marketplace de plugins
- Processeurs personnalisés

**🛠️ Outils pour Développeurs**
- Intégrations IDE
- Linting en temps réel
- Complétion intelligente

### Envie de Contribuer ?

Nous accueillons les contributions ! Les domaines prioritaires incluent :
- Algorithmes de détection des dépendances des conteneurs
- Intégrations de fournisseurs cloud
- Documentation et exemples
- Optimisations de performance

🤝 [Guide de Contribution](https://github.com/Olympus-chain/autocompose/blob/main/CONTRIBUTING.md)