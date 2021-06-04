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

pub fn index_string(_: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let t = v.remove(0);
    let t1 = v.remove(0);
    match (t, t1) {
        (DefaultTypes::Str(s), DefaultTypes::Int(i)) => {
            vec![DefaultTypes::Str(s[i as usize..i as usize].to_string())]
        },
        _ => {
            v
        }
    }
}

pub fn load() -> Lib {
    let mut s = Lib::new("strings");
    s.add("toString", to_string.as_obj());
    s.add("indexString", index_string.as_obj());
    s
}
