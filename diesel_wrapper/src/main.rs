use std::env;
use std::process::Command;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    Command::new(env!("DIESEL_WRAPPER_EXE"))
        .args(&args)
        .status()
        .unwrap();
}
