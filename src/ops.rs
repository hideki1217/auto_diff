use crate::digest::{Diff, Error, Eval, T};
use num::{One, Zero};
use std::fmt;

#[derive(Clone, Copy, Debug)]
struct Const(T);
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
}
impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
struct Var;
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
}
impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x")
    }
}

#[derive(Clone, Copy, Debug)]
struct Add<Lhs, Rhs>(Lhs, Rhs);
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
struct Sub<Lhs, Rhs>(Lhs, Rhs);
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
struct Mul<Lhs, Rhs>(Lhs, Rhs);
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
struct Neg<X>(X);
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
struct Pow<X>(X, T);
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
struct Exp<X>(X);
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
struct Log<X>(X);
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
struct Sin<X>(X);
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
struct Cos<X>(X);
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
}
