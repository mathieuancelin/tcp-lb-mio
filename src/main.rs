#[macro_use]
extern crate log;
extern crate env_logger;
extern crate argparse;
extern crate mio;

mod backend;
mod tcplb;

use std::sync::{Arc, Mutex};
use std::process::exit;
use std::env;

use argparse::{ArgumentParser, Store, Collect};
use mio::*;

fn main() {
    let mut servers: Vec<String> = Vec::new();
    let mut bind = "127.0.0.1:8000".to_string();
    let mut log_level = "info".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Dynamic TCP load balancer");

        ap.refer(&mut servers).add_argument("server", Collect, "Servers to load balance");

        ap.refer(&mut bind).add_option(&["-b", "--bind"],
                                       Store,
                                       "Bind the load balancer to address:port (127.0.0.1:8000)");

        ap.refer(&mut log_level).add_option(&["-l", "--log"],
                                            Store,
                                            "Log level [debug, info, warn, error] (info)");

        ap.parse_args_or_exit();
    }

    env::set_var("RUST_LOG", log_level);

    env_logger::init();

    if servers.is_empty() {
        error!("Need at least one server to load balance");
        exit(1);
    }

    let backend = Arc::new(Mutex::new(backend::RoundRobinBackend::new(servers).unwrap()));

    let mut proxy = tcplb::Proxy::new(&bind, backend.clone());
    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register_opt(&proxy.listen_sock,
                            Token(1),
                            EventSet::readable(),
                            PollOpt::edge()).unwrap();

    event_loop.run(&mut proxy).unwrap();
}
