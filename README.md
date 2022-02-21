# auto_diff
auto diff done on compiling
when rust support const evaluation, this concept will completelly get able;
Now this is able partly.
I implement it.

# Example
```
let sigmoid = Mul(Exp(Var), Add(Const(1.0), Exp(Var))); // exp(x)/(1+exp)
println!("{}", sigmoid);  // (exp(x)) * (((1) + (exp(x)))^-1)
assert_eq!(sigmoid.eval(0.0), 0.0);
let sigmoid_diff = sigmoid.diff().unwrap();
println!("{}", sigmoid_diff);  // (((exp(x)) * (1)) * (((1) + (exp(x)))^-1)) + ((exp(x)) * ((-1) * ((((1) + (exp(x)))^-2) * ((0) + ((exp(x)) * (1))))))
``` 

easily implmentable
```
// implement Relu
wrap_for_diff_and_eval!(Relu, Step, |x| if x >= 0.0 { x } else { 0.0 } );
wrap_for_diff_and_eval!(Step, |x| if x >= 0.0 { 1.0 } else { 0.0 });
```
