# Documentación AutoCompose v1.0

¡Bienvenido a AutoCompose v1.0! Esta es la versión original que proporciona funcionalidad básica para generar archivos Docker Compose desde contenedores en ejecución.

> **Nota:** Esta es una versión heredada. Para características mejoradas, mejor rendimiento y detección mejorada de contenedores, considere actualizar a la [versión 1.5](../../v1.5/es/index.html).

## ¿Qué es AutoCompose?

AutoCompose es una herramienta de línea de comandos que genera automáticamente archivos Docker Compose desde tus contenedores Docker en ejecución. Simplifica el proceso de creación de archivos compose extrayendo la configuración de contenedores existentes.

### Características Clave

- **Generación Automática:** Crea docker-compose.yml desde contenedores en ejecución
- **Configuración Básica:** Extrae configuraciones esenciales de contenedores
- **CLI Simple:** Interfaz de línea de comandos fácil de usar
- **Salida YAML:** Formato estándar de Docker Compose

## Instalación

### Requisitos

- Linux o macOS
- Docker Engine 19.03+
- Python 3.6+ o binario pre-compilado

### Descargar Binario

```bash
# Descargar la versión v1.0
wget https://github.com/Olympus-chain/autocompose/releases/download/v1.0.0/autocompose
chmod +x autocompose
sudo mv autocompose /usr/local/bin/

# Verificar instalación
autocompose --version
```

### Instalar desde Fuentes

```bash
# Clonar repositorio (tag v1.0)
git clone --branch v1.0.0 https://github.com/Olympus-chain/autocompose.git
cd autocompose-podman-docker

# Instalar
make install
```

## Inicio Rápido

### Ejemplo Básico

Generar un archivo compose desde todos los contenedores en ejecución:

```bash
# Generar docker-compose.yml
autocompose

# Salida a archivo específico
autocompose -o mi-compose.yml

# Mostrar salida sin guardar
autocompose --stdout
```

### Qué se Exporta

AutoCompose v1.0 extrae la siguiente información:

- Imagen y etiqueta del contenedor
- Nombre del contenedor
- Variables de entorno
- Mapeo de puertos
- Montajes de volúmenes
- Política de reinicio
- Redes (básico)

## Uso Básico

### Sintaxis del Comando

```bash
autocompose [OPCIONES]

Donde OPCIONES pueden ser:
  -o, --output ARCHIVO    Archivo de salida (defecto: docker-compose.yml)
  --stdout                Imprimir a stdout en lugar de archivo
  -v, --version           Mostrar versión
  -h, --help              Mostrar mensaje de ayuda
```

### Flujo de Trabajo Simple

1. Inicia tus contenedores manualmente con docker run
2. Configúralos según sea necesario
3. Ejecuta autocompose para generar el archivo compose
4. Usa el archivo generado para futuros despliegues

```bash
# Ejemplo: Ejecutar contenedores manualmente
docker run -d --name web -p 80:80 nginx
docker run -d --name db -e MYSQL_ROOT_PASSWORD=secreto mysql:5.7

# Generar archivo compose
autocompose

# Ver el resultado
cat docker-compose.yml
```

## Opciones de Comando

### Opciones Disponibles

| Opción | Descripción | Por Defecto |
|--------|-------------|---------|
| `-o, --output` | Ruta del archivo de salida | docker-compose.yml |
| `--stdout` | Imprimir a salida estándar | false |
| `-v, --version` | Mostrar información de versión | - |
| `-h, --help` | Mostrar mensaje de ayuda | - |

### Variables de Entorno

```bash
# Ubicación del socket Docker (si no es estándar)
export DOCKER_HOST=tcp://localhost:2375

# Ejecutar autocompose
autocompose
```

## Formato de Salida

### Estructura Generada

AutoCompose v1.0 genera un archivo Docker Compose v2 estándar:

