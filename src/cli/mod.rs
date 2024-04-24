// Thank you to Hoverbear
// https://hoverbear.org/blog/instrumenting-axum-projects/

use clap::Parser;
use std::net::{IpAddr, Ipv6Addr};
use url::Url;

mod instrumentation;
mod logger;

const DEFAULT_HOST: IpAddr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
const DEFAULT_PORT: u16 = 8080;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[clap(long, env = "HOST", default_value_t = DEFAULT_HOST)]
    pub(crate) host: IpAddr,

    #[clap(long, env = "PORT", default_value_t = DEFAULT_PORT)]
    pub(crate) port: u16,

    #[clap(long, env = "EPISODE_URL")]
    pub(crate) episode_url: Url,
}
