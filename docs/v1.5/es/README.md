# Documentación AutoCompose v1.5

¡Bienvenido a AutoCompose v1.5! Esta versión mejorada trae mejor soporte para Docker y Podman, opciones de filtrado avanzadas y optimizaciones de rendimiento.

> **Destacados de la versión 1.5:** Detección mejorada de contenedores, extracción optimizada de configuración de red, mejor mapeo de volúmenes y rendimiento optimizado para grandes despliegues.

## ¿Qué es AutoCompose?

AutoCompose es una poderosa herramienta de línea de comandos que genera automáticamente archivos Docker Compose a partir de tus contenedores en ejecución. Simplifica el proceso de conversión de despliegues de contenedores existentes en configuraciones reproducibles y versionadas.

### Beneficios Clave

- **Automatización:** Sin escritura manual de YAML - extracción automática de configuraciones
- **Precisión:** Captura configuraciones exactas de contenedores incluyendo redes, volúmenes y variables de entorno
- **Flexibilidad:** Soporte para entornos Docker y Podman
- **Inteligencia:** Filtrado inteligente y validación para garantizar salida limpia y optimizada

## Instalación

### Requisitos del Sistema

- Linux, macOS, o Windows (con WSL2)
- Docker Engine 20.10+ o Podman 3.0+
- Rust 1.65+ (para compilar desde fuentes)

### Binarios Pre-compilados

Descarga la última versión para tu plataforma:

```bash
# Linux/macOS
curl -L https://github.com/Olympus-chain/autocompose/releases/latest/download/autocompose-linux-amd64 -o autocompose
chmod +x autocompose
sudo mv autocompose /usr/local/bin/

# Verificar instalación
autocompose --version
```

### Compilar desde Fuentes

```bash
# Clonar el repositorio
git clone https://github.com/Olympus-chain/autocompose.git
cd autocompose-podman-docker

# Compilar con Cargo
cargo build --release

# Instalar
sudo cp target/release/autocompose /usr/local/bin/
```

### Instalación con Docker

También puedes ejecutar AutoCompose usando Docker:

```bash
# Crear un alias para uso fácil
alias autocompose='docker run --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/drasrax/autocompose:latest'

# Usar la herramienta
autocompose --help
```

## Inicio Rápido

### Exportación Básica

Genera un archivo Docker Compose desde todos los contenedores en ejecución:

```bash
# Exportar todos los contenedores en ejecución
autocompose

# Exportar a un archivo específico
autocompose -o mi-stack.yml

# Vista previa sin guardar
autocompose --dry-run
```

### Exportación Filtrada

Exportar contenedores específicos basándose en criterios:

```bash
# Exportar contenedores que coincidan con un patrón
autocompose --filter "web-*"

# Exportar solo contenedores en ejecución
autocompose --running-only

# Excluir contenedores del sistema
autocompose --exclude-system
```

### Modo Interactivo

Seleccionar contenedores interactivamente:

```bash
# Lanzar selección interactiva
autocompose --interactive

# Con vista previa
autocompose --interactive --preview
```

## Características

### Detección de Contenedores

AutoCompose v1.5 presenta capacidades mejoradas de detección de contenedores:

- Detección automática de contenedores Docker y Podman
- Soporte para despliegues Podman sin root
- Detección de dependencias y enlaces entre contenedores
- Agrupación inteligente de contenedores relacionados

### Extracción de Configuración

Extracción completa de configuraciones de contenedores:

| Categoría | Campos Extraídos | Mejoras v1.5 |
|-----------|------------------|--------------|
| Info Básica | Imagen, nombre, comando, directorio de trabajo | Resolución mejorada de etiquetas |
| Red | Puertos, redes, hostname, DNS | Soporte IPv6, drivers personalizados |
| Almacenamiento | Volúmenes, bind mounts, tmpfs | Opciones de driver de volumen |
| Tiempo de Ejecución | Entorno, etiquetas, política de reinicio | Configuraciones de healthcheck |
| Seguridad | Capacidades, opciones de seguridad | Contextos SELinux, AppArmor |

### Formatos de Salida

Múltiples formatos de salida para diferentes casos de uso:

```bash
# YAML estándar (por defecto)
autocompose -o docker-compose.yml

# Formato JSON
autocompose --format json -o compose.json

# YAML con versión específica
autocompose --compose-version 3.8 -o compose-v3.8.yml

# Salida compacta
autocompose --compact -o minimal.yml
```

## Uso Básico

### Estructura del Comando

