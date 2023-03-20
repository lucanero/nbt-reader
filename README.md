example:
```rust
fn main() {
    let mut parser = NBTParser::new(include_bytes!("hello_world.nbt").to_vec());

    println!("{:#?}", parser.parse());
    /*
    output:
    
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