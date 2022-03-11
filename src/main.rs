pub trait Expr {
   fn eval(&self) -> f64;
   fn to_string(&self) -> String;
}

struct Constant {
    num: f64
}

impl Expr for Constant {
    fn eval(&self) -> f64 {
        self.num
    }

    fn to_string(&self) -> String {
        self.num.to_string()
    }
}

struct BinaryPlus<'a, T> where T: Expr + ?Sized {
    lhs: &'a T,
    rhs: &'a T,
}

impl<T> Expr for BinaryPlus<'_, T> where T: Expr {
    fn eval(&self) -> f64 {
        self.lhs.eval() + self.rhs.eval()
    }

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

//fn main() {
//    
//    let mut foos = Vec::new();
//    foos.push(Foo {bars: vec![Bar::TWO, Bar::THREE]});
//    foos.push(Foo { bars: vec![] });
//
//    let mut foos2 = Vec::new();
//    for x in &foos {
//        foos2.push(x.clone());
//    }
//}
