use num_bigint::*;

pub trait Lift: Sized {
    type Abstract;
    fn up(self) -> Self::Abstract;
    fn down(a: Self::Abstract) -> Self;
}

impl<T> Lift for Vec<T> {
    type Abstract = Vec<T>;
    fn up(self) -> Self::Abstract {
        self
    }
    fn down(x: Self::Abstract) -> Self {
        x
    }
}

impl Lift for () {
    type Abstract = ();
    fn up(self) -> Self::Abstract {
        ()
    }
    fn down(_: Self::Abstract) -> Self {
        ()
    }
}

use std::sync::Arc;

#[derive(Clone)]
pub struct Fn1<I, O> {
    pub f: Arc<dyn Fn(I) -> O>,
    pub string: String,
}

#[derive(Clone)]
pub struct FnR1<I, O> {
    pub f: Arc<dyn for<'a> Fn(&'a I) -> O>,
    pub string: String,
}

impl<I: PrintRust, O: PrintRust> PrintRust for FnR1<I, O> {
    fn print_as_rust(&self) -> String {
        format!("({})", self.string)
    }
    fn print_type() -> String {
        format!("Fn(&{}) -> {}", I::print_type(), O::print_type())
    }
}

impl<I: PrintRust, O: PrintRust> PrintRust for Fn1<I, O> {
    fn print_as_rust(&self) -> String {
        format!("({})", self.string)
    }
    fn print_type() -> String {
        format!("Fn({}) -> {}", I::print_type(), O::print_type())
    }
}

impl<I, O> std::hash::Hash for Fn1<I, O> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        false.hash(state);
        self.string.hash(state);
    }
}

impl<I, O> std::hash::Hash for FnR1<I, O> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        true.hash(state);
        self.string.hash(state);
    }
}

