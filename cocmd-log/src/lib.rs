pub use tracing;
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn set_tracing(verbose: bool) {
    let level = if verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };
    Registry::default()
        .with(tracing_tree::HierarchicalLayer::new(2))
        .with(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .with_env_var("LOG")
                .from_env_lossy(),
        )
        .init();
}

#[macro_export]
macro_rules! cocmd_info {
  ($($tts:tt)*) => {
    tracing::info!($($tts)*);
  }
}

#[macro_export]
macro_rules! cocmd_warn {
  ($($tts:tt)*) => {
    tracing::warn!($($tts)*);
  }
}

#[macro_export]
macro_rules! cocmd_error {
  ($($tts:tt)*) => {
    tracing::error!($($tts)*);
  }
}
