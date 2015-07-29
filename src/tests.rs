use ::slist::Stack;
use ::slist::List::{Cons,Nil};
use super::State;
use super::cell::Atom::*;
use super::cell::SVMCell::*;
use super::Inst::*;

use quickcheck::quickcheck;

use test::Bencher;
/*
#[test]
#[should_panic(expected="[fatal]: expected an instruction on control stack")]
fn test_empty_eval_fail() {
    State::new().eval(None,false);
}

#[test]
#[should_panic(expected="List index 0 out of range")]
fn test_ld_empty_env_fail() {
    State {
        stack:      Stack::empty(),
        env:        Stack::empty(),
        control:    list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(1)), AtomCell(SInt(0))))),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="[fatal][LD]: expected list in $e, found 'w'")]
fn test_ld_unexpected_env_fail() {
    State {
        stack:      Stack::empty(),
        env:        list!(AtomCell(Char('w'))),
        control:    list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(1)), AtomCell(SInt(1))))),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="fatal][LD]: expected pair, found (0 . nil)\n[fatal] new control: nil")]
fn test_ld_arg_too_short_fail() {
    State {
        stack:      Stack::empty(),
        env:        Stack::empty(),
        control:    list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(0))))),
        dump:       Stack::empty(),
    }.eval(None, false);
}
#[test]
#[should_panic(expected="[fatal][LD]: expected pair, found (0 . (1 . (1 . nil)))\n[fatal] new control: nil")]
fn test_ld_arg_too_long_fail() {
    State {
        stack:      Stack::empty(),
        env:        Stack::empty(),
        control:    list!(InstCell(LD),ListCell(box list!(AtomCell(SInt(0)), AtomCell(SInt(1)), AtomCell(SInt(1))))),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="[fatal][ADD]: expected first operand, found Some(((1 . nil), nil))")]
fn test_add_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(ADD)),
        dump:       Stack::empty(),
    }.eval(None, false);
}


#[test]
#[should_panic(expected="[fatal][SUB]: expected first operand, found Some(((1 . nil), nil))")]
fn test_sub_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(SUB)),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="[fatal][DIV]: expected first operand, found Some(((1 . nil), nil))")]
fn test_div_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(DIV)),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="[fatal][FDIV]: Expected first operand to be atom, found list or instruction")]
fn test_fdiv_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(FDIV)),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="[fatal][MUL]: expected first operand, found Some(((1 . nil), nil))")]
fn test_mul_unexpected_first_arg_fail () {
    State {
        stack:      list!(ListCell(box list!(AtomCell(SInt(1))))),
        env:        Stack::empty(),
        control:    list!(InstCell(MUL)),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="[fatal][ADD]: expected second operand, found Some((nil, nil))")]
fn test_add_type_error () {
    State {
        stack:      list!(AtomCell(SInt(1)), list_cell![]),
        env:        Stack::empty(),
        control:    list!(InstCell(ADD)),
        dump:       Stack::empty(),
    }.eval(None, false);
}
#[test]
#[should_panic(expected="[fatal][SUB]: expected second operand, found Some((nil, nil))")]
fn test_sub_type_error () {
    State {
        stack:      list!(AtomCell(SInt(1)), list_cell![]),
        env:        Stack::empty(),
        control:    list!(InstCell(SUB)),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="[fatal][DIV]: expected second operand, found Some((nil, nil))")]
fn test_div_type_error () {
    State {
        stack:      list!(AtomCell(SInt(1)), list_cell![]),
        env:        Stack::empty(),
        control:    list!(InstCell(DIV)),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="[fatal][FDIV]: TypeError: expected compatible operands, found (FDIV 1 nil)")]
fn test_fdiv_type_error () {
   State {
        stack:      list!(AtomCell(SInt(1)), list_cell![]),
        env:        Stack::empty(),
        control:    list!(InstCell(FDIV)),
        dump:       Stack::empty(),
    }.eval(None, false);
}

#[test]
#[should_panic(expected="[fatal][MUL]: expected second operand, found Some((nil, nil))")]
fn test_mul_type_error () {
    State {
        stack:      list!(AtomCell(SInt(1)), list_cell![]),
        env:        Stack::empty(),
        control:    list!(InstCell(MUL)),
        dump:       Stack::empty(),
    }.eval(None, false);
}*/

// ----- QuickCheck property tests (WIP) ------------------------------
#[test]
fn prop_eval_ldc_sint () {
    fn prop  (x: i64) -> bool {
        let state = State {
            stack: Stack::empty(),
            env: Stack::empty(),
            control: list!(InstCell(LDC),AtomCell(SInt(x))),
            dump: Stack::empty()
        }.eval(None, true).unwrap().0;

        state.stack.peek() == Some(&AtomCell(SInt(x)))
    }
    quickcheck(prop as fn(i64) -> bool);
}

