use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

// Inicializa el sistema de logging de la aplicación usando el crate `tracing`.
//
// La salida se envía a stdout (salida estándar) sin timestamps,
// ya que al ejecutarse bajo systemd, journald añade automáticamente
// sus propios metadatos (timestamp, unit, etc.) a cada línea capturada.
//
// El nivel de log se controla mediante la variable de entorno `RUST_LOG`.
// Ejemplo: `RUST_LOG=info` o `RUST_LOG=rs_backend=debug,info`.
pub fn init() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().without_time())
        .init();

    Ok(())
}
