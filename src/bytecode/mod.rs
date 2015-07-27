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
//!   0x1D  | STOP          |
//!   0x1E  | reserved      |
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
//!    identify extant atom types, while 0xC5 ... 0xCE are reserved for future use.
//!
//!    Once an atom constant identifying byte is read, the bytes that follow it will be read as
//!    that type of atom. The number of bytes read depends on the length of the atom type, which is
//!    determined using the identifying bytes. The following identifying bytes correspond to the
//!    following atom types:
//!
//! + 0xC1: uint atom (64-bit unsigned integer)
//! + 0xC2: sint atom (64-bit signed integer)
//! + 0xC3: char atom (32-bit Unicode scalar value)
//! + 0xC4: float atom (64-bit double-precision floating point number
//!
//!    If additional primitive data types are added to the Seax VM, the bytes 0xC5 to 0xCF will
//!    be used to identify those types.
//!
//!    Note that the type tag identifying a constant may be extracted by byte-masking the
//!    identifying byte with the number 0x0F.
//!

extern crate byteorder;

use self::byteorder::{ByteOrder, BigEndian, ReadBytesExt, WriteBytesExt};

use std::error::Error;
use std::io::Read;
use std::fmt;
use std::char;

use super::slist::List;
use super::slist::List::*;
use super::{SVMCell,Atom,Inst};
use super::SVMCell::*;
use super::Atom::*;
use super::Inst::*;

#[cfg(test)]
mod tests;

/// exported constants
#[stable(feature="decode", since="0.3.0")]
pub const IDENT_BYTES: u16 = 0x5ECD;
#[stable(feature="decode", since="0.3.0")]
pub const VERSION: u16     = 0x0000;

/// block reserved for future opcodes
const RESERVED_START: u8  = 0x1E;
const RESERVED_LEN: u8    = 0x12;
/// block reserved for typetags
const CONST_START: u8     = 0xC1;
const CONST_LEN: u8       = 0x0E;
/// important bytecodes
const BYTE_CONS: u8       = 0xC0;
const BYTE_NIL: u8        = 0x00;

#[unstable(feature="decode")]
pub fn decode_program<R>(source: R) -> Result<List<SVMCell>, String>
    where R: Read
{
    unimplemented!()
    // let mut decoder = Decoder::new(&mut source);
    // decoder
    //     .check_ident_bytes()
    //     .and_then(|| decoder.check_version()
    //                         .or_else(|why| { warn!("{}", why); Ok(()) })
    //         )
    //     .and_then(||
    //         unimplemented!() // todo: build list from iterator in error-safe way
    //         )

}

#[stable(feature="decode", since="0.2.6")]
pub struct Decoder<'a, R: 'a> {
    source: &'a mut R,
    num_read: usize
}

#[stable(feature="decode", since="0.2.6")]
fn decode_inst(byte: &u8) -> Result<Inst, String> {
    match *byte {
        BYTE_NIL => Ok(NIL),
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
        0x1A => Ok(CAR),
        0x1B => Ok(CDR),
        0x1C => Ok(LDC),
        0x1D => Ok(STOP),
        b if b >= RESERVED_START &&
             b <= (RESERVED_START + RESERVED_LEN) =>
            Err(format!("Unimplemented: reserved byte {:#X}", b)),
        b if b > (RESERVED_START + RESERVED_LEN) =>
            Err(String::from("byte too high")),
        _  => unreachable!() // Should require an act of God.
    }
}


