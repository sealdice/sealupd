mod cli;
mod consts;
mod decompress;
mod log;
mod proc_wait;
mod term_color;

fn main() {
    println!(
        "{} v{} --- Updater for SealDice.",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
