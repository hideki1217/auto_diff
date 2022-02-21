use std::fmt;

use crate::*;
use crate::digest::*;
use super::std_ops::*;

macro_rules! impl_display {
    ($n: ident, $s: expr) => {
        impl<X> fmt::Display for $n<X>
        where
            X: fmt::Display,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $s, self.0)
            }
        }
    };
}

wrap_for_diff_and_eval!(Exp, Exp, |x| x.exp());
impl_display!(Exp, "exp({})");

#[derive(Clone, Copy, Debug)]
pub struct Log<X>(pub X);
impl_call!(Log, |x| x.ln());
impl<X: Diff> Diff for Log<X> {
    type Result = Mul<Pow<X>, X::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Mul(Pow(self.0, -1.0), self.0.diff()?))
    }
}
impl_eval!(Log);
impl_display!(Log, "log({})");

#[derive(Clone, Copy, Debug)]
pub struct Sin<X>(pub X);
impl_call!(Sin, |x|x.sin());
impl<X: Diff> Diff for Sin<X> {
    type Result = Mul<Cos<X>, X::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Mul(Cos(self.0), self.0.diff()?))
    }
}
impl_eval!(Sin);
impl_display!(Sin, "sin({})");

#[derive(Clone, Copy, Debug)]
pub struct Cos<X>(pub X);
impl_call!(Cos, |x| x.cos());
impl<X: Diff> Diff for Cos<X> {
    type Result = Mul<Neg<Sin<X>>, X::Result>;

    fn diff(&self) -> Result<Self::Result, Error> {
        Ok(Mul(Neg(Sin(self.0)), self.0.diff()?))
    }
}
impl_eval!(Cos);
impl_display!(Cos, "cos({})");

