pub mod core;
pub mod attribute;

fn main() {
    let map = core::AttributeMap::new();
    println!("Hello, world! {:?}", map);
    //                            ^^^^   this is marked as error in my IntelliJ Rust

    let a = attribute::String::StringFilter1;
    // When I type this, it will popup for the first attribute:: but then when I type String and forward it shows nothing.
}
