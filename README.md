# Binary Deserialize

A crate for deserialiazing binary structures with ease.

Heads Up! This is a very experimental library with no support for alignment and little documentation; use with caution.

```toml
binde = "0.1"
```

# Example

```rust
use binde::{BinaryDeserialize, LittleEndian, deserialize};
use std::io::Cursor;

#[derive(BinaryDeserialize, Debug, PartialEq, Eq)]
struct CoolStructure {
    a: u16,
    b: i8,
}

fn main() {
    assert_eq!(CoolStructure::SIZE, 3);

    let cursor = Cursor::new([0xDF, 0x27, 0x95]);
    let cool_structure: CoolStructure = deserialize::<LittleEndian, _, _>(cursor).unwrap();
    assert_eq!(cool_structure, CoolStructure { a: 10207, b: -107 })
}
```
