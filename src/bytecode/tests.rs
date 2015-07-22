use super::{Encode,Decoder};
use ::cell::{Atom,Inst,SVMCell};

use std::io::Cursor;

use quickcheck::quickcheck;

macro_rules! impl_encode_test {
    ($name:ident, $it:expr) => {
        #[test]
        fn $name () {
            let cell = $it;
            let encoded = cell.emit();
            let decoded = Decoder::new(&mut Cursor::new(encoded)).next_cell();
            assert_eq!(Ok(Some(cell)), decoded)
        }

    }
}

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

impl_encode_test!(
    test_encode_uint_zero,
    SVMCell::AtomCell(Atom::UInt(0))
);
impl_encode_test!(
    test_encode_inst_nil,
    SVMCell::InstCell(Inst::NIL)
);
impl_encode_test!(
    test_encode_inst_ld,
    SVMCell::InstCell(Inst::LD)
);
impl_encode_test!(
    test_encode_inst_ldf,
    SVMCell::InstCell(Inst::LDF)
);
impl_encode_test!(
    test_encode_inst_ap,
    SVMCell::InstCell(Inst::AP)
);
impl_encode_test!(
    test_encode_inst_apcc,
    SVMCell::InstCell(Inst::APCC)
);
impl_encode_test!(
    test_encode_inst_join,
    SVMCell::InstCell(Inst::JOIN)
);
impl_encode_test!(
    test_encode_inst_rap,
    SVMCell::InstCell(Inst::RAP)
);
impl_encode_test!(
    test_encode_inst_ret,
    SVMCell::InstCell(Inst::RET)
);
impl_encode_test!(
    test_encode_inst_dum,
    SVMCell::InstCell(Inst::DUM)
);
impl_encode_test!(
    test_encode_inst_sel,
    SVMCell::InstCell(Inst::SEL)
);
impl_encode_test!(
    test_encode_inst_add,
    SVMCell::InstCell(Inst::ADD)
);
impl_encode_test!(
    test_encode_inst_sub,
    SVMCell::InstCell(Inst::SUB)
);
impl_encode_test!(
    test_encode_inst_mul,
    SVMCell::InstCell(Inst::MUL)
);
impl_encode_test!(
    test_encode_inst_div,
    SVMCell::InstCell(Inst::DIV)
);
impl_encode_test!(
    test_encode_inst_mod,
    SVMCell::InstCell(Inst::MOD)
);
impl_encode_test!(
    test_encode_inst_fdiv,
    SVMCell::InstCell(Inst::FDIV)
);
impl_encode_test!(
    test_encode_inst_eq,
    SVMCell::InstCell(Inst::EQ)
);
impl_encode_test!(
    test_encode_inst_gt,
    SVMCell::InstCell(Inst::GT)
);
impl_encode_test!(
    test_encode_inst_gte,
    SVMCell::InstCell(Inst::GTE)
);
impl_encode_test!(
    test_encode_inst_lt,
    SVMCell::InstCell(Inst::LT)
);
impl_encode_test!(
    test_encode_inst_lte,
    SVMCell::InstCell(Inst::LTE)
);
impl_encode_test!(
    test_encode_inst_atom,
    SVMCell::InstCell(Inst::ATOM)
);
impl_encode_test!(
    test_encode_inst_null,
    SVMCell::InstCell(Inst::NULL)
);
impl_encode_test!(
    test_encode_inst_cons,
    SVMCell::InstCell(Inst::CONS)
);
impl_encode_test!(
    test_encode_inst_car,
    SVMCell::InstCell(Inst::CAR)
);
impl_encode_test!(
    test_encode_inst_cdr,
    SVMCell::InstCell(Inst::CDR)
);
impl_encode_test!(
    test_encode_inst_ldc,
    SVMCell::InstCell(Inst::LDC)
);
impl_encode_test!(
    test_encode_inst_stop,
    SVMCell::InstCell(Inst::STOP)
);
