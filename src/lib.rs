mod math;
mod string;

use structures::structs::{Env, DefaultTypes, Function, Table};
use std::rc::Rc;
pub use math::load as load_math;
pub use string::load as load_string;

pub struct Lib {
    n: &'static str,
    f: Vec<(&'static str, Function)>,
}

impl Lib {
    pub fn new(n: &'static str) -> Self {
        Self {
            n,
            f: vec!()
        }
    }
    pub fn add(&mut self, name: &'static str, s: Function) {
        self.f.push((name, s));
    }
    pub fn load(&self, e: &mut Env) {
        let mut temp_t = Table::new();
        for x in &self.f {
            temp_t.set(x.0.to_string(), DefaultTypes::Function(x.1.clone()));
        }
        e.add_variable(self.n, DefaultTypes::Table(temp_t));
    }

}

trait AsFuncObj {
    fn as_obj(&'static self) -> Function;
}

impl<T> AsFuncObj for T
where T: Fn(&mut Env, Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    fn as_obj(&'static self) -> Function {
        Function::new(Rc::new(self))
    }
}

pub fn loader(e: &mut Env) {
    load_math().load(e);
    load_string().load(e);
}