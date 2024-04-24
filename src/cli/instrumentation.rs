use color_eyre::eyre::WrapErr;
use std::{error::Error, io::IsTerminal};
use tracing::Subscriber;
use tracing_subscriber::{
    filter::Directive,
    layer::{Layer, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
    EnvFilter,
};

use super::logger::Logger;

#[derive(clap::Args, Debug, Default)]
pub(crate) struct Instrumentation {
    #[clap(
        short='v',
        env="VERBOSITY",
        long,
        action = clap::ArgAction::Count,
        global=true
    )]
    pub verbose: u8,

    #[clap(
        long,
        env = "LOGGER",
        default_value_t = Default::default(),
        global = true
    )]
    pub(crate) logger: Logger,

    #[clap(
        long = "log-filters",
        env = "LOG_FILTERS",
        value_delimiter = ',',
        num_args = 0..,
        global = true
    )]
    pub(crate) log_directives: Vec<Directive>,
}

impl Instrumentation {
    pub(crate) fn log_level(&self) -> String {
        match self.verbose {
            0 => "info",
            1 => "debug",
            _ => "trace",
        }
        .to_string()
    }

    pub(crate) fn setup(&self) -> color_eyre::Result<()> {
        let filter_layer = self.filter_layer()?;
        let registry = tracing_subscriber::registry()
            .with(filter_layer)
            .with(tracing_error::ErrorLayer::default());
        match self.logger {
            Logger::Compact => registry.with(self.fmt_layer_compact()).try_init()?,
            Logger::Full => registry.with(self.fmt_layer_full()).try_init()?,
            Logger::Pretty => registry.with(self.fmt_layer_pretty()).try_init()?,
            Logger::Json => registry.with(self.fmt_layer_json()).try_init()?,
        }
        Ok(())
    }

    pub(crate) fn filter_layer(&self) -> color_eyre::Result<EnvFilter> {
        let mut filter_layer = match EnvFilter::try_from_default_env() {
            Ok(layer) => layer,
            Err(e) => {
                if let Some(source) = e.source() {
                    match source.downcast_ref::<std::env::VarError>() {
                        Some(std::env::VarError::NotPresent) => (),
                        _ => return Err(e).wrap_err_with(|| "parsing RUST_LOG directives"),
                    }
                }

                if self.log_directives.is_empty() {
                    EnvFilter::try_new(&format!(
                        "{}={}",
                        env!("CARGO_PKG_NAME").replace('-', "_"),
                        self.log_level()
                    ))?
                } else {
                    EnvFilter::try_new("")?
                }
            }
        };

        for directive in &self.log_directives {
            filter_layer = filter_layer.add_directive(directive.clone())
        }
        Ok(filter_layer)
    }

    pub(crate) fn fmt_layer_compact<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(std::io::stderr().is_terminal())
            .with_writer(std::io::stderr)
            .compact()
            .without_time()
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
    }

    pub(crate) fn fmt_layer_full<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(std::io::stderr().is_terminal())
            .with_writer(std::io::stderr)
    }

    pub(crate) fn fmt_layer_pretty<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(std::io::stderr().is_terminal())
            .with_writer(std::io::stderr)
            .pretty()
    }

    pub(crate) fn fmt_layer_json<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(std::io::stderr().is_terminal())
            .with_writer(std::io::stderr)
            .json()
    }
}
