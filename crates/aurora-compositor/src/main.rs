mod state;

use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use smithay::reexports::calloop::{
    timer::{TimeoutAction, Timer},
    EventLoop,
};

use crate::state::AuroraState;

fn main() -> Result<()> {
    init_tracing();
    tracing::info!(version = env!("CARGO_PKG_VERSION"), "Aurora OS — compositor iniciando");

    let mut event_loop: EventLoop<'static, AuroraState> =
        EventLoop::try_new().context("falha ao criar o event loop (calloop)")?;
    let mut state = AuroraState::new(&event_loop)?;

    // Prova de vida do loop: fonte real de eventos, não um sleep.
    state
        .loop_handle
        .insert_source(Timer::from_duration(Duration::from_secs(1)), |_, _, state| {
            tracing::debug!(uptime_ms = state.uptime_ms(), "heartbeat do event loop");
            TimeoutAction::ToDuration(Duration::from_secs(1))
        })
        .map_err(|err| anyhow!("falha ao registrar o timer de heartbeat: {err}"))?;

    // Modo de verificação determinística: encerra sozinho em 3s.
    if std::env::args().any(|arg| arg == "--selftest") {
        state
            .loop_handle
            .insert_source(Timer::from_duration(Duration::from_secs(3)), |_, _, state| {
                tracing::info!("selftest concluído, encerrando");
                state.running = false;
                TimeoutAction::Drop
            })
            .map_err(|err| anyhow!("falha ao registrar o timer do selftest: {err}"))?;
    }

    event_loop
        .run(Some(Duration::from_millis(16)), &mut state, |state| {
            if !state.running {
                state.loop_signal.stop();
            }
        })
        .context("o event loop terminou com erro")?;

    tracing::info!(uptime_ms = state.uptime_ms(), "Aurora encerrado");
    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::try_from_env("AURORA_LOG")
        .or_else(|_| EnvFilter::try_from_default_env())
        .unwrap_or_else(|_| EnvFilter::new("info,aurora_compositor=debug"));

    tracing_subscriber::fmt().with_env_filter(filter).init();
}