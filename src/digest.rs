use std::fmt;
use num::{Zero, One};


pub type T = f64;
#[derive(Debug)]
pub struct Error;

#[derive(Clone, Copy, Debug)]
pub struct Const(pub T);

#[derive(Clone, Copy, Debug)]
pub struct Var;

pub trait Eval {
    fn eval(&self, x: T) -> T;
    fn const_eval(&self) -> Result<Const,()>;
}
pub trait Diff: Copy {
    type Result;
    fn diff(&self) -> Result<Self::Result, Error> {
        Err(Error)
    }
}


impl Diff for Const {
    type Result = Const;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Const(T::zero()))
    }
}
impl Eval for Const {
    fn eval(&self, _: T) -> T {
        self.0
    }
    fn const_eval(&self) -> Result<Const,()> {
        Ok(*self)
    }
}
impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl Diff for Var {
    type Result = Const;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Const(T::one()))
    }
}
impl Eval for Var {
    fn eval(&self, x: T) -> T {
        x
    }

    fn const_eval(&self) -> Result<Const,()> {
        Err(())
    }
}
impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x")
    }
}

