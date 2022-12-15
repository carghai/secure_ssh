use crate::client::client_main;
use crate::host::host_main;

mod client;
mod command;
mod db;
mod host;
mod input;
pub mod ram_var;

const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";

const NAME: &str = "test";

fn main() {
    db::client().expect("please set redis key correctly");
    if input::y_n("y for client, n host") {
        println!("starting client version");
        client_main()
    } else {
        println!("starting host version");
        host_main();
    }
}