#[test]
fn prop_eval_ldc_uint () {
    fn prop (x: u64) -> bool {
        let state = State {
            stack: Stack::empty(),
            env: Stack::empty(),
            control: list!(InstCell(LDC),AtomCell(UInt(x))),
            dump: Stack::empty()
        }.eval(None, true).unwrap().0;

        state.stack.peek() == Some(&AtomCell(UInt(x)))
    }

    quickcheck(prop as fn(u64) -> bool);
}

#[test]
fn prop_eval_ldc_float () {
    fn prop (x: f64) -> bool {
        let state = State {
            stack: Stack::empty(),
            env: Stack::empty(),
            control: list!(InstCell(LDC),AtomCell(Float(x))),
            dump: Stack::empty()
        }.eval(None, true).unwrap().0;

        state.stack.peek() == Some(&AtomCell(Float(x)))
    }
    quickcheck(prop as fn(f64) -> bool);
}

#[test]
fn test_empty_state() {
    let state = State::new();
    assert_eq!(state.stack.length(), 0);
    assert_eq!(state.env.length(), 0);
    assert_eq!(state.control.length(), 0);
    assert_eq!(state.dump.length(), 0);
}

#[test]
fn test_eval_nil () {
    let mut state = State {
        stack: Stack::empty(),
        env: Stack::empty(),
        control: list!(InstCell(NIL),AtomCell(SInt(1))),
        dump: Stack::empty()
    };
    assert_eq!(state.stack.peek(), None);
    state = state.eval(None,true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));
}

#[test]
fn test_eval_ldc () {
    let mut state = State::new();
    assert_eq!(state.stack.peek(), None);
    state = State {
        stack: state.stack,
        env: state.env,
        control: list!(InstCell(LDC),AtomCell(SInt(1))),
        dump: state.dump
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(1))));

    state = State {
        stack: state.stack,
        env: state.env,
        control: list!(InstCell(LDC),AtomCell(Char('a'))),
        dump: state.dump
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Char('a'))));

    state = State {
        stack: state.stack,
        env: state.env,
        control: list!(InstCell(LDC),AtomCell(Float(1.0f64))),
        dump: state.dump
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.0f64))));
}

#[test]
fn test_eval_ld () {
    let state = State {
        stack: Stack::empty(),
        env: list!(list_cell![
            AtomCell(SInt(155)),
            AtomCell(UInt(388))
            ]),
        control: list!(
            InstCell(LD),
            list_cell![ AtomCell(SInt(1)), AtomCell(SInt(2)) ]
            ),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(388))));
}

#[test]
fn test_eval_ldf () {
        let state = State {
        stack: Stack::empty(),
        env: list!(
            list_cell![ AtomCell(SInt(155)), AtomCell(UInt(388))   ],
            list_cell![ AtomCell(Float(6.66)), AtomCell(SInt(666)) ]
            ),
        control: list!(InstCell(LDF), list_cell![AtomCell(SInt(133))] ),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(
        state.stack.peek(),
        Some(&list_cell![
                list_cell![ AtomCell(SInt(133)) ],
                list_cell![ AtomCell(SInt(155)), AtomCell(UInt(388)) ]
            ])
    );
}

#[test]
fn test_eval_join() {
    let state = State {
        stack: Stack::empty(),
        env: Stack::empty(),
        control: list!(InstCell(JOIN)),
        dump: list!(list_cell![
                AtomCell(SInt(1)),
                AtomCell(SInt(2))
            ])
    }.eval(None, true).unwrap().0;
    assert_eq!(state.dump.peek(), None);
    assert_eq!(state.control[0u64], AtomCell(SInt(1)));
    assert_eq!(state.control[1u64], AtomCell(SInt(2)));
}

#[test]
fn test_eval_add () {
    // ---- Unsigned int addition ----
    let mut state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(2))));

    // ---- Signed int addition ----
    state = State {
        stack: list!(AtomCell(SInt(-1)), AtomCell(SInt(-1))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-2))));

    // ---- Float-float addition ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(Float(1.5))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0))));

    // ---- Float-int type lifting addition ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(2.5))));
    state = State {
        stack: list!(AtomCell(Float(3.5)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(ADD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(4.5))));
}

