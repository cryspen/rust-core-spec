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

impl PrintRust for u8 {
    fn print_as_rust(&self) -> String {
        format!("{}u8", self)
    }
    fn print_type() -> String {
        format!("u8")
    }
}

impl PrintRust for usize {
    fn print_as_rust(&self) -> String {
        format!("{}usize", self)
    }
    fn print_type() -> String {
        format!("usize")
    }
}

impl PrintRust for i8 {
    fn print_as_rust(&self) -> String {
        format!("{}i8", self)
    }
    fn print_type() -> String {
        format!("i8")
    }
}

impl Lift for i8 {
    type Abstract = BigInt;
    fn up(self) -> Self::Abstract {
        BigInt::from(self)
    }
    fn down(x: Self::Abstract) -> Self {
        use num_traits::cast::ToPrimitive;
        x.to_i8().unwrap()
    }
}

impl Lift for u8 {
    type Abstract = BigInt;
    fn up(self) -> Self::Abstract {
        BigInt::from(self)
    }
    fn down(x: Self::Abstract) -> Self {
        use num_traits::cast::ToPrimitive;
        x.to_u8().unwrap()
    }
}

impl Lift for u16 {
    type Abstract = BigInt;
    fn up(self) -> Self::Abstract {
        BigInt::from(self)
    }
    fn down(x: Self::Abstract) -> Self {
        use num_traits::cast::ToPrimitive;
        x.to_u16().unwrap()
    }
}

impl Lift for u128 {
    type Abstract = BigInt;
    fn up(self) -> Self::Abstract {
        BigInt::from(self)
    }
    fn down(x: Self::Abstract) -> Self {
        use num_traits::cast::ToPrimitive;
        x.to_u128().unwrap()
    }
}
impl Lift for i128 {
    type Abstract = BigInt;
    fn up(self) -> Self::Abstract {
        BigInt::from(self)
    }
    fn down(x: Self::Abstract) -> Self {
        use num_traits::cast::ToPrimitive;
        x.to_i128().unwrap()
    }
}
impl Lift for i64 {
    type Abstract = BigInt;
    fn up(self) -> Self::Abstract {
        BigInt::from(self)
    }
    fn down(x: Self::Abstract) -> Self {
        use num_traits::cast::ToPrimitive;
        x.to_i64().unwrap()
    }
}
impl Lift for u64 {
    type Abstract = BigInt;
    fn up(self) -> Self::Abstract {
        BigInt::from(self)
    }
    fn down(x: Self::Abstract) -> Self {
        use num_traits::cast::ToPrimitive;
        x.to_u64().unwrap()
    }
}
impl Lift for i32 {
    type Abstract = BigInt;
    fn up(self) -> Self::Abstract {
        BigInt::from(self)
    }
    fn down(x: Self::Abstract) -> Self {
        use num_traits::cast::ToPrimitive;
        x.to_i32().unwrap()
    }
}

#[macro_export]
macro_rules! doesn_t_panic {
    ($e:expr) => {
        std::panic::catch_unwind(|| $e).is_ok()
    };
}

#[macro_export]
macro_rules! panics {
    ($e:expr) => {
        std::panic::catch_unwind(|| $e).is_err()
    };
}

#[macro_export]
macro_rules! catch_panic {
    ($e:expr) => {
        std::panic::catch_unwind(|| $e).ok()
    };
}

#[allow(unused)]
pub(crate) use catch_panic;
#[allow(unused)]
pub(crate) use doesn_t_panic;
#[allow(unused)]
pub(crate) use panics;