```bash
autocompose [OPCIONES] [CONTENEDORES...]

OPCIONES:
    -o, --output <ARCHIVO>        Archivo de salida (defecto: docker-compose.yml)
    -f, --format <FORMATO>        Formato de salida: yaml, json (defecto: yaml)
    -v, --compose-version <VER>   Versión del archivo compose (defecto: 3.8)
    --dry-run                     Vista previa sin escribir archivo
    --interactive                 Selección interactiva de contenedores
    --help                        Mostrar información de ayuda
```

### Flujos de Trabajo Comunes

#### Entorno de Desarrollo

Exportar tu stack de desarrollo:

```bash
# Exportar contenedores de desarrollo
autocompose --filter "dev-*" -o dev-compose.yml

# Incluir solo servicios específicos
autocompose dev-web dev-db dev-redis -o dev-stack.yml

# Con red personalizada
autocompose --network dev-network -o dev-compose.yml
```

#### Migración a Producción

Preparar contenedores para despliegue en producción:

```bash
# Exportar con configuración de producción
autocompose --running-only \
  --exclude-system \
  --remove-caps \
  -o production-compose.yml

# Validar la salida
autocompose validate production-compose.yml
```

## Comandos Docker

### Opciones Específicas de Docker

```bash
# Conectar a daemon Docker remoto
autocompose --docker-host tcp://remote:2375

# Usar contexto Docker específico
autocompose --context production

# Incluir etiquetas Docker
autocompose --include-labels

# Preservar IDs de contenedores
autocompose --preserve-ids
```

### Configuración de Red

Extracción avanzada de red para Docker:

```bash
# Incluir redes personalizadas
autocompose --include-networks

# Mapear alias de red
autocompose --preserve-aliases

# Incluir opciones de driver de red
autocompose --network-details
```

### Gestión de Volúmenes

```bash
# Incluir volúmenes nombrados
autocompose --include-volumes

# Convertir bind mounts a volúmenes
autocompose --convert-mounts

# Incluir opciones de driver de volumen
autocompose --volume-details
```

## Comandos Podman

### Características Específicas de Podman

AutoCompose v1.5 incluye soporte mejorado para Podman:

```bash
# Podman sin root
autocompose --podman-rootless

# Incluir configuraciones de pods
autocompose --include-pods

# Integración con SystemD
autocompose --systemd-compatible

# Etiquetas SELinux
autocompose --preserve-selinux
```

### Gestión de Pods

```bash
# Exportar pods completos
autocompose --pod mi-app-pod

# Agrupar por pods
autocompose --group-by-pod

# Incluir contenedores infra
autocompose --include-infra
```

## Opciones de Filtrado

### Filtrado por Nombre

```bash
# Patrones con comodines
autocompose --filter "app-*"

# Expresiones regulares
autocompose --filter-regex "^(web|api)-.*"

# Múltiples filtros
autocompose --filter "web-*" --filter "api-*"

# Patrones de exclusión
autocompose --exclude "test-*" --exclude "*-temp"
```

### Filtrado por Estado

```bash
# Solo contenedores en ejecución
autocompose --running-only

# Incluir contenedores detenidos
autocompose --all

# Por estado del contenedor
autocompose --state running,paused

# Por estado de salud
autocompose --health healthy
```

### Filtrado por Etiqueta

```bash
# Filtrar por etiqueta
autocompose --label-filter "environment=production"

# Múltiples etiquetas (Y)
autocompose --label-filter "app=myapp" --label-filter "tier=frontend"

# Etiqueta existe
autocompose --has-label "backup"

# Patrón de etiqueta
autocompose --label-regex "version=2\.*"
```

## Configuración

### Archivo de Configuración

AutoCompose soporta archivos de configuración para ajustes persistentes:

```bash
# Crear configuración por defecto
autocompose config init

# Ubicación: ~/.autocompose/config.yml
# Editar con tus ajustes preferidos
```

### Opciones de Configuración

```yaml
# Ejemplo config.yml
defaults:
  output: docker-compose.yml
  format: yaml
  compose_version: "3.8"
  
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

### Variables de Entorno

```bash
# Sobrescribir archivo de configuración
export AUTOCOMPOSE_CONFIG=/ruta/a/config.yml

# Socket Docker
export DOCKER_HOST=tcp://localhost:2375

# Directorio de salida
export AUTOCOMPOSE_OUTPUT_DIR=/compose-files

# Nivel de log
export AUTOCOMPOSE_LOG_LEVEL=debug
```

## Validación

### Validación Integrada

AutoCompose v1.5 incluye validación completa:

```bash
# Validar archivo generado
autocompose validate docker-compose.yml

# Validar con versión específica
autocompose validate -v 3.8 docker-compose.yml

# Validación estricta
autocompose validate --strict docker-compose.yml