#[test]
fn test_eval_sub () {
    // ---- Unsigned int subtraction ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(0))));

    // ---- Signed int subtraction----
    state = State {
        stack: list!(AtomCell(SInt(-3)), AtomCell(SInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-6))));

    // ---- Float-float subtraction ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(-0.5))));

    // ---- Float-int type lifting subtraction ----
    state = State {
        stack: list!(AtomCell(Float(2.5)), AtomCell(SInt(-2))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(4.5))));

    state = State {
        stack: list!(AtomCell(Float(3.5)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(SUB)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));
}

#[test]
fn test_eval_mul () {
    // ---- Unsigned int multiplication ----
    let mut state = State {
        stack: list!(AtomCell(UInt(2)), AtomCell(UInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(6))));

    // ---- Signed int multiplication----
    state = State {
        stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(-3))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(6))));

    // ---- Float-float multiplication ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0))));

    // ---- Float-int type lifting multiplication ----
    state = State {
        stack: list!(AtomCell(Float(1.5)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0))));

    state = State {
        stack: list!(AtomCell(Float(3.5)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MUL)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(7.0))));
}

#[test]
fn test_eval_div () {
    // ---- Unsigned int divison ----
    let mut state = State {
        stack: list!(AtomCell(UInt(6)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(3))));

    // ---- Signed int divison ----
    state = State {
        stack: list!(AtomCell(SInt(-6)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-3))));

    // ---- Float-float divison ----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));

    // ---- Float-int type lifting divison ----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));

    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(DIV)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));
}

#[test]
fn test_eval_fdiv () {
    // ---- Unsigned int divison ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(FDIV)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));

    // ---- Signed int divison ----
    state = State {
        stack: list!(AtomCell(SInt(-3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(FDIV)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(-1.5))));

    // ---- Float-float divison ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(FDIV)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(1.5))));
}

#[test]
fn test_eval_mod () {
    // ---- Unsigned int modulus ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(UInt(3%2))));

    // ---- Signed int modulus ----
    state = State {
        stack: list!(AtomCell(SInt(-3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(-3%2))));

    // ---- Float-float modulus---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0%2.0))));

    // ---- Float-int type lifting modulus----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0%2.0))));

    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(MOD)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Float(3.0%2.0))));
}

