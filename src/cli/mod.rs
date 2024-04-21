// Thank you to Hoverbear
// https://hoverbear.org/blog/instrumenting-axum-projects/

use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};

const DEFAULT_BIND_ADDR: SocketAddr =
    SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 8080);

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(long, env = "BIND", default_value_t = DEFAULT_BIND_ADDR)]
    pub(crate) bind: SocketAddr,

    #[clap(long, env = "EPISODE_URL", default_value_t = DEFAULT_BIND_ADDR)]
    pub(crate) episode_url: SocketAddr,
}