# Verificar problemas de seguridad
autocompose validate --security docker-compose.yml
```

### Verificaciones de Validación

- **Sintaxis:** Validación de sintaxis YAML/JSON
- **Esquema:** Cumplimiento del esquema del archivo compose
- **Referencias:** Referencias de red, volumen y servicio
- **Seguridad:** Contenedores privilegiados, capacidades, bind mounts
- **Mejores Prácticas:** Límites de recursos, health checks, políticas de reinicio

### Salida de Validación

```
# Ejemplo de salida de validación
✓ Sintaxis válida
✓ Compatible con esquema (versión 3.8)
⚠ Advertencia: El servicio 'web' usa la etiqueta 'latest'
⚠ Advertencia: El servicio 'db' no tiene health check
✗ Error: La red 'frontend' está referenciada pero no definida
✗ Error: El volumen 'data' tiene opciones de driver inválidas

Resumen: 2 errores, 2 advertencias
```

## Mejores Prácticas

### Recomendaciones de Seguridad

- Siempre revisar archivos generados antes del despliegue
- Eliminar privilegios y capacidades innecesarias
- Usar etiquetas de imagen específicas en lugar de 'latest'
- Implementar gestión adecuada de secretos
- Establecer límites de recursos apropiados

### Consejos de Rendimiento

- Usar `--running-only` para procesamiento más rápido
- Filtrar contenedores para reducir tiempo de procesamiento
- Habilitar caché para exportaciones repetidas
- Usar modo compacto para archivos más pequeños

### Mantenimiento

- Versionar tus archivos compose
- Documentar modificaciones personalizadas
- Validación regular de archivos compose
- Mantener AutoCompose actualizado

## Solución de Problemas

### Problemas Comunes

#### Errores de Conexión

```bash
# Verificar daemon Docker
docker info

# Verificar permisos del socket
ls -la /var/run/docker.sock

# Usar sudo si es necesario
sudo autocompose

# Especificar socket explícitamente
autocompose --docker-socket /var/run/docker.sock
```

#### Permiso Denegado

```bash
# Agregar usuario al grupo docker
sudo usermod -aG docker $USER

# Cerrar sesión e iniciar sesión nuevamente
# O usar newgrp
newgrp docker
```

#### Salida Vacía

```bash
# Verificar si hay contenedores en ejecución
docker ps

# Incluir todos los contenedores
autocompose --all

# Verificar filtros
autocompose --no-filters

# Habilitar logs de depuración
AUTOCOMPOSE_LOG_LEVEL=debug autocompose
```

### Modo Depuración

```bash
# Habilitar salida de depuración
autocompose --debug

# Logs detallados
autocompose -vvv

# Dry run con depuración
autocompose --dry-run --debug

# Exportar información de depuración
autocompose debug-info > debug.txt
```

## Referencia CLI

### Opciones Globales

| Opción | Corta | Descripción | Por Defecto |
|--------|-------|-------------|-------------|
| `--output` | `-o` | Ruta del archivo de salida | docker-compose.yml |
| `--format` | `-f` | Formato de salida (yaml/json) | yaml |
| `--compose-version` | `-v` | Versión del archivo compose | 3.8 |
| `--dry-run` | | Vista previa sin escribir | false |
| `--interactive` | `-i` | Selección interactiva | false |
| `--help` | `-h` | Mostrar mensaje de ayuda | |
| `--version` | `-V` | Mostrar versión | |

### Opciones de Filtrado

| Opción | Descripción | Ejemplo |
|--------|-------------|---------|
| `--filter` | Filtrar por patrón de nombre | `--filter "web-*"` |
| `--exclude` | Excluir por patrón | `--exclude "*-test"` |
| `--running-only` | Solo contenedores en ejecución | `--running-only` |
| `--all` | Incluir contenedores detenidos | `--all` |
| `--label-filter` | Filtrar por etiqueta | `--label-filter "env=prod"` |

## Referencia API

### Uso como Biblioteca

AutoCompose puede usarse como biblioteca en proyectos Rust:

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

### Tipos Principales

```rust
// Configuración
pub struct Config {
    pub docker_socket: String,
    pub compose_version: String,
    pub output_format: Format,
}

// Opciones de filtrado
pub struct FilterOptions {
    pub patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub running_only: bool,
    pub labels: HashMap<String, String>,
}

// Archivo compose generado
pub struct ComposeFile {
    pub version: String,
    pub services: HashMap<String, Service>,
    pub networks: Option<HashMap<String, Network>>,
    pub volumes: Option<HashMap<String, Volume>>,
}
```

## Ejemplos

### Aplicación Multi-Servicio

```bash
# Contenedores en ejecución:
# - webapp (nginx)
# - api (node:14)
# - database (postgres:13)
# - cache (redis:6)

# Exportar stack completo
autocompose -o fullstack.yml

# Resultado:
```

```yaml
version: '3.8'
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