#[test]
fn test_eval_eq () {
    // ---- Unsigned int equality ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    // ---- Signed int equality ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(3))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));


    // ---- Float equality ----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(3.0))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(-2.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(Float(2.11)), AtomCell(Float(2.1))),
        env: Stack::empty(),
        control: list!(InstCell(EQ)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

}

#[test]
fn test_eval_gt () {
    // ---- Unsigned int greater-than ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    // ---- Signed int greater-than ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));


    // ---- Float greater-than----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(1.0))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(-2.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(Float(2.11)), AtomCell(Float(2.1))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    // ---- Mixed type greater-than ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

}

#[test]
fn test_eval_gte () {
    // ---- Unsigned int greater-than ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));


    // ---- Signed int greater-than ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(1)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(1)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));


    // ---- Float greater-than----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(Float(1.0))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    // ---- Mixed type greater-than-equal ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(GTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));
}

#[test]
fn test_eval_lt () {
    // ---- Unsigned int greater-than ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));


    // ---- Signed int greater-than ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(SInt(-2)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(2)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));


    // ---- Float greater-than----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(1.0))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(Float(-2.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(2.11)), AtomCell(Float(2.1))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

       state = State {
        stack: list!(AtomCell(Float(2.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    // ---- Mixed type greater-than ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LT)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

}

#[test]
fn test_eval_lte () {
    // ---- Unsigned int greater-than ----
    let mut state = State {
        stack: list!(AtomCell(UInt(3)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(UInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );


    // ---- Signed int greater-than ----
    state = State {
        stack: list!(AtomCell(SInt(3)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(SInt(1)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(SInt(1)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );


    // ---- Float greater-than----
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(Float(1.0))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    // ---- Mixed type greater-than-equal ---
    state = State {
        stack: list!(AtomCell(Float(3.0)), AtomCell(SInt(2))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(AtomCell(Float(1.0)), AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![])
        // TODO: this expects wrong float behaviour, fix
    );

    state = State {
        stack: list!(AtomCell(UInt(1)), AtomCell(Float(2.0))),
        env: Stack::empty(),
        control: list!(InstCell(LTE)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );
}

#[test]
fn test_eval_ret() {
    let state = State {
        stack: list!(AtomCell(SInt(100)), AtomCell(SInt(320))),
        env: Stack::empty(),
        control: list!(InstCell(RET)),
        dump: list!(
            list_cell![ AtomCell(Char('S')), AtomCell(Char('L')) ],
            list_cell![
                list_cell![ AtomCell(Char('E')), AtomCell(Char('L')) ],
                list_cell![ AtomCell(Char('E')), AtomCell(Char('D')) ]
            ],
            list_cell![ AtomCell(Char('C')), AtomCell(Char('L')) ]
        )
    }.eval(None, true).unwrap().0;
    // stack should have return arg + first elem on dump
    assert_eq!(state.stack.peek(), Some(&AtomCell(SInt(100)))); // test these using peek for now since indexing is borked
    assert_eq!(state.stack[0u64], AtomCell(SInt(100)));
    assert_eq!(state.stack[1u64], AtomCell(Char('S')));
    assert_eq!(state.stack[2u64], AtomCell(Char('L')));
    // env should have second element from dump
    assert_eq!(
        state.env.peek(),
        Some(&list_cell![ AtomCell(Char('E')), AtomCell(Char('L')) ])
        );
    assert_eq!(
        state.env[0u64],
        list_cell![ AtomCell(Char('E')), AtomCell(Char('L')) ]
        );
    assert_eq!(
        state.env[1u64],
        list_cell![ AtomCell(Char('E')), AtomCell(Char('D')) ]
        );
    // control should have third element from dump
    assert_eq!(state.control.peek(), Some(&AtomCell(Char('C'))));
    assert_eq!(state.control[0u64], AtomCell(Char('C')));
    assert_eq!(state.control[1u64], AtomCell(Char('L')));
    assert_eq!(state.dump.peek(), None);
}

#[test]
fn test_eval_dum() {
    let state = State {
        stack: Stack::empty(),
        env: list!(list_cell![ AtomCell(Char('a')) ]),
        control: list!(InstCell(DUM)),
        dump: Stack::empty(),
    }.eval(None, true).unwrap().0;
    assert_eq!(state.env.peek(), Some(&list_cell![]));
}

#[test]
fn test_eval_ap() {
    let state = State {
        stack: list!(list_cell![
                list_cell![
                    InstCell(RET),
                    InstCell(ADD),
                    AtomCell(SInt(1)),
                    InstCell(LDC),
                    list_cell![
                        AtomCell(UInt(0)),
                        AtomCell(UInt(0))
                    ],
                    InstCell(LD)
                ],
                list_cell![ list_cell![ AtomCell(SInt(1)) ] ]
            ],
            list_cell![ AtomCell(Char('Q')) ]
            ),
        env: list!(list_cell![ AtomCell(Char('D')) ]),
        control: list!(InstCell(AP), InstCell(DUM)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), None );
    assert_eq!(
        state.control,
        list!(
            InstCell(RET),
            InstCell(ADD),
            AtomCell(SInt(1)),
            InstCell(LDC),
            list_cell![ AtomCell(UInt(0)), AtomCell(UInt(0)) ],
            InstCell(LD)));
    assert_eq!(
        state.env,
        list!(
            list_cell![ AtomCell(Char('Q')) ],
            list_cell![ AtomCell(SInt(1)) ]
        )
    );
    //assert_eq!(state.dump, list!(ListCell(box list!(InstCell(DUM))),ListCell(box list!(ListCell(box list!(AtomCell(Char('D'))))))));
}

#[test]
fn test_eval_atom() {
    // true cases
    let mut state = State {
        stack: list!(AtomCell(SInt(1))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(UInt(0))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Char('C'))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Char('A'))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    state = State {
        stack: list!(AtomCell(Float(1.23f64))),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert!(
        state.stack.peek() != Some(&list_cell![]) &&
        state.stack.peek() != None
        );

    // false cases
    state = State {
        stack: list!(InstCell(DUM)),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));

    state = State {
        stack: list!(list_cell![]),
        env: Stack::empty(),
        control: list!(InstCell(ATOM)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![]));
}

#[test]
fn test_eval_car() {
    let state = State {
        stack: list!(list_cell![ AtomCell(Char('A')),AtomCell(Char('B')) ]),
        env: Stack::empty(),
        control: list!(InstCell(CAR)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&AtomCell(Char('A'))));
}

#[test]
fn test_eval_cdr() {
    let state = State {
        stack:list!(list_cell![ AtomCell(Char('A')),AtomCell(Char('B')) ]),
        env: Stack::empty(),
        control: list!(InstCell(CDR)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), Some(&list_cell![ AtomCell(Char('B')) ]) );
}

#[test]
fn test_eval_cons() {
    let state = State {
        stack: list!(
            AtomCell(Char('A')),
            list_cell![ AtomCell(Char('B')) ,AtomCell(Char('C')) ]
            ),
        env: Stack::empty(),
        control: list!(InstCell(CONS)),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(
        state.stack.peek(),
        Some(&list_cell![
            AtomCell(Char('A')), AtomCell(Char('B')), AtomCell(Char('C'))
        ])
    );
}

#[test]
fn test_eval_sel_true() {
    // true case
    let state = State {
        stack: list!(list_cell![]),
        env: Stack::empty(),
        control: list!(
            InstCell(SEL),
            list_cell![ InstCell(ATOM) ], // should be on stack if true
            list_cell![ InstCell(NIL) ], // should be on stack if false
            InstCell(JOIN) // this is just here so that we can assert that it goes on the dump
            ),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), None); // stack should be empty
    assert_eq!(state.control.peek(), Some(&InstCell(NIL)));
    assert_eq!(state.dump.peek(), Some(&list_cell![ InstCell(JOIN) ]) ); // next instruction on dump
}

#[test]
fn test_eval_sel_false() {
    // false case
    let state = State {
        stack: list!(list_cell![ AtomCell(SInt(1)) ]),
        env: Stack::empty(),
        control: list!(
            InstCell(SEL),
            list_cell![ InstCell(ATOM) ], // should be on stack if true
            list_cell![ InstCell(NIL) ], // should be on stack if false
            InstCell(JOIN) // this is just here so that we can assert that it goes on the dump
            ),
        dump: Stack::empty()
    }.eval(None, true).unwrap().0;
    assert_eq!(state.stack.peek(), None); // stack should be empty
    assert_eq!(state.control.peek(), Some(&InstCell(ATOM)));
    assert_eq!(state.dump.peek(), Some(&list_cell![ InstCell(JOIN) ]) ); // next instruction on dump
}

#[test]
fn test_eval_null() {
    // true case
    assert_eq!(
        State {
            stack: list!(AtomCell(SInt(1))),
            env: Stack::empty(),
            control: list!(InstCell(NULL)),
            dump: Stack::empty(),
        }.eval(None,true).unwrap().0.stack.peek(),
        Some(&list_cell![])
        );
    // false case
    assert_eq!(
        State {
            stack: list!(list_cell![]),
            env: Stack::empty(),
            control: list!(InstCell(NULL)),
            dump: Stack::empty(),
        }.eval(None,true).unwrap().0.stack.peek(),
        Some(&list_cell![ AtomCell(SInt(1)) ])
        );
}

#[bench]
fn bench_list_creation(b: &mut Bencher) {
    b.iter(|| {
        super::eval_program(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS)
        ), true)
    })
}

#[bench]
fn bench_list_car(b: &mut Bencher) {
    b.iter(|| {
        super::eval_program(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(CAR)
        ), true)
    })
}

#[bench]
fn bench_list_cdr(b: &mut Bencher) {
    b.iter(|| {
        super::eval_program(list!(
            InstCell(NIL),
            InstCell(LDC), AtomCell(SInt(10)), InstCell(CONS),
            InstCell(LDC), AtomCell(SInt(20)), InstCell(CONS),
            InstCell(CDR)
        ), true)
    })
}

#[bench]
fn bench_simple_add(b: &mut Bencher) {
    b.iter(|| {
        super::eval_program(list!(
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(ADD)
        ), true)
    })
}

#[bench]
fn bench_nested_arith(b: &mut Bencher) {
    b.iter(|| {
        super::eval_program(list!(
            InstCell(LDC), AtomCell(SInt(5)),
            InstCell(LDC), AtomCell(SInt(5)),
            InstCell(ADD),
            InstCell(LDC), AtomCell(SInt(20)),
            InstCell(SUB)
        ), true)
    })
}

#[bench]
fn bench_basic_branching_1(b: &mut Bencher) {
    b.iter(|| {
        super::eval_program(list!(
            InstCell(LDC), AtomCell(SInt(1)), InstCell(LDC), AtomCell(SInt(1)),
            InstCell(SUB),
            InstCell(LDC), AtomCell(SInt(0)),
            InstCell(EQ),
            InstCell(SEL),
            list_cell![
                InstCell(LDC),
                AtomCell(SInt(1)),
                InstCell(JOIN)
            ],
            list_cell![
                InstCell(NIL),
                InstCell(JOIN)
            ]
        ), true)
    })
}

#[bench]
fn bench_basic_branching_2(b: &mut Bencher) {
    b.iter(|| {
        super::eval_program(list!(
            InstCell(NIL), InstCell(NULL),
            InstCell(SEL),
                list_cell![ InstCell(LDC), AtomCell(SInt(10)), InstCell(JOIN) ],
                list_cell![ InstCell(LDC), AtomCell(SInt(20)), InstCell(JOIN) ],
            InstCell(LDC), AtomCell(SInt(10)),
            InstCell(ADD)
        ), true)
    })
}