#[stable(feature="decode", since="0.2.6")]
impl<'a, R> Decoder<'a, R>
    where R: Read
{
    #[stable(feature="decode", since="0.3.0")]
    pub fn check_ident_bytes(&mut self) -> Result<(), String> {
        self.source
            .read_u16::<BigEndian>()
            .map_err(|why| String::from(why.description()))
            .and_then(|ident| {
                self.num_read += 2;
                match ident {
                    IDENT_BYTES => Ok(()),
                    other_bytes => Err(
                        format!("invalid identifying bytes {:#X}", other_bytes)
                    )
                }
            })
    }

    #[stable(feature="decode", since="0.3.0")]
    pub fn check_version(&mut self) -> Result<(), String> {
        self.source
            .read_u16::<BigEndian>()
            .map_err(|why| String::from(why.description()))
            .and_then(|version| {
                self.num_read += 2;
                match version {
                    VERSION => Ok(()),
                    bytes   => Err( // I expect this will generate a warning
                                    // at the call site...
                        format!("mismatched version {}, expected {}",
                            bytes, version)
                    )
                }
            })
    }

    #[stable(feature="decode", since="0.2.6")]
    pub fn new(src: &'a mut R) -> Decoder<'a, R> {
        Decoder {
            source: src,
            num_read: 0
        }
    }

    /// Returns the number of bytes read by the decoder
    #[stable(feature="decode", since="0.2.6")]
    pub fn num_read(&self) -> usize {
        self.num_read
    }

    #[stable(feature="decode", since="0.2.6")]
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
    // Decodes a CONS cell
    #[stable(feature="decode", since="0.2.6")]
    fn decode_cons(&mut self) -> Result<Option<Box<List<SVMCell>>>, String> {
        self.next_cell()
            .and_then(|car|
                car.ok_or(String::from("EOF while decoding CONS cell"))
            )
            .map(|car| {
                debug!("Decoded {:?}, {} bytes read", car, self.num_read);
                car
            })
            .and_then(|car| {
                let mut buf = [0;1];
                try!(self.source.read(&mut buf) // try to get next byte
                         .map_err(|why| String::from(why.description())));
                self.num_read += 1;
                match buf[0] {
                    BYTE_CONS =>
                        self.decode_cons()
                            .and_then(|cdr| cdr.ok_or(
                                String::from("EOF while decoding CONS")) )
                            .map( |cdr| (car, cdr) ),
                    BYTE_NIL  => Ok((car, box Nil)),
                    b         => Err(
                        format!("Unexpected byte {:#X} while decoding CONS", b)
                    )
                }
            })
            .map(|(car, cdr)| Some(box Cons(car, cdr)) )
    }

    /// Decodes the next cell in the source
    #[stable(feature="decode", since="0.2.6")]
    pub fn next_cell(&mut self) -> Result<Option<SVMCell>,String> {
        let mut buf = [0;1];
        match self.source.read(&mut buf) {
            Ok(1)   => { // a byte was read
                self.num_read += 1;
                debug!("Read {:#X}, {} bytes read", buf[0], self.num_read);
                match buf[0] {
                    b if b < 0x30 => decode_inst(&b)
                                        .map(SVMCell::InstCell)
                                        .map(Some),
                    b if b >= CONST_START &&
                         b < (CONST_START + CONST_LEN) =>
                                    self.decode_const(&b)
                                        .map(SVMCell::AtomCell)
                                        .map(Some),
                    BYTE_CONS    => self.decode_cons()
                                        .map(|cell|
                                              cell.map(SVMCell::ListCell)
                                        ),
                    b            => Err(format!("Unsupported byte {:#X}", b))
                }
            },
            Ok(0)    => Ok(None), //  we're out of bytes - EOF
            Ok(_)    => unreachable!(), //
            Err(why) => Err(String::from(why.description()))
        }
    }

}

#[stable(feature="decode", since="0.2.6")]
impl<'a, R> Iterator for Decoder<'a, R> where R: Read {
    #[stable(feature="decode", since="0.2.6")]
    type Item = SVMCell;

    #[stable(feature="decode", since="0.2.6")]
    fn next(&mut self) -> Option<SVMCell> {
        self.next_cell()
            .unwrap()
    }
}
#[stable(feature="decode", since="0.2.6")]
impl<'a, R> fmt::Debug for Decoder<'a, R>  where R: fmt::Debug {
    #[stable(feature="decode", since="0.2.6")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Decoding from: {:?}, {} bytes read",
            self.source,
            self.num_read
        )
    }

}

#[stable(feature="encode", since="0.2.6")]
pub trait Encode {
    #[stable(feature="encode", since="0.2.6")]
    fn emit(&self) -> Vec<u8>;
}

#[stable(feature="encode", since="0.2.6")]
impl Encode for SVMCell {
    #[stable(feature="encode", since="0.2.6")]
    fn emit(&self) -> Vec<u8> {
        match *self {
            AtomCell(ref atom) => atom.emit(),
            InstCell(ref inst) => inst.emit(),
            ListCell(box ref list) => list.emit()
        }
    }
}

#[stable(feature="encode", since="0.2.6")]
impl Encode for Atom {
    #[stable(feature="encode", since="0.2.6")]
    fn emit(&self) -> Vec<u8> {
        match *self {
            UInt(value) => {
                let mut buf = vec![0xC1];
                buf.write_u64::<BigEndian>(value)
                   .unwrap();
                buf
            },
            SInt(value) => {
                let mut buf = vec![0xC2];
                buf.write_i64::<BigEndian>(value)
                   .unwrap();
                buf
            },
            Char(value) => {
                let mut buf = vec![0xC3];
                buf.write_u32::<BigEndian>(value as u32)
                   .unwrap();
                buf
            },
            Float(value) => {
                let mut buf = vec![0xC4];
                buf.write_f64::<BigEndian>(value)
                   .unwrap();
                buf
            }
        }
    }
}

#[stable(feature="encode", since="0.2.6")]
impl Encode for Inst {
    #[stable(feature="encode", since="0.2.6")]
    fn emit(&self) -> Vec<u8> {
        match *self {
            NIL     => vec![BYTE_NIL],
            LD      => vec![0x01],
            LDF     => vec![0x02],
            AP      => vec![0x03],
            APCC    => vec![0x04],
            JOIN    => vec![0x05],
            RAP     => vec![0x06],
            RET     => vec![0x07],
            DUM     => vec![0x08],
            SEL     => vec![0x09],
            ADD     => vec![0x0A],
            SUB     => vec![0x0B],
            MUL     => vec![0x0C],
            DIV     => vec![0x0D],
            MOD     => vec![0x0E],
            FDIV    => vec![0x0F],
            EQ      => vec![0x10],
            GT      => vec![0x11],
            GTE     => vec![0x12],
            LT      => vec![0x13],
            LTE     => vec![0x14],
            ATOM    => vec![0x15],
            NULL    => vec![0x16],
            READC   => vec![0x17],
            WRITEC  => vec![0x18],
            CONS    => vec![0x19],
            CAR     => vec![0x1A],
            CDR     => vec![0x1B],
            LDC     => vec![0x1C],
            STOP    => vec![0x1D]
        }
    }
}

#[stable(feature="encode", since="0.2.6")]
impl<T> Encode for List<T> where T: Encode {
    #[stable(feature="encode", since="0.2.6")]
    fn emit(&self) -> Vec<u8> {
        match *self {
            Cons(ref it, box ref tail) => {
                let mut result = vec![BYTE_CONS];
                result.push_all(&it.emit());
                result.push_all(&tail.emit());
                result
            },
            Nil => vec![BYTE_NIL]
        }
    }
}
