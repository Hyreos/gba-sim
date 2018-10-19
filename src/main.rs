mod cpu;
mod rom;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let cart = rom::Cartridge::new_from_file(&args[1]).unwrap();
    println!("Cartridge info:");
    println!("{:?}", cart);
}
