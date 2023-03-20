# Notes
`NBTParser::from_path` will decompress the file if the first three bytes are `1F 8B 08` (GZIP DEFLATE)
(Should probably remove this, as I shouldn't be handling decompression at all and only accept raw NBT data.)

You can technically create an NBT file using `Tag::as_bytes()` after manually creating Tag enums. (call on the root compound)

one of my first rust projects, not sure how I would make NBT file creation properly.

# Usage
```rust
use crate::nbt::NBTParser;

fn main() {
    let mut parser = NBTParser::new(include_bytes!("hello_world.nbt").to_vec());
    // returns a Tag (should be Tag::Compound(tags: Vec<Tag>))
    
    println!("{:#?}", parser.parse());

    /* expected output:
    Named(
        String(
            "hello world",
        ),
        Compound(
            [
                Named(
                    String(
                        "name",
                    ),
                    String(
                        "Bananrama",
                    ),
                ),
                End,
            ],
        ),
    )
    */
}
```

# TODO
Probably won't get to this, however, it is worth leaving here.
- Add support for SNBT, including helper `Tag::as_snbt()`
- Proper form of NBT data creation, instead of creating `Tag` enums (?)

# Dependencies
- flate2 = "1.0.25"