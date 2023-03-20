mod nbt;
mod reader;
use crate::nbt::NBTParser;

fn main() {
    let mut parser = NBTParser::new(include_bytes!("hello_world.nbt").to_vec());
    let root = parser.parse();
    
    println!("{:#?}", root);
}