use super::Decoder;
use super::Encode;
use ::cell::Atom;
use ::cell::SVMCell;

use std::io::Cursor;

use quickcheck::quickcheck;

#[test]
fn prop_encode_float () {
    fn prop (x: f64) -> bool {
        let cell = SVMCell::AtomCell(Atom::Float(x));
        let encoded = cell.emit();
        let decoded = Decoder::new(&mut Cursor::new(encoded)).next_cell();
        decoded == Ok(Some(cell))
    }
    quickcheck(prop as fn(f64) -> bool);
}

#[test]
fn prop_encode_uint () {
    fn prop (x: u64) -> bool {
        let cell = SVMCell::AtomCell(Atom::UInt(x));
        let encoded = cell.emit();
        let decoded = Decoder::new(&mut Cursor::new(encoded)).next_cell();
        decoded == Ok(Some(cell))
    }
    quickcheck(prop as fn(u64) -> bool);
}

#[test]
fn prop_encode_sint () {
    fn prop (x: i64) -> bool {
        let cell = SVMCell::AtomCell(Atom::SInt(x));
        let encoded = cell.emit();
        let decoded = Decoder::new(&mut Cursor::new(encoded)).next_cell();
        decoded == Ok(Some(cell))
    }
    quickcheck(prop as fn(i64) -> bool);
}

#[test]
fn prop_encode_char () {
    fn prop (x: char) -> bool {
        let cell = SVMCell::AtomCell(Atom::Char(x));
        let encoded = cell.emit();
        let decoded = Decoder::new(&mut Cursor::new(encoded)).next_cell();
        decoded == Ok(Some(cell))
    }
    quickcheck(prop as fn(char) -> bool);
}

#[test]
fn test_encode_uint_zero () {
    let cell = SVMCell::AtomCell(Atom::UInt(0));
    let encoded = cell.emit();
    let decoded = Decoder::new(&mut Cursor::new(encoded)).next_cell();
    assert_eq!(decoded, Ok(Some(cell)))
}
