use byteorder::ReadBytesExt;
use std::io::{Read, Result};

pub use binde_derive::BinaryDeserialize;
pub use byteorder::{BigEndian, ByteOrder, LittleEndian};

/// Deserialize a binary structure
///
/// # Example
/// ```rust
/// use binde::{BinaryDeserialize, LittleEndian, deserialize};
/// use std::io::Cursor;
///
/// #[derive(BinaryDeserialize, Debug, PartialEq, Eq)]
/// struct CoolStructure {
///     a: u16,
///     b: i8,
/// }
///
/// let cursor = Cursor::new([0xDF, 0x27, 0x95]);
/// let cool_structure: CoolStructure = deserialize::<LittleEndian, _, _>(cursor).unwrap();
/// assert_eq!(cool_structure, CoolStructure { a: 10207, b: -107 })
/// ```
pub fn deserialize<E, T, R>(reader: R) -> Result<T>
where
    E: ByteOrder,
    T: BinaryDeserialize,
    R: Read,
{
    T::deserialize::<E, R>(reader)
}

pub fn size_of<T>() -> usize
where
    T: BinaryDeserialize,
{
    T::SIZE
}

pub trait BinaryDeserialize: Sized {
    const SIZE: usize;

    fn deserialize<E, R>(reader: R) -> Result<Self>
    where
        E: ByteOrder,
        R: Read;
}

macro_rules! impl_byte_deserialize {
    ($ty:ty, $size:literal, $read_fn:ident) => {
        impl BinaryDeserialize for $ty {
            const SIZE: usize = $size;

            fn deserialize<E, R>(mut reader: R) -> Result<Self>
            where
                E: ByteOrder,
                R: Read,
            {
                reader.$read_fn()
            }
        }
    };
}

macro_rules! impl_primitive_deserialize {
    ($ty:ty, $size:literal, $read_fn:ident) => {
        impl BinaryDeserialize for $ty {
            const SIZE: usize = $size;

            fn deserialize<E, R>(mut reader: R) -> Result<Self>
            where
                E: ByteOrder,
                R: Read,
            {
                reader.$read_fn::<E>()
            }
        }
    };
}

impl_byte_deserialize!(u8, 1, read_u8);
impl_byte_deserialize!(i8, 1, read_i8);
impl_primitive_deserialize!(u16, 2, read_u16);
impl_primitive_deserialize!(i16, 2, read_i16);
impl_primitive_deserialize!(u32, 4, read_u32);
impl_primitive_deserialize!(i32, 4, read_i32);
impl_primitive_deserialize!(u64, 8, read_u64);
impl_primitive_deserialize!(i64, 8, read_i64);
impl_primitive_deserialize!(u128, 16, read_u128);
impl_primitive_deserialize!(i128, 16, read_i128);
