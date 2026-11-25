mod commands; //commands
mod environment_manager; //js bc rust analyzer is a mf
mod handler; // handler
mod runtime_manager;
mod shell;
mod utils;

fn main() {
    shell::shell();
}
