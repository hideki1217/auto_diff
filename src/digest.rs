
pub type T = f64;
#[derive(Debug)]
pub struct Error;

pub trait Eval {
    fn eval(&self, x: T) -> T;
}
pub trait Diff: Copy {
    type Result;
    fn diff(&self) -> Result<Self::Result, Error>;
}

