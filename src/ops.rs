use num::Zero;

use crate::digest::*;
use std::fmt;



#[derive(Clone, Copy, Debug)]
pub struct Add<Lhs, Rhs>(pub Lhs, pub Rhs);
impl<Lhs: Diff, Rhs: Diff> Diff for Add<Lhs, Rhs> {
    type Result = Add<Lhs::Result, Rhs::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Add(self.0.diff()?, self.1.diff()?))
    }
}
impl<Lhs, Rhs> Eval for Add<Rhs, Lhs>
where
    Lhs: Eval,
    Rhs: Eval,
{
    fn eval(&self, x: T) -> T {
        self.0.eval(x) + self.1.eval(x)
    }

    fn const_eval(&self) -> Result<Const,()> {
        Ok(Const(self.0.const_eval()?.0 + self.1.const_eval()?.0))
    }
}
impl<Lhs, Rhs> fmt::Display for Add<Rhs, Lhs>
where
    Lhs: fmt::Display,
    Rhs: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) + ({})", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sub<Lhs, Rhs>(pub Lhs, pub Rhs);
impl<Lhs: Diff, Rhs: Diff> Diff for Sub<Lhs, Rhs> {
    type Result = Sub<Lhs::Result, Rhs::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Sub(self.0.diff()?, self.1.diff()?))
    }
}
impl<Lhs, Rhs> Eval for Sub<Rhs, Lhs>
where
    Lhs: Eval,
    Rhs: Eval,
{
    fn eval(&self, x: T) -> T {
        self.0.eval(x) - self.1.eval(x)
    }

    fn const_eval(&self) -> Result<Const,()> {
        Ok(Const(self.0.const_eval()?.0 - self.1.const_eval()?.0))
    }
}
impl<Lhs, Rhs> fmt::Display for Sub<Rhs, Lhs>
where
    Lhs: fmt::Display,
    Rhs: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) - ({})", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mul<Lhs, Rhs>(pub Lhs, pub Rhs);
impl<Lhs: Diff, Rhs: Diff> Diff for Mul<Lhs, Rhs> {
    type Result = Add<Mul<Lhs::Result, Rhs>, Mul<Lhs, Rhs::Result>>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Add(
            Mul(self.0.diff()?, self.1),
            Mul(self.0, self.1.diff()?),
        ))
    }
}
impl<Lhs, Rhs> Eval for Mul<Rhs, Lhs>
where
    Lhs: Eval,
    Rhs: Eval,
{
    fn eval(&self, x: T) -> T {
        self.0.eval(x) * self.1.eval(x)
    }

    fn const_eval(&self) -> Result<Const,()> {
        let lhs = self.0.const_eval();
        let rhs = self.1.const_eval();
        match (lhs,rhs) {
            (Ok(Const(l)),_) if l.is_zero() => Ok(Const(T::zero())),
            (_,Ok(Const(r))) if r.is_zero() => Ok(Const(T::zero())),
            (Ok(Const(l)), Ok(Const(r))) => Ok(Const(l*r)),
            _ => Err(())
        }
    }
}
impl<Lhs, Rhs> fmt::Display for Mul<Rhs, Lhs>
where
    Lhs: fmt::Display,
    Rhs: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) * ({})", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Neg<X>(pub X);
impl<X: Diff> Diff for Neg<X> {
    type Result = Neg<X::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Neg(self.0.diff()?))
    }
}
impl<X: Eval> Eval for Neg<X> {
    fn eval(&self, x: T) -> T {
        -self.0.eval(x)
    }

    fn const_eval(&self) -> Result<Const,()> {
        Ok(Const(-self.0.const_eval()?.0))
    }
}
impl<X> fmt::Display for Neg<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-({})", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pow<X>(X, T);
impl<X: Diff> Diff for Pow<X> {
    type Result = Mul<Const, Mul<Pow<X>, X::Result>>;

    fn diff(&self) -> Result<Self::Result, Error> {
        assert_ne!(self.1, 0.0);
        Ok(Mul(
            Const(self.1),
            Mul(Pow(self.0, self.1 - 1.0), self.0.diff()?),
        ))
    }
}
impl<X: Eval> Eval for Pow<X> {
    fn eval(&self, x: T) -> T {
        self.0.eval(x).powf(self.1)
    }

    fn const_eval(&self) -> Result<Const,()> {
        Ok(Const(self.0.const_eval()?.0.powf(self.1)))
    }
}
impl<X> fmt::Display for Pow<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})^{}", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Exp<X>(pub X);
impl<X: Diff> Diff for Exp<X> {
    type Result = Mul<Exp<X>, X::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Mul(Exp(self.0), self.0.diff()?))
    }
}
impl<X: Eval> Eval for Exp<X> {
    fn eval(&self, x: T) -> T {
        self.0.eval(x).exp()
    }

    fn const_eval(&self) -> Result<Const,()> {
        Ok(Const(self.0.const_eval()?.0.exp()))
    }
}
impl<X> fmt::Display for Exp<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exp({})", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Log<X>(pub X);
impl<X: Diff> Diff for Log<X> {
    type Result = Mul<Pow<X>, X::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Mul(Pow(self.0, -1.0), self.0.diff()?))
    }
}
impl<X: Eval> Eval for Log<X> {
    fn eval(&self, x: T) -> T {
        self.0.eval(x).ln()
    }

    fn const_eval(&self) -> Result<Const,()> {
        Ok(Const(self.0.const_eval()?.0.ln()))
    }
}
impl<X> fmt::Display for Log<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "log({})", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sin<X>(pub X);
impl<X: Diff> Diff for Sin<X> {
    type Result = Mul<Cos<X>, X::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Mul(Cos(self.0), self.0.diff()?))
    }
}
impl<X: Eval> Eval for Sin<X> {
    fn eval(&self, x: T) -> T {
        self.0.eval(x).sin()
    }

    fn const_eval(&self) -> Result<Const,()> {
        Ok(Const(self.0.const_eval()?.0.sin()))
    }
}
impl<X> fmt::Display for Sin<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sin({})", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cos<X>(pub X);
impl<X: Diff> Diff for Cos<X> {
    type Result = Mul<Neg<Sin<X>>, X::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Mul(Neg(Sin(self.0)), self.0.diff()?))
    }
}
impl<X: Eval> Eval for Cos<X> {
    fn eval(&self, x: T) -> T {
        self.0.eval(x).cos()
    }

    fn const_eval(&self) -> Result<Const,()> {
        Ok(Const(self.0.const_eval()?.0.cos()))
    }
}
impl<X> fmt::Display for Cos<X>
where
    X: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cos({})", self.0)
    }
}

#[cfg(test)]
mod tests {
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
