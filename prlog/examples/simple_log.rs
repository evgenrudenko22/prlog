use log::{error, info, warn};
use prlog::Config;

fn main() {
    prlog::init(Config::default()).unwrap();

    info!("Hello, world!");
    warn!("Warning");
    error!("Some error!");
}