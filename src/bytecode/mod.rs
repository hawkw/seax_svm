//! Bytecode
//! ========
//!
//! This module contains functions for encoding and decoding Seax bytecode.
//!
//! Seax Bytecode Format
//! ====================
//!
//! Seax Bytecode Standard Revision 0, June 11th, 2015
//!
//! I: The preamble
//! ---------------
//!
//! All Seax Bytecode files begin with a preamble. This preamble consists of the following:
//!
//! 1. The identifying bytes 0x5ECD. These bytes, chosen based on a poorly-advised attempt to
//!    spell out the abbreviation SECD in hexadecimal, identify the file as a Seax bytecode file.
//! 2. A 16-bit unsigned integer that represents the version of the Seax bytecode format that the
//!    file was encoded with. This number is used to determine how the remainder of the file should
//!    be decoded. This document is Revision 0 of the Seax Bytecode format, so the version
//!    should be 0x0000.
//!
//! Future revisions of this standard will allow additional metadata, such as the author's
//! cryptographic signature, checksums for ensuring the executable's integrity, and directives to
//! the virtual machine, to be placed in the preamble as well.
//!
//! II: Instructions
//! ----------------
//!
//! All Seax VM instructions are encoded using single byes. The Seax opcodes occupy the
//! space 0x00 to 0x30, with the bytes 0x1D through 0x3F being reserved for future use.
//!
//! The following table shows all of the currently available SVM opcodes.
//!
//! | Value | Name          | Description
//! +-------+---------------+--------------------------------------------------------------------
//!   0x00  | NIL           | Pushes an empty list (nil) onto `$s`.
//!   0x01  | LD (a . b)    | Pushes the variable at `$e[a][b]` onto the stack.
//!   0x02  | LDF f         | Constructs a closure from the list `f` and the current environment,
//!                           and pushes it to `$s`.
//!   0x03  | AP c          | Applies the function closure `c`.
//!   0x04  | APCC c        | Applies the function closure `c` and pushes a continuation on `$d`.
//!   0x05  | JOIN          | Returns control to the calling scope at the end of a `SEL`.
//!   0x06  | RAP c         | Applies the recursive closure `c`.
//!   0x07  | RET           | Returns control from a function to the calling function.
//!   0x08  | DUM           | Pushes a dummy environment to `$e` for applying a recursive function.
//!   0x09  | SEL a         | Applies the first list of instructions on `$c` if `a` is non-nil,
//!                           or the second list if it is nil.
//!   0x0A  | ADD a b       |
//!   0x0B  | SUB a b       |
//!   0x0C  | MUL a b       |
//!   0x0D  | DIV a b       |
//!   0x0E  | MOD a b       |
//!   0x0F  | FDIV a b      |
//!   0x10  | EQ a b        |
//!   0x11  | GT a b        |
//!   0x12  | GTE a b       |
//!   0x13  | LT a b        |
//!   0x14  | LTE a b       |
//!   0x15  | ATOM a        |
//!   0x16  | NULL a        |
//!   0x17  | READC         |
//!   0x18  | WRITEC        |
//!   0x19  | CONS a b      |
//!   0x1A  | CAR (a . b)   |
//!   0x1B  | CDR (a . b)   |
//!   0x1C  | LDC           |
//!   0x1D  | reserved      |
//!         |     ...       |
//!   0x30  | reserved      |
//!
//! III: Constants
//! --------------
//!
//! Constants are identified by a preceeding constant-identification byte. Constant-identification
//! bytes comprise the range of bytes between 0xC0 and 0xCF, inclusive, and the NIL byte, 0x00.
//!
//! Constants are expected in two places: following either an LDC (LoaD Constant) instruction,
//! or following an instruction that expects a list on $c, such as a SEL instruction.
//!
//! 1. CONS Cells (0xC0)
//!
//!    0xC0 identifies the beginning of a CONS cell constant. It may then be followed by the
//!    identification byte denoting any other constant, which will be interpreted as the CAR part
//!    of the CONS cell. This is followed by the actual bytecode data for the CAR part of the CONS
//!    cell, which is of the length specified by the type identified by the identification byte.
//!
//!    After the constant for the CAR part, there must be an additional identification byte, which
//!    identifies the type of the CONS cell's CDR part. Unlike the CAR part, this may be any
//!    constant type, including another CONS cell. This identification byte is again followed by
//!    the bytecode for the data stored in the CONS cell's CDR part, whose length is determined
//!    by the type identified by the identification byte.
//!
//!    Alternatively, the CAR or CDR parts of a CONS cell may also contain a Seax instruction. In
//!    such a case, the identification byte is replaced by that instruction's opcode. The opcode
//!    comprises the entirity of the CAR or CDR part, and any further data is interpreted as
//!    appropriate (i.e., if the opcode is the CAR part, another opcode or identifying byte will
//!    be expected, while if the opcode is in the CDR part, a new instruction or constant will
//!    be expected.)
//!
//! 2. Atom constants (0xC1 ... 0xCF)
//!
//!    Any constants that are not CONS cells are atom constants. Atom constants are identified by
//!    bytes in the range between 0xC1 and 0xCF, inclusive. Currently, 0xC1, 0xC2, 0xC3, and 0xC4
//!    identify extant atom types, while 0xC5 ... 0xCF are reserved for future use.
//!
//!    Once an atom constant identifying byte is read, the bytes that follow it will be read as
//!    that type of atom. The number of bytes read depends on the length of the atom type, which is
//!    determined using the identifying bytes. The following identifying bytes correspond to the
//!    following atom types:
//!
//! + 0xC1: uint atom (64-bit unsigned integer)
//! + 0xC2: sint atom (64-bit signed integer)
//! + 0xC3: char atom (32-bit Unicode scalar value)
//! + 0xC4: float atom (64-bit double-precision floating point number)
//!
//!    If additional primitive data types are added to the Seax VM, the bytes 0xC5 to 0xCF will
//!    be used to identify those types.
//!
//!    Note that the type tag identifying a constant may be extracted by byte-masking the
//!    identifying byte with the number 0x0F.
//!

