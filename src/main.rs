use auto_diff::digest::{Diff, Eval, Const, Var};
use auto_diff::ops::*;
use auto_diff::wrap_for_diff_and_eval;


wrap_for_diff_and_eval!(Relu, Step, |x| if x >= 0.0 { x } else { 0.0 } );
wrap_for_diff_and_eval!(Step, |x| if x >= 0.0 { 1.0 } else { 0.0 });

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
