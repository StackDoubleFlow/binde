use binde::{BinaryDeserialize, LittleEndian};
use std::io::Cursor;

#[derive(BinaryDeserialize, Debug, PartialEq, Eq)]
pub struct Il2CppMethodDefinition {
    pub name_index: i32,
    pub declaring_type: i32,
    pub return_type: i32,
    pub parameter_start: i32,
    pub generic_container_index: i32,
    pub token: u32,
    pub flags: u16,
    pub iflags: u16,
    pub slot: u16,
    pub parameter_count: u16,
}

#[test]
fn method_definition() {
    let bytes = [
        0x3E, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x49, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x06, 0x96, 0x00, 0x00, 0x00, 0xFF, 0xFF,
        0x01, 0x00,
    ];
    let cur = Cursor::new(bytes);
    let method_def: Il2CppMethodDefinition = binde::deserialize::<LittleEndian, _, _>(cur).unwrap();

    assert_eq!(
        method_def,
        Il2CppMethodDefinition {
            name_index: 62,
            declaring_type: 1,
            return_type: 73,
            parameter_start: 0,
            generic_container_index: -1,
            token: 100663297,
            flags: 150,
            iflags: 0,
            slot: 65535,
            parameter_count: 1,
        }
    );
    assert_eq!(Il2CppMethodDefinition::SIZE, 32);
}
