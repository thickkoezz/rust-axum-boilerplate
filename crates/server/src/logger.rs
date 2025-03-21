use tracing_appender::{non_blocking, non_blocking::WorkerGuard, rolling::daily};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};
use utils::{config, config::server_config::CargoEnv};

pub struct Logger;

impl Logger {
  // pub fn init(cargo_env: CargoEnv) -> WorkerGuard {
  pub fn init() -> WorkerGuard {
    let cfg = config::get();
    let (non_blocking, guard) = match cfg.cargo_env {
      CargoEnv::Development => {
        let console_logger = std::io::stdout();
        non_blocking(console_logger)
      }
      CargoEnv::Production => {
        let file_logger = daily("logs", "log");
        non_blocking(file_logger)
      }
    };

    // Set the default verbosity level for the root of the dependency graph.
    // env var: `RUST_LOG`
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
      format!(
        "{}={},tower_http={}",
        env!("CARGO_PKG_NAME"),
        cfg.rust_log,
        cfg.rust_log
      )
      .into()
    });

    registry()
      .with(env_filter)
      .with(fmt::layer().with_writer(non_blocking))
      .init();

    guard
  }
}