### Entorno de Desarrollo

```bash
# Exportar con sobreescrituras de desarrollo
autocompose \
  --filter "dev-*" \
  --include-labels \
  --preserve-mounts \
  -o docker-compose.dev.yml

# El resultado incluye:
# - Bind mounts del código fuente
# - Variables de entorno de desarrollo
# - Puertos de depuración expuestos
# - Sin políticas de reinicio
```

### Despliegue en Producción

```bash
# Exportación estricta para producción
autocompose \
  --running-only \
  --exclude-system \
  --remove-caps \
  --add-healthchecks \
  --resource-limits \
  -o docker-compose.prod.yml

# Incluye:
# - Health checks para todos los servicios
# - Límites de recursos (CPU/Memoria)
# - Políticas de reinicio
# - Sin contenedores privilegiados
# - Etiquetas de imagen específicas
```

## Historial de Cambios

### Versión 1.5.0 (Actual)

- **Detección Mejorada:** Mejor detección de relaciones y dependencias entre contenedores
- **Filtrado Avanzado:** Nuevos filtros regex, filtrado basado en etiquetas y filtrado por estado
- **Mejoras Podman:** Mejor soporte sin root, configuraciones de pods, integración con SystemD
- **Mejoras de Red:** Soporte IPv6, drivers de red personalizados, preservar alias
- **Rendimiento:** 3x más rápido para grandes despliegues, inspección paralela de contenedores
- **Validación:** Validación completa con verificaciones de seguridad y recomendaciones de mejores prácticas
- **Formatos de Salida:** Añadida salida JSON, modo compacto, servicios ordenados

### Versión 1.0.0

- Lanzamiento inicial
- Soporte básico de Docker
- Salida YAML
- Filtrado simple

## Hoja de ruta

Nuestra visión para AutoCompose es convertirse en la herramienta más completa y fácil de usar para la conversión de contenedores a compose. Esto es lo que viene a continuación:

### Versión 1.6 - T3 2025

#### Mejoras Principales

**🔗 Dependencias de Contenedores**
- Detección automática de relaciones `depends_on`
- Dependencias basadas en health checks
- Análisis del orden de inicio

**💾 Gestión de Volúmenes**
- Definiciones adecuadas de volúmenes
- Opciones de driver de volumen
- Recomendaciones de backup

**🔍 Filtrado Avanzado**
- Filtrado basado en etiquetas
- Filtrado temporal
- Filtrado basado en recursos

**🌐 Mejoras de Red**
- Soporte de redes externas
- Opciones avanzadas de driver
- Mejoras IPv6

### Versión 1.7 - T4 2025

#### Características Avanzadas

**☸️ Integración Kubernetes**
- Conversión de Pod a Compose
- Soporte de ConfigMaps y Secrets
- Conversión básica de charts Helm

**📋 Compose Multi-Etapa**
- Archivos específicos por entorno
- Gestión de overrides
- Capacidades de fusión de archivos

**🔨 Soporte de Contexto de Build**
- Detección de Dockerfile
- Argumentos de build
- Builds multi-etapa

**🖥️ Interfaz Web**
- UI web interactiva
- Editor visual de dependencias
- Vista previa en tiempo real

### Versión 2.0 - T1 2026

#### Características Empresariales

**🐝 Modo Swarm**
- Generación de archivos stack
- Restricciones de ubicación
- Réplicas de servicio

**🔄 Sincronización Bidireccional**
- Compose a contenedores
- Detección de diferencias en vivo
- Propagación de actualizaciones

**🚀 Integración CI/CD**
- GitHub Actions
- Plantillas GitLab CI
- Plugins Jenkins

**📊 Monitoreo**
- Etiquetas Prometheus
- Overlays de monitoreo
- Generación de alertas

### Versión 2.1+ - 2026+

#### Visión Futura

**🤖 Impulsado por IA**
- Optimización ML
- Escalado predictivo
- Detección de anomalías

**☁️ Multi-Nube**
- Soporte AWS ECS
- Azure Container Instances
- Google Cloud Run

**🔌 Sistema de Plugins**
- Extensiones de terceros
- Marketplace de plugins
- Procesadores personalizados

**🛠️ Herramientas para Desarrolladores**
- Integraciones IDE
- Linting en tiempo real
- Completado inteligente

### ¿Quieres Contribuir?

¡Damos la bienvenida a las contribuciones! Las áreas prioritarias incluyen:

- Algoritmos de detección de dependencias de contenedores
- Integraciones de proveedores de nube
- Documentación y ejemplos
- Optimizaciones de rendimiento

[🤝 Guía de Contribución](https://github.com/Olympus-chain/autocompose/blob/main/CONTRIBUTING.md)