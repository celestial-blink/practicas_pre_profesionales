# Prácticas Perú Pro - Backend & Server

Servidor web y API backend desarrollado en **Rust** utilizando **Actix Web**, **Maud** (Server-Side Rendering) y **SQLx** para la plataforma de búsqueda y gestión de prácticas pre-profesionales y profesionales en el Perú (**Prácticas Perú Pro**).

---

## 🚀 Tecnologías Principales

- **Lenguaje**: Rust (Edition 2024)
- **Web Framework**: [Actix Web](https://actix.rs/) `v4.13`
- **Template Engine**: [Maud](https://maud.lambda.xyz/) `v0.27` (HTML generado en tiempo de compilación con macros de Rust)
- **Base de Datos**: [SQLx](https://github.com/launchbadge/sqlx) `v0.8` (Conexiones asíncronas a MariaDB / MySQL)
- **Logging & Tracing**: `tracing`, `tracing-actix-web`, `tracing-subscriber`
- **Desarrollo**: `watchexec` para recarga en vivo en entorno local

---

## 📁 Estructura del Proyecto

```text
rs-backend/
├── assets/                 # Recursos de compilación y assets auxiliares
├── public/                 # Archivos estáticos (imágenes, favicons, logos, uploads)
├── src/
│   ├── main.rs             # Punto de entrada de la aplicación y configuración de rutas
│   ├── config.rs           # Carga de variables de entorno y configuración
│   ├── data.rs             # Tipos y estructuras de datos
│   ├── general_types.rs    # Definiciones de estado global (CacheState, State)
│   ├── helpers.rs          # Funciones auxiliares y utilidades
│   ├── macros.rs           # Macros personalizadas de Rust
│   ├── maud/               # Componentes y páginas HTML renderizadas con Maud
│   │   ├── components/     # Componentes reutilizables (header, footer, head, etc.)
│   │   └── pages/          # Vistas (home, ofertas, convocatorias, departamento, etc.)
│   ├── middleware/         # Middlewares HTTP (ej. autenticación de la API)
│   ├── modules/            # Módulos de dominio y endpoints API (/api/v1)
│   │   ├── convocatorias/
│   │   ├── ofertas/
│   │   ├── organizaciones/
│   │   └── pre_ofertas/
│   ├── t_logs.rs           # Inicialización de tracing / logging
│   └── types.rs            # Tipos adicionales de la aplicación
├── .env                    # Configuración de variables de entorno (local)
├── dev.sh                  # Script bash para ejecutar con watchexec (live-reload)
├── Cargo.toml              # Dependencias y manifiesto de Rust
└── README.md
```

---

## ⚙️ Configuración de Variables de Entorno (`.env`)

Crea un archivo `.env` en la raíz del proyecto con las siguientes variables:

```env
# Base de Datos MariaDB / MySQL
DATABASE_URL=mysql://root:root@localhost:3307/practicas?timezone=America%2FLima

# Puerto del Servidor (Por defecto: 8083)
PORT=8083

# Configuración de Logging
RUST_LOG=tracing,actix_web=warn,rs_backend=info,sqlx=warn

# Rutas de almacenamiento
TEMP_DIR=./temp
STORAGE_DIR=./public
UPLOAD_LOGO_DIR=./public/images/organizaciones
LOG_DIR=logs

# Dominio y Modo
IS_DEV=true
DOMAIN=https://www.practicasperupro.com
```

---

## 💻 Desarrollo Local

### Requisitos Previos

- [Rust & Cargo](https://www.rust-lang.org/) (Versión reciente compatible con edición 2024)
- Servidor de base de datos **MariaDB** / **MySQL**
- [Watchexec](https://github.com/watchexec/watchexec) *(Opcional, para live reload)*:
  ```bash
  cargo install watchexec-cli
  ```

### Ejecutar Servidor

1. **Modo estándar de Cargo**:
   ```bash
   cargo run
   ```

2. **Modo desarrollo con Live Reload (Recomendado)**:
   ```bash
   chmod +x dev.sh
   ./dev.sh
   ```

El servidor estará escuchando por defecto en `http://127.0.0.1:8083`.

---

## 🗺️ Rutas Principales

### Vistas Web (Server-Side Rendered)

- `/` - Página principal (Home)
- `/practicas-peru` - Búsqueda de prácticas
- `/oferta-practicas/[alias_oferta]` - Detalle de oferta de prácticas
- `/convocatorias-practicas/[alias_convocatoria]` - Detalle de convocatoria
- `/departamento/[alias_departamento]` - Ofertas por departamento
- `/nivel/[alias_nivel]` - Ofertas por nivel educativo
- `/modalidad/[alias_modalidad]` - Ofertas por modalidad (Presencial / Remoto / Híbrido)
- `/formacion/[alias_formacion]` - Ofertas por carrera o área de formación
- `/organizacion/[alias_organizacion]` - Perfil y ofertas por organización

### API REST (`/api/v1`)

Todos los endpoints bajo `/api/v1` utilizan el middleware de autenticación `api_auth_middleware`.

- **Health Check**: `GET /health`
- **Pre-ofertas**: `POST /api/v1/pre-ofertas`, `POST /api/v1/pre-ofertas/insert-many`, `GET /api/v1/pre-ofertas/:id`, `PUT /api/v1/pre-ofertas/:id`
- **Organizaciones**: `GET /api/v1/organizaciones`, `POST /api/v1/organizaciones`, `PUT /api/v1/organizaciones/:id`, `GET /api/v1/organizaciones/ruc/:ruc`
- **Ofertas**: `GET /api/v1/ofertas`, `POST /api/v1/ofertas`, `PUT /api/v1/ofertas/:id`
- **Convocatorias**: `GET /api/v1/convocatorias`, `POST /api/v1/convocatorias`, `PUT /api/v1/convocatorias/:id`

---

## 📦 Compilación y Despliegue en Producción

### 1. Compilación Release

```bash
cargo build --release
```

El ejecutable optimizado se generará en `target/release/rs_backend`.

### 2. Configuración con Nginx (Reverse Proxy)

El archivo `nginx-config` incluido define la configuración para redirigir tráfico al servicio en el puerto `8083` y servir archivos estáticos desde `/public/`:

```nginx
server {
    listen 9000 default_server;
    server_name 127.0.0.1;

    location /public/ {
        alias /var/www/practicasperupro/public/;
        try_files $uri $uri/ =404;
    }

    location / {
        proxy_pass http://127.0.0.1:8083;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 3. Servicio Systemd

Puedes configurar el servicio utilizando el archivo `practicasperupro.service`:

```bash
sudo cp practicasperupro.service /etc/systemd/system/practicasperupro.service
sudo systemctl daemon-reload
sudo systemctl enable practicasperupro
sudo systemctl start practicasperupro
```
