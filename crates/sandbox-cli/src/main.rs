extern crate proc_macro;

use sde_specs::Schema;
use sde_specs_macro::Schema;

#[derive(Schema)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

fn main() {
    Point::to_schema();

    let sample = syn::parse_str::<syn::Item>("pub struct Point {
        pub x: u32,
        pub y: u32,
    }").unwrap();

    dbg!("Sample {:?}", sample);
}
