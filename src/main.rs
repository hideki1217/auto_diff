use core::fmt;

use auto_diff::digest::{Diff, Eval, T};
use auto_diff::ops::*;

#[derive(Clone, Copy, Debug)]
struct Relu<X>(X);
impl<X: Diff> Diff for Relu<X> {
    type Result = Mul<Step<X>,X::Result>;
    fn diff(&self) -> Result<Self::Result, auto_diff::digest::Error> {
        Ok(Mul(Step(self.0), self.0.diff()?))
    }
}
impl<X:Eval> Eval for Relu<X> {
    fn eval(&self, x: T) -> T {
        let x = self.0.eval(x);
        if x >= 0.0 { x } else { 0.0 }
    }
}

#[derive(Clone, Copy, Debug)]
struct Step<X>(X);
impl<X: Diff> Diff for Step<X>{
    type Result = ();
}
impl<X:Eval> Eval for Step<X> {
    fn eval(&self, x: T) -> T {
        let x = self.0.eval(x);
        if x >= 0.0 { 1.0 } else { 0.0 }
    }
}


fn main() {
    let sigmoid = Mul(Exp(Var), Add(Const(1.0), Exp(Var)));
    let sigmoid_diff = sigmoid.diff().unwrap();
    println!("{}", sigmoid_diff);

    let x = 1.0_f64.exp();
    let relu = Relu(Log(Var));
    let relu_diff = relu.diff().unwrap();
    println!("relu({x}) = {}", relu.eval(x));
    println!("relu_diff({x}) = {}", relu_diff.eval(x));

    assert!(relu_diff.diff().is_err());
}
