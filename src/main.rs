mod cmd;
mod config;

use cmd::command_line;
use config::message;

fn main() {
    message::greet();

    let _ = command_line(); // Main shell features
}