```yaml
version: '2'
services:
  nombre_contenedor:
    image: imagen:etiqueta
    container_name: nombre_contenedor
    environment:
      - ENV_VAR=valor
    ports:
      - "host:contenedor"
    volumes:
      - /ruta/host:/ruta/contenedor
    restart: politica
```

### Ejemplo de Salida

Para una aplicación web simple:

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
      - MYSQL_ROOT_PASSWORD=contraseña
      - MYSQL_DATABASE=miapp
    volumes:
      - /var/lib/mysql:/var/lib/mysql
    restart: always
```

## Ejemplos

### Servidor Web Simple

```bash
# Ejecutar contenedor nginx
docker run -d \
  --name servidorweb \
  -p 8080:80 \
  -v ~/sitio:/usr/share/nginx/html:ro \
  nginx:alpine

# Generar archivo compose
autocompose -o servidorweb-compose.yml

# Resultado:
version: '2'
services:
  servidorweb:
    image: nginx:alpine
    container_name: servidorweb
    ports:
      - "8080:80"
    volumes:
      - ~/sitio:/usr/share/nginx/html:ro
```

### Contenedor de Base de Datos

```bash
# Ejecutar PostgreSQL
docker run -d \
  --name postgres-db \
  -e POSTGRES_PASSWORD=micontraseña \
  -e POSTGRES_DB=midb \
  -v postgres-data:/var/lib/postgresql/data \
  -p 5432:5432 \
  postgres:12

# Generar archivo compose
autocompose

# El resultado incluye variables de entorno y volúmenes
```

### Configuración Multi-Contenedor

```bash
# Ejecutar múltiples contenedores
docker run -d --name frontend -p 3000:3000 mi-app:frontend
docker run -d --name backend -p 5000:5000 --link frontend mi-app:backend
docker run -d --name cache redis:alpine

# Generar archivo compose completo
autocompose -o stack-completo.yml

# Crea archivo compose con los tres servicios
```

## Solución de Problemas

### Problemas Comunes

#### No se encontraron contenedores

Asegúrate de que el daemon Docker esté ejecutándose y los contenedores estén activos:

```bash
# Verificar estado de Docker
docker info

# Listar contenedores en ejecución
docker ps

# Si no hay contenedores ejecutándose, inicia algunos primero
docker run -d nginx
```

#### Permiso denegado

Agrega tu usuario al grupo docker o usa sudo:

```bash
# Agregar usuario al grupo docker
sudo usermod -aG docker $USER

# O ejecutar con sudo
sudo autocompose
```

#### No se puede conectar a Docker

Verifica el socket de Docker:

```bash
# Ubicación del socket por defecto
ls -la /var/run/docker.sock

# Si usas socket personalizado
export DOCKER_HOST=unix:///ruta/a/docker.sock
autocompose
```

### Obtener Ayuda

```bash
# Mostrar ayuda
autocompose --help

# Verificar versión
autocompose --version

# Reportar problemas
# https://github.com/Olympus-chain/autocompose/issues
```

## Limitaciones

### Limitaciones Conocidas en v1.0

- **Solo Docker:** Sin soporte para Podman
- **Extracción Básica:** Opciones de configuración limitadas extraídas
- **Sin Filtrado:** Exporta todos los contenedores en ejecución
- **Redes:** Solo soporte básico de red
- **Versión Compose:** Solo soporta formato versión 2
- **Sin Validación:** Los archivos generados no son validados

### Características No Incluidas

- Health checks
- Límites de recursos
- Configuraciones de red personalizadas
- Opciones de seguridad
- Etiquetas
- Configuración de registro

> **Recomendación de Actualización:** Para estas características y más, actualiza a [AutoCompose v1.5](../../v1.5/es/index.html) que incluye detección completa de contenedores, filtrado avanzado, validación y soporte para Docker y Podman.

### Ajustes Manuales

Después de la generación, puede que necesites editar manualmente el archivo compose para:

- Agregar opciones de configuración faltantes
- Definir redes personalizadas
- Establecer restricciones de recursos
- Agregar health checks
- Configurar registro