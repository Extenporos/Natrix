mod shell;
mod commands; //commands
mod handler; // handler
mod environment_manager; //js bc rust analyzer is a mf
mod utils;
mod runtime_manager;
fn main() {
    shell::shell();
}
