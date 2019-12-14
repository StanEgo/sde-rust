use sde_frames::Schema;
use sde_specs::Schema;

#[derive(Schema)]
struct Point {
    x: u32,
    y: u32,
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("SDE CLI v{}", VERSION);
    Point::to_schema();
}
