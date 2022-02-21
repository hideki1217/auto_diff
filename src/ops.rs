
mod std_ops;
mod functions;

pub use std_ops::*;
pub use functions::*;

#[cfg(test)]
mod tests {
    use crate::digest::{Const, Diff, Eval, Var};

    use super::*;

    #[test]
    fn const_test() {
        let x = Const(16_f64);
        let dx = x.diff().unwrap();
        assert_eq!(x.eval(6.0), 16.0);
        assert_eq!(dx.eval(6.0), 0.0);
    }

    #[test]
    fn add_test() {
        let x = Const(16_f64);
        let y = Const(4_f64);
        let x_add_y = Add(x, y);
        let res = x_add_y.diff().unwrap();
        assert_eq!(x_add_y.eval(6.0), 20.0);
        assert_eq!(res.eval(6.0), 0.0);
    }

    #[test]
    fn mul_test() {
        let x = Const(16_f64);
        let y = Var;
        let x_mul_y = Mul(x, y);
        assert_eq!(x_mul_y.eval(4.0), 64.0);
        let res = x_mul_y.diff().unwrap();
        assert_eq!(res.eval(6.0), 16.0);
    }

    #[test]
    fn pow_test() {
        let x = Pow(Var, 6.0);
        assert_eq!(x.eval(2.0), 64.0);
        let dx = x.diff().unwrap();
        assert_eq!(dx.eval(2.0), 192.0);
    }

    #[test]
    fn exp_test() {
        let x = Exp(Mul(Const(2.0), Pow(Var, 5.0))); // exp(2x^5)
        assert_eq!(x.eval(2.0), 2.0_f64.powi(6).exp());

        let dx = x.diff().unwrap(); // 10x^4exp(2x^5)
        println!("{}", dx);
        assert_eq!(dx.eval(2.0), 10.0 * 2.0_f64.powi(4) * 2.0_f64.powi(6).exp())
    }

    #[test]
    fn const_eval_test() {
        let y = Mul(Const(0.0), Exp(Var));
        assert_eq!(y.const_eval().unwrap().0, 0.0);

        let y = Add(Mul(Const(1.0), Const(2.0)), Exp(Log(Const(2.0))));
        assert_eq!(y.const_eval().unwrap().0, 4.0);

        let y = Add(Mul(Const(1.0), Const(2.0)), Exp(Log(Var)));
        assert!(y.const_eval().is_err());
    }
}
