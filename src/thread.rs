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
            e.exit("Attempting to wait a negative amount of seconds", e.cline())
        }
        let dur = Duration::from_millis(s as u64);
        sleep(dur);
        v
    } else {
        e.exit("Wait did not get a number of milliseconds", e.cline());
    }
}

pub fn load() -> Lib {
    let mut s = Lib::new("thread");
    s.add("wait", wait.as_obj());
    s
}
