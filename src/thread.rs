//! Library for functions that deal with thread
//! Currently: wait,

use crate::{AsFuncObj, Lib};
use std::thread::sleep;
use structures::structs::{DefaultTypes, Env};
use std::time::Duration;

pub fn wait(e: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let t = v.remove(0);
    if let DefaultTypes::Int(s) = t {
        if s<0 {
            panic!("Wait did not get a positive number of milliseconds");
        }
        let dur = Duration::from_millis(s as u64);
        sleep(dur);
        v
    } else {
        println!("Wait did not get a number of milliseconds");
        e.exit();
        v
    }
}

pub fn load() -> Lib {
    let mut s = Lib::new("thread");
    s.add("wait", wait.as_obj());
    s
}