extern crate byteorder;

use self::byteorder::BigEndian; // big-endian chosen arbitrarily because I'm Good At Computers
use self::byteorder::{ReadBytesExt,WriteBytesExt};

use std::error::Error;
use std::io::Read;
use std::char;

use super::slist::List;
use super::SVMCell;
use super::SVMCell::*;
use super::Atom;
use super::Inst::*;
use super::Inst;

/// block reserved for future opcodes
const RESERVED_START: u8 = 0x1D;
const RESERVED_LEN: u8   = 0x13;

const VERSION: u16       = 0x0000;

#[unstable(feature="decode")]
pub struct Decoder<'a, R: 'a> {
    source: &'a mut R,
    num_read: usize
}

#[unstable(feature="decode")]
fn decode_inst(byte: &u8) -> Result<Inst, String> {
    match *byte {
        0x00 => Ok(NIL),
        0x01 => Ok(LD),
        0x02 => Ok(LDF),
        0x03 => Ok(AP),
        0x04 => Ok(APCC),
        0x05 => Ok(JOIN),
        0x06 => Ok(RAP),
        0x07 => Ok(RET),
        0x08 => Ok(DUM),
        0x09 => Ok(SEL),
        0x0A => Ok(ADD),
        0x0B => Ok(SUB),
        0x0C => Ok(MUL),
        0x0D => Ok(DIV),
        0x0E => Ok(MOD),
        0x0F => Ok(FDIV),
        0x10 => Ok(EQ),
        0x11 => Ok(GT),
        0x12 => Ok(GTE),
        0x13 => Ok(LT),
        0x14 => Ok(LTE),
        0x15 => Ok(ATOM),
        0x16 => Ok(NULL),
        0x17 => Ok(READC),
        0x18 => Ok(WRITEC),
        0x19 => Ok(CONS),
        0x1A => Ok(CDR),
        0x1B => Ok(CAR),
        0x1C => Ok(LDC),
        b if b >= RESERVED_START && b <= (RESERVED_START + RESERVED_LEN) =>
            Err(format!("Unimplemented: reserved byte {:?}", b)),
        b if b > (RESERVED_START + RESERVED_LEN) => Err(String::from("byte too high")),
        _  => panic!("Got a byte that was weird") // Should require an act of God.
    }
}


#[unstable(feature="decode")]
impl<'a, R> Decoder<'a, R> where R: Read {

