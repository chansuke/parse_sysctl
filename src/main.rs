pub mod config;
pub mod errors;
pub mod parser;
pub mod types;

use env_logger::Env;

fn main() {
    // Initialize the logger.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Calling the parser.
    parser::parse();
}
