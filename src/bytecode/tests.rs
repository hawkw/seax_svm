use super::Decoder;
use ::cell::Atom;

use quickcheck::quickcheck;

#[test]
fn prop_encode_float () {
    fn prop (x: f64) -> bool {
        let cell = Atom::Float(x);
        Decoder::new(&cell.emit()).decode_const() == cell
    }
    quickcheck(prop as fn(f64) -> bool);
}
