//! Library for functions that only work with numbers
//! Currently: multiply, larger, abs, neg (Division is not implemented, because decimals aren't)

use crate::{AsFuncObj, Lib};
use structures::structs::{DefaultTypes, Env};

pub fn multiply(_: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let int1 = v.remove(0);
    let int2 = v.remove(0);
    match (int1, int2) {
        (DefaultTypes::Int(i1), DefaultTypes::Int(i2)) => {
            vec![DefaultTypes::Int(i1 * i2)]
        }
        (_, _) => v,
    }
}

pub fn larger(_: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let int1 = v.remove(0);
    let int2 = v.remove(0);
    match (int1, int2) {
        (DefaultTypes::Int(i1), DefaultTypes::Int(i2)) => {
            vec![DefaultTypes::Bool(i1 > i2)]
        }
        (_, _) => {
            println!("Attempting to call eq on different types");
            v
        }
    }
}

pub fn abs(_: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let int1 = v.remove(0);
    match int1 {
        DefaultTypes::Int(i1) => {
            vec![DefaultTypes::Int(i1.abs())]
        }
        _ => v,
    }
}

pub fn as_negative(_: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let int1 = v.remove(0);
    match int1 {
        DefaultTypes::Int(i1) => {
            vec![DefaultTypes::Int(-(i1.abs()))]
        }
        _ => v,
    }
}

pub fn load() -> Lib {
    let mut s = Lib::new("math");
    s.add("multiply", multiply.as_obj());
    s.add("larger", larger.as_obj());
    s.add("abs", abs.as_obj());
    s.add("neg", as_negative.as_obj());
    s
}
