use num::Zero;
use std::fmt;

use crate::digest::*;


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
pub struct Pow<X>(pub X, pub T);
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