    #[unstable(feature="decode")]
    pub fn new(src: &'a mut R) -> Decoder<'a, R>{
        Decoder {
            source: src,
            num_read: 0
        }
    }

    /// Returns the number of bytes read by the decoder
    #[unstable(feature="decode")]
    pub fn num_read(&self) -> usize {
        self.num_read
    }

    #[unstable(feature="decode")]
    fn decode_const(&mut self, byte: &u8) -> Result<Atom, String> {
        match *byte & 0x0F { // extract the type tag
            1 => {
                self.num_read += 8; // this should be more FP i guess
                self.source
                    .read_u64::<BigEndian>()
                    .map(Atom::UInt)
                    .map_err(|why| String::from(why.description()))
                },
            2 => {
                self.num_read += 8;
                self.source
                    .read_i64::<BigEndian>()
                    .map(Atom::SInt)
                    .map_err(|why| String::from(why.description()))
                },
            3 => {
                self.num_read += 4;
                self.source
                    .read_u32::<BigEndian>()
                    .map_err( |why | String::from(why.description()))
                    .and_then(|byte|
                        char::from_u32(byte)
                            .ok_or(String::from("Could not read character."))
                        )
                    .map(Atom::Char)
                },
            4 => {
                self.num_read += 8;
                self.source
                    .read_f64::<BigEndian>()
                    .map(Atom::Float)
                    .map_err(|why| String::from(why.description()))
                },
            _ => unimplemented!()
        }
    }

    fn decode_cons(&mut self) -> Result<Option<Box<List<SVMCell>>>, String> {
        let mut buf = [0;1];
        match self.source.read(&mut buf) {
            Ok(0)       => Err(String::from(
                "Reached end of source unexpectedly while decoding cons cell")),
            Ok(_)       => self.next_cell()
                               .and_then(|car| (car, try!(self.next_cell())) )
                               .and_then(|(car, cdr)| Some(box list!(car,cdr))
                                )
            Err(why)    => Err(String::from(why.description()))
        }

    }

    /// Decodes the next cell in the source
    #[unstable(feature="decode")]
    pub fn next_cell(&mut self) -> Result<Option<SVMCell>,String> {
        let mut buf = [0;1];
        match self.source.read(&mut buf) {
            Ok(1)   => { // a byte was read
                self.num_read += 1;
                match buf[0] {
                    b if b < 0x30               => decode_inst(&b)
                                                       .map(SVMCell::InstCell)
                                                       .map(Some),
                    b if b > 0xC1 && b < 0xCF   => self.decode_const(&b)
                                                       .map(SVMCell::AtomCell)
                                                       .map(Some),
                    0xC0                        => self.decode_cons()
                                                       .map(|cell| cell.map(SVMCell::ListCell)),
                    b                           => Err(format!("Unsupported byte {:?}", b))
                }
            },
            Ok(0)    => Ok(None), //  we're out of bytes - EOF
            Ok(_)    => panic!("[next_cell] Read too many bytes! This shouldn't happen"),
            Err(why) => Err(String::from(why.description()))
        }
    }

}

#[unstable(feature="decode")]
impl<'a, R> Iterator for Decoder<'a, R> where R: Read {
    #[unstable(feature="decode")]
    type Item = SVMCell;

    #[unstable(feature="decode")]
    fn next(&mut self) -> Option<SVMCell> {
        self.next_cell()
            .unwrap()
    }
}

#[unstable(feature="encode")]
pub trait Encode {
    #[unstable(feature="encode")]
    fn emit<'a>(&'a self) -> &'a [u8];
}

#[unstable(feature="encode")]
impl Encode for SVMCell {
    #[unstable(feature="encode")]
    fn emit<'a>(&'a self) -> &'a [u8] {
        match *self {
            AtomCell(ref atom) => atom.emit(),
            InstCell(ref inst) => inst.emit(),
            ListCell(ref list) => list.emit()
        }
    }
}

#[unstable(feature="encode")]
impl Encode for AtomCell {
    #[unstable(feature="encode")]
    fn emit<'a>(&'a self) -> &'a [u8] {
        match *self {
            UInt(ref value) => {
                let mut buf = [0x00; 9];
                buf[0] = 0xC1;
                &buf[1..8].write_u64<BigEndian>(value);
                &'a buf
            },
            _ => unimplemented!()
        }
    }
}

#[unstable(feature="encode")]
impl Encode for InstCell {
    #[unstable(feature="encode")]
    fn emit<'a>(&'a self) -> &'a [u8] {
        unimplemented!()
    }
}

#[unstable(feature="encode")]
impl Encode for ListCell {
    #[unstable(feature="encode")]
    fn emit<'a>(&'a self) -> &'a [u8] {
        unimplemented!()
    }
}
