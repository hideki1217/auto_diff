extern crate num;

pub mod digest;
pub mod ops;

#[macro_export]
macro_rules! impl_call {
    ($n:ident, |$x: ident| $e: expr) => {
        impl<X> $n<X> {
            pub fn call($x: $crate::digest::T) ->  $crate::digest::T{
                $e
            }
        }
    };
}

#[macro_export]
macro_rules! impl_eval {
    ($callable: ident) => {
        impl<X:Eval> Eval for $callable<X> {
            fn eval(&self, x: $crate::digest::T) -> $crate::digest::T {
                $callable::<X>::call(self.0.eval(x))
            }
        
            fn const_eval(&self) -> Result<Const,()> {
                Ok(Const($callable::<X>::call(self.0.const_eval()?.0)))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_diff {
    ($n: ident, $dy: ident) => {
        impl<X: $crate::digest::Diff> $crate::digest::Diff for $n<X> {
            type Result = Mul<$dy<X>,X::Result>;
            fn diff(&self) -> Result<Self::Result, $crate::digest::Error> {
                Ok(Mul($dy(self.0), self.0.diff()?))
            }
        }
    };
    ($n: ident) => {
        impl<X: $crate::digest::Diff> $crate::digest::Diff for $n<X> {
            type Result = ();
        }
    };
}

#[macro_export]
macro_rules! wrap_for_diff_and_eval {
    ($n: ident, $dy: ident, |$x: ident| $e: expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $n<X>(pub X);
        $crate::impl_call!($n,|$x|$e);
        $crate::impl_diff!($n, $dy);
        $crate::impl_eval!($n);
    };
    ($n: ident, |$x: ident| $e: expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $n<X>(pub X);
        
        $crate::impl_call!($n,|$x|$e);
        $crate::impl_diff!($n);
        $crate::impl_eval!($n);
    };
}