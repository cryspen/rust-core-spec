use partial_compute::*;

pub trait PrintRust {
    fn print_as_rust(&self) -> String;
}

impl<T: PrintRust> PrintRust for Vec<T> {
    fn print_as_rust(&self) -> String {
        format!(
            "vec![{}]",
            self.iter()
                .map(T::print_as_rust)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl PrintRust for u8 {
    fn print_as_rust(&self) -> String {
        format!("{}u8", self)
    }
}

impl PrintRust for usize {
    fn print_as_rust(&self) -> String {
        format!("{}usize", self)
    }
}

impl PrintRust for i8 {
    fn print_as_rust(&self) -> String {
        format!("{}i8", self)
    }
}

// #[partial_eval(variables = [x, y])]
// fn f<X, T>(a: X, x: u8, y: u8, z: T) -> bool {
//     let a = x + y;
//     eval(x + y) == x + y
// }

// #[partial_eval(variables = [x])]
// fn ff(x: Vec<u8>) -> bool {
//     x.get(3) == x.get(12)
// }

#[partial_eval(variables = [x])]
fn tnorm<T: Clone + PrintRust>(x: Vec<T>, y: T) -> bool {
    x[0] == y
}

#[test]
fn main() {
    // eprintln!("{}", ff(vec![123]))
    // eprintln!("{}", f(1, 3));
    eprintln!("{}", tnorm(vec![1u8]));
}