impl<I, O> Eq for Fn1<I, O> {}
impl<I, O> PartialEq for Fn1<I, O> {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl<I, O> Eq for FnR1<I, O> {}
impl<I, O> PartialEq for FnR1<I, O> {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

macro_rules! mkfn {
    (|$pat:ident : $ty:ty| $body:expr) => {
        Fn1::<$ty, _> {
            f: Arc::new(move |$pat: $ty| $body),
            string: format!("{}", stringify!(|$pat: $ty| $body)),
        }
    };
    ($str:expr, |$pat:ident : $ty:ty| $body:expr) => {
        Fn1::<$ty, _> {
            f: Arc::new(move |$pat: $ty| $body),
            string: $str,
        }
    };
}

macro_rules! mkfnr {
    ($str:expr, |$pat:ident : &$ty:ty| $body:expr) => {
        FnR1::<$ty, _> {
            f: Arc::new(move |$pat: &$ty| $body),
            string: $str,
        }
    };
}

impl<'a> arbitrary::Arbitrary<'a> for Fn1<u8, bool> {
    fn arbitrary(_: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(mkfn!(|x: u8| x > 128))
    }
}

impl<'a> arbitrary::Arbitrary<'a> for FnR1<u8, bool> {
    fn arbitrary(_: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(mkfnr!("|x: &u8| *x > 128".into(), |x: &u8| *x > 128))
    }
}

impl TypicalEdgeCases for Fn1<u8, bool> {
    fn edge_cases() -> Vec<Self> {
        vec![]
            .into_iter()
            .chain(
                u8::edge_cases()
                    .into_iter()
                    .map(|y| mkfn!(format!("|x: u8| x < {}", y.print_as_rust()), |x: u8| x < y)),
            )
            .collect()
    }
}

impl TypicalEdgeCases for FnR1<u8, bool> {
    fn edge_cases() -> Vec<Self> {
        vec![]
            .into_iter()
            .chain(u8::edge_cases().into_iter().map(|y| {
                mkfnr!(format!("|x: &u8| *x < {}", y.print_as_rust()), |x: &u8| *x
                    < y)
            }))
            .collect()
    }
}

impl<'a> arbitrary::Arbitrary<'a> for Fn1<u8, u8> {
    fn arbitrary(_: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(mkfn!("|x: u8| x".into(), |x: u8| x))
    }
}

impl TypicalEdgeCases for Fn1<u8, u8> {
    fn edge_cases() -> Vec<Self> {
        vec![mkfn!(|x: u8| x.wrapping_add(x))]
            .into_iter()
            .chain(u8::edge_cases().into_iter().map(|y| {
                mkfn!(
                    format!("|x: u8| x.wrapping_add({})", y.print_as_rust()),
                    |x: u8| x.wrapping_add(y)
                )
            }))
            .collect()
    }
}

pub trait TypicalEdgeCases: Sized {
    fn edge_cases() -> Vec<Self>;
}

trait IntType: num_traits::int::PrimInt {}
impl IntType for u8 {}
impl IntType for i8 {}
impl IntType for u16 {}
impl IntType for i16 {}
impl IntType for u32 {}
impl IntType for i32 {}
impl IntType for u64 {}
impl IntType for i64 {}
impl IntType for u128 {}
impl IntType for i128 {}
impl IntType for usize {}
impl IntType for isize {}

impl<T: IntType> TypicalEdgeCases for T {
    fn edge_cases() -> Vec<Self> {
        vec![Self::min_value(), Self::zero(), Self::max_value()]
    }
}

impl<T: TypicalEdgeCases + Clone> TypicalEdgeCases for Vec<T> {
    fn edge_cases() -> Vec<Self> {
        vec![vec![]]
            .into_iter()
            .chain(T::edge_cases().into_iter().map(|x| vec![x]))
            .collect()
    }
}
impl<T: TypicalEdgeCases + Clone> TypicalEdgeCases for Option<T> {
    fn edge_cases() -> Vec<Self> {
        vec![None]
            .into_iter()
            .chain(T::edge_cases().into_iter().map(Some))
            .collect()
    }
}
impl<T: TypicalEdgeCases + Clone, U: TypicalEdgeCases + Clone> TypicalEdgeCases for (T, U) {
    fn edge_cases() -> Vec<Self> {
        T::edge_cases()
            .iter()
            .flat_map(|x| {
                U::edge_cases()
                    .iter()
                    .map(|y| (x.clone(), y.clone()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl<T: TypicalEdgeCases + Clone, U: TypicalEdgeCases + Clone, V: TypicalEdgeCases + Clone>
    TypicalEdgeCases for (T, U, V)
{
    fn edge_cases() -> Vec<Self> {
        <((T, U), V)>::edge_cases()
            .iter()
            .map(|((t, u), v)| (t.clone(), u.clone(), v.clone()))
            .collect()
    }
}

pub trait PrintRust {
    fn print_as_rust(&self) -> String;
    fn print_type() -> String;
}

impl PrintRust for bool {
    fn print_as_rust(&self) -> String {
        format!("{}", self)
    }
    fn print_type() -> String {
        "bool".into()
    }
}

impl<T: PrintRust> PrintRust for Vec<T> {
    fn print_as_rust(&self) -> String {
        if self.is_empty() {
            format!("Vec::<{}>::new()", T::print_type())
        } else {
            format!(
                "vec![{}]",
                self.iter()
                    .map(T::print_as_rust)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
    fn print_type() -> String {
        format!("Vec<{}>", T::print_type())
    }
}

impl<T: PrintRust> PrintRust for Option<T> {
    fn print_as_rust(&self) -> String {
        if self.is_none() {
            format!("Option::<{}>::None", T::print_type())
        } else {
            match self {
                Some(value) => format!("Some({})", value.print_as_rust()),
                None => format!("None"),
            }
        }
    }
    fn print_type() -> String {
        format!("Option<{}>", T::print_type())
    }
}

macro_rules! print_rust_num_lit {
    ($ty:ident) => {
        impl PrintRust for $ty {
            fn print_as_rust(&self) -> String {
                format!("{}{}", self, stringify!($ty))
            }
            fn print_type() -> String {
                format!("{}", stringify!($ty))
            }
        }
        impl Lift for $ty {
            type Abstract = BigInt;
            fn up(self) -> Self::Abstract {
                BigInt::from(self)
            }
            fn down(x: Self::Abstract) -> Self {
                use num_traits::cast::ToPrimitive;
                paste::paste! {
                    x.[<to_ $ty>]().unwrap()
                }
            }
        }
    };
}

print_rust_num_lit!(u8);
print_rust_num_lit!(u16);
print_rust_num_lit!(u32);
print_rust_num_lit!(u64);
print_rust_num_lit!(u128);
print_rust_num_lit!(usize);
print_rust_num_lit!(i8);
print_rust_num_lit!(i16);
print_rust_num_lit!(i32);
print_rust_num_lit!(i64);
print_rust_num_lit!(i128);
print_rust_num_lit!(isize);

#[macro_export]
macro_rules! catch_unwind_set_hook {
    ($f:expr) => {{
        use std::panic;
        let previous_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = panic::catch_unwind($f);
        panic::set_hook(previous_hook);
        result
    }};
}

#[macro_export]
macro_rules! doesn_t_panic {
    ($e:expr) => {
        catch_unwind_set_hook!(|| $e).is_ok()
    };
}

#[macro_export]
macro_rules! panics {
    ($e:expr) => {
        catch_unwind_set_hook!(|| $e).is_err()
    };
}

#[macro_export]
macro_rules! catch_panic {
    ($e:expr) => {
        catch_unwind_set_hook!(|| $e).ok()
    };
}

#[allow(unused)]
pub(crate) use catch_panic;
#[allow(unused)]
pub(crate) use doesn_t_panic;
#[allow(unused)]
pub(crate) use panics;
