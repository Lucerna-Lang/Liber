//! Library for functions that create or modify strings
//! Currently: toString,

use crate::{AsFuncObj, Lib};
use structures::structs::{DefaultTypes, Env};

pub fn to_string(_: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let t = v.remove(0);
    if let DefaultTypes::Int(s) = t {
        vec![DefaultTypes::Str(s.to_string())]
    } else if let DefaultTypes::Bool(b) = t {
        vec![DefaultTypes::Str(b.to_string())]
    } else {
        v
    }
}

pub fn load() -> Lib {
    let mut s = Lib::new("strings");
    s.add("toString", to_string.as_obj());
    s
}
