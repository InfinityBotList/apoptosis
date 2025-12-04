pub(crate) mod service;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

fn main() {
    println!("Hello, world!");
}
