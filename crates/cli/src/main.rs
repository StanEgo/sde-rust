const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("SDE CLI v{}", VERSION);
}
