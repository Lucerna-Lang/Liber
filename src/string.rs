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

pub fn index_string(_: &mut Env, mut values: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let original_string_type = values.remove(0);
    let index_type = values.remove(0);
    dbg!("Called index string");
    match (original_string_type, index_type) {
        (DefaultTypes::Str(original_string), DefaultTypes::Int(index)) => {
            let indexed_char = original_string.chars().nth(index as usize).unwrap();
            dbg!(&indexed_char);
            vec![DefaultTypes::Str(indexed_char.to_string())]
        },
        _ => {
            values
        }
    }
}

pub fn load() -> Lib {
    let mut s = Lib::new("strings");
    s.add("toString", to_string.as_obj());
    s.add("indexString", index_string.as_obj());
    s
}
