use arbitrary::{Arbitrary, Unstructured};
use once_cell::sync::Lazy;
use rand::RngCore;
use std::sync::Mutex;

pub static OUTPUT: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    Mutex::new(vec![
        "//! This module contains placeholder functions decorated with contracts and concrete tests."
            .into(),
    ])
});

pub trait Random {
    fn random() -> Self;
}

impl<T: for<'a> Arbitrary<'a>> Random for T {
    fn random() -> T {
        random()
    }
}

pub fn random<T: for<'a> Arbitrary<'a>>() -> T {
    let mut rng = rand::thread_rng();
    let raw_data: &mut [u8] = &mut [0; 512];
    rng.fill_bytes(raw_data);
    T::arbitrary(&mut Unstructured::new(raw_data)).unwrap()
}

#[macro_export]
macro_rules! tests {
    {
        header: $header:literal,
        ident: $ident:ident,
        $(contract! $contract:tt),*
            $(,)?
    } => {
        {
            let doc = {
                let mut docstr = String::new();
                docstr += &format!("# {}\n", $header);
                $({
                    $crate::contract! $contract
                    docstr += &format!("{}\n", make_doc());
                };)*
                docstr
            };
            OUTPUT.lock().unwrap().extend(
                doc.trim().split("\n").map(|line| format!("/// {line}")).chain(
                    [format!("pub fn {}{}", stringify!($ident), "(){}")].into_iter()
                )
            );
        }
    }
}

#[macro_export]
macro_rules! default_value {
    ($x:tt) => {
        $x
    };
    ($x:tt $y:tt) => {
        $x
    };
}

#[macro_export]
macro_rules! contract {
    {
        header: $header:literal,
        inputs: $(<$tinput:ident>)?[$($input:ident : $input_ty:ty),*],
        precondition: $pre_body: expr,
        postcondition: $post_body:expr
        $(,test_vector: [$($test_vector:expr),*])?
        $(,n: $n:literal)?
        $(,)?
    } => {
        #[allow(unused)]
        fn make_doc() -> String {
            use $crate::lifts::*;
            fn pre<$($tinput:Clone + PrintRust + core::fmt::Debug)?>($($input: $input_ty),*) -> bool {
                $pre_body
            }
            #[partial_compute::partial_compute(variables = [$($input),*])]
            fn post<$($tinput:Clone + PrintRust)?>($($input: $input_ty),*) -> bool {
                $post_body
            }
            fn make_doc() -> String {
                const HEADER: &str = $header;
                const INPUTS: &str = stringify!($($input: $input_ty),*);
                const PRE: &str = stringify!($pre_body);
                const POST: &str = partial_compute::drop_eval_in_expr_as_str!($post_body);
                format!("## {HEADER}\n__Inputs:__ `{INPUTS}`  \n__Precondition:__ `{PRE}`  \n__Postcondition:__ `{POST}`  \n")
            }

            use itertools::Itertools;
            $(type $tinput = u8;)?
            const DEFAULT_N: usize = 5;
            let mut n: usize = $crate::default_value!($($n)? DEFAULT_N);
            type TheType = ($($input_ty),*);
            let edge_cases: Vec<TheType> = TheType::edge_cases().iter().cloned().filter(|($($input),*)| pre($(<$input_ty as std::clone::Clone>::clone($input)),*)).collect();
            if(edge_cases.len() >= DEFAULT_N * 1/2 && $crate::default_value!($($n)? 0) == 0 && $crate::default_value!($($n)? 1) == 1) {
                n += edge_cases.len() - DEFAULT_N * 1/2;
            }
            let test_vector: Vec<TheType> =
                [$($($test_vector),*)?].iter().cloned().chain(
                    edge_cases.iter().cloned()
                )
                .chain(
                    std::iter::repeat_with(Random::random)
                        .take(100_000)
                )
                .unique()
                .filter(|($($input),*)| pre($(<$input_ty as std::clone::Clone>::clone($input)),*))
                .take(n).collect();
            if(test_vector.len() != n) {
                panic!("\n\nCould not find enough examples\n\n");
            }
            let asserts = test_vector.into_iter().map(
                |($($input),*)|
                format!("assert!({});", post($($input),*))
            ).collect::<Vec<_>>().join("\n");
            format!("{}```
# use core_spec::lifts::*;
# use core_spec::*;
# #[allow(arithmetic_overflow)] {}
{asserts}
# {}
```", make_doc(), "{", "}")
        }
    }
}

#[allow(unused)]
pub(crate) use contract;
#[allow(unused)]
pub(crate) use default_value;
#[allow(unused)]
pub(crate) use tests;
