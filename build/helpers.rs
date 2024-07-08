use arbitrary::{Arbitrary, Unstructured};
use once_cell::sync::Lazy;
use rand::RngCore;
use std::sync::Mutex;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OutKind {
    Rustdoc,
    VanillaBin,
}
pub static COUNT: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static FUNCTIONS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
pub fn register_function(r#fn: &str) {
    FUNCTIONS.lock().unwrap().push(r#fn.into());
}
pub fn list_functions() -> Vec<String> {
    FUNCTIONS.lock().unwrap().clone()
}

pub fn prefix_lines<'a>(prefix: &'a str, line: &'a str) -> impl Iterator<Item = String> + 'a {
    line.trim()
        .split("\n")
        .map(move |line| format!("{prefix}{line}"))
}

const _: () = {
    static OUTPUT: Lazy<Mutex<Vec<(OutKind, String)>>> = Lazy::new(|| Mutex::new(vec![]));

    impl OutKind {
        pub fn write<S: Into<String>>(self, line: S) {
            OUTPUT.lock().unwrap().push((self, line.into()));
        }
        pub fn write_lines<S: Into<String>>(self, lines: impl Iterator<Item = S>) {
            OUTPUT
                .lock()
                .unwrap()
                .extend(lines.map(|x| (self, x.into())));
        }
        pub fn write_prefixed<S: Into<String>>(self, prefix: &str, line: S) {
            let line: String = line.into();
            self.write_lines(prefix_lines(prefix, &line));
        }
        pub fn dump(self) -> String {
            OUTPUT
                .lock()
                .unwrap()
                .iter()
                .filter(|(k, _)| *k == self)
                .map(|(_, line)| line)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
};

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

pub fn rust_format(contents: &str) -> String {
    use std::io::Write;
    use std::process::{Command, Stdio};
    match Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(mut fmt) => {
            write!(fmt.stdin.as_mut().expect("stdin"), "{contents}").unwrap();
            let out = fmt
                .wait_with_output()
                .expect("Failed to get output from `rustfmt`");
            std::str::from_utf8(&out.stdout).unwrap().to_string()
        }
        Err(err) => {
            eprintln!("Could not run `rustfmt`: {err}.\nReverting to `prettyplease`.");
            prettyplease::unparse(
                &syn::parse_str(contents).expect(&format!("Could not parse {contents}")),
            )
        }
    }
}
pub fn rust_format_expr(contents: &str) -> String {
    let contents = rust_format(&format!("fn dummy_fn(){{{}}}", contents));
    contents
        .strip_prefix("fn dummy_fn() {")
        .unwrap()
        .strip_suffix("}\n")
        .unwrap()
        .split("\n")
        .filter(|line| line.trim() != "")
        .collect::<Vec<_>>()
        .join("\n")
        .into()
}

/// Defines a set of contracts that specify the semantics of an
/// item. This macro expects (1) an English header that explains what
/// the contracts are specifying, (2) a unique identifier for this set
/// of contracts and (3) a list of contracts.
#[macro_export]
macro_rules! tests {
    {
        header: $header:expr,
        ident: $ident:ident,
        $(contract! $contract:tt),*
            $(,)?
    } => {
        {
            let contracts: Vec<RenderedContract> = vec![$({$crate::contract! $contract make_doc(stringify!($ident))}),*];

            let doc = {
                let mut docstr = String::new();
                docstr += &format!("# {}\n", $header);
                docstr += &contracts.iter().map(|RenderedContract {header, asserts}| format!("{}```
# use core_spec::lifts::*;
# use core_spec::*;
# use num_traits::Euclid;
# #[allow(arithmetic_overflow)] {{
{asserts}
# }}
```", header)).collect::<Vec<_>>().join("\n");
                docstr
            };

            const FN: &str = stringify!($ident);
            register_function(FN);

            OutKind::Rustdoc.write_lines(
                doc.trim().split("\n").map(|line| format!("/// {line}")).chain(
                    [format!("pub fn {}{}", FN, "(){}")].into_iter()
                )
            );

            {
                OutKind::VanillaBin.write_prefixed("/// ", $header);
                let mut body = vec![];
                body.push(format!("pub fn {}{}", FN, "(){"));
                body.push(format!(r##"  eprintln!(r#"Testing "{}"... ({} contracts)"#);"##, $header, contracts.len()));
                body.push(format!(r##"  eprint!("  ");"##));
                for RenderedContract {header, asserts} in contracts.iter() {
                    body.extend(prefix_lines("  // ", header));
                    body.push("  {".into());
                    body.push(rust_format_expr(&asserts));
                    body.push("  }".into());
                    body.push(r##"  eprint!("... ");"##.into());
                }
                body.push(r#"  eprintln!("✓\n");"#.into());
                OutKind::VanillaBin.write_prefixed("", &body.join("\n"));
                OutKind::VanillaBin.write("}");
            }
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

pub struct RenderedContract {
    pub header: String,
    pub asserts: String,
}

/// Defines a contract. This macro expects the following named
/// arguments:

/// - `header`: an English description of the contract.  
/// - `inputs`: a comma-separated list of typed binders, optionally
/// prefixed with generic types arguments without bound constraints
/// (i.e. `<A, B>`). Example: `<T>[x: T, y: u8]`.  
/// - `precondition`: a Rust boolean expression that can refer to any
/// declared inputs.  
/// - `postcondition`: a Rust boolean expression that can refer to any
/// declared inputs.  
/// - `test_vector` (optional): an explicit test vectors. Example:
/// `[(1,2), (3,4)]`.  
/// - `strategy`: a strategy for picking random values. See
/// `lifts::SmallInt` for an example.  
/// - `n`: the number of tests to be generated.
#[macro_export]
macro_rules! contract {
    {
        header: $header:expr,
        inputs: $(<$tinput:ident>)?[$($input:ident : $input_ty:ty),*],
        precondition: $pre_body: expr,
        postcondition: $post_body:expr
            $(,test_vector: [$($test_vector:expr),*])?
            $(,strategy: $strategy:ident)?
            $(,n: $n:literal)?
            $(,n_min: $n_min:literal)?
            $(,)?
    } => {
        #[allow(unused)]
        fn make_doc(context: &str) -> RenderedContract {
            use $crate::lifts::*;
            use num_traits::Euclid;

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
                const DEFAULT_N: usize = 6;
            let mut n: usize = $crate::default_value!($($n)? DEFAULT_N);
            type OriginalType = ($($input_ty),*);
            type Strategy = OriginalType;
            fn unwrap_strategy(x: Strategy) -> OriginalType {
                x
            }
            {
                #[duplicate::duplicate_item(
                    composite_type;
                    [ $($input_ty),* ];
                )]
                $(
                    type Strategy = $strategy<composite_type>;
                    fn unwrap_strategy(x: Strategy) -> OriginalType {
                        x.unwrap_strategy_poly()
                    }
                )?
                type IgnoreMe = ();
                type TheType = Strategy;

                let mut edge_cases: Vec<TheType> = TheType::edge_cases().iter().cloned().filter(|inputs| {
                    let ($($input),*) = unwrap_strategy(inputs.clone());
                    pre($($input),*)
                }).collect();
                let original_n = n;
                if(edge_cases.len() >= DEFAULT_N * 1/2 && $crate::default_value!($($n)? 0) == 0 && $crate::default_value!($($n)? 1) == 1) {
                    n += DEFAULT_N * 2/3;
                    use rand::thread_rng;
                    use rand::seq::SliceRandom;
                    edge_cases.shuffle(&mut thread_rng());
                }
                let generate_test_vectors = |n: usize, sample: usize, debug: bool| {
                    let test_vector: Vec<TheType> =
                        [$($($test_vector),*)?].iter().cloned().chain(
                            edge_cases.iter().cloned().take(original_n)
                        )
                        .chain(
                            std::iter::repeat_with(Random::random)
                                .take(sample)
                        )
                        .unique()
                        .filter(|inputs| {
                            let ($($input),*) = unwrap_strategy(inputs.clone());
                            if debug {
                                eprint!("pre(");
                                $(
                                    eprint!("{}, ", ($input).clone().print_as_rust());
                                );*
                            }
                            let valid = pre($($input),*);
                            if debug {
                                eprint!(") = {valid}\n");
                            }
                            valid
                        })
                        .take(n).collect();

                    let test_vector: Vec<_> = test_vector.into_iter().map(unwrap_strategy).collect();
                    test_vector
                };
                let mut test_vector = generate_test_vectors(n, 100_000, false);
                $(
                    if(test_vector.len() != n) {
                        n = $n_min;
                        let mut test_vector = generate_test_vectors(n, 100_000, false);
                    }
                )?

                    if(test_vector.len() != n) {
                        const HEADER: &str = $header;
                        eprintln!("\n\nCould not find enough examples\n\npre={}\n\ncontext={context}\nheader={HEADER}\n\ntest_vector={}", stringify!($pre_body), test_vector.print_as_rust());
                        generate_test_vectors(n, 30, true);
                        panic!();
                    }
                *COUNT.lock().unwrap() += test_vector.len();
                let asserts = test_vector.into_iter().map(
                    |($($input),*)|
                    format!("assert!({});", post($($input),*))
                ).collect::<Vec<_>>().join("\n");
                RenderedContract {
                    header: make_doc(),
                    asserts,
                }
            }
        }
    }
}

#[allow(unused)]
pub(crate) use contract;
#[allow(unused)]
pub(crate) use default_value;
#[allow(unused)]
pub(crate) use tests;
