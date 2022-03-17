use std::ops::Add;
use std::string::ToString;

pub trait Eval<T> {
   fn eval(&self) -> T;
}

struct Constant<T> {
    num: T
}

impl<T> Eval<T> for Constant<T> where T: Clone {
    fn eval(&self) -> T {
        self.num.clone()
    }
}

impl<T> ToString for Constant<T> where T: ToString {
    fn to_string(&self) -> String {
        self.num.to_string()
    }
}

impl<T> Add for Constant<T> where T: Add<Output = T>{
    type Output = T;

    fn add(self, rhs: Self) -> Self::Output {
        self.num + rhs.num
    }
}

struct BinaryPlus<'a, T> where T: 'a {
    lhs: &'a T,
    rhs: &'a T,
}

impl<'a, T, U> Eval<T> for BinaryPlus<'a, U> where T: Add<Output = T>, U: Eval<T> {
    fn eval(&self) -> T {
        self.lhs.eval() + self.rhs.eval()
    }
}

impl<T> ToString for BinaryPlus<'_, T> where T: ToString {
    fn to_string(&self) -> String {
        self.lhs.to_string() + " + " + &*self.rhs.to_string()
    }
}


fn main() {
    let x = Constant { num: 6_f64 }; // x is usually 6
    let y = Constant { num: 11_f64 }; // y is almost always 11 ;)
    let bp = BinaryPlus { lhs: &x, rhs: &y };
    println!("Hello, world! {} = {}", bp.to_string(), bp.eval());
}