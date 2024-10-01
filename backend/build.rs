mod config;
mod log;

use log::log_config::init_logging;

fn main() {
    init_logging();
}
