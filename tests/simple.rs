use binde::{BinaryDeserialize, LittleEndian};
use std::io::Cursor;

#[test]
fn primitive() {
    let cur = Cursor::new(&[0x42, 0x52]);
    let num = u16::deserialize::<LittleEndian, _>(cur).unwrap();
    assert_eq!(num, 0x5242);
}
