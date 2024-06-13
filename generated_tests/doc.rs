//! This module contains placeholder functions decorated with contracts and concrete tests. There are 5043 tests.
/// # Properties for [`Option::is_some`]
/// ## `is_some` is a shorthand to pattern matching
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `v.is_some()
///         == (match v {
///             Some(_) => true,
///             None => false,
///         })`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.is_some()
///         == (match Option::<u8>::None {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(0u8).is_some()
///         == (match Some(0u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(1u8).is_some()
///         == (match Some(1u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(254u8).is_some()
///         == (match Some(254u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(255u8).is_some()
///         == (match Some(255u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(43u8).is_some()
///         == (match Some(43u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(181u8).is_some()
///         == (match Some(181u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(169u8).is_some()
///         == (match Some(169u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(8u8).is_some()
///         == (match Some(8u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// assert!(Some(191u8).is_some()
///         == (match Some(191u8) {
///             Some(_) => true,
///             None => false,
///         }));
/// # }
/// ```
pub fn option_is_some() {}
/// # Properties for [`Option::is_none`]
/// ## `is_none` is a shorthand to pattern matching
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `v.is_none()
///         == (match v {
///             Some(_) => false,
///             None => true,
///         })`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.is_none()
///         == (match Option::<u8>::None {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(0u8).is_none()
///         == (match Some(0u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(1u8).is_none()
///         == (match Some(1u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(254u8).is_none()
///         == (match Some(254u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(255u8).is_none()
///         == (match Some(255u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(204u8).is_none()
///         == (match Some(204u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(170u8).is_none()
///         == (match Some(170u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(192u8).is_none()
///         == (match Some(192u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(80u8).is_none()
///         == (match Some(80u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// assert!(Some(2u8).is_none()
///         == (match Some(2u8) {
///             Some(_) => false,
///             None => true,
///         }));
/// # }
/// ```
pub fn option_is_none() {}
/// # Properties for [`Option::expect`]
/// ## Unwrapping a [`None`] with `expect` always panic
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `panics!(v.expect("message"))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(Option::< u8 >::None.expect("message")));
/// # }
/// ```
/// ## Unwrapping a [`Some(_)`] with `expect` always succeeds
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_some()`  
/// __Postcondition:__ `doesn_t_panic!(v.expect("message"))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(doesn_t_panic!(Some(0u8).expect("message")));
/// assert!(doesn_t_panic!(Some(1u8).expect("message")));
/// assert!(doesn_t_panic!(Some(254u8).expect("message")));
/// assert!(doesn_t_panic!(Some(255u8).expect("message")));
/// assert!(doesn_t_panic!(Some(109u8).expect("message")));
/// assert!(doesn_t_panic!(Some(103u8).expect("message")));
/// assert!(doesn_t_panic!(Some(172u8).expect("message")));
/// assert!(doesn_t_panic!(Some(208u8).expect("message")));
/// assert!(doesn_t_panic!(Some(212u8).expect("message")));
/// assert!(doesn_t_panic!(Some(97u8).expect("message")));
/// # }
/// ```
/// ## Wrapping a value in a `Some` and unwrapping is identity
/// __Inputs:__ `v : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).unwrap() == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).unwrap() == 0u8);
/// assert!(Some(1u8).unwrap() == 1u8);
/// assert!(Some(254u8).unwrap() == 254u8);
/// assert!(Some(255u8).unwrap() == 255u8);
/// assert!(Some(75u8).unwrap() == 75u8);
/// assert!(Some(115u8).unwrap() == 115u8);
/// assert!(Some(215u8).unwrap() == 215u8);
/// assert!(Some(117u8).unwrap() == 117u8);
/// assert!(Some(174u8).unwrap() == 174u8);
/// assert!(Some(198u8).unwrap() == 198u8);
/// # }
/// ```
pub fn option_expect() {}
/// # Properties for [`Option::map`]
/// ## Applying `f` on `Some(v)` via `map` is equal to wrapping in `Some` the application of `v` to `f`
/// __Inputs:__ `v : T, f : Fn1<T, T>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).map(f) == Some((f)(v))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(x)))
///         == Some(((|x: u8| x.wrapping_add(x)))(0u8)));
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(0u8)))
///         == Some(((|x: u8| x.wrapping_add(0u8)))(0u8)));
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(1u8)))
///         == Some(((|x: u8| x.wrapping_add(1u8)))(0u8)));
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(254u8)))
///         == Some(((|x: u8| x.wrapping_add(254u8)))(0u8)));
/// assert!(Some(0u8).map((|x: u8| x.wrapping_add(255u8)))
///         == Some(((|x: u8| x.wrapping_add(255u8)))(0u8)));
/// assert!(Some(1u8).map((|x: u8| x.wrapping_add(x)))
///         == Some(((|x: u8| x.wrapping_add(x)))(1u8)));
/// assert!(Some(1u8).map((|x: u8| x.wrapping_add(0u8)))
///         == Some(((|x: u8| x.wrapping_add(0u8)))(1u8)));
/// assert!(Some(1u8).map((|x: u8| x.wrapping_add(1u8)))
///         == Some(((|x: u8| x.wrapping_add(1u8)))(1u8)));
/// assert!(Some(1u8).map((|x: u8| x.wrapping_add(254u8)))
///         == Some(((|x: u8| x.wrapping_add(254u8)))(1u8)));
/// assert!(Some(1u8).map((|x: u8| x.wrapping_add(255u8)))
///         == Some(((|x: u8| x.wrapping_add(255u8)))(1u8)));
/// # }
/// ```
/// ## Mapping a `None` is the identity
/// __Inputs:__ `v : Option<T>, f : Fn1<T, T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `v.map(f) == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.map((|x: u8| x.wrapping_add(x))) == Option::<u8>::None);
/// # }
/// ```
pub fn option_map() {}
/// # Properties for [`Option::filter`]
/// ## The filtering of `Some(v)` with a predicate `f` being non-empty is equivalent to applying a predicate `f` on `v`
/// __Inputs:__ `v : T, f : FnR1<T, bool>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).filter(f).is_some() == f(&v)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).filter((|x: &u8| *x < 0u8)).is_some() == (|x: &u8| *x < 0u8)(&0u8));
/// assert!(Some(0u8).filter((|x: &u8| *x < 1u8)).is_some() == (|x: &u8| *x < 1u8)(&0u8));
/// assert!(Some(0u8).filter((|x: &u8| *x < 254u8)).is_some() == (|x: &u8| *x < 254u8)(&0u8));
/// assert!(Some(0u8).filter((|x: &u8| *x < 255u8)).is_some() == (|x: &u8| *x < 255u8)(&0u8));
/// assert!(Some(1u8).filter((|x: &u8| *x < 0u8)).is_some() == (|x: &u8| *x < 0u8)(&1u8));
/// assert!(Some(1u8).filter((|x: &u8| *x < 1u8)).is_some() == (|x: &u8| *x < 1u8)(&1u8));
/// assert!(Some(1u8).filter((|x: &u8| *x < 254u8)).is_some() == (|x: &u8| *x < 254u8)(&1u8));
/// assert!(Some(1u8).filter((|x: &u8| *x < 255u8)).is_some() == (|x: &u8| *x < 255u8)(&1u8));
/// assert!(Some(254u8).filter((|x: &u8| *x < 0u8)).is_some() == (|x: &u8| *x < 0u8)(&254u8));
/// assert!(Some(254u8).filter((|x: &u8| *x < 1u8)).is_some() == (|x: &u8| *x < 1u8)(&254u8));
/// # }
/// ```
/// ## Filtering a `None` is the identity
/// __Inputs:__ `v : Option<T>, f : FnR1<T, bool>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `v.filter(f) == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.filter((|x: &u8| *x < 0u8)) == Option::<u8>::None);
/// # }
/// ```
pub fn option_filter() {}
/// # Properties for [`Option::flatten`]
/// ## Nested `Some`s
/// __Inputs:__ `x : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(Some(x)).flatten() == Some(x)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(Some(0u8)).flatten() == Some(0u8));
/// assert!(Some(Some(1u8)).flatten() == Some(1u8));
/// assert!(Some(Some(254u8)).flatten() == Some(254u8));
/// assert!(Some(Some(255u8)).flatten() == Some(255u8));
/// assert!(Some(Some(154u8)).flatten() == Some(154u8));
/// assert!(Some(Some(110u8)).flatten() == Some(110u8));
/// assert!(Some(Some(80u8)).flatten() == Some(80u8));
/// assert!(Some(Some(124u8)).flatten() == Some(124u8));
/// assert!(Some(Some(180u8)).flatten() == Some(180u8));
/// assert!(Some(Some(233u8)).flatten() == Some(233u8));
/// # }
/// ```
/// ## Nested or direct `None` flattens to None
/// __Inputs:__ `x : Option<Option<T>>`  
/// __Precondition:__ `x.is_none() || x.unwrap().is_none()`  
/// __Postcondition:__ `x.flatten() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<Option<u8>>::None.flatten() == None);
/// assert!(Some(Option::<u8>::None).flatten() == None);
/// # }
/// ```
pub fn option_flatten() {}
/// # Properties for [`Option::take`]
/// ## Take steals a value
/// __Inputs:__ `x : Option<T>`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{
///         let mut y = x.clone();
///         y.take() == x && y.is_none()
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let mut y = Option::<u8>::None.clone();
///         y.take() == Option::<u8>::None && y.is_none()
///     });
/// assert!({
///         let mut y = Some(0u8).clone();
///         y.take() == Some(0u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(1u8).clone();
///         y.take() == Some(1u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(254u8).clone();
///         y.take() == Some(254u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(255u8).clone();
///         y.take() == Some(255u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(16u8).clone();
///         y.take() == Some(16u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(9u8).clone();
///         y.take() == Some(9u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(48u8).clone();
///         y.take() == Some(48u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(108u8).clone();
///         y.take() == Some(108u8) && y.is_none()
///     });
/// assert!({
///         let mut y = Some(191u8).clone();
///         y.take() == Some(191u8) && y.is_none()
///     });
/// # }
/// ```
pub fn option_take() {}
/// # Properties for [`Option::zip`]
/// ## Zipping two non-empty options
/// __Inputs:__ `x : T, y : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(x).zip(Some(y)) == Some((x, y))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).zip(Some(0u8)) == Some((0u8, 0u8)));
/// assert!(Some(0u8).zip(Some(1u8)) == Some((0u8, 1u8)));
/// assert!(Some(0u8).zip(Some(254u8)) == Some((0u8, 254u8)));
/// assert!(Some(0u8).zip(Some(255u8)) == Some((0u8, 255u8)));
/// assert!(Some(1u8).zip(Some(0u8)) == Some((1u8, 0u8)));
/// assert!(Some(1u8).zip(Some(1u8)) == Some((1u8, 1u8)));
/// assert!(Some(1u8).zip(Some(254u8)) == Some((1u8, 254u8)));
/// assert!(Some(1u8).zip(Some(255u8)) == Some((1u8, 255u8)));
/// assert!(Some(254u8).zip(Some(0u8)) == Some((254u8, 0u8)));
/// assert!(Some(254u8).zip(Some(1u8)) == Some((254u8, 1u8)));
/// # }
/// ```
/// ## Zipping two options when one is a `None` makes `None`
/// __Inputs:__ `x : Option<T>, y : Option<T>`  
/// __Precondition:__ `x.is_none() || y.is_none()`  
/// __Postcondition:__ `x.zip(y).is_none()`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Option::<u8>::None.zip(Option::<u8>::None).is_none());
/// assert!(Option::<u8>::None.zip(Some(0u8)).is_none());
/// assert!(Option::<u8>::None.zip(Some(1u8)).is_none());
/// assert!(Option::<u8>::None.zip(Some(254u8)).is_none());
/// assert!(Option::<u8>::None.zip(Some(255u8)).is_none());
/// assert!(Some(0u8).zip(Option::<u8>::None).is_none());
/// assert!(Some(1u8).zip(Option::<u8>::None).is_none());
/// assert!(Some(254u8).zip(Option::<u8>::None).is_none());
/// assert!(Some(255u8).zip(Option::<u8>::None).is_none());
/// assert!(Option::<u8>::None.zip(Some(122u8)).is_none());
/// # }
/// ```
pub fn option_zip() {}
/// # Properties for [`Option::unwrap`]
/// ## Unwrapping a [`None`] always panic
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `panics!(v.unwrap())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(Option::< u8 >::None.unwrap()));
/// # }
/// ```
/// ## Unwrapping a [`Some(_)`] always succeeds
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_some()`  
/// __Postcondition:__ `doesn_t_panic!(v.unwrap())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(doesn_t_panic!(Some(0u8).unwrap()));
/// assert!(doesn_t_panic!(Some(1u8).unwrap()));
/// assert!(doesn_t_panic!(Some(254u8).unwrap()));
/// assert!(doesn_t_panic!(Some(255u8).unwrap()));
/// assert!(doesn_t_panic!(Some(89u8).unwrap()));
/// assert!(doesn_t_panic!(Some(157u8).unwrap()));
/// assert!(doesn_t_panic!(Some(236u8).unwrap()));
/// assert!(doesn_t_panic!(Some(160u8).unwrap()));
/// assert!(doesn_t_panic!(Some(188u8).unwrap()));
/// assert!(doesn_t_panic!(Some(224u8).unwrap()));
/// # }
/// ```
pub fn option_unwrap() {}
/// # Properties for [`Option::as_mut`]
/// ## In place update via `as_mut` is equivalent to functional update
/// __Inputs:__ `v : Option<u8>`  
/// __Precondition:__ `v.is_some() && v.unwrap() < 50`  
/// __Postcondition:__ `{
///         let (v_unwrapped, mut v_mut) = (v.unwrap().clone(), v);
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(0u8).unwrap().clone(), Some(0u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(1u8).unwrap().clone(), Some(1u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(11u8).unwrap().clone(), Some(11u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(36u8).unwrap().clone(), Some(36u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(49u8).unwrap().clone(), Some(49u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(9u8).unwrap().clone(), Some(9u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(8u8).unwrap().clone(), Some(8u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(32u8).unwrap().clone(), Some(32u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(37u8).unwrap().clone(), Some(37u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(28u8).unwrap().clone(), Some(28u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// # }
/// ```
pub fn option_as_mut() {}
/// # Properties for [`Option::as_slice`]
/// ## [`None.as_slice()`] should always result in an empty slice
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `{ v.as_slice().is_empty() }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ Option::<u8>::None.as_slice().is_empty() });
/// # }
/// ```
/// ## [`Some(v).as_slice()`] should always result in a slice containing exactly `v`
/// __Inputs:__ `v : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ Some(v).as_slice() == [v] }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ Some(0u8).as_slice() == [0u8] });
/// assert!({ Some(1u8).as_slice() == [1u8] });
/// assert!({ Some(254u8).as_slice() == [254u8] });
/// assert!({ Some(255u8).as_slice() == [255u8] });
/// assert!({ Some(165u8).as_slice() == [165u8] });
/// assert!({ Some(117u8).as_slice() == [117u8] });
/// assert!({ Some(225u8).as_slice() == [225u8] });
/// assert!({ Some(229u8).as_slice() == [229u8] });
/// assert!({ Some(142u8).as_slice() == [142u8] });
/// assert!({ Some(206u8).as_slice() == [206u8] });
/// # }
/// ```
pub fn option_as_slice() {}
/// # Properties for [`Vec`]
/// ## Indexing
/// __Inputs:__ `v : Vec<u8>, i : usize`  
/// __Precondition:__ `v.len() > 0`  
/// __Postcondition:__ `v.get(i % v.len()) == Some(&v[i % v.len()])`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(vec![0u8].get(0usize) == Some(&0u8));
/// assert!(vec![0u8].get(0usize) == Some(&0u8));
/// assert!(vec![0u8].get(0usize) == Some(&0u8));
/// assert!(vec![0u8].get(0usize) == Some(&0u8));
/// assert!(vec![1u8].get(0usize) == Some(&1u8));
/// assert!(vec![1u8].get(0usize) == Some(&1u8));
/// assert!(vec![1u8].get(0usize) == Some(&1u8));
/// assert!(vec![1u8].get(0usize) == Some(&1u8));
/// assert!(vec![254u8].get(0usize) == Some(&254u8));
/// assert!(vec![254u8].get(0usize) == Some(&254u8));
/// # }
/// ```
pub fn slice_get() {}
/// # Properties for [`core::ops::Rem::<u8>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u8::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 % 1u8 == 0u8);
/// assert!(0u8 % 254u8 == 0u8);
/// assert!(0u8 % 255u8 == 0u8);
/// assert!(1u8 % 1u8 == 0u8);
/// assert!(1u8 % 254u8 == 1u8);
/// assert!(1u8 % 255u8 == 1u8);
/// assert!(254u8 % 1u8 == 0u8);
/// assert!(254u8 % 254u8 == 0u8);
/// assert!(254u8 % 255u8 == 254u8);
/// assert!(255u8 % 1u8 == 0u8);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u8 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u8_rem() {}
/// # Properties for [`u8::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u8::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_rem(1u8) == Some(0u8));
/// assert!(0u8.checked_rem(254u8) == Some(0u8));
/// assert!(0u8.checked_rem(255u8) == Some(0u8));
/// assert!(1u8.checked_rem(1u8) == Some(0u8));
/// assert!(1u8.checked_rem(254u8) == Some(1u8));
/// assert!(1u8.checked_rem(255u8) == Some(1u8));
/// assert!(254u8.checked_rem(1u8) == Some(0u8));
/// assert!(254u8.checked_rem(254u8) == Some(0u8));
/// assert!(254u8.checked_rem(255u8) == Some(254u8));
/// assert!(255u8.checked_rem(1u8) == Some(0u8));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u8.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u8_checked_rem() {}
/// # Properties for [`u8::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `x == u8::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u8::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_neg() == Some(0u8));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `x != u8::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.checked_neg() == None);
/// assert!(254u8.checked_neg() == None);
/// assert!(255u8.checked_neg() == None);
/// assert!(26u8.checked_neg() == None);
/// assert!(235u8.checked_neg() == None);
/// assert!(144u8.checked_neg() == None);
/// assert!(173u8.checked_neg() == None);
/// assert!(43u8.checked_neg() == None);
/// assert!(195u8.checked_neg() == None);
/// assert!(189u8.checked_neg() == None);
/// # }
/// ```
pub fn t_u8_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u8>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x << y == u8::down((x.up() << y) & u8::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 << 0u32 == 0u8);
/// assert!(0u8 << 1u32 == 0u8);
/// assert!(1u8 << 0u32 == 1u8);
/// assert!(1u8 << 1u32 == 2u8);
/// assert!(254u8 << 0u32 == 254u8);
/// assert!(254u8 << 1u32 == 252u8);
/// assert!(255u8 << 0u32 == 255u8);
/// assert!(255u8 << 1u32 == 254u8);
/// assert!(170u8 << 2u32 == 168u8);
/// assert!(194u8 << 1u32 == 132u8);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u8 << 4294967294u32));
/// assert!(panics!(0u8 << 4294967295u32));
/// assert!(panics!(1u8 << 4294967294u32));
/// assert!(panics!(1u8 << 4294967295u32));
/// assert!(panics!(254u8 << 4294967294u32));
/// assert!(panics!(254u8 << 4294967295u32));
/// assert!(panics!(255u8 << 4294967294u32));
/// assert!(panics!(255u8 << 4294967295u32));
/// assert!(panics!(202u8 << 104u32));
/// assert!(panics!(248u8 << 55u32));
/// # }
/// ```
pub fn core_ops_shl_u8_shl() {}
/// # Properties for [`core::ops::Shl::<u8>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u8::down((x.up() << y) & u8::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_shl(0u32) == Some(0u8));
/// assert!(0u8.checked_shl(1u32) == Some(0u8));
/// assert!(1u8.checked_shl(0u32) == Some(1u8));
/// assert!(1u8.checked_shl(1u32) == Some(2u8));
/// assert!(254u8.checked_shl(0u32) == Some(254u8));
/// assert!(254u8.checked_shl(1u32) == Some(252u8));
/// assert!(255u8.checked_shl(0u32) == Some(255u8));
/// assert!(255u8.checked_shl(1u32) == Some(254u8));
/// assert!(61u8.checked_shl(7u32) == Some(128u8));
/// assert!(1u8.checked_shl(5u32) == Some(32u8));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_shl(4294967294u32) == None);
/// assert!(0u8.checked_shl(4294967295u32) == None);
/// assert!(1u8.checked_shl(4294967294u32) == None);
/// assert!(1u8.checked_shl(4294967295u32) == None);
/// assert!(254u8.checked_shl(4294967294u32) == None);
/// assert!(254u8.checked_shl(4294967295u32) == None);
/// assert!(255u8.checked_shl(4294967294u32) == None);
/// assert!(255u8.checked_shl(4294967295u32) == None);
/// assert!(88u8.checked_shl(95u32) == None);
/// assert!(205u8.checked_shl(61u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u8_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u8>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u8::down((x.up() << y) & u8::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.overflowing_shl(0u32) == (0u8, false));
/// assert!(0u8.overflowing_shl(1u32) == (0u8, false));
/// assert!(1u8.overflowing_shl(0u32) == (1u8, false));
/// assert!(1u8.overflowing_shl(1u32) == (2u8, false));
/// assert!(254u8.overflowing_shl(0u32) == (254u8, false));
/// assert!(254u8.overflowing_shl(1u32) == (252u8, false));
/// assert!(255u8.overflowing_shl(0u32) == (255u8, false));
/// assert!(255u8.overflowing_shl(1u32) == (254u8, false));
/// assert!(4u8.overflowing_shl(6u32) == (0u8, false));
/// assert!(3u8.overflowing_shl(3u32) == (24u8, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u8::down((x.up() << (y & (u8::BITS - 1)) & u8::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.overflowing_shl(4294967294u32) == (0u8, true));
/// assert!(0u8.overflowing_shl(4294967295u32) == (0u8, true));
/// assert!(1u8.overflowing_shl(4294967294u32) == (64u8, true));
/// assert!(1u8.overflowing_shl(4294967295u32) == (128u8, true));
/// assert!(254u8.overflowing_shl(4294967294u32) == (128u8, true));
/// assert!(254u8.overflowing_shl(4294967295u32) == (0u8, true));
/// assert!(255u8.overflowing_shl(4294967294u32) == (192u8, true));
/// assert!(255u8.overflowing_shl(4294967295u32) == (128u8, true));
/// assert!(67u8.overflowing_shl(28u32) == (48u8, true));
/// assert!(222u8.overflowing_shl(130u32) == (120u8, true));
/// # }
/// ```
pub fn core_ops_shl_u8_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u8>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x >> y == u8::down((x.up() >> y) & u8::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 >> 0u32 == 0u8);
/// assert!(0u8 >> 1u32 == 0u8);
/// assert!(1u8 >> 0u32 == 1u8);
/// assert!(1u8 >> 1u32 == 0u8);
/// assert!(254u8 >> 0u32 == 254u8);
/// assert!(254u8 >> 1u32 == 127u8);
/// assert!(255u8 >> 0u32 == 255u8);
/// assert!(255u8 >> 1u32 == 127u8);
/// assert!(177u8 >> 6u32 == 2u8);
/// assert!(182u8 >> 7u32 == 1u8);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u8 >> 4294967294u32));
/// assert!(panics!(0u8 >> 4294967295u32));
/// assert!(panics!(1u8 >> 4294967294u32));
/// assert!(panics!(1u8 >> 4294967295u32));
/// assert!(panics!(254u8 >> 4294967294u32));
/// assert!(panics!(254u8 >> 4294967295u32));
/// assert!(panics!(255u8 >> 4294967294u32));
/// assert!(panics!(255u8 >> 4294967295u32));
/// assert!(panics!(110u8 >> 14u32));
/// assert!(panics!(58u8 >> 64u32));
/// # }
/// ```
pub fn core_ops_shr_u8_shr() {}
/// # Properties for [`core::ops::Shr::<u8>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u8::down((x.up() >> y) & u8::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_shr(0u32) == Some(0u8));
/// assert!(0u8.checked_shr(1u32) == Some(0u8));
/// assert!(1u8.checked_shr(0u32) == Some(1u8));
/// assert!(1u8.checked_shr(1u32) == Some(0u8));
/// assert!(254u8.checked_shr(0u32) == Some(254u8));
/// assert!(254u8.checked_shr(1u32) == Some(127u8));
/// assert!(255u8.checked_shr(0u32) == Some(255u8));
/// assert!(255u8.checked_shr(1u32) == Some(127u8));
/// assert!(52u8.checked_shr(2u32) == Some(13u8));
/// assert!(179u8.checked_shr(6u32) == Some(2u8));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_shr(4294967294u32) == None);
/// assert!(0u8.checked_shr(4294967295u32) == None);
/// assert!(1u8.checked_shr(4294967294u32) == None);
/// assert!(1u8.checked_shr(4294967295u32) == None);
/// assert!(254u8.checked_shr(4294967294u32) == None);
/// assert!(254u8.checked_shr(4294967295u32) == None);
/// assert!(255u8.checked_shr(4294967294u32) == None);
/// assert!(255u8.checked_shr(4294967295u32) == None);
/// assert!(55u8.checked_shr(44u32) == None);
/// assert!(154u8.checked_shr(103u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u8_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u8>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y < u8::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u8::down((x.up() >> y) & u8::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.overflowing_shr(0u32) == (0u8, false));
/// assert!(0u8.overflowing_shr(1u32) == (0u8, false));
/// assert!(1u8.overflowing_shr(0u32) == (1u8, false));
/// assert!(1u8.overflowing_shr(1u32) == (0u8, false));
/// assert!(254u8.overflowing_shr(0u32) == (254u8, false));
/// assert!(254u8.overflowing_shr(1u32) == (127u8, false));
/// assert!(255u8.overflowing_shr(0u32) == (255u8, false));
/// assert!(255u8.overflowing_shr(1u32) == (127u8, false));
/// assert!(181u8.overflowing_shr(3u32) == (22u8, false));
/// assert!(74u8.overflowing_shr(1u32) == (37u8, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u8, y : u32`  
/// __Precondition:__ `y >= u8::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u8::down((x.up() >> (y & (u8::BITS - 1)) & u8::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.overflowing_shr(4294967294u32) == (0u8, true));
/// assert!(0u8.overflowing_shr(4294967295u32) == (0u8, true));
/// assert!(1u8.overflowing_shr(4294967294u32) == (0u8, true));
/// assert!(1u8.overflowing_shr(4294967295u32) == (0u8, true));
/// assert!(254u8.overflowing_shr(4294967294u32) == (3u8, true));
/// assert!(254u8.overflowing_shr(4294967295u32) == (1u8, true));
/// assert!(255u8.overflowing_shr(4294967294u32) == (3u8, true));
/// assert!(255u8.overflowing_shr(4294967295u32) == (1u8, true));
/// assert!(207u8.overflowing_shr(81u32) == (103u8, true));
/// assert!(70u8.overflowing_shr(130u32) == (17u8, true));
/// # }
/// ```
pub fn core_ops_shr_u8_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u8>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u8::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 / 1u8 == 0u8);
/// assert!(0u8 / 254u8 == 0u8);
/// assert!(0u8 / 255u8 == 0u8);
/// assert!(1u8 / 1u8 == 1u8);
/// assert!(1u8 / 254u8 == 0u8);
/// assert!(1u8 / 255u8 == 0u8);
/// assert!(254u8 / 1u8 == 254u8);
/// assert!(254u8 / 254u8 == 1u8);
/// assert!(254u8 / 255u8 == 0u8);
/// assert!(255u8 / 1u8 == 255u8);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(254u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(255u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(56u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(146u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(113u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(224u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(62u8 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(189u8 / 0) } });
/// # }
/// ```
pub fn core_ops_div_u8_div() {}
/// # Properties for [`u8::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u8::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_div(1u8) == 0u8);
/// assert!(0u8.saturating_div(254u8) == 0u8);
/// assert!(0u8.saturating_div(255u8) == 0u8);
/// assert!(1u8.saturating_div(1u8) == 1u8);
/// assert!(1u8.saturating_div(254u8) == 0u8);
/// assert!(1u8.saturating_div(255u8) == 0u8);
/// assert!(254u8.saturating_div(1u8) == 254u8);
/// assert!(254u8.saturating_div(254u8) == 1u8);
/// assert!(254u8.saturating_div(255u8) == 0u8);
/// assert!(255u8.saturating_div(1u8) == 255u8);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(254u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(255u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(5u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(198u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(225u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(75u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(208u8.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(94u8.saturating_div(0)) } });
/// # }
/// ```
pub fn core_ops_div_u8_saturating_div() {}
/// # Properties for [`u8::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u8::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_div(1u8) == Some(0u8));
/// assert!(0u8.checked_div(254u8) == Some(0u8));
/// assert!(0u8.checked_div(255u8) == Some(0u8));
/// assert!(1u8.checked_div(1u8) == Some(1u8));
/// assert!(1u8.checked_div(254u8) == Some(0u8));
/// assert!(1u8.checked_div(255u8) == Some(0u8));
/// assert!(254u8.checked_div(1u8) == Some(254u8));
/// assert!(254u8.checked_div(254u8) == Some(1u8));
/// assert!(254u8.checked_div(255u8) == Some(0u8));
/// assert!(255u8.checked_div(1u8) == Some(255u8));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_div(0) == None);
/// assert!(1u8.checked_div(0) == None);
/// assert!(254u8.checked_div(0) == None);
/// assert!(255u8.checked_div(0) == None);
/// assert!(234u8.checked_div(0) == None);
/// assert!(250u8.checked_div(0) == None);
/// assert!(196u8.checked_div(0) == None);
/// assert!(130u8.checked_div(0) == None);
/// assert!(66u8.checked_div(0) == None);
/// assert!(70u8.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u8_checked_div() {}
/// # Properties for [`core::ops::Mul::<u8>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x * y == u8::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 * 0u8 == 0u8);
/// assert!(0u8 * 1u8 == 0u8);
/// assert!(0u8 * 254u8 == 0u8);
/// assert!(0u8 * 255u8 == 0u8);
/// assert!(1u8 * 0u8 == 0u8);
/// assert!(1u8 * 1u8 == 1u8);
/// assert!(1u8 * 254u8 == 254u8);
/// assert!(1u8 * 255u8 == 255u8);
/// assert!(254u8 * 0u8 == 0u8);
/// assert!(254u8 * 1u8 == 254u8);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() > u8::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(254u8 * 254u8));
/// assert!(panics!(254u8 * 255u8));
/// assert!(panics!(255u8 * 254u8));
/// assert!(panics!(255u8 * 255u8));
/// assert!(panics!(75u8 * 134u8));
/// assert!(panics!(186u8 * 204u8));
/// assert!(panics!(206u8 * 168u8));
/// assert!(panics!(159u8 * 247u8));
/// assert!(panics!(123u8 * 226u8));
/// assert!(panics!(243u8 * 93u8));
/// # }
/// ```
pub fn core_ops_mul_u8_mul() {}
/// # Properties for [`u8::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u8::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_mul(0u8) == Some(0u8));
/// assert!(0u8.checked_mul(1u8) == Some(0u8));
/// assert!(0u8.checked_mul(254u8) == Some(0u8));
/// assert!(0u8.checked_mul(255u8) == Some(0u8));
/// assert!(1u8.checked_mul(0u8) == Some(0u8));
/// assert!(1u8.checked_mul(1u8) == Some(1u8));
/// assert!(1u8.checked_mul(254u8) == Some(254u8));
/// assert!(1u8.checked_mul(255u8) == Some(255u8));
/// assert!(254u8.checked_mul(0u8) == Some(0u8));
/// assert!(254u8.checked_mul(1u8) == Some(254u8));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.checked_mul(254u8) == None);
/// assert!(254u8.checked_mul(255u8) == None);
/// assert!(255u8.checked_mul(254u8) == None);
/// assert!(255u8.checked_mul(255u8) == None);
/// assert!(161u8.checked_mul(45u8) == None);
/// assert!(77u8.checked_mul(121u8) == None);
/// assert!(24u8.checked_mul(159u8) == None);
/// assert!(187u8.checked_mul(75u8) == None);
/// assert!(95u8.checked_mul(99u8) == None);
/// assert!(158u8.checked_mul(234u8) == None);
/// # }
/// ```
pub fn core_ops_mul_u8_checked_mul() {}
/// # Properties for [`u8::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u8::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.overflowing_mul(0u8) == (0u8, false));
/// assert!(0u8.overflowing_mul(1u8) == (0u8, false));
/// assert!(0u8.overflowing_mul(254u8) == (0u8, false));
/// assert!(0u8.overflowing_mul(255u8) == (0u8, false));
/// assert!(1u8.overflowing_mul(0u8) == (0u8, false));
/// assert!(1u8.overflowing_mul(1u8) == (1u8, false));
/// assert!(1u8.overflowing_mul(254u8) == (254u8, false));
/// assert!(1u8.overflowing_mul(255u8) == (255u8, false));
/// assert!(254u8.overflowing_mul(0u8) == (0u8, false));
/// assert!(254u8.overflowing_mul(1u8) == (254u8, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.overflowing_mul(254u8) == (4u8, true));
/// assert!(254u8.overflowing_mul(255u8) == (2u8, true));
/// assert!(255u8.overflowing_mul(254u8) == (2u8, true));
/// assert!(255u8.overflowing_mul(255u8) == (1u8, true));
/// assert!(118u8.overflowing_mul(139u8) == (18u8, true));
/// assert!(175u8.overflowing_mul(78u8) == (82u8, true));
/// assert!(6u8.overflowing_mul(232u8) == (112u8, true));
/// assert!(162u8.overflowing_mul(182u8) == (44u8, true));
/// assert!(208u8.overflowing_mul(158u8) == (96u8, true));
/// assert!(166u8.overflowing_mul(186u8) == (156u8, true));
/// # }
/// ```
pub fn core_ops_mul_u8_overflowing_mul() {}
/// # Properties for [`u8::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u8::down((x.up() * y.up()).min(u8::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_mul(0u8) == 0u8);
/// assert!(0u8.saturating_mul(1u8) == 0u8);
/// assert!(0u8.saturating_mul(254u8) == 0u8);
/// assert!(0u8.saturating_mul(255u8) == 0u8);
/// assert!(1u8.saturating_mul(0u8) == 0u8);
/// assert!(1u8.saturating_mul(1u8) == 1u8);
/// assert!(1u8.saturating_mul(254u8) == 254u8);
/// assert!(1u8.saturating_mul(255u8) == 255u8);
/// assert!(254u8.saturating_mul(0u8) == 0u8);
/// assert!(254u8.saturating_mul(1u8) == 254u8);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u8::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_mul(0u8) == 0u8);
/// assert!(0u8.saturating_mul(1u8) == 0u8);
/// assert!(0u8.saturating_mul(254u8) == 0u8);
/// assert!(0u8.saturating_mul(255u8) == 0u8);
/// assert!(1u8.saturating_mul(0u8) == 0u8);
/// assert!(1u8.saturating_mul(1u8) == 1u8);
/// assert!(1u8.saturating_mul(254u8) == 254u8);
/// assert!(1u8.saturating_mul(255u8) == 255u8);
/// assert!(254u8.saturating_mul(0u8) == 0u8);
/// assert!(254u8.saturating_mul(1u8) == 254u8);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u8::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(254u8.saturating_mul(254u8) == u8::MAX);
/// assert!(254u8.saturating_mul(255u8) == u8::MAX);
/// assert!(255u8.saturating_mul(254u8) == u8::MAX);
/// assert!(255u8.saturating_mul(255u8) == u8::MAX);
/// assert!(144u8.saturating_mul(172u8) == u8::MAX);
/// assert!(186u8.saturating_mul(95u8) == u8::MAX);
/// assert!(140u8.saturating_mul(242u8) == u8::MAX);
/// assert!(115u8.saturating_mul(77u8) == u8::MAX);
/// assert!(248u8.saturating_mul(160u8) == u8::MAX);
/// assert!(194u8.saturating_mul(202u8) == u8::MAX);
/// # }
/// ```
pub fn core_ops_mul_u8_saturating_mul() {}
/// # Properties for [`u8::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u8::down((x.up() * y.up()) % (u8::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_mul(0u8) == 0u8);
/// assert!(0u8.wrapping_mul(1u8) == 0u8);
/// assert!(0u8.wrapping_mul(254u8) == 0u8);
/// assert!(0u8.wrapping_mul(255u8) == 0u8);
/// assert!(1u8.wrapping_mul(0u8) == 0u8);
/// assert!(1u8.wrapping_mul(1u8) == 1u8);
/// assert!(1u8.wrapping_mul(254u8) == 254u8);
/// assert!(1u8.wrapping_mul(255u8) == 255u8);
/// assert!(254u8.wrapping_mul(0u8) == 0u8);
/// assert!(254u8.wrapping_mul(1u8) == 254u8);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() * y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u8::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_mul(0u8) == 0u8);
/// assert!(0u8.wrapping_mul(1u8) == 0u8);
/// assert!(0u8.wrapping_mul(254u8) == 0u8);
/// assert!(0u8.wrapping_mul(255u8) == 0u8);
/// assert!(1u8.wrapping_mul(0u8) == 0u8);
/// assert!(1u8.wrapping_mul(1u8) == 1u8);
/// assert!(1u8.wrapping_mul(254u8) == 254u8);
/// assert!(1u8.wrapping_mul(255u8) == 255u8);
/// assert!(254u8.wrapping_mul(0u8) == 0u8);
/// assert!(254u8.wrapping_mul(1u8) == 254u8);
/// # }
/// ```
pub fn core_ops_mul_u8_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u8>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 - 0u8 == 0u8);
/// assert!(1u8 - 0u8 == 1u8);
/// assert!(1u8 - 1u8 == 0u8);
/// assert!(254u8 - 0u8 == 254u8);
/// assert!(254u8 - 1u8 == 253u8);
/// assert!(254u8 - 254u8 == 0u8);
/// assert!(255u8 - 0u8 == 255u8);
/// assert!(255u8 - 1u8 == 254u8);
/// assert!(255u8 - 254u8 == 1u8);
/// assert!(255u8 - 255u8 == 0u8);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u8 - 1u8));
/// assert!(panics!(0u8 - 254u8));
/// assert!(panics!(0u8 - 255u8));
/// assert!(panics!(1u8 - 254u8));
/// assert!(panics!(1u8 - 255u8));
/// assert!(panics!(254u8 - 255u8));
/// assert!(panics!(75u8 - 171u8));
/// assert!(panics!(11u8 - 98u8));
/// assert!(panics!(94u8 - 207u8));
/// assert!(panics!(239u8 - 244u8));
/// # }
/// ```
pub fn core_ops_add_u8_sub() {}
/// # Properties for [`u8::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_sub(0u8) == 0u8);
/// assert!(1u8.wrapping_sub(0u8) == 1u8);
/// assert!(1u8.wrapping_sub(1u8) == 0u8);
/// assert!(254u8.wrapping_sub(0u8) == 254u8);
/// assert!(254u8.wrapping_sub(1u8) == 253u8);
/// assert!(254u8.wrapping_sub(254u8) == 0u8);
/// assert!(255u8.wrapping_sub(0u8) == 255u8);
/// assert!(255u8.wrapping_sub(1u8) == 254u8);
/// assert!(255u8.wrapping_sub(254u8) == 1u8);
/// assert!(255u8.wrapping_sub(255u8) == 0u8);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u8::down(x.up() - y.up() + u8::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_sub(1u8) == 255u8);
/// assert!(0u8.wrapping_sub(254u8) == 2u8);
/// assert!(0u8.wrapping_sub(255u8) == 1u8);
/// assert!(1u8.wrapping_sub(254u8) == 3u8);
/// assert!(1u8.wrapping_sub(255u8) == 2u8);
/// assert!(254u8.wrapping_sub(255u8) == 255u8);
/// assert!(105u8.wrapping_sub(112u8) == 249u8);
/// assert!(25u8.wrapping_sub(162u8) == 119u8);
/// assert!(28u8.wrapping_sub(191u8) == 93u8);
/// assert!(21u8.wrapping_sub(153u8) == 124u8);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u8::down((x.up() - y.up()).rem_euclid(&(u8::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_sub(0u8) == 0u8);
/// assert!(0u8.wrapping_sub(1u8) == 255u8);
/// assert!(0u8.wrapping_sub(254u8) == 2u8);
/// assert!(0u8.wrapping_sub(255u8) == 1u8);
/// assert!(1u8.wrapping_sub(0u8) == 1u8);
/// assert!(1u8.wrapping_sub(1u8) == 0u8);
/// assert!(1u8.wrapping_sub(254u8) == 3u8);
/// assert!(1u8.wrapping_sub(255u8) == 2u8);
/// assert!(254u8.wrapping_sub(0u8) == 254u8);
/// assert!(254u8.wrapping_sub(1u8) == 253u8);
/// # }
/// ```
pub fn core_ops_add_u8_wrapping_sub() {}
/// # Properties for [`u8::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u8::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_sub(0u8) == Some(0u8));
/// assert!(1u8.checked_sub(0u8) == Some(1u8));
/// assert!(1u8.checked_sub(1u8) == Some(0u8));
/// assert!(254u8.checked_sub(0u8) == Some(254u8));
/// assert!(254u8.checked_sub(1u8) == Some(253u8));
/// assert!(254u8.checked_sub(254u8) == Some(0u8));
/// assert!(255u8.checked_sub(0u8) == Some(255u8));
/// assert!(255u8.checked_sub(1u8) == Some(254u8));
/// assert!(255u8.checked_sub(254u8) == Some(1u8));
/// assert!(255u8.checked_sub(255u8) == Some(0u8));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_sub(1u8) == None);
/// assert!(0u8.checked_sub(254u8) == None);
/// assert!(0u8.checked_sub(255u8) == None);
/// assert!(1u8.checked_sub(254u8) == None);
/// assert!(1u8.checked_sub(255u8) == None);
/// assert!(254u8.checked_sub(255u8) == None);
/// assert!(130u8.checked_sub(151u8) == None);
/// assert!(62u8.checked_sub(98u8) == None);
/// assert!(28u8.checked_sub(162u8) == None);
/// assert!(139u8.checked_sub(225u8) == None);
/// # }
/// ```
pub fn core_ops_add_u8_checked_sub() {}
/// # Properties for [`u8::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u8::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_sub(0u8) == 0u8);
/// assert!(0u8.saturating_sub(1u8) == 0u8);
/// assert!(0u8.saturating_sub(254u8) == 0u8);
/// assert!(0u8.saturating_sub(255u8) == 0u8);
/// assert!(1u8.saturating_sub(0u8) == 1u8);
/// assert!(1u8.saturating_sub(1u8) == 0u8);
/// assert!(1u8.saturating_sub(254u8) == 0u8);
/// assert!(1u8.saturating_sub(255u8) == 0u8);
/// assert!(254u8.saturating_sub(0u8) == 254u8);
/// assert!(254u8.saturating_sub(1u8) == 253u8);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_sub(0u8) == 0u8);
/// assert!(1u8.saturating_sub(0u8) == 1u8);
/// assert!(1u8.saturating_sub(1u8) == 0u8);
/// assert!(254u8.saturating_sub(0u8) == 254u8);
/// assert!(254u8.saturating_sub(1u8) == 253u8);
/// assert!(254u8.saturating_sub(254u8) == 0u8);
/// assert!(255u8.saturating_sub(0u8) == 255u8);
/// assert!(255u8.saturating_sub(1u8) == 254u8);
/// assert!(255u8.saturating_sub(254u8) == 1u8);
/// assert!(255u8.saturating_sub(255u8) == 0u8);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_sub(1u8) == 0);
/// assert!(0u8.saturating_sub(254u8) == 0);
/// assert!(0u8.saturating_sub(255u8) == 0);
/// assert!(1u8.saturating_sub(254u8) == 0);
/// assert!(1u8.saturating_sub(255u8) == 0);
/// assert!(254u8.saturating_sub(255u8) == 0);
/// assert!(117u8.saturating_sub(171u8) == 0);
/// assert!(89u8.saturating_sub(254u8) == 0);
/// assert!(180u8.saturating_sub(211u8) == 0);
/// assert!(116u8.saturating_sub(147u8) == 0);
/// # }
/// ```
pub fn core_ops_sub_u8_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u8>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x + y == u8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 + 0u8 == 0u8);
/// assert!(0u8 + 1u8 == 1u8);
/// assert!(0u8 + 254u8 == 254u8);
/// assert!(0u8 + 255u8 == 255u8);
/// assert!(1u8 + 0u8 == 1u8);
/// assert!(1u8 + 1u8 == 2u8);
/// assert!(1u8 + 254u8 == 255u8);
/// assert!(254u8 + 0u8 == 254u8);
/// assert!(254u8 + 1u8 == 255u8);
/// assert!(255u8 + 0u8 == 255u8);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1u8 + 255u8));
/// assert!(panics!(254u8 + 254u8));
/// assert!(panics!(254u8 + 255u8));
/// assert!(panics!(255u8 + 1u8));
/// assert!(panics!(255u8 + 254u8));
/// assert!(panics!(255u8 + 255u8));
/// assert!(panics!(127u8 + 168u8));
/// assert!(panics!(235u8 + 225u8));
/// assert!(panics!(75u8 + 212u8));
/// assert!(panics!(52u8 + 224u8));
/// # }
/// ```
pub fn core_ops_add_u8_add() {}
/// # Properties for [`u8::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u8::down((x.up() + y.up()) % (u8::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_add(0u8) == 0u8);
/// assert!(0u8.wrapping_add(1u8) == 1u8);
/// assert!(0u8.wrapping_add(254u8) == 254u8);
/// assert!(0u8.wrapping_add(255u8) == 255u8);
/// assert!(1u8.wrapping_add(0u8) == 1u8);
/// assert!(1u8.wrapping_add(1u8) == 2u8);
/// assert!(1u8.wrapping_add(254u8) == 255u8);
/// assert!(1u8.wrapping_add(255u8) == 0u8);
/// assert!(254u8.wrapping_add(0u8) == 254u8);
/// assert!(254u8.wrapping_add(1u8) == 255u8);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.wrapping_add(0u8) == 0u8);
/// assert!(0u8.wrapping_add(1u8) == 1u8);
/// assert!(0u8.wrapping_add(254u8) == 254u8);
/// assert!(0u8.wrapping_add(255u8) == 255u8);
/// assert!(1u8.wrapping_add(0u8) == 1u8);
/// assert!(1u8.wrapping_add(1u8) == 2u8);
/// assert!(1u8.wrapping_add(254u8) == 255u8);
/// assert!(254u8.wrapping_add(0u8) == 254u8);
/// assert!(254u8.wrapping_add(1u8) == 255u8);
/// assert!(255u8.wrapping_add(0u8) == 255u8);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u8::down(x.up() + y.up() - u8::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.wrapping_add(255u8) == 0u8);
/// assert!(254u8.wrapping_add(254u8) == 252u8);
/// assert!(254u8.wrapping_add(255u8) == 253u8);
/// assert!(255u8.wrapping_add(1u8) == 0u8);
/// assert!(255u8.wrapping_add(254u8) == 253u8);
/// assert!(255u8.wrapping_add(255u8) == 254u8);
/// assert!(168u8.wrapping_add(171u8) == 83u8);
/// assert!(70u8.wrapping_add(206u8) == 20u8);
/// assert!(117u8.wrapping_add(174u8) == 35u8);
/// assert!(220u8.wrapping_add(202u8) == 166u8);
/// # }
/// ```
pub fn core_ops_add_u8_wrapping_add() {}
/// # Properties for [`u8::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u8::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.checked_add(0u8) == Some(0u8));
/// assert!(0u8.checked_add(1u8) == Some(1u8));
/// assert!(0u8.checked_add(254u8) == Some(254u8));
/// assert!(0u8.checked_add(255u8) == Some(255u8));
/// assert!(1u8.checked_add(0u8) == Some(1u8));
/// assert!(1u8.checked_add(1u8) == Some(2u8));
/// assert!(1u8.checked_add(254u8) == Some(255u8));
/// assert!(254u8.checked_add(0u8) == Some(254u8));
/// assert!(254u8.checked_add(1u8) == Some(255u8));
/// assert!(255u8.checked_add(0u8) == Some(255u8));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.checked_add(255u8) == None);
/// assert!(254u8.checked_add(254u8) == None);
/// assert!(254u8.checked_add(255u8) == None);
/// assert!(255u8.checked_add(1u8) == None);
/// assert!(255u8.checked_add(254u8) == None);
/// assert!(255u8.checked_add(255u8) == None);
/// assert!(247u8.checked_add(184u8) == None);
/// assert!(51u8.checked_add(247u8) == None);
/// assert!(138u8.checked_add(159u8) == None);
/// assert!(73u8.checked_add(203u8) == None);
/// # }
/// ```
pub fn core_ops_add_u8_checked_add() {}
/// # Properties for [`u8::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u8::down((x.up() + y.up()).min(u8::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_add(0u8) == 0u8);
/// assert!(0u8.saturating_add(1u8) == 1u8);
/// assert!(0u8.saturating_add(254u8) == 254u8);
/// assert!(0u8.saturating_add(255u8) == 255u8);
/// assert!(1u8.saturating_add(0u8) == 1u8);
/// assert!(1u8.saturating_add(1u8) == 2u8);
/// assert!(1u8.saturating_add(254u8) == 255u8);
/// assert!(1u8.saturating_add(255u8) == 255u8);
/// assert!(254u8.saturating_add(0u8) == 254u8);
/// assert!(254u8.saturating_add(1u8) == 255u8);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() <= u8::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.saturating_add(0u8) == 0u8);
/// assert!(0u8.saturating_add(1u8) == 1u8);
/// assert!(0u8.saturating_add(254u8) == 254u8);
/// assert!(0u8.saturating_add(255u8) == 255u8);
/// assert!(1u8.saturating_add(0u8) == 1u8);
/// assert!(1u8.saturating_add(1u8) == 2u8);
/// assert!(1u8.saturating_add(254u8) == 255u8);
/// assert!(254u8.saturating_add(0u8) == 254u8);
/// assert!(254u8.saturating_add(1u8) == 255u8);
/// assert!(255u8.saturating_add(0u8) == 255u8);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() > u8::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u8::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u8.saturating_add(255u8) == u8::MAX);
/// assert!(254u8.saturating_add(254u8) == u8::MAX);
/// assert!(254u8.saturating_add(255u8) == u8::MAX);
/// assert!(255u8.saturating_add(1u8) == u8::MAX);
/// assert!(255u8.saturating_add(254u8) == u8::MAX);
/// assert!(255u8.saturating_add(255u8) == u8::MAX);
/// assert!(43u8.saturating_add(243u8) == u8::MAX);
/// assert!(173u8.saturating_add(179u8) == u8::MAX);
/// assert!(185u8.saturating_add(170u8) == u8::MAX);
/// assert!(215u8.saturating_add(194u8) == u8::MAX);
/// # }
/// ```
pub fn core_ops_add_u8_saturating_add() {}
/// # Properties for [`core::ops::Rem::<u16>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u16::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 % 1u16 == 0u16);
/// assert!(0u16 % 65534u16 == 0u16);
/// assert!(0u16 % 65535u16 == 0u16);
/// assert!(1u16 % 1u16 == 0u16);
/// assert!(1u16 % 65534u16 == 1u16);
/// assert!(1u16 % 65535u16 == 1u16);
/// assert!(65534u16 % 1u16 == 0u16);
/// assert!(65534u16 % 65534u16 == 0u16);
/// assert!(65534u16 % 65535u16 == 65534u16);
/// assert!(65535u16 % 1u16 == 0u16);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u16 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u16_rem() {}
/// # Properties for [`u16::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u16::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_rem(1u16) == Some(0u16));
/// assert!(0u16.checked_rem(65534u16) == Some(0u16));
/// assert!(0u16.checked_rem(65535u16) == Some(0u16));
/// assert!(1u16.checked_rem(1u16) == Some(0u16));
/// assert!(1u16.checked_rem(65534u16) == Some(1u16));
/// assert!(1u16.checked_rem(65535u16) == Some(1u16));
/// assert!(65534u16.checked_rem(1u16) == Some(0u16));
/// assert!(65534u16.checked_rem(65534u16) == Some(0u16));
/// assert!(65534u16.checked_rem(65535u16) == Some(65534u16));
/// assert!(65535u16.checked_rem(1u16) == Some(0u16));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u16.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u16_checked_rem() {}
/// # Properties for [`u16::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `x == u16::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u16::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_neg() == Some(0u16));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `x != u16::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.checked_neg() == None);
/// assert!(65534u16.checked_neg() == None);
/// assert!(65535u16.checked_neg() == None);
/// assert!(12300u16.checked_neg() == None);
/// assert!(59658u16.checked_neg() == None);
/// assert!(11314u16.checked_neg() == None);
/// assert!(58339u16.checked_neg() == None);
/// assert!(40954u16.checked_neg() == None);
/// assert!(3555u16.checked_neg() == None);
/// assert!(27725u16.checked_neg() == None);
/// # }
/// ```
pub fn t_u16_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u16>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x << y == u16::down((x.up() << y) & u16::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 << 0u32 == 0u16);
/// assert!(0u16 << 1u32 == 0u16);
/// assert!(1u16 << 0u32 == 1u16);
/// assert!(1u16 << 1u32 == 2u16);
/// assert!(65534u16 << 0u32 == 65534u16);
/// assert!(65534u16 << 1u32 == 65532u16);
/// assert!(65535u16 << 0u32 == 65535u16);
/// assert!(65535u16 << 1u32 == 65534u16);
/// assert!(17109u16 << 1u32 == 34218u16);
/// assert!(39799u16 << 9u32 == 60928u16);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u16 << 4294967294u32));
/// assert!(panics!(0u16 << 4294967295u32));
/// assert!(panics!(1u16 << 4294967294u32));
/// assert!(panics!(1u16 << 4294967295u32));
/// assert!(panics!(65534u16 << 4294967294u32));
/// assert!(panics!(65534u16 << 4294967295u32));
/// assert!(panics!(65535u16 << 4294967294u32));
/// assert!(panics!(65535u16 << 4294967295u32));
/// assert!(panics!(26691u16 << 69u32));
/// assert!(panics!(46388u16 << 115u32));
/// # }
/// ```
pub fn core_ops_shl_u16_shl() {}
/// # Properties for [`core::ops::Shl::<u16>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u16::down((x.up() << y) & u16::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_shl(0u32) == Some(0u16));
/// assert!(0u16.checked_shl(1u32) == Some(0u16));
/// assert!(1u16.checked_shl(0u32) == Some(1u16));
/// assert!(1u16.checked_shl(1u32) == Some(2u16));
/// assert!(65534u16.checked_shl(0u32) == Some(65534u16));
/// assert!(65534u16.checked_shl(1u32) == Some(65532u16));
/// assert!(65535u16.checked_shl(0u32) == Some(65535u16));
/// assert!(65535u16.checked_shl(1u32) == Some(65534u16));
/// assert!(42036u16.checked_shl(8u32) == Some(13312u16));
/// assert!(15670u16.checked_shl(4u32) == Some(54112u16));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_shl(4294967294u32) == None);
/// assert!(0u16.checked_shl(4294967295u32) == None);
/// assert!(1u16.checked_shl(4294967294u32) == None);
/// assert!(1u16.checked_shl(4294967295u32) == None);
/// assert!(65534u16.checked_shl(4294967294u32) == None);
/// assert!(65534u16.checked_shl(4294967295u32) == None);
/// assert!(65535u16.checked_shl(4294967294u32) == None);
/// assert!(65535u16.checked_shl(4294967295u32) == None);
/// assert!(34071u16.checked_shl(103u32) == None);
/// assert!(26372u16.checked_shl(87u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u16_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u16>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u16::down((x.up() << y) & u16::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.overflowing_shl(0u32) == (0u16, false));
/// assert!(0u16.overflowing_shl(1u32) == (0u16, false));
/// assert!(1u16.overflowing_shl(0u32) == (1u16, false));
/// assert!(1u16.overflowing_shl(1u32) == (2u16, false));
/// assert!(65534u16.overflowing_shl(0u32) == (65534u16, false));
/// assert!(65534u16.overflowing_shl(1u32) == (65532u16, false));
/// assert!(65535u16.overflowing_shl(0u32) == (65535u16, false));
/// assert!(65535u16.overflowing_shl(1u32) == (65534u16, false));
/// assert!(20996u16.overflowing_shl(9u32) == (2048u16, false));
/// assert!(60245u16.overflowing_shl(9u32) == (43520u16, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u16::down((x.up() << (y & (u16::BITS - 1)) & u16::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.overflowing_shl(4294967294u32) == (0u16, true));
/// assert!(0u16.overflowing_shl(4294967295u32) == (0u16, true));
/// assert!(1u16.overflowing_shl(4294967294u32) == (16384u16, true));
/// assert!(1u16.overflowing_shl(4294967295u32) == (32768u16, true));
/// assert!(65534u16.overflowing_shl(4294967294u32) == (32768u16, true));
/// assert!(65534u16.overflowing_shl(4294967295u32) == (0u16, true));
/// assert!(65535u16.overflowing_shl(4294967294u32) == (49152u16, true));
/// assert!(65535u16.overflowing_shl(4294967295u32) == (32768u16, true));
/// assert!(37576u16.overflowing_shl(124u32) == (32768u16, true));
/// assert!(2473u16.overflowing_shl(19u32) == (19784u16, true));
/// # }
/// ```
pub fn core_ops_shl_u16_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u16>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x >> y == u16::down((x.up() >> y) & u16::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 >> 0u32 == 0u16);
/// assert!(0u16 >> 1u32 == 0u16);
/// assert!(1u16 >> 0u32 == 1u16);
/// assert!(1u16 >> 1u32 == 0u16);
/// assert!(65534u16 >> 0u32 == 65534u16);
/// assert!(65534u16 >> 1u32 == 32767u16);
/// assert!(65535u16 >> 0u32 == 65535u16);
/// assert!(65535u16 >> 1u32 == 32767u16);
/// assert!(6884u16 >> 11u32 == 3u16);
/// assert!(92u16 >> 13u32 == 0u16);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u16 >> 4294967294u32));
/// assert!(panics!(0u16 >> 4294967295u32));
/// assert!(panics!(1u16 >> 4294967294u32));
/// assert!(panics!(1u16 >> 4294967295u32));
/// assert!(panics!(65534u16 >> 4294967294u32));
/// assert!(panics!(65534u16 >> 4294967295u32));
/// assert!(panics!(65535u16 >> 4294967294u32));
/// assert!(panics!(65535u16 >> 4294967295u32));
/// assert!(panics!(30054u16 >> 116u32));
/// assert!(panics!(6497u16 >> 48u32));
/// # }
/// ```
pub fn core_ops_shr_u16_shr() {}
/// # Properties for [`core::ops::Shr::<u16>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u16::down((x.up() >> y) & u16::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_shr(0u32) == Some(0u16));
/// assert!(0u16.checked_shr(1u32) == Some(0u16));
/// assert!(1u16.checked_shr(0u32) == Some(1u16));
/// assert!(1u16.checked_shr(1u32) == Some(0u16));
/// assert!(65534u16.checked_shr(0u32) == Some(65534u16));
/// assert!(65534u16.checked_shr(1u32) == Some(32767u16));
/// assert!(65535u16.checked_shr(0u32) == Some(65535u16));
/// assert!(65535u16.checked_shr(1u32) == Some(32767u16));
/// assert!(2063u16.checked_shr(7u32) == Some(16u16));
/// assert!(33113u16.checked_shr(15u32) == Some(1u16));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_shr(4294967294u32) == None);
/// assert!(0u16.checked_shr(4294967295u32) == None);
/// assert!(1u16.checked_shr(4294967294u32) == None);
/// assert!(1u16.checked_shr(4294967295u32) == None);
/// assert!(65534u16.checked_shr(4294967294u32) == None);
/// assert!(65534u16.checked_shr(4294967295u32) == None);
/// assert!(65535u16.checked_shr(4294967294u32) == None);
/// assert!(65535u16.checked_shr(4294967295u32) == None);
/// assert!(51949u16.checked_shr(129u32) == None);
/// assert!(40613u16.checked_shr(47u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u16_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u16>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y < u16::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u16::down((x.up() >> y) & u16::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.overflowing_shr(0u32) == (0u16, false));
/// assert!(0u16.overflowing_shr(1u32) == (0u16, false));
/// assert!(1u16.overflowing_shr(0u32) == (1u16, false));
/// assert!(1u16.overflowing_shr(1u32) == (0u16, false));
/// assert!(65534u16.overflowing_shr(0u32) == (65534u16, false));
/// assert!(65534u16.overflowing_shr(1u32) == (32767u16, false));
/// assert!(65535u16.overflowing_shr(0u32) == (65535u16, false));
/// assert!(65535u16.overflowing_shr(1u32) == (32767u16, false));
/// assert!(16556u16.overflowing_shr(6u32) == (258u16, false));
/// assert!(51785u16.overflowing_shr(0u32) == (51785u16, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u16, y : u32`  
/// __Precondition:__ `y >= u16::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u16::down((x.up() >> (y & (u16::BITS - 1)) & u16::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.overflowing_shr(4294967294u32) == (0u16, true));
/// assert!(0u16.overflowing_shr(4294967295u32) == (0u16, true));
/// assert!(1u16.overflowing_shr(4294967294u32) == (0u16, true));
/// assert!(1u16.overflowing_shr(4294967295u32) == (0u16, true));
/// assert!(65534u16.overflowing_shr(4294967294u32) == (3u16, true));
/// assert!(65534u16.overflowing_shr(4294967295u32) == (1u16, true));
/// assert!(65535u16.overflowing_shr(4294967294u32) == (3u16, true));
/// assert!(65535u16.overflowing_shr(4294967295u32) == (1u16, true));
/// assert!(15319u16.overflowing_shr(117u32) == (478u16, true));
/// assert!(45313u16.overflowing_shr(104u32) == (177u16, true));
/// # }
/// ```
pub fn core_ops_shr_u16_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u16>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u16::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 / 1u16 == 0u16);
/// assert!(0u16 / 65534u16 == 0u16);
/// assert!(0u16 / 65535u16 == 0u16);
/// assert!(1u16 / 1u16 == 1u16);
/// assert!(1u16 / 65534u16 == 0u16);
/// assert!(1u16 / 65535u16 == 0u16);
/// assert!(65534u16 / 1u16 == 65534u16);
/// assert!(65534u16 / 65534u16 == 1u16);
/// assert!(65534u16 / 65535u16 == 0u16);
/// assert!(65535u16 / 1u16 == 65535u16);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(65534u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(65535u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(55231u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(20203u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(39738u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(26220u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(57853u16 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(55149u16 / 0) } });
/// # }
/// ```
pub fn core_ops_div_u16_div() {}
/// # Properties for [`u16::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u16::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_div(1u16) == 0u16);
/// assert!(0u16.saturating_div(65534u16) == 0u16);
/// assert!(0u16.saturating_div(65535u16) == 0u16);
/// assert!(1u16.saturating_div(1u16) == 1u16);
/// assert!(1u16.saturating_div(65534u16) == 0u16);
/// assert!(1u16.saturating_div(65535u16) == 0u16);
/// assert!(65534u16.saturating_div(1u16) == 65534u16);
/// assert!(65534u16.saturating_div(65534u16) == 1u16);
/// assert!(65534u16.saturating_div(65535u16) == 0u16);
/// assert!(65535u16.saturating_div(1u16) == 65535u16);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(65534u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(65535u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(27937u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(22389u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(51472u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(52287u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(27487u16.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(59335u16.saturating_div(0)) } });
/// # }
/// ```
pub fn core_ops_div_u16_saturating_div() {}
/// # Properties for [`u16::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u16::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_div(1u16) == Some(0u16));
/// assert!(0u16.checked_div(65534u16) == Some(0u16));
/// assert!(0u16.checked_div(65535u16) == Some(0u16));
/// assert!(1u16.checked_div(1u16) == Some(1u16));
/// assert!(1u16.checked_div(65534u16) == Some(0u16));
/// assert!(1u16.checked_div(65535u16) == Some(0u16));
/// assert!(65534u16.checked_div(1u16) == Some(65534u16));
/// assert!(65534u16.checked_div(65534u16) == Some(1u16));
/// assert!(65534u16.checked_div(65535u16) == Some(0u16));
/// assert!(65535u16.checked_div(1u16) == Some(65535u16));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_div(0) == None);
/// assert!(1u16.checked_div(0) == None);
/// assert!(65534u16.checked_div(0) == None);
/// assert!(65535u16.checked_div(0) == None);
/// assert!(20775u16.checked_div(0) == None);
/// assert!(31397u16.checked_div(0) == None);
/// assert!(40733u16.checked_div(0) == None);
/// assert!(28299u16.checked_div(0) == None);
/// assert!(1520u16.checked_div(0) == None);
/// assert!(59395u16.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u16_checked_div() {}
/// # Properties for [`core::ops::Mul::<u16>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x * y == u16::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 * 0u16 == 0u16);
/// assert!(0u16 * 1u16 == 0u16);
/// assert!(0u16 * 65534u16 == 0u16);
/// assert!(0u16 * 65535u16 == 0u16);
/// assert!(1u16 * 0u16 == 0u16);
/// assert!(1u16 * 1u16 == 1u16);
/// assert!(1u16 * 65534u16 == 65534u16);
/// assert!(1u16 * 65535u16 == 65535u16);
/// assert!(65534u16 * 0u16 == 0u16);
/// assert!(65534u16 * 1u16 == 65534u16);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() > u16::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(65534u16 * 65534u16));
/// assert!(panics!(65534u16 * 65535u16));
/// assert!(panics!(65535u16 * 65534u16));
/// assert!(panics!(65535u16 * 65535u16));
/// assert!(panics!(33366u16 * 64461u16));
/// assert!(panics!(43075u16 * 20311u16));
/// assert!(panics!(39414u16 * 19827u16));
/// assert!(panics!(32906u16 * 46374u16));
/// assert!(panics!(9737u16 * 8680u16));
/// assert!(panics!(12878u16 * 54696u16));
/// # }
/// ```
pub fn core_ops_mul_u16_mul() {}
/// # Properties for [`u16::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u16::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_mul(0u16) == Some(0u16));
/// assert!(0u16.checked_mul(1u16) == Some(0u16));
/// assert!(0u16.checked_mul(65534u16) == Some(0u16));
/// assert!(0u16.checked_mul(65535u16) == Some(0u16));
/// assert!(1u16.checked_mul(0u16) == Some(0u16));
/// assert!(1u16.checked_mul(1u16) == Some(1u16));
/// assert!(1u16.checked_mul(65534u16) == Some(65534u16));
/// assert!(1u16.checked_mul(65535u16) == Some(65535u16));
/// assert!(65534u16.checked_mul(0u16) == Some(0u16));
/// assert!(65534u16.checked_mul(1u16) == Some(65534u16));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.checked_mul(65534u16) == None);
/// assert!(65534u16.checked_mul(65535u16) == None);
/// assert!(65535u16.checked_mul(65534u16) == None);
/// assert!(65535u16.checked_mul(65535u16) == None);
/// assert!(42576u16.checked_mul(41856u16) == None);
/// assert!(16911u16.checked_mul(24628u16) == None);
/// assert!(25152u16.checked_mul(22578u16) == None);
/// assert!(4666u16.checked_mul(39062u16) == None);
/// assert!(45193u16.checked_mul(46336u16) == None);
/// assert!(53570u16.checked_mul(24125u16) == None);
/// # }
/// ```
pub fn core_ops_mul_u16_checked_mul() {}
/// # Properties for [`u16::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u16::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.overflowing_mul(0u16) == (0u16, false));
/// assert!(0u16.overflowing_mul(1u16) == (0u16, false));
/// assert!(0u16.overflowing_mul(65534u16) == (0u16, false));
/// assert!(0u16.overflowing_mul(65535u16) == (0u16, false));
/// assert!(1u16.overflowing_mul(0u16) == (0u16, false));
/// assert!(1u16.overflowing_mul(1u16) == (1u16, false));
/// assert!(1u16.overflowing_mul(65534u16) == (65534u16, false));
/// assert!(1u16.overflowing_mul(65535u16) == (65535u16, false));
/// assert!(65534u16.overflowing_mul(0u16) == (0u16, false));
/// assert!(65534u16.overflowing_mul(1u16) == (65534u16, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.overflowing_mul(65534u16) == (4u16, true));
/// assert!(65534u16.overflowing_mul(65535u16) == (2u16, true));
/// assert!(65535u16.overflowing_mul(65534u16) == (2u16, true));
/// assert!(65535u16.overflowing_mul(65535u16) == (1u16, true));
/// assert!(59183u16.overflowing_mul(4157u16) == (1587u16, true));
/// assert!(39862u16.overflowing_mul(20546u16) == (1260u16, true));
/// assert!(32750u16.overflowing_mul(52100u16) == (45240u16, true));
/// assert!(34305u16.overflowing_mul(49572u16) == (39332u16, true));
/// assert!(24363u16.overflowing_mul(57181u16) == (1951u16, true));
/// assert!(26829u16.overflowing_mul(35336u16) == (51304u16, true));
/// # }
/// ```
pub fn core_ops_mul_u16_overflowing_mul() {}
/// # Properties for [`u16::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u16::down((x.up() * y.up()).min(u16::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_mul(0u16) == 0u16);
/// assert!(0u16.saturating_mul(1u16) == 0u16);
/// assert!(0u16.saturating_mul(65534u16) == 0u16);
/// assert!(0u16.saturating_mul(65535u16) == 0u16);
/// assert!(1u16.saturating_mul(0u16) == 0u16);
/// assert!(1u16.saturating_mul(1u16) == 1u16);
/// assert!(1u16.saturating_mul(65534u16) == 65534u16);
/// assert!(1u16.saturating_mul(65535u16) == 65535u16);
/// assert!(65534u16.saturating_mul(0u16) == 0u16);
/// assert!(65534u16.saturating_mul(1u16) == 65534u16);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u16::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_mul(0u16) == 0u16);
/// assert!(0u16.saturating_mul(1u16) == 0u16);
/// assert!(0u16.saturating_mul(65534u16) == 0u16);
/// assert!(0u16.saturating_mul(65535u16) == 0u16);
/// assert!(1u16.saturating_mul(0u16) == 0u16);
/// assert!(1u16.saturating_mul(1u16) == 1u16);
/// assert!(1u16.saturating_mul(65534u16) == 65534u16);
/// assert!(1u16.saturating_mul(65535u16) == 65535u16);
/// assert!(65534u16.saturating_mul(0u16) == 0u16);
/// assert!(65534u16.saturating_mul(1u16) == 65534u16);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u16::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(65534u16.saturating_mul(65534u16) == u16::MAX);
/// assert!(65534u16.saturating_mul(65535u16) == u16::MAX);
/// assert!(65535u16.saturating_mul(65534u16) == u16::MAX);
/// assert!(65535u16.saturating_mul(65535u16) == u16::MAX);
/// assert!(17862u16.saturating_mul(27066u16) == u16::MAX);
/// assert!(8359u16.saturating_mul(62211u16) == u16::MAX);
/// assert!(34745u16.saturating_mul(54090u16) == u16::MAX);
/// assert!(27127u16.saturating_mul(27391u16) == u16::MAX);
/// assert!(1014u16.saturating_mul(450u16) == u16::MAX);
/// assert!(47393u16.saturating_mul(37489u16) == u16::MAX);
/// # }
/// ```
pub fn core_ops_mul_u16_saturating_mul() {}
/// # Properties for [`u16::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u16::down((x.up() * y.up()) % (u16::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_mul(0u16) == 0u16);
/// assert!(0u16.wrapping_mul(1u16) == 0u16);
/// assert!(0u16.wrapping_mul(65534u16) == 0u16);
/// assert!(0u16.wrapping_mul(65535u16) == 0u16);
/// assert!(1u16.wrapping_mul(0u16) == 0u16);
/// assert!(1u16.wrapping_mul(1u16) == 1u16);
/// assert!(1u16.wrapping_mul(65534u16) == 65534u16);
/// assert!(1u16.wrapping_mul(65535u16) == 65535u16);
/// assert!(65534u16.wrapping_mul(0u16) == 0u16);
/// assert!(65534u16.wrapping_mul(1u16) == 65534u16);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() * y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u16::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_mul(0u16) == 0u16);
/// assert!(0u16.wrapping_mul(1u16) == 0u16);
/// assert!(0u16.wrapping_mul(65534u16) == 0u16);
/// assert!(0u16.wrapping_mul(65535u16) == 0u16);
/// assert!(1u16.wrapping_mul(0u16) == 0u16);
/// assert!(1u16.wrapping_mul(1u16) == 1u16);
/// assert!(1u16.wrapping_mul(65534u16) == 65534u16);
/// assert!(1u16.wrapping_mul(65535u16) == 65535u16);
/// assert!(65534u16.wrapping_mul(0u16) == 0u16);
/// assert!(65534u16.wrapping_mul(1u16) == 65534u16);
/// # }
/// ```
pub fn core_ops_mul_u16_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u16>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 - 0u16 == 0u16);
/// assert!(1u16 - 0u16 == 1u16);
/// assert!(1u16 - 1u16 == 0u16);
/// assert!(65534u16 - 0u16 == 65534u16);
/// assert!(65534u16 - 1u16 == 65533u16);
/// assert!(65534u16 - 65534u16 == 0u16);
/// assert!(65535u16 - 0u16 == 65535u16);
/// assert!(65535u16 - 1u16 == 65534u16);
/// assert!(65535u16 - 65534u16 == 1u16);
/// assert!(65535u16 - 65535u16 == 0u16);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u16 - 1u16));
/// assert!(panics!(0u16 - 65534u16));
/// assert!(panics!(0u16 - 65535u16));
/// assert!(panics!(1u16 - 65534u16));
/// assert!(panics!(1u16 - 65535u16));
/// assert!(panics!(65534u16 - 65535u16));
/// assert!(panics!(20989u16 - 51259u16));
/// assert!(panics!(25165u16 - 43717u16));
/// assert!(panics!(3653u16 - 17732u16));
/// assert!(panics!(2366u16 - 11195u16));
/// # }
/// ```
pub fn core_ops_add_u16_sub() {}
/// # Properties for [`u16::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_sub(0u16) == 0u16);
/// assert!(1u16.wrapping_sub(0u16) == 1u16);
/// assert!(1u16.wrapping_sub(1u16) == 0u16);
/// assert!(65534u16.wrapping_sub(0u16) == 65534u16);
/// assert!(65534u16.wrapping_sub(1u16) == 65533u16);
/// assert!(65534u16.wrapping_sub(65534u16) == 0u16);
/// assert!(65535u16.wrapping_sub(0u16) == 65535u16);
/// assert!(65535u16.wrapping_sub(1u16) == 65534u16);
/// assert!(65535u16.wrapping_sub(65534u16) == 1u16);
/// assert!(65535u16.wrapping_sub(65535u16) == 0u16);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u16::down(x.up() - y.up() + u16::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_sub(1u16) == 65535u16);
/// assert!(0u16.wrapping_sub(65534u16) == 2u16);
/// assert!(0u16.wrapping_sub(65535u16) == 1u16);
/// assert!(1u16.wrapping_sub(65534u16) == 3u16);
/// assert!(1u16.wrapping_sub(65535u16) == 2u16);
/// assert!(65534u16.wrapping_sub(65535u16) == 65535u16);
/// assert!(14389u16.wrapping_sub(17414u16) == 62511u16);
/// assert!(52087u16.wrapping_sub(59923u16) == 57700u16);
/// assert!(13183u16.wrapping_sub(31310u16) == 47409u16);
/// assert!(30765u16.wrapping_sub(33988u16) == 62313u16);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u16::down((x.up() - y.up()).rem_euclid(&(u16::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_sub(0u16) == 0u16);
/// assert!(0u16.wrapping_sub(1u16) == 65535u16);
/// assert!(0u16.wrapping_sub(65534u16) == 2u16);
/// assert!(0u16.wrapping_sub(65535u16) == 1u16);
/// assert!(1u16.wrapping_sub(0u16) == 1u16);
/// assert!(1u16.wrapping_sub(1u16) == 0u16);
/// assert!(1u16.wrapping_sub(65534u16) == 3u16);
/// assert!(1u16.wrapping_sub(65535u16) == 2u16);
/// assert!(65534u16.wrapping_sub(0u16) == 65534u16);
/// assert!(65534u16.wrapping_sub(1u16) == 65533u16);
/// # }
/// ```
pub fn core_ops_add_u16_wrapping_sub() {}
/// # Properties for [`u16::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u16::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_sub(0u16) == Some(0u16));
/// assert!(1u16.checked_sub(0u16) == Some(1u16));
/// assert!(1u16.checked_sub(1u16) == Some(0u16));
/// assert!(65534u16.checked_sub(0u16) == Some(65534u16));
/// assert!(65534u16.checked_sub(1u16) == Some(65533u16));
/// assert!(65534u16.checked_sub(65534u16) == Some(0u16));
/// assert!(65535u16.checked_sub(0u16) == Some(65535u16));
/// assert!(65535u16.checked_sub(1u16) == Some(65534u16));
/// assert!(65535u16.checked_sub(65534u16) == Some(1u16));
/// assert!(65535u16.checked_sub(65535u16) == Some(0u16));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_sub(1u16) == None);
/// assert!(0u16.checked_sub(65534u16) == None);
/// assert!(0u16.checked_sub(65535u16) == None);
/// assert!(1u16.checked_sub(65534u16) == None);
/// assert!(1u16.checked_sub(65535u16) == None);
/// assert!(65534u16.checked_sub(65535u16) == None);
/// assert!(12128u16.checked_sub(44317u16) == None);
/// assert!(5665u16.checked_sub(17801u16) == None);
/// assert!(2325u16.checked_sub(5898u16) == None);
/// assert!(418u16.checked_sub(33044u16) == None);
/// # }
/// ```
pub fn core_ops_add_u16_checked_sub() {}
/// # Properties for [`u16::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u16::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_sub(0u16) == 0u16);
/// assert!(0u16.saturating_sub(1u16) == 0u16);
/// assert!(0u16.saturating_sub(65534u16) == 0u16);
/// assert!(0u16.saturating_sub(65535u16) == 0u16);
/// assert!(1u16.saturating_sub(0u16) == 1u16);
/// assert!(1u16.saturating_sub(1u16) == 0u16);
/// assert!(1u16.saturating_sub(65534u16) == 0u16);
/// assert!(1u16.saturating_sub(65535u16) == 0u16);
/// assert!(65534u16.saturating_sub(0u16) == 65534u16);
/// assert!(65534u16.saturating_sub(1u16) == 65533u16);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_sub(0u16) == 0u16);
/// assert!(1u16.saturating_sub(0u16) == 1u16);
/// assert!(1u16.saturating_sub(1u16) == 0u16);
/// assert!(65534u16.saturating_sub(0u16) == 65534u16);
/// assert!(65534u16.saturating_sub(1u16) == 65533u16);
/// assert!(65534u16.saturating_sub(65534u16) == 0u16);
/// assert!(65535u16.saturating_sub(0u16) == 65535u16);
/// assert!(65535u16.saturating_sub(1u16) == 65534u16);
/// assert!(65535u16.saturating_sub(65534u16) == 1u16);
/// assert!(65535u16.saturating_sub(65535u16) == 0u16);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_sub(1u16) == 0);
/// assert!(0u16.saturating_sub(65534u16) == 0);
/// assert!(0u16.saturating_sub(65535u16) == 0);
/// assert!(1u16.saturating_sub(65534u16) == 0);
/// assert!(1u16.saturating_sub(65535u16) == 0);
/// assert!(65534u16.saturating_sub(65535u16) == 0);
/// assert!(19257u16.saturating_sub(63701u16) == 0);
/// assert!(6285u16.saturating_sub(34562u16) == 0);
/// assert!(28854u16.saturating_sub(40277u16) == 0);
/// assert!(7156u16.saturating_sub(34455u16) == 0);
/// # }
/// ```
pub fn core_ops_sub_u16_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u16>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x + y == u16::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 + 0u16 == 0u16);
/// assert!(0u16 + 1u16 == 1u16);
/// assert!(0u16 + 65534u16 == 65534u16);
/// assert!(0u16 + 65535u16 == 65535u16);
/// assert!(1u16 + 0u16 == 1u16);
/// assert!(1u16 + 1u16 == 2u16);
/// assert!(1u16 + 65534u16 == 65535u16);
/// assert!(65534u16 + 0u16 == 65534u16);
/// assert!(65534u16 + 1u16 == 65535u16);
/// assert!(65535u16 + 0u16 == 65535u16);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1u16 + 65535u16));
/// assert!(panics!(65534u16 + 65534u16));
/// assert!(panics!(65534u16 + 65535u16));
/// assert!(panics!(65535u16 + 1u16));
/// assert!(panics!(65535u16 + 65534u16));
/// assert!(panics!(65535u16 + 65535u16));
/// assert!(panics!(64786u16 + 20740u16));
/// assert!(panics!(43444u16 + 56392u16));
/// assert!(panics!(59361u16 + 10675u16));
/// assert!(panics!(42329u16 + 63742u16));
/// # }
/// ```
pub fn core_ops_add_u16_add() {}
/// # Properties for [`u16::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u16::down((x.up() + y.up()) % (u16::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_add(0u16) == 0u16);
/// assert!(0u16.wrapping_add(1u16) == 1u16);
/// assert!(0u16.wrapping_add(65534u16) == 65534u16);
/// assert!(0u16.wrapping_add(65535u16) == 65535u16);
/// assert!(1u16.wrapping_add(0u16) == 1u16);
/// assert!(1u16.wrapping_add(1u16) == 2u16);
/// assert!(1u16.wrapping_add(65534u16) == 65535u16);
/// assert!(1u16.wrapping_add(65535u16) == 0u16);
/// assert!(65534u16.wrapping_add(0u16) == 65534u16);
/// assert!(65534u16.wrapping_add(1u16) == 65535u16);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u16::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.wrapping_add(0u16) == 0u16);
/// assert!(0u16.wrapping_add(1u16) == 1u16);
/// assert!(0u16.wrapping_add(65534u16) == 65534u16);
/// assert!(0u16.wrapping_add(65535u16) == 65535u16);
/// assert!(1u16.wrapping_add(0u16) == 1u16);
/// assert!(1u16.wrapping_add(1u16) == 2u16);
/// assert!(1u16.wrapping_add(65534u16) == 65535u16);
/// assert!(65534u16.wrapping_add(0u16) == 65534u16);
/// assert!(65534u16.wrapping_add(1u16) == 65535u16);
/// assert!(65535u16.wrapping_add(0u16) == 65535u16);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u16::down(x.up() + y.up() - u16::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.wrapping_add(65535u16) == 0u16);
/// assert!(65534u16.wrapping_add(65534u16) == 65532u16);
/// assert!(65534u16.wrapping_add(65535u16) == 65533u16);
/// assert!(65535u16.wrapping_add(1u16) == 0u16);
/// assert!(65535u16.wrapping_add(65534u16) == 65533u16);
/// assert!(65535u16.wrapping_add(65535u16) == 65534u16);
/// assert!(34453u16.wrapping_add(33258u16) == 2175u16);
/// assert!(48367u16.wrapping_add(25299u16) == 8130u16);
/// assert!(56006u16.wrapping_add(50886u16) == 41356u16);
/// assert!(20981u16.wrapping_add(57137u16) == 12582u16);
/// # }
/// ```
pub fn core_ops_add_u16_wrapping_add() {}
/// # Properties for [`u16::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u16::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.checked_add(0u16) == Some(0u16));
/// assert!(0u16.checked_add(1u16) == Some(1u16));
/// assert!(0u16.checked_add(65534u16) == Some(65534u16));
/// assert!(0u16.checked_add(65535u16) == Some(65535u16));
/// assert!(1u16.checked_add(0u16) == Some(1u16));
/// assert!(1u16.checked_add(1u16) == Some(2u16));
/// assert!(1u16.checked_add(65534u16) == Some(65535u16));
/// assert!(65534u16.checked_add(0u16) == Some(65534u16));
/// assert!(65534u16.checked_add(1u16) == Some(65535u16));
/// assert!(65535u16.checked_add(0u16) == Some(65535u16));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.checked_add(65535u16) == None);
/// assert!(65534u16.checked_add(65534u16) == None);
/// assert!(65534u16.checked_add(65535u16) == None);
/// assert!(65535u16.checked_add(1u16) == None);
/// assert!(65535u16.checked_add(65534u16) == None);
/// assert!(65535u16.checked_add(65535u16) == None);
/// assert!(46701u16.checked_add(33628u16) == None);
/// assert!(51603u16.checked_add(63391u16) == None);
/// assert!(53146u16.checked_add(18836u16) == None);
/// assert!(27988u16.checked_add(39623u16) == None);
/// # }
/// ```
pub fn core_ops_add_u16_checked_add() {}
/// # Properties for [`u16::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u16::down((x.up() + y.up()).min(u16::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_add(0u16) == 0u16);
/// assert!(0u16.saturating_add(1u16) == 1u16);
/// assert!(0u16.saturating_add(65534u16) == 65534u16);
/// assert!(0u16.saturating_add(65535u16) == 65535u16);
/// assert!(1u16.saturating_add(0u16) == 1u16);
/// assert!(1u16.saturating_add(1u16) == 2u16);
/// assert!(1u16.saturating_add(65534u16) == 65535u16);
/// assert!(1u16.saturating_add(65535u16) == 65535u16);
/// assert!(65534u16.saturating_add(0u16) == 65534u16);
/// assert!(65534u16.saturating_add(1u16) == 65535u16);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() <= u16::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u16::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.saturating_add(0u16) == 0u16);
/// assert!(0u16.saturating_add(1u16) == 1u16);
/// assert!(0u16.saturating_add(65534u16) == 65534u16);
/// assert!(0u16.saturating_add(65535u16) == 65535u16);
/// assert!(1u16.saturating_add(0u16) == 1u16);
/// assert!(1u16.saturating_add(1u16) == 2u16);
/// assert!(1u16.saturating_add(65534u16) == 65535u16);
/// assert!(65534u16.saturating_add(0u16) == 65534u16);
/// assert!(65534u16.saturating_add(1u16) == 65535u16);
/// assert!(65535u16.saturating_add(0u16) == 65535u16);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `x.up() + y.up() > u16::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u16::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u16.saturating_add(65535u16) == u16::MAX);
/// assert!(65534u16.saturating_add(65534u16) == u16::MAX);
/// assert!(65534u16.saturating_add(65535u16) == u16::MAX);
/// assert!(65535u16.saturating_add(1u16) == u16::MAX);
/// assert!(65535u16.saturating_add(65534u16) == u16::MAX);
/// assert!(65535u16.saturating_add(65535u16) == u16::MAX);
/// assert!(25864u16.saturating_add(59110u16) == u16::MAX);
/// assert!(49933u16.saturating_add(64454u16) == u16::MAX);
/// assert!(11406u16.saturating_add(62706u16) == u16::MAX);
/// assert!(32920u16.saturating_add(64028u16) == u16::MAX);
/// # }
/// ```
pub fn core_ops_add_u16_saturating_add() {}
/// # Properties for [`core::ops::Rem::<u32>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u32::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 % 1u32 == 0u32);
/// assert!(0u32 % 4294967294u32 == 0u32);
/// assert!(0u32 % 4294967295u32 == 0u32);
/// assert!(1u32 % 1u32 == 0u32);
/// assert!(1u32 % 4294967294u32 == 1u32);
/// assert!(1u32 % 4294967295u32 == 1u32);
/// assert!(4294967294u32 % 1u32 == 0u32);
/// assert!(4294967294u32 % 4294967294u32 == 0u32);
/// assert!(4294967294u32 % 4294967295u32 == 4294967294u32);
/// assert!(4294967295u32 % 1u32 == 0u32);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u32 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u32_rem() {}
/// # Properties for [`u32::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u32::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_rem(1u32) == Some(0u32));
/// assert!(0u32.checked_rem(4294967294u32) == Some(0u32));
/// assert!(0u32.checked_rem(4294967295u32) == Some(0u32));
/// assert!(1u32.checked_rem(1u32) == Some(0u32));
/// assert!(1u32.checked_rem(4294967294u32) == Some(1u32));
/// assert!(1u32.checked_rem(4294967295u32) == Some(1u32));
/// assert!(4294967294u32.checked_rem(1u32) == Some(0u32));
/// assert!(4294967294u32.checked_rem(4294967294u32) == Some(0u32));
/// assert!(4294967294u32.checked_rem(4294967295u32) == Some(4294967294u32));
/// assert!(4294967295u32.checked_rem(1u32) == Some(0u32));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u32.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u32_checked_rem() {}
/// # Properties for [`u32::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `x == u32::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u32::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_neg() == Some(0u32));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `x != u32::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.checked_neg() == None);
/// assert!(4294967294u32.checked_neg() == None);
/// assert!(4294967295u32.checked_neg() == None);
/// assert!(1672822649u32.checked_neg() == None);
/// assert!(4254016533u32.checked_neg() == None);
/// assert!(1395561315u32.checked_neg() == None);
/// assert!(2900078602u32.checked_neg() == None);
/// assert!(3301030968u32.checked_neg() == None);
/// assert!(2773106103u32.checked_neg() == None);
/// assert!(1497900163u32.checked_neg() == None);
/// # }
/// ```
pub fn t_u32_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u32>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x << y == u32::down((x.up() << y) & u32::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 << 0u32 == 0u32);
/// assert!(0u32 << 1u32 == 0u32);
/// assert!(1u32 << 0u32 == 1u32);
/// assert!(1u32 << 1u32 == 2u32);
/// assert!(4294967294u32 << 0u32 == 4294967294u32);
/// assert!(4294967294u32 << 1u32 == 4294967292u32);
/// assert!(4294967295u32 << 0u32 == 4294967295u32);
/// assert!(4294967295u32 << 1u32 == 4294967294u32);
/// assert!(2254956623u32 << 7u32 == 871638912u32);
/// assert!(695317769u32 << 31u32 == 2147483648u32);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u32 << 4294967294u32));
/// assert!(panics!(0u32 << 4294967295u32));
/// assert!(panics!(1u32 << 4294967294u32));
/// assert!(panics!(1u32 << 4294967295u32));
/// assert!(panics!(4294967294u32 << 4294967294u32));
/// assert!(panics!(4294967294u32 << 4294967295u32));
/// assert!(panics!(4294967295u32 << 4294967294u32));
/// assert!(panics!(4294967295u32 << 4294967295u32));
/// assert!(panics!(475485670u32 << 74u32));
/// assert!(panics!(547946040u32 << 57u32));
/// # }
/// ```
pub fn core_ops_shl_u32_shl() {}
/// # Properties for [`core::ops::Shl::<u32>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u32::down((x.up() << y) & u32::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_shl(0u32) == Some(0u32));
/// assert!(0u32.checked_shl(1u32) == Some(0u32));
/// assert!(1u32.checked_shl(0u32) == Some(1u32));
/// assert!(1u32.checked_shl(1u32) == Some(2u32));
/// assert!(4294967294u32.checked_shl(0u32) == Some(4294967294u32));
/// assert!(4294967294u32.checked_shl(1u32) == Some(4294967292u32));
/// assert!(4294967295u32.checked_shl(0u32) == Some(4294967295u32));
/// assert!(4294967295u32.checked_shl(1u32) == Some(4294967294u32));
/// assert!(3799416260u32.checked_shl(17u32) == Some(4219994112u32));
/// assert!(702518212u32.checked_shl(10u32) == Some(2119110656u32));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_shl(4294967294u32) == None);
/// assert!(0u32.checked_shl(4294967295u32) == None);
/// assert!(1u32.checked_shl(4294967294u32) == None);
/// assert!(1u32.checked_shl(4294967295u32) == None);
/// assert!(4294967294u32.checked_shl(4294967294u32) == None);
/// assert!(4294967294u32.checked_shl(4294967295u32) == None);
/// assert!(4294967295u32.checked_shl(4294967294u32) == None);
/// assert!(4294967295u32.checked_shl(4294967295u32) == None);
/// assert!(1943825771u32.checked_shl(117u32) == None);
/// assert!(3716313881u32.checked_shl(69u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u32_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u32>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u32::down((x.up() << y) & u32::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.overflowing_shl(0u32) == (0u32, false));
/// assert!(0u32.overflowing_shl(1u32) == (0u32, false));
/// assert!(1u32.overflowing_shl(0u32) == (1u32, false));
/// assert!(1u32.overflowing_shl(1u32) == (2u32, false));
/// assert!(4294967294u32.overflowing_shl(0u32) == (4294967294u32, false));
/// assert!(4294967294u32.overflowing_shl(1u32) == (4294967292u32, false));
/// assert!(4294967295u32.overflowing_shl(0u32) == (4294967295u32, false));
/// assert!(4294967295u32.overflowing_shl(1u32) == (4294967294u32, false));
/// assert!(400316005u32.overflowing_shl(19u32) == (3005743104u32, false));
/// assert!(2866573500u32.overflowing_shl(2u32) == (2876359408u32, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u32::down((x.up() << (y & (u32::BITS - 1)) & u32::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.overflowing_shl(4294967294u32) == (0u32, true));
/// assert!(0u32.overflowing_shl(4294967295u32) == (0u32, true));
/// assert!(1u32.overflowing_shl(4294967294u32) == (1073741824u32, true));
/// assert!(1u32.overflowing_shl(4294967295u32) == (2147483648u32, true));
/// assert!(4294967294u32.overflowing_shl(4294967294u32) == (2147483648u32, true));
/// assert!(4294967294u32.overflowing_shl(4294967295u32) == (0u32, true));
/// assert!(4294967295u32.overflowing_shl(4294967294u32) == (3221225472u32, true));
/// assert!(4294967295u32.overflowing_shl(4294967295u32) == (2147483648u32, true));
/// assert!(338589925u32.overflowing_shl(84u32) == (2387607552u32, true));
/// assert!(4174045773u32.overflowing_shl(111u32) == (1898348544u32, true));
/// # }
/// ```
pub fn core_ops_shl_u32_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u32>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x >> y == u32::down((x.up() >> y) & u32::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 >> 0u32 == 0u32);
/// assert!(0u32 >> 1u32 == 0u32);
/// assert!(1u32 >> 0u32 == 1u32);
/// assert!(1u32 >> 1u32 == 0u32);
/// assert!(4294967294u32 >> 0u32 == 4294967294u32);
/// assert!(4294967294u32 >> 1u32 == 2147483647u32);
/// assert!(4294967295u32 >> 0u32 == 4294967295u32);
/// assert!(4294967295u32 >> 1u32 == 2147483647u32);
/// assert!(2302834954u32 >> 10u32 == 2248862u32);
/// assert!(3792606408u32 >> 14u32 == 231482u32);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u32 >> 4294967294u32));
/// assert!(panics!(0u32 >> 4294967295u32));
/// assert!(panics!(1u32 >> 4294967294u32));
/// assert!(panics!(1u32 >> 4294967295u32));
/// assert!(panics!(4294967294u32 >> 4294967294u32));
/// assert!(panics!(4294967294u32 >> 4294967295u32));
/// assert!(panics!(4294967295u32 >> 4294967294u32));
/// assert!(panics!(4294967295u32 >> 4294967295u32));
/// assert!(panics!(1528807375u32 >> 70u32));
/// assert!(panics!(1539239185u32 >> 133u32));
/// # }
/// ```
pub fn core_ops_shr_u32_shr() {}
/// # Properties for [`core::ops::Shr::<u32>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u32::down((x.up() >> y) & u32::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_shr(0u32) == Some(0u32));
/// assert!(0u32.checked_shr(1u32) == Some(0u32));
/// assert!(1u32.checked_shr(0u32) == Some(1u32));
/// assert!(1u32.checked_shr(1u32) == Some(0u32));
/// assert!(4294967294u32.checked_shr(0u32) == Some(4294967294u32));
/// assert!(4294967294u32.checked_shr(1u32) == Some(2147483647u32));
/// assert!(4294967295u32.checked_shr(0u32) == Some(4294967295u32));
/// assert!(4294967295u32.checked_shr(1u32) == Some(2147483647u32));
/// assert!(3400033017u32.checked_shr(10u32) == Some(3320344u32));
/// assert!(2564460521u32.checked_shr(0u32) == Some(2564460521u32));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_shr(4294967294u32) == None);
/// assert!(0u32.checked_shr(4294967295u32) == None);
/// assert!(1u32.checked_shr(4294967294u32) == None);
/// assert!(1u32.checked_shr(4294967295u32) == None);
/// assert!(4294967294u32.checked_shr(4294967294u32) == None);
/// assert!(4294967294u32.checked_shr(4294967295u32) == None);
/// assert!(4294967295u32.checked_shr(4294967294u32) == None);
/// assert!(4294967295u32.checked_shr(4294967295u32) == None);
/// assert!(4045310377u32.checked_shr(51u32) == None);
/// assert!(2428186409u32.checked_shr(119u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u32_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u32>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y < u32::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u32::down((x.up() >> y) & u32::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.overflowing_shr(0u32) == (0u32, false));
/// assert!(0u32.overflowing_shr(1u32) == (0u32, false));
/// assert!(1u32.overflowing_shr(0u32) == (1u32, false));
/// assert!(1u32.overflowing_shr(1u32) == (0u32, false));
/// assert!(4294967294u32.overflowing_shr(0u32) == (4294967294u32, false));
/// assert!(4294967294u32.overflowing_shr(1u32) == (2147483647u32, false));
/// assert!(4294967295u32.overflowing_shr(0u32) == (4294967295u32, false));
/// assert!(4294967295u32.overflowing_shr(1u32) == (2147483647u32, false));
/// assert!(2623803240u32.overflowing_shr(24u32) == (156u32, false));
/// assert!(2172058569u32.overflowing_shr(21u32) == (1035u32, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y >= u32::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u32::down((x.up() >> (y & (u32::BITS - 1)) & u32::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.overflowing_shr(4294967294u32) == (0u32, true));
/// assert!(0u32.overflowing_shr(4294967295u32) == (0u32, true));
/// assert!(1u32.overflowing_shr(4294967294u32) == (0u32, true));
/// assert!(1u32.overflowing_shr(4294967295u32) == (0u32, true));
/// assert!(4294967294u32.overflowing_shr(4294967294u32) == (3u32, true));
/// assert!(4294967294u32.overflowing_shr(4294967295u32) == (1u32, true));
/// assert!(4294967295u32.overflowing_shr(4294967294u32) == (3u32, true));
/// assert!(4294967295u32.overflowing_shr(4294967295u32) == (1u32, true));
/// assert!(3841178764u32.overflowing_shr(55u32) == (457u32, true));
/// assert!(922394575u32.overflowing_shr(66u32) == (230598643u32, true));
/// # }
/// ```
pub fn core_ops_shr_u32_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u32>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u32::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 / 1u32 == 0u32);
/// assert!(0u32 / 4294967294u32 == 0u32);
/// assert!(0u32 / 4294967295u32 == 0u32);
/// assert!(1u32 / 1u32 == 1u32);
/// assert!(1u32 / 4294967294u32 == 0u32);
/// assert!(1u32 / 4294967295u32 == 0u32);
/// assert!(4294967294u32 / 1u32 == 4294967294u32);
/// assert!(4294967294u32 / 4294967294u32 == 1u32);
/// assert!(4294967294u32 / 4294967295u32 == 0u32);
/// assert!(4294967295u32 / 1u32 == 4294967295u32);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4294967294u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4294967295u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(2330809583u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(508462633u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(602814284u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1946474152u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1311212636u32 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1359574792u32 / 0) } });
/// # }
/// ```
pub fn core_ops_div_u32_div() {}
/// # Properties for [`u32::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u32::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_div(1u32) == 0u32);
/// assert!(0u32.saturating_div(4294967294u32) == 0u32);
/// assert!(0u32.saturating_div(4294967295u32) == 0u32);
/// assert!(1u32.saturating_div(1u32) == 1u32);
/// assert!(1u32.saturating_div(4294967294u32) == 0u32);
/// assert!(1u32.saturating_div(4294967295u32) == 0u32);
/// assert!(4294967294u32.saturating_div(1u32) == 4294967294u32);
/// assert!(4294967294u32.saturating_div(4294967294u32) == 1u32);
/// assert!(4294967294u32.saturating_div(4294967295u32) == 0u32);
/// assert!(4294967295u32.saturating_div(1u32) == 4294967295u32);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4294967294u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(4294967295u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(348789308u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(657258334u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(254872448u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(442158443u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(2512110774u32.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(79635956u32.saturating_div(0)) } });
/// # }
/// ```
pub fn core_ops_div_u32_saturating_div() {}
/// # Properties for [`u32::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u32::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_div(1u32) == Some(0u32));
/// assert!(0u32.checked_div(4294967294u32) == Some(0u32));
/// assert!(0u32.checked_div(4294967295u32) == Some(0u32));
/// assert!(1u32.checked_div(1u32) == Some(1u32));
/// assert!(1u32.checked_div(4294967294u32) == Some(0u32));
/// assert!(1u32.checked_div(4294967295u32) == Some(0u32));
/// assert!(4294967294u32.checked_div(1u32) == Some(4294967294u32));
/// assert!(4294967294u32.checked_div(4294967294u32) == Some(1u32));
/// assert!(4294967294u32.checked_div(4294967295u32) == Some(0u32));
/// assert!(4294967295u32.checked_div(1u32) == Some(4294967295u32));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_div(0) == None);
/// assert!(1u32.checked_div(0) == None);
/// assert!(4294967294u32.checked_div(0) == None);
/// assert!(4294967295u32.checked_div(0) == None);
/// assert!(3307477888u32.checked_div(0) == None);
/// assert!(3017088286u32.checked_div(0) == None);
/// assert!(35174105u32.checked_div(0) == None);
/// assert!(3768926363u32.checked_div(0) == None);
/// assert!(2220859038u32.checked_div(0) == None);
/// assert!(3778555895u32.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u32_checked_div() {}
/// # Properties for [`core::ops::Mul::<u32>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x * y == u32::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 * 0u32 == 0u32);
/// assert!(0u32 * 1u32 == 0u32);
/// assert!(0u32 * 4294967294u32 == 0u32);
/// assert!(0u32 * 4294967295u32 == 0u32);
/// assert!(1u32 * 0u32 == 0u32);
/// assert!(1u32 * 1u32 == 1u32);
/// assert!(1u32 * 4294967294u32 == 4294967294u32);
/// assert!(1u32 * 4294967295u32 == 4294967295u32);
/// assert!(4294967294u32 * 0u32 == 0u32);
/// assert!(4294967294u32 * 1u32 == 4294967294u32);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() > u32::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(4294967294u32 * 4294967294u32));
/// assert!(panics!(4294967294u32 * 4294967295u32));
/// assert!(panics!(4294967295u32 * 4294967294u32));
/// assert!(panics!(4294967295u32 * 4294967295u32));
/// assert!(panics!(806269403u32 * 603636858u32));
/// assert!(panics!(391574662u32 * 615669695u32));
/// assert!(panics!(114315988u32 * 2148033275u32));
/// assert!(panics!(3446595501u32 * 2632332736u32));
/// assert!(panics!(919242029u32 * 4197113456u32));
/// assert!(panics!(1505590832u32 * 4168150147u32));
/// # }
/// ```
pub fn core_ops_mul_u32_mul() {}
/// # Properties for [`u32::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u32::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_mul(0u32) == Some(0u32));
/// assert!(0u32.checked_mul(1u32) == Some(0u32));
/// assert!(0u32.checked_mul(4294967294u32) == Some(0u32));
/// assert!(0u32.checked_mul(4294967295u32) == Some(0u32));
/// assert!(1u32.checked_mul(0u32) == Some(0u32));
/// assert!(1u32.checked_mul(1u32) == Some(1u32));
/// assert!(1u32.checked_mul(4294967294u32) == Some(4294967294u32));
/// assert!(1u32.checked_mul(4294967295u32) == Some(4294967295u32));
/// assert!(4294967294u32.checked_mul(0u32) == Some(0u32));
/// assert!(4294967294u32.checked_mul(1u32) == Some(4294967294u32));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.checked_mul(4294967294u32) == None);
/// assert!(4294967294u32.checked_mul(4294967295u32) == None);
/// assert!(4294967295u32.checked_mul(4294967294u32) == None);
/// assert!(4294967295u32.checked_mul(4294967295u32) == None);
/// assert!(3284022496u32.checked_mul(250213727u32) == None);
/// assert!(2021931237u32.checked_mul(3903434952u32) == None);
/// assert!(1885950087u32.checked_mul(64109548u32) == None);
/// assert!(1065806213u32.checked_mul(2879542038u32) == None);
/// assert!(2044727585u32.checked_mul(3593930758u32) == None);
/// assert!(3396638382u32.checked_mul(724288920u32) == None);
/// # }
/// ```
pub fn core_ops_mul_u32_checked_mul() {}
/// # Properties for [`u32::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u32::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.overflowing_mul(0u32) == (0u32, false));
/// assert!(0u32.overflowing_mul(1u32) == (0u32, false));
/// assert!(0u32.overflowing_mul(4294967294u32) == (0u32, false));
/// assert!(0u32.overflowing_mul(4294967295u32) == (0u32, false));
/// assert!(1u32.overflowing_mul(0u32) == (0u32, false));
/// assert!(1u32.overflowing_mul(1u32) == (1u32, false));
/// assert!(1u32.overflowing_mul(4294967294u32) == (4294967294u32, false));
/// assert!(1u32.overflowing_mul(4294967295u32) == (4294967295u32, false));
/// assert!(4294967294u32.overflowing_mul(0u32) == (0u32, false));
/// assert!(4294967294u32.overflowing_mul(1u32) == (4294967294u32, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.overflowing_mul(4294967294u32) == (4u32, true));
/// assert!(4294967294u32.overflowing_mul(4294967295u32) == (2u32, true));
/// assert!(4294967295u32.overflowing_mul(4294967294u32) == (2u32, true));
/// assert!(4294967295u32.overflowing_mul(4294967295u32) == (1u32, true));
/// assert!(329578213u32.overflowing_mul(965566865u32) == (2602409141u32, true));
/// assert!(818643731u32.overflowing_mul(2104278983u32) == (1476530373u32, true));
/// assert!(323122389u32.overflowing_mul(2836715635u32) == (1997681583u32, true));
/// assert!(490630758u32.overflowing_mul(511133671u32) == (4183681034u32, true));
/// assert!(4189100289u32.overflowing_mul(2714684693u32) == (2781895189u32, true));
/// assert!(1459596041u32.overflowing_mul(2207448176u32) == (3797528560u32, true));
/// # }
/// ```
pub fn core_ops_mul_u32_overflowing_mul() {}
/// # Properties for [`u32::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u32::down((x.up() * y.up()).min(u32::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_mul(0u32) == 0u32);
/// assert!(0u32.saturating_mul(1u32) == 0u32);
/// assert!(0u32.saturating_mul(4294967294u32) == 0u32);
/// assert!(0u32.saturating_mul(4294967295u32) == 0u32);
/// assert!(1u32.saturating_mul(0u32) == 0u32);
/// assert!(1u32.saturating_mul(1u32) == 1u32);
/// assert!(1u32.saturating_mul(4294967294u32) == 4294967294u32);
/// assert!(1u32.saturating_mul(4294967295u32) == 4294967295u32);
/// assert!(4294967294u32.saturating_mul(0u32) == 0u32);
/// assert!(4294967294u32.saturating_mul(1u32) == 4294967294u32);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u32::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_mul(0u32) == 0u32);
/// assert!(0u32.saturating_mul(1u32) == 0u32);
/// assert!(0u32.saturating_mul(4294967294u32) == 0u32);
/// assert!(0u32.saturating_mul(4294967295u32) == 0u32);
/// assert!(1u32.saturating_mul(0u32) == 0u32);
/// assert!(1u32.saturating_mul(1u32) == 1u32);
/// assert!(1u32.saturating_mul(4294967294u32) == 4294967294u32);
/// assert!(1u32.saturating_mul(4294967295u32) == 4294967295u32);
/// assert!(4294967294u32.saturating_mul(0u32) == 0u32);
/// assert!(4294967294u32.saturating_mul(1u32) == 4294967294u32);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u32::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(4294967294u32.saturating_mul(4294967294u32) == u32::MAX);
/// assert!(4294967294u32.saturating_mul(4294967295u32) == u32::MAX);
/// assert!(4294967295u32.saturating_mul(4294967294u32) == u32::MAX);
/// assert!(4294967295u32.saturating_mul(4294967295u32) == u32::MAX);
/// assert!(3503561740u32.saturating_mul(500634042u32) == u32::MAX);
/// assert!(1975444363u32.saturating_mul(3771941046u32) == u32::MAX);
/// assert!(2262346119u32.saturating_mul(3458183511u32) == u32::MAX);
/// assert!(1520070629u32.saturating_mul(3479149259u32) == u32::MAX);
/// assert!(484890479u32.saturating_mul(3778335293u32) == u32::MAX);
/// assert!(251902533u32.saturating_mul(3913566720u32) == u32::MAX);
/// # }
/// ```
pub fn core_ops_mul_u32_saturating_mul() {}
/// # Properties for [`u32::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u32::down((x.up() * y.up()) % (u32::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_mul(0u32) == 0u32);
/// assert!(0u32.wrapping_mul(1u32) == 0u32);
/// assert!(0u32.wrapping_mul(4294967294u32) == 0u32);
/// assert!(0u32.wrapping_mul(4294967295u32) == 0u32);
/// assert!(1u32.wrapping_mul(0u32) == 0u32);
/// assert!(1u32.wrapping_mul(1u32) == 1u32);
/// assert!(1u32.wrapping_mul(4294967294u32) == 4294967294u32);
/// assert!(1u32.wrapping_mul(4294967295u32) == 4294967295u32);
/// assert!(4294967294u32.wrapping_mul(0u32) == 0u32);
/// assert!(4294967294u32.wrapping_mul(1u32) == 4294967294u32);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() * y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u32::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_mul(0u32) == 0u32);
/// assert!(0u32.wrapping_mul(1u32) == 0u32);
/// assert!(0u32.wrapping_mul(4294967294u32) == 0u32);
/// assert!(0u32.wrapping_mul(4294967295u32) == 0u32);
/// assert!(1u32.wrapping_mul(0u32) == 0u32);
/// assert!(1u32.wrapping_mul(1u32) == 1u32);
/// assert!(1u32.wrapping_mul(4294967294u32) == 4294967294u32);
/// assert!(1u32.wrapping_mul(4294967295u32) == 4294967295u32);
/// assert!(4294967294u32.wrapping_mul(0u32) == 0u32);
/// assert!(4294967294u32.wrapping_mul(1u32) == 4294967294u32);
/// # }
/// ```
pub fn core_ops_mul_u32_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u32>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 - 0u32 == 0u32);
/// assert!(1u32 - 0u32 == 1u32);
/// assert!(1u32 - 1u32 == 0u32);
/// assert!(4294967294u32 - 0u32 == 4294967294u32);
/// assert!(4294967294u32 - 1u32 == 4294967293u32);
/// assert!(4294967294u32 - 4294967294u32 == 0u32);
/// assert!(4294967295u32 - 0u32 == 4294967295u32);
/// assert!(4294967295u32 - 1u32 == 4294967294u32);
/// assert!(4294967295u32 - 4294967294u32 == 1u32);
/// assert!(4294967295u32 - 4294967295u32 == 0u32);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u32 - 1u32));
/// assert!(panics!(0u32 - 4294967294u32));
/// assert!(panics!(0u32 - 4294967295u32));
/// assert!(panics!(1u32 - 4294967294u32));
/// assert!(panics!(1u32 - 4294967295u32));
/// assert!(panics!(4294967294u32 - 4294967295u32));
/// assert!(panics!(1004229070u32 - 4058523626u32));
/// assert!(panics!(868945825u32 - 2974289101u32));
/// assert!(panics!(1907629492u32 - 3108193105u32));
/// assert!(panics!(2238019117u32 - 3668401188u32));
/// # }
/// ```
pub fn core_ops_add_u32_sub() {}
/// # Properties for [`u32::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_sub(0u32) == 0u32);
/// assert!(1u32.wrapping_sub(0u32) == 1u32);
/// assert!(1u32.wrapping_sub(1u32) == 0u32);
/// assert!(4294967294u32.wrapping_sub(0u32) == 4294967294u32);
/// assert!(4294967294u32.wrapping_sub(1u32) == 4294967293u32);
/// assert!(4294967294u32.wrapping_sub(4294967294u32) == 0u32);
/// assert!(4294967295u32.wrapping_sub(0u32) == 4294967295u32);
/// assert!(4294967295u32.wrapping_sub(1u32) == 4294967294u32);
/// assert!(4294967295u32.wrapping_sub(4294967294u32) == 1u32);
/// assert!(4294967295u32.wrapping_sub(4294967295u32) == 0u32);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u32::down(x.up() - y.up() + u32::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_sub(1u32) == 4294967295u32);
/// assert!(0u32.wrapping_sub(4294967294u32) == 2u32);
/// assert!(0u32.wrapping_sub(4294967295u32) == 1u32);
/// assert!(1u32.wrapping_sub(4294967294u32) == 3u32);
/// assert!(1u32.wrapping_sub(4294967295u32) == 2u32);
/// assert!(4294967294u32.wrapping_sub(4294967295u32) == 4294967295u32);
/// assert!(8151796u32.wrapping_sub(4138508904u32) == 164610188u32);
/// assert!(182942205u32.wrapping_sub(2437678781u32) == 2040230720u32);
/// assert!(2108047790u32.wrapping_sub(3818072652u32) == 2584942434u32);
/// assert!(373347858u32.wrapping_sub(700522038u32) == 3967793116u32);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u32::down((x.up() - y.up()).rem_euclid(&(u32::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_sub(0u32) == 0u32);
/// assert!(0u32.wrapping_sub(1u32) == 4294967295u32);
/// assert!(0u32.wrapping_sub(4294967294u32) == 2u32);
/// assert!(0u32.wrapping_sub(4294967295u32) == 1u32);
/// assert!(1u32.wrapping_sub(0u32) == 1u32);
/// assert!(1u32.wrapping_sub(1u32) == 0u32);
/// assert!(1u32.wrapping_sub(4294967294u32) == 3u32);
/// assert!(1u32.wrapping_sub(4294967295u32) == 2u32);
/// assert!(4294967294u32.wrapping_sub(0u32) == 4294967294u32);
/// assert!(4294967294u32.wrapping_sub(1u32) == 4294967293u32);
/// # }
/// ```
pub fn core_ops_add_u32_wrapping_sub() {}
/// # Properties for [`u32::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u32::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_sub(0u32) == Some(0u32));
/// assert!(1u32.checked_sub(0u32) == Some(1u32));
/// assert!(1u32.checked_sub(1u32) == Some(0u32));
/// assert!(4294967294u32.checked_sub(0u32) == Some(4294967294u32));
/// assert!(4294967294u32.checked_sub(1u32) == Some(4294967293u32));
/// assert!(4294967294u32.checked_sub(4294967294u32) == Some(0u32));
/// assert!(4294967295u32.checked_sub(0u32) == Some(4294967295u32));
/// assert!(4294967295u32.checked_sub(1u32) == Some(4294967294u32));
/// assert!(4294967295u32.checked_sub(4294967294u32) == Some(1u32));
/// assert!(4294967295u32.checked_sub(4294967295u32) == Some(0u32));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_sub(1u32) == None);
/// assert!(0u32.checked_sub(4294967294u32) == None);
/// assert!(0u32.checked_sub(4294967295u32) == None);
/// assert!(1u32.checked_sub(4294967294u32) == None);
/// assert!(1u32.checked_sub(4294967295u32) == None);
/// assert!(4294967294u32.checked_sub(4294967295u32) == None);
/// assert!(2920400358u32.checked_sub(4120879736u32) == None);
/// assert!(1198853015u32.checked_sub(3114680784u32) == None);
/// assert!(678134293u32.checked_sub(1388874476u32) == None);
/// assert!(549730899u32.checked_sub(2657962374u32) == None);
/// # }
/// ```
pub fn core_ops_add_u32_checked_sub() {}
/// # Properties for [`u32::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u32::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_sub(0u32) == 0u32);
/// assert!(0u32.saturating_sub(1u32) == 0u32);
/// assert!(0u32.saturating_sub(4294967294u32) == 0u32);
/// assert!(0u32.saturating_sub(4294967295u32) == 0u32);
/// assert!(1u32.saturating_sub(0u32) == 1u32);
/// assert!(1u32.saturating_sub(1u32) == 0u32);
/// assert!(1u32.saturating_sub(4294967294u32) == 0u32);
/// assert!(1u32.saturating_sub(4294967295u32) == 0u32);
/// assert!(4294967294u32.saturating_sub(0u32) == 4294967294u32);
/// assert!(4294967294u32.saturating_sub(1u32) == 4294967293u32);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_sub(0u32) == 0u32);
/// assert!(1u32.saturating_sub(0u32) == 1u32);
/// assert!(1u32.saturating_sub(1u32) == 0u32);
/// assert!(4294967294u32.saturating_sub(0u32) == 4294967294u32);
/// assert!(4294967294u32.saturating_sub(1u32) == 4294967293u32);
/// assert!(4294967294u32.saturating_sub(4294967294u32) == 0u32);
/// assert!(4294967295u32.saturating_sub(0u32) == 4294967295u32);
/// assert!(4294967295u32.saturating_sub(1u32) == 4294967294u32);
/// assert!(4294967295u32.saturating_sub(4294967294u32) == 1u32);
/// assert!(4294967295u32.saturating_sub(4294967295u32) == 0u32);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_sub(1u32) == 0);
/// assert!(0u32.saturating_sub(4294967294u32) == 0);
/// assert!(0u32.saturating_sub(4294967295u32) == 0);
/// assert!(1u32.saturating_sub(4294967294u32) == 0);
/// assert!(1u32.saturating_sub(4294967295u32) == 0);
/// assert!(4294967294u32.saturating_sub(4294967295u32) == 0);
/// assert!(2166361461u32.saturating_sub(3011383717u32) == 0);
/// assert!(1388311675u32.saturating_sub(3587735247u32) == 0);
/// assert!(257428035u32.saturating_sub(259068825u32) == 0);
/// assert!(552305293u32.saturating_sub(2810225552u32) == 0);
/// # }
/// ```
pub fn core_ops_sub_u32_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u32>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x + y == u32::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 + 0u32 == 0u32);
/// assert!(0u32 + 1u32 == 1u32);
/// assert!(0u32 + 4294967294u32 == 4294967294u32);
/// assert!(0u32 + 4294967295u32 == 4294967295u32);
/// assert!(1u32 + 0u32 == 1u32);
/// assert!(1u32 + 1u32 == 2u32);
/// assert!(1u32 + 4294967294u32 == 4294967295u32);
/// assert!(4294967294u32 + 0u32 == 4294967294u32);
/// assert!(4294967294u32 + 1u32 == 4294967295u32);
/// assert!(4294967295u32 + 0u32 == 4294967295u32);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1u32 + 4294967295u32));
/// assert!(panics!(4294967294u32 + 4294967294u32));
/// assert!(panics!(4294967294u32 + 4294967295u32));
/// assert!(panics!(4294967295u32 + 1u32));
/// assert!(panics!(4294967295u32 + 4294967294u32));
/// assert!(panics!(4294967295u32 + 4294967295u32));
/// assert!(panics!(2850270144u32 + 2551307174u32));
/// assert!(panics!(2635371929u32 + 3019471218u32));
/// assert!(panics!(3978654288u32 + 1574092400u32));
/// assert!(panics!(3615851548u32 + 1746100017u32));
/// # }
/// ```
pub fn core_ops_add_u32_add() {}
/// # Properties for [`u32::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u32::down((x.up() + y.up()) % (u32::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_add(0u32) == 0u32);
/// assert!(0u32.wrapping_add(1u32) == 1u32);
/// assert!(0u32.wrapping_add(4294967294u32) == 4294967294u32);
/// assert!(0u32.wrapping_add(4294967295u32) == 4294967295u32);
/// assert!(1u32.wrapping_add(0u32) == 1u32);
/// assert!(1u32.wrapping_add(1u32) == 2u32);
/// assert!(1u32.wrapping_add(4294967294u32) == 4294967295u32);
/// assert!(1u32.wrapping_add(4294967295u32) == 0u32);
/// assert!(4294967294u32.wrapping_add(0u32) == 4294967294u32);
/// assert!(4294967294u32.wrapping_add(1u32) == 4294967295u32);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u32::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.wrapping_add(0u32) == 0u32);
/// assert!(0u32.wrapping_add(1u32) == 1u32);
/// assert!(0u32.wrapping_add(4294967294u32) == 4294967294u32);
/// assert!(0u32.wrapping_add(4294967295u32) == 4294967295u32);
/// assert!(1u32.wrapping_add(0u32) == 1u32);
/// assert!(1u32.wrapping_add(1u32) == 2u32);
/// assert!(1u32.wrapping_add(4294967294u32) == 4294967295u32);
/// assert!(4294967294u32.wrapping_add(0u32) == 4294967294u32);
/// assert!(4294967294u32.wrapping_add(1u32) == 4294967295u32);
/// assert!(4294967295u32.wrapping_add(0u32) == 4294967295u32);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u32::down(x.up() + y.up() - u32::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.wrapping_add(4294967295u32) == 0u32);
/// assert!(4294967294u32.wrapping_add(4294967294u32) == 4294967292u32);
/// assert!(4294967294u32.wrapping_add(4294967295u32) == 4294967293u32);
/// assert!(4294967295u32.wrapping_add(1u32) == 0u32);
/// assert!(4294967295u32.wrapping_add(4294967294u32) == 4294967293u32);
/// assert!(4294967295u32.wrapping_add(4294967295u32) == 4294967294u32);
/// assert!(1019330352u32.wrapping_add(3309770107u32) == 34133163u32);
/// assert!(4119428541u32.wrapping_add(2077020046u32) == 1901481291u32);
/// assert!(3522153225u32.wrapping_add(1980892613u32) == 1208078542u32);
/// assert!(3899566944u32.wrapping_add(2848058673u32) == 2452658321u32);
/// # }
/// ```
pub fn core_ops_add_u32_wrapping_add() {}
/// # Properties for [`u32::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u32::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.checked_add(0u32) == Some(0u32));
/// assert!(0u32.checked_add(1u32) == Some(1u32));
/// assert!(0u32.checked_add(4294967294u32) == Some(4294967294u32));
/// assert!(0u32.checked_add(4294967295u32) == Some(4294967295u32));
/// assert!(1u32.checked_add(0u32) == Some(1u32));
/// assert!(1u32.checked_add(1u32) == Some(2u32));
/// assert!(1u32.checked_add(4294967294u32) == Some(4294967295u32));
/// assert!(4294967294u32.checked_add(0u32) == Some(4294967294u32));
/// assert!(4294967294u32.checked_add(1u32) == Some(4294967295u32));
/// assert!(4294967295u32.checked_add(0u32) == Some(4294967295u32));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.checked_add(4294967295u32) == None);
/// assert!(4294967294u32.checked_add(4294967294u32) == None);
/// assert!(4294967294u32.checked_add(4294967295u32) == None);
/// assert!(4294967295u32.checked_add(1u32) == None);
/// assert!(4294967295u32.checked_add(4294967294u32) == None);
/// assert!(4294967295u32.checked_add(4294967295u32) == None);
/// assert!(4208341041u32.checked_add(447213998u32) == None);
/// assert!(3739734136u32.checked_add(1417078504u32) == None);
/// assert!(3044300746u32.checked_add(1608222497u32) == None);
/// assert!(2280287362u32.checked_add(2469565899u32) == None);
/// # }
/// ```
pub fn core_ops_add_u32_checked_add() {}
/// # Properties for [`u32::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u32::down((x.up() + y.up()).min(u32::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_add(0u32) == 0u32);
/// assert!(0u32.saturating_add(1u32) == 1u32);
/// assert!(0u32.saturating_add(4294967294u32) == 4294967294u32);
/// assert!(0u32.saturating_add(4294967295u32) == 4294967295u32);
/// assert!(1u32.saturating_add(0u32) == 1u32);
/// assert!(1u32.saturating_add(1u32) == 2u32);
/// assert!(1u32.saturating_add(4294967294u32) == 4294967295u32);
/// assert!(1u32.saturating_add(4294967295u32) == 4294967295u32);
/// assert!(4294967294u32.saturating_add(0u32) == 4294967294u32);
/// assert!(4294967294u32.saturating_add(1u32) == 4294967295u32);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() <= u32::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u32::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.saturating_add(0u32) == 0u32);
/// assert!(0u32.saturating_add(1u32) == 1u32);
/// assert!(0u32.saturating_add(4294967294u32) == 4294967294u32);
/// assert!(0u32.saturating_add(4294967295u32) == 4294967295u32);
/// assert!(1u32.saturating_add(0u32) == 1u32);
/// assert!(1u32.saturating_add(1u32) == 2u32);
/// assert!(1u32.saturating_add(4294967294u32) == 4294967295u32);
/// assert!(4294967294u32.saturating_add(0u32) == 4294967294u32);
/// assert!(4294967294u32.saturating_add(1u32) == 4294967295u32);
/// assert!(4294967295u32.saturating_add(0u32) == 4294967295u32);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `x.up() + y.up() > u32::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u32::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u32.saturating_add(4294967295u32) == u32::MAX);
/// assert!(4294967294u32.saturating_add(4294967294u32) == u32::MAX);
/// assert!(4294967294u32.saturating_add(4294967295u32) == u32::MAX);
/// assert!(4294967295u32.saturating_add(1u32) == u32::MAX);
/// assert!(4294967295u32.saturating_add(4294967294u32) == u32::MAX);
/// assert!(4294967295u32.saturating_add(4294967295u32) == u32::MAX);
/// assert!(4193662466u32.saturating_add(1908165972u32) == u32::MAX);
/// assert!(3336069656u32.saturating_add(1038360159u32) == u32::MAX);
/// assert!(2425527983u32.saturating_add(1923355685u32) == u32::MAX);
/// assert!(1168955548u32.saturating_add(3593515112u32) == u32::MAX);
/// # }
/// ```
pub fn core_ops_add_u32_saturating_add() {}
/// # Properties for [`core::ops::Rem::<u64>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u64::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 % 1u64 == 0u64);
/// assert!(0u64 % 18446744073709551614u64 == 0u64);
/// assert!(0u64 % 18446744073709551615u64 == 0u64);
/// assert!(1u64 % 1u64 == 0u64);
/// assert!(1u64 % 18446744073709551614u64 == 1u64);
/// assert!(1u64 % 18446744073709551615u64 == 1u64);
/// assert!(18446744073709551614u64 % 1u64 == 0u64);
/// assert!(18446744073709551614u64 % 18446744073709551614u64 == 0u64);
/// assert!(18446744073709551614u64 % 18446744073709551615u64 == 18446744073709551614u64);
/// assert!(18446744073709551615u64 % 1u64 == 0u64);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u64 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u64_rem() {}
/// # Properties for [`u64::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u64::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_rem(1u64) == Some(0u64));
/// assert!(0u64.checked_rem(18446744073709551614u64) == Some(0u64));
/// assert!(0u64.checked_rem(18446744073709551615u64) == Some(0u64));
/// assert!(1u64.checked_rem(1u64) == Some(0u64));
/// assert!(1u64.checked_rem(18446744073709551614u64) == Some(1u64));
/// assert!(1u64.checked_rem(18446744073709551615u64) == Some(1u64));
/// assert!(18446744073709551614u64.checked_rem(1u64) == Some(0u64));
/// assert!(18446744073709551614u64.checked_rem(18446744073709551614u64) == Some(0u64));
/// assert!(18446744073709551614u64.checked_rem(18446744073709551615u64)
///         == Some(18446744073709551614u64));
/// assert!(18446744073709551615u64.checked_rem(1u64) == Some(0u64));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u64.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u64_checked_rem() {}
/// # Properties for [`u64::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `x == u64::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u64::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_neg() == Some(0u64));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `x != u64::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.checked_neg() == None);
/// assert!(18446744073709551614u64.checked_neg() == None);
/// assert!(18446744073709551615u64.checked_neg() == None);
/// assert!(7596379512058075457u64.checked_neg() == None);
/// assert!(12881611614535417689u64.checked_neg() == None);
/// assert!(11805617311570121706u64.checked_neg() == None);
/// assert!(12091778351387499385u64.checked_neg() == None);
/// assert!(15000310991105026595u64.checked_neg() == None);
/// assert!(10082613225993088229u64.checked_neg() == None);
/// assert!(16100426270340460346u64.checked_neg() == None);
/// # }
/// ```
pub fn t_u64_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u64>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x << y == u64::down((x.up() << y) & u64::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 << 0u32 == 0u64);
/// assert!(0u64 << 1u32 == 0u64);
/// assert!(1u64 << 0u32 == 1u64);
/// assert!(1u64 << 1u32 == 2u64);
/// assert!(18446744073709551614u64 << 0u32 == 18446744073709551614u64);
/// assert!(18446744073709551614u64 << 1u32 == 18446744073709551612u64);
/// assert!(18446744073709551615u64 << 0u32 == 18446744073709551615u64);
/// assert!(18446744073709551615u64 << 1u32 == 18446744073709551614u64);
/// assert!(7304425672964718663u64 << 5u32 == 12380692650356377824u64);
/// assert!(2949820707068815308u64 << 47u32 == 15268891586646245376u64);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u64 << 4294967294u32));
/// assert!(panics!(0u64 << 4294967295u32));
/// assert!(panics!(1u64 << 4294967294u32));
/// assert!(panics!(1u64 << 4294967295u32));
/// assert!(panics!(18446744073709551614u64 << 4294967294u32));
/// assert!(panics!(18446744073709551614u64 << 4294967295u32));
/// assert!(panics!(18446744073709551615u64 << 4294967294u32));
/// assert!(panics!(18446744073709551615u64 << 4294967295u32));
/// assert!(panics!(4932004448021458560u64 << 125u32));
/// assert!(panics!(2704380696099286584u64 << 97u32));
/// # }
/// ```
pub fn core_ops_shl_u64_shl() {}
/// # Properties for [`core::ops::Shl::<u64>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u64::down((x.up() << y) & u64::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_shl(0u32) == Some(0u64));
/// assert!(0u64.checked_shl(1u32) == Some(0u64));
/// assert!(1u64.checked_shl(0u32) == Some(1u64));
/// assert!(1u64.checked_shl(1u32) == Some(2u64));
/// assert!(18446744073709551614u64.checked_shl(0u32) == Some(18446744073709551614u64));
/// assert!(18446744073709551614u64.checked_shl(1u32) == Some(18446744073709551612u64));
/// assert!(18446744073709551615u64.checked_shl(0u32) == Some(18446744073709551615u64));
/// assert!(18446744073709551615u64.checked_shl(1u32) == Some(18446744073709551614u64));
/// assert!(16281468443837457315u64.checked_shl(55u32) == Some(15096065950945902592u64));
/// assert!(940476404047205427u64.checked_shl(3u32) == Some(7523811232377643416u64));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_shl(4294967294u32) == None);
/// assert!(0u64.checked_shl(4294967295u32) == None);
/// assert!(1u64.checked_shl(4294967294u32) == None);
/// assert!(1u64.checked_shl(4294967295u32) == None);
/// assert!(18446744073709551614u64.checked_shl(4294967294u32) == None);
/// assert!(18446744073709551614u64.checked_shl(4294967295u32) == None);
/// assert!(18446744073709551615u64.checked_shl(4294967294u32) == None);
/// assert!(18446744073709551615u64.checked_shl(4294967295u32) == None);
/// assert!(10956011751875164004u64.checked_shl(71u32) == None);
/// assert!(10449482474537458070u64.checked_shl(65u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u64_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u64>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u64::down((x.up() << y) & u64::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.overflowing_shl(0u32) == (0u64, false));
/// assert!(0u64.overflowing_shl(1u32) == (0u64, false));
/// assert!(1u64.overflowing_shl(0u32) == (1u64, false));
/// assert!(1u64.overflowing_shl(1u32) == (2u64, false));
/// assert!(18446744073709551614u64.overflowing_shl(0u32) == (18446744073709551614u64, false));
/// assert!(18446744073709551614u64.overflowing_shl(1u32) == (18446744073709551612u64, false));
/// assert!(18446744073709551615u64.overflowing_shl(0u32) == (18446744073709551615u64, false));
/// assert!(18446744073709551615u64.overflowing_shl(1u32) == (18446744073709551614u64, false));
/// assert!(14566165920727943924u64.overflowing_shl(60u32) == (4611686018427387904u64, false));
/// assert!(17824388856559525433u64.overflowing_shl(3u32) == (13467902336509342152u64, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u64::down((x.up() << (y & (u64::BITS - 1)) & u64::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.overflowing_shl(4294967294u32) == (0u64, true));
/// assert!(0u64.overflowing_shl(4294967295u32) == (0u64, true));
/// assert!(1u64.overflowing_shl(4294967294u32) == (4611686018427387904u64, true));
/// assert!(1u64.overflowing_shl(4294967295u32) == (9223372036854775808u64, true));
/// assert!(18446744073709551614u64.overflowing_shl(4294967294u32)
///         == (9223372036854775808u64, true));
/// assert!(18446744073709551614u64.overflowing_shl(4294967295u32) == (0u64, true));
/// assert!(18446744073709551615u64.overflowing_shl(4294967294u32)
///         == (13835058055282163712u64, true));
/// assert!(18446744073709551615u64.overflowing_shl(4294967295u32)
///         == (9223372036854775808u64, true));
/// assert!(18320419598770639521u64.overflowing_shl(66u32) == (17941446173953903236u64, true));
/// assert!(645671683472031025u64.overflowing_shl(129u32) == (1291343366944062050u64, true));
/// # }
/// ```
pub fn core_ops_shl_u64_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u64>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x >> y == u64::down((x.up() >> y) & u64::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 >> 0u32 == 0u64);
/// assert!(0u64 >> 1u32 == 0u64);
/// assert!(1u64 >> 0u32 == 1u64);
/// assert!(1u64 >> 1u32 == 0u64);
/// assert!(18446744073709551614u64 >> 0u32 == 18446744073709551614u64);
/// assert!(18446744073709551614u64 >> 1u32 == 9223372036854775807u64);
/// assert!(18446744073709551615u64 >> 0u32 == 18446744073709551615u64);
/// assert!(18446744073709551615u64 >> 1u32 == 9223372036854775807u64);
/// assert!(12534181843375368542u64 >> 13u32 == 1530051494552657u64);
/// assert!(13278430874474347684u64 >> 28u32 == 49466009715u64);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u64 >> 4294967294u32));
/// assert!(panics!(0u64 >> 4294967295u32));
/// assert!(panics!(1u64 >> 4294967294u32));
/// assert!(panics!(1u64 >> 4294967295u32));
/// assert!(panics!(18446744073709551614u64 >> 4294967294u32));
/// assert!(panics!(18446744073709551614u64 >> 4294967295u32));
/// assert!(panics!(18446744073709551615u64 >> 4294967294u32));
/// assert!(panics!(18446744073709551615u64 >> 4294967295u32));
/// assert!(panics!(8419451774923738105u64 >> 133u32));
/// assert!(panics!(16281197548258678682u64 >> 88u32));
/// # }
/// ```
pub fn core_ops_shr_u64_shr() {}
/// # Properties for [`core::ops::Shr::<u64>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u64::down((x.up() >> y) & u64::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_shr(0u32) == Some(0u64));
/// assert!(0u64.checked_shr(1u32) == Some(0u64));
/// assert!(1u64.checked_shr(0u32) == Some(1u64));
/// assert!(1u64.checked_shr(1u32) == Some(0u64));
/// assert!(18446744073709551614u64.checked_shr(0u32) == Some(18446744073709551614u64));
/// assert!(18446744073709551614u64.checked_shr(1u32) == Some(9223372036854775807u64));
/// assert!(18446744073709551615u64.checked_shr(0u32) == Some(18446744073709551615u64));
/// assert!(18446744073709551615u64.checked_shr(1u32) == Some(9223372036854775807u64));
/// assert!(12451032371532804483u64.checked_shr(25u32) == Some(371069680796u64));
/// assert!(6046971845521284715u64.checked_shr(36u32) == Some(87995021u64));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_shr(4294967294u32) == None);
/// assert!(0u64.checked_shr(4294967295u32) == None);
/// assert!(1u64.checked_shr(4294967294u32) == None);
/// assert!(1u64.checked_shr(4294967295u32) == None);
/// assert!(18446744073709551614u64.checked_shr(4294967294u32) == None);
/// assert!(18446744073709551614u64.checked_shr(4294967295u32) == None);
/// assert!(18446744073709551615u64.checked_shr(4294967294u32) == None);
/// assert!(18446744073709551615u64.checked_shr(4294967295u32) == None);
/// assert!(10414996414696540706u64.checked_shr(85u32) == None);
/// assert!(4704229838398786843u64.checked_shr(115u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u64_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u64>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y < u64::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u64::down((x.up() >> y) & u64::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.overflowing_shr(0u32) == (0u64, false));
/// assert!(0u64.overflowing_shr(1u32) == (0u64, false));
/// assert!(1u64.overflowing_shr(0u32) == (1u64, false));
/// assert!(1u64.overflowing_shr(1u32) == (0u64, false));
/// assert!(18446744073709551614u64.overflowing_shr(0u32) == (18446744073709551614u64, false));
/// assert!(18446744073709551614u64.overflowing_shr(1u32) == (9223372036854775807u64, false));
/// assert!(18446744073709551615u64.overflowing_shr(0u32) == (18446744073709551615u64, false));
/// assert!(18446744073709551615u64.overflowing_shr(1u32) == (9223372036854775807u64, false));
/// assert!(7870801920857798419u64.overflowing_shr(54u32) == (436u64, false));
/// assert!(387578346505279991u64.overflowing_shr(30u32) == (360960463u64, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u64, y : u32`  
/// __Precondition:__ `y >= u64::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u64::down((x.up() >> (y & (u64::BITS - 1)) & u64::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.overflowing_shr(4294967294u32) == (0u64, true));
/// assert!(0u64.overflowing_shr(4294967295u32) == (0u64, true));
/// assert!(1u64.overflowing_shr(4294967294u32) == (0u64, true));
/// assert!(1u64.overflowing_shr(4294967295u32) == (0u64, true));
/// assert!(18446744073709551614u64.overflowing_shr(4294967294u32) == (3u64, true));
/// assert!(18446744073709551614u64.overflowing_shr(4294967295u32) == (1u64, true));
/// assert!(18446744073709551615u64.overflowing_shr(4294967294u32) == (3u64, true));
/// assert!(18446744073709551615u64.overflowing_shr(4294967295u32) == (1u64, true));
/// assert!(13198058060865433229u64.overflowing_shr(92u32) == (49166597652u64, true));
/// assert!(3348114193510803951u64.overflowing_shr(139u32) == (1634821383550197u64, true));
/// # }
/// ```
pub fn core_ops_shr_u64_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u64>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u64::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 / 1u64 == 0u64);
/// assert!(0u64 / 18446744073709551614u64 == 0u64);
/// assert!(0u64 / 18446744073709551615u64 == 0u64);
/// assert!(1u64 / 1u64 == 1u64);
/// assert!(1u64 / 18446744073709551614u64 == 0u64);
/// assert!(1u64 / 18446744073709551615u64 == 0u64);
/// assert!(18446744073709551614u64 / 1u64 == 18446744073709551614u64);
/// assert!(18446744073709551614u64 / 18446744073709551614u64 == 1u64);
/// assert!(18446744073709551614u64 / 18446744073709551615u64 == 0u64);
/// assert!(18446744073709551615u64 / 1u64 == 18446744073709551615u64);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18446744073709551614u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18446744073709551615u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(2393958748281874330u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(746810819810400436u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(15939434817728022736u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(3350277772759675243u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(16310632540691174614u64 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(7233347218202530137u64 / 0) } });
/// # }
/// ```
pub fn core_ops_div_u64_div() {}
/// # Properties for [`u64::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u64::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_div(1u64) == 0u64);
/// assert!(0u64.saturating_div(18446744073709551614u64) == 0u64);
/// assert!(0u64.saturating_div(18446744073709551615u64) == 0u64);
/// assert!(1u64.saturating_div(1u64) == 1u64);
/// assert!(1u64.saturating_div(18446744073709551614u64) == 0u64);
/// assert!(1u64.saturating_div(18446744073709551615u64) == 0u64);
/// assert!(18446744073709551614u64.saturating_div(1u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.saturating_div(18446744073709551614u64) == 1u64);
/// assert!(18446744073709551614u64.saturating_div(18446744073709551615u64) == 0u64);
/// assert!(18446744073709551615u64.saturating_div(1u64) == 18446744073709551615u64);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u64.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u64.saturating_div(0)) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(18446744073709551614u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(18446744073709551615u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(3950878071502712414u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(18017139965939639956u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(12460156549055878498u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(10327778219516246657u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(16270588813781260747u64.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(11934723279988346822u64.saturating_div(0)) }
///     });
/// # }
/// ```
pub fn core_ops_div_u64_saturating_div() {}
/// # Properties for [`u64::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u64::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_div(1u64) == Some(0u64));
/// assert!(0u64.checked_div(18446744073709551614u64) == Some(0u64));
/// assert!(0u64.checked_div(18446744073709551615u64) == Some(0u64));
/// assert!(1u64.checked_div(1u64) == Some(1u64));
/// assert!(1u64.checked_div(18446744073709551614u64) == Some(0u64));
/// assert!(1u64.checked_div(18446744073709551615u64) == Some(0u64));
/// assert!(18446744073709551614u64.checked_div(1u64) == Some(18446744073709551614u64));
/// assert!(18446744073709551614u64.checked_div(18446744073709551614u64) == Some(1u64));
/// assert!(18446744073709551614u64.checked_div(18446744073709551615u64) == Some(0u64));
/// assert!(18446744073709551615u64.checked_div(1u64) == Some(18446744073709551615u64));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_div(0) == None);
/// assert!(1u64.checked_div(0) == None);
/// assert!(18446744073709551614u64.checked_div(0) == None);
/// assert!(18446744073709551615u64.checked_div(0) == None);
/// assert!(15559151411352182142u64.checked_div(0) == None);
/// assert!(14244507728068681356u64.checked_div(0) == None);
/// assert!(6281259585859798577u64.checked_div(0) == None);
/// assert!(3824937262333489096u64.checked_div(0) == None);
/// assert!(15870854470801241940u64.checked_div(0) == None);
/// assert!(8543656521456711216u64.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u64_checked_div() {}
/// # Properties for [`core::ops::Mul::<u64>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x * y == u64::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 * 0u64 == 0u64);
/// assert!(0u64 * 1u64 == 0u64);
/// assert!(0u64 * 18446744073709551614u64 == 0u64);
/// assert!(0u64 * 18446744073709551615u64 == 0u64);
/// assert!(1u64 * 0u64 == 0u64);
/// assert!(1u64 * 1u64 == 1u64);
/// assert!(1u64 * 18446744073709551614u64 == 18446744073709551614u64);
/// assert!(1u64 * 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(18446744073709551614u64 * 0u64 == 0u64);
/// assert!(18446744073709551614u64 * 1u64 == 18446744073709551614u64);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() > u64::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551614u64 * 18446744073709551614u64));
/// assert!(panics!(18446744073709551614u64 * 18446744073709551615u64));
/// assert!(panics!(18446744073709551615u64 * 18446744073709551614u64));
/// assert!(panics!(18446744073709551615u64 * 18446744073709551615u64));
/// assert!(panics!(1703239041926352127u64 * 7129991621635276219u64));
/// assert!(panics!(4168996873330739366u64 * 11295234750997514136u64));
/// assert!(panics!(8784123806474818923u64 * 5764907836902487194u64));
/// assert!(panics!(10559355064452806683u64 * 12847790058493922663u64));
/// assert!(panics!(11835040565162213512u64 * 14461049681861693270u64));
/// assert!(panics!(6494901747412170345u64 * 5571912404186521405u64));
/// # }
/// ```
pub fn core_ops_mul_u64_mul() {}
/// # Properties for [`u64::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u64::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_mul(0u64) == Some(0u64));
/// assert!(0u64.checked_mul(1u64) == Some(0u64));
/// assert!(0u64.checked_mul(18446744073709551614u64) == Some(0u64));
/// assert!(0u64.checked_mul(18446744073709551615u64) == Some(0u64));
/// assert!(1u64.checked_mul(0u64) == Some(0u64));
/// assert!(1u64.checked_mul(1u64) == Some(1u64));
/// assert!(1u64.checked_mul(18446744073709551614u64) == Some(18446744073709551614u64));
/// assert!(1u64.checked_mul(18446744073709551615u64) == Some(18446744073709551615u64));
/// assert!(18446744073709551614u64.checked_mul(0u64) == Some(0u64));
/// assert!(18446744073709551614u64.checked_mul(1u64) == Some(18446744073709551614u64));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.checked_mul(18446744073709551614u64) == None);
/// assert!(18446744073709551614u64.checked_mul(18446744073709551615u64) == None);
/// assert!(18446744073709551615u64.checked_mul(18446744073709551614u64) == None);
/// assert!(18446744073709551615u64.checked_mul(18446744073709551615u64) == None);
/// assert!(9862057476669960003u64.checked_mul(6937918249578856831u64) == None);
/// assert!(5173110774864214841u64.checked_mul(12935809653858684130u64) == None);
/// assert!(4102833335846059815u64.checked_mul(16809065469832485029u64) == None);
/// assert!(14922657639000448212u64.checked_mul(16097467861226567488u64) == None);
/// assert!(16132962588227065316u64.checked_mul(17684605140475820048u64) == None);
/// assert!(5946642786719268001u64.checked_mul(10818287167336678389u64) == None);
/// # }
/// ```
pub fn core_ops_mul_u64_checked_mul() {}
/// # Properties for [`u64::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u64::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.overflowing_mul(0u64) == (0u64, false));
/// assert!(0u64.overflowing_mul(1u64) == (0u64, false));
/// assert!(0u64.overflowing_mul(18446744073709551614u64) == (0u64, false));
/// assert!(0u64.overflowing_mul(18446744073709551615u64) == (0u64, false));
/// assert!(1u64.overflowing_mul(0u64) == (0u64, false));
/// assert!(1u64.overflowing_mul(1u64) == (1u64, false));
/// assert!(1u64.overflowing_mul(18446744073709551614u64) == (18446744073709551614u64, false));
/// assert!(1u64.overflowing_mul(18446744073709551615u64) == (18446744073709551615u64, false));
/// assert!(18446744073709551614u64.overflowing_mul(0u64) == (0u64, false));
/// assert!(18446744073709551614u64.overflowing_mul(1u64) == (18446744073709551614u64, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.overflowing_mul(18446744073709551614u64) == (4u64, true));
/// assert!(18446744073709551614u64.overflowing_mul(18446744073709551615u64) == (2u64, true));
/// assert!(18446744073709551615u64.overflowing_mul(18446744073709551614u64) == (2u64, true));
/// assert!(18446744073709551615u64.overflowing_mul(18446744073709551615u64) == (1u64, true));
/// assert!(13195139549544680652u64.overflowing_mul(2283988923174603735u64)
///         == (12087666251553768276u64, true));
/// assert!(12722360789284704144u64.overflowing_mul(5687553636082995914u64)
///         == (762287324007688096u64, true));
/// assert!(472655799547661441u64.overflowing_mul(2204136525185030660u64)
///         == (2960527894354807812u64, true));
/// assert!(2983965396577106891u64.overflowing_mul(13337576900273806172u64)
///         == (10112170683917624820u64, true));
/// assert!(7452442574016340908u64.overflowing_mul(925614786037394238u64)
///         == (3913203079594706856u64, true));
/// assert!(13608300127983576701u64.overflowing_mul(12780928886142458952u64)
///         == (910211355232059176u64, true));
/// # }
/// ```
pub fn core_ops_mul_u64_overflowing_mul() {}
/// # Properties for [`u64::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u64::down((x.up() * y.up()).min(u64::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_mul(0u64) == 0u64);
/// assert!(0u64.saturating_mul(1u64) == 0u64);
/// assert!(0u64.saturating_mul(18446744073709551614u64) == 0u64);
/// assert!(0u64.saturating_mul(18446744073709551615u64) == 0u64);
/// assert!(1u64.saturating_mul(0u64) == 0u64);
/// assert!(1u64.saturating_mul(1u64) == 1u64);
/// assert!(1u64.saturating_mul(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(1u64.saturating_mul(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.saturating_mul(0u64) == 0u64);
/// assert!(18446744073709551614u64.saturating_mul(1u64) == 18446744073709551614u64);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u64::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_mul(0u64) == 0u64);
/// assert!(0u64.saturating_mul(1u64) == 0u64);
/// assert!(0u64.saturating_mul(18446744073709551614u64) == 0u64);
/// assert!(0u64.saturating_mul(18446744073709551615u64) == 0u64);
/// assert!(1u64.saturating_mul(0u64) == 0u64);
/// assert!(1u64.saturating_mul(1u64) == 1u64);
/// assert!(1u64.saturating_mul(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(1u64.saturating_mul(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.saturating_mul(0u64) == 0u64);
/// assert!(18446744073709551614u64.saturating_mul(1u64) == 18446744073709551614u64);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u64::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614u64.saturating_mul(18446744073709551614u64) == u64::MAX);
/// assert!(18446744073709551614u64.saturating_mul(18446744073709551615u64) == u64::MAX);
/// assert!(18446744073709551615u64.saturating_mul(18446744073709551614u64) == u64::MAX);
/// assert!(18446744073709551615u64.saturating_mul(18446744073709551615u64) == u64::MAX);
/// assert!(7787693623157690452u64.saturating_mul(3490380110791788858u64) == u64::MAX);
/// assert!(8547232556606313442u64.saturating_mul(16468823452198069287u64) == u64::MAX);
/// assert!(17763422981668440206u64.saturating_mul(9125442801883734935u64) == u64::MAX);
/// assert!(5326423811895384367u64.saturating_mul(10758129864114311540u64) == u64::MAX);
/// assert!(6775235519681112414u64.saturating_mul(2919742483580896700u64) == u64::MAX);
/// assert!(4370886835889403627u64.saturating_mul(948386729448976206u64) == u64::MAX);
/// # }
/// ```
pub fn core_ops_mul_u64_saturating_mul() {}
/// # Properties for [`u64::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u64::down((x.up() * y.up()) % (u64::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_mul(0u64) == 0u64);
/// assert!(0u64.wrapping_mul(1u64) == 0u64);
/// assert!(0u64.wrapping_mul(18446744073709551614u64) == 0u64);
/// assert!(0u64.wrapping_mul(18446744073709551615u64) == 0u64);
/// assert!(1u64.wrapping_mul(0u64) == 0u64);
/// assert!(1u64.wrapping_mul(1u64) == 1u64);
/// assert!(1u64.wrapping_mul(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(1u64.wrapping_mul(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.wrapping_mul(0u64) == 0u64);
/// assert!(18446744073709551614u64.wrapping_mul(1u64) == 18446744073709551614u64);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() * y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u64::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_mul(0u64) == 0u64);
/// assert!(0u64.wrapping_mul(1u64) == 0u64);
/// assert!(0u64.wrapping_mul(18446744073709551614u64) == 0u64);
/// assert!(0u64.wrapping_mul(18446744073709551615u64) == 0u64);
/// assert!(1u64.wrapping_mul(0u64) == 0u64);
/// assert!(1u64.wrapping_mul(1u64) == 1u64);
/// assert!(1u64.wrapping_mul(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(1u64.wrapping_mul(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.wrapping_mul(0u64) == 0u64);
/// assert!(18446744073709551614u64.wrapping_mul(1u64) == 18446744073709551614u64);
/// # }
/// ```
pub fn core_ops_mul_u64_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u64>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 - 0u64 == 0u64);
/// assert!(1u64 - 0u64 == 1u64);
/// assert!(1u64 - 1u64 == 0u64);
/// assert!(18446744073709551614u64 - 0u64 == 18446744073709551614u64);
/// assert!(18446744073709551614u64 - 1u64 == 18446744073709551613u64);
/// assert!(18446744073709551614u64 - 18446744073709551614u64 == 0u64);
/// assert!(18446744073709551615u64 - 0u64 == 18446744073709551615u64);
/// assert!(18446744073709551615u64 - 1u64 == 18446744073709551614u64);
/// assert!(18446744073709551615u64 - 18446744073709551614u64 == 1u64);
/// assert!(18446744073709551615u64 - 18446744073709551615u64 == 0u64);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u64 - 1u64));
/// assert!(panics!(0u64 - 18446744073709551614u64));
/// assert!(panics!(0u64 - 18446744073709551615u64));
/// assert!(panics!(1u64 - 18446744073709551614u64));
/// assert!(panics!(1u64 - 18446744073709551615u64));
/// assert!(panics!(18446744073709551614u64 - 18446744073709551615u64));
/// assert!(panics!(1997668652313639283u64 - 5544618440645251137u64));
/// assert!(panics!(11961838921438417049u64 - 16875742856165594038u64));
/// assert!(panics!(13258004590662894703u64 - 15139693995714335365u64));
/// assert!(panics!(3460428537107697943u64 - 9306171168058016252u64));
/// # }
/// ```
pub fn core_ops_add_u64_sub() {}
/// # Properties for [`u64::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_sub(0u64) == 0u64);
/// assert!(1u64.wrapping_sub(0u64) == 1u64);
/// assert!(1u64.wrapping_sub(1u64) == 0u64);
/// assert!(18446744073709551614u64.wrapping_sub(0u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.wrapping_sub(1u64) == 18446744073709551613u64);
/// assert!(18446744073709551614u64.wrapping_sub(18446744073709551614u64) == 0u64);
/// assert!(18446744073709551615u64.wrapping_sub(0u64) == 18446744073709551615u64);
/// assert!(18446744073709551615u64.wrapping_sub(1u64) == 18446744073709551614u64);
/// assert!(18446744073709551615u64.wrapping_sub(18446744073709551614u64) == 1u64);
/// assert!(18446744073709551615u64.wrapping_sub(18446744073709551615u64) == 0u64);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u64::down(x.up() - y.up() + u64::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_sub(1u64) == 18446744073709551615u64);
/// assert!(0u64.wrapping_sub(18446744073709551614u64) == 2u64);
/// assert!(0u64.wrapping_sub(18446744073709551615u64) == 1u64);
/// assert!(1u64.wrapping_sub(18446744073709551614u64) == 3u64);
/// assert!(1u64.wrapping_sub(18446744073709551615u64) == 2u64);
/// assert!(18446744073709551614u64.wrapping_sub(18446744073709551615u64)
///         == 18446744073709551615u64);
/// assert!(5878459738075178336u64.wrapping_sub(7183990408370643356u64)
///         == 17141213403414086596u64);
/// assert!(3730329075477823665u64.wrapping_sub(5982794385126705711u64)
///         == 16194278764060669570u64);
/// assert!(7034167705933885151u64.wrapping_sub(11812137164304713243u64)
///         == 13668774615338723524u64);
/// assert!(11333767894596695119u64.wrapping_sub(18189835492054069584u64)
///         == 11590676476252177151u64);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u64::down((x.up() - y.up()).rem_euclid(&(u64::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_sub(0u64) == 0u64);
/// assert!(0u64.wrapping_sub(1u64) == 18446744073709551615u64);
/// assert!(0u64.wrapping_sub(18446744073709551614u64) == 2u64);
/// assert!(0u64.wrapping_sub(18446744073709551615u64) == 1u64);
/// assert!(1u64.wrapping_sub(0u64) == 1u64);
/// assert!(1u64.wrapping_sub(1u64) == 0u64);
/// assert!(1u64.wrapping_sub(18446744073709551614u64) == 3u64);
/// assert!(1u64.wrapping_sub(18446744073709551615u64) == 2u64);
/// assert!(18446744073709551614u64.wrapping_sub(0u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.wrapping_sub(1u64) == 18446744073709551613u64);
/// # }
/// ```
pub fn core_ops_add_u64_wrapping_sub() {}
/// # Properties for [`u64::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u64::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_sub(0u64) == Some(0u64));
/// assert!(1u64.checked_sub(0u64) == Some(1u64));
/// assert!(1u64.checked_sub(1u64) == Some(0u64));
/// assert!(18446744073709551614u64.checked_sub(0u64) == Some(18446744073709551614u64));
/// assert!(18446744073709551614u64.checked_sub(1u64) == Some(18446744073709551613u64));
/// assert!(18446744073709551614u64.checked_sub(18446744073709551614u64) == Some(0u64));
/// assert!(18446744073709551615u64.checked_sub(0u64) == Some(18446744073709551615u64));
/// assert!(18446744073709551615u64.checked_sub(1u64) == Some(18446744073709551614u64));
/// assert!(18446744073709551615u64.checked_sub(18446744073709551614u64) == Some(1u64));
/// assert!(18446744073709551615u64.checked_sub(18446744073709551615u64) == Some(0u64));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_sub(1u64) == None);
/// assert!(0u64.checked_sub(18446744073709551614u64) == None);
/// assert!(0u64.checked_sub(18446744073709551615u64) == None);
/// assert!(1u64.checked_sub(18446744073709551614u64) == None);
/// assert!(1u64.checked_sub(18446744073709551615u64) == None);
/// assert!(18446744073709551614u64.checked_sub(18446744073709551615u64) == None);
/// assert!(3569049899647364694u64.checked_sub(8349829280912453498u64) == None);
/// assert!(955842643384470358u64.checked_sub(16301180411706013455u64) == None);
/// assert!(7390255355564826313u64.checked_sub(14570076170362408364u64) == None);
/// assert!(2312027009061992542u64.checked_sub(10212580519852998538u64) == None);
/// # }
/// ```
pub fn core_ops_add_u64_checked_sub() {}
/// # Properties for [`u64::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u64::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_sub(0u64) == 0u64);
/// assert!(0u64.saturating_sub(1u64) == 0u64);
/// assert!(0u64.saturating_sub(18446744073709551614u64) == 0u64);
/// assert!(0u64.saturating_sub(18446744073709551615u64) == 0u64);
/// assert!(1u64.saturating_sub(0u64) == 1u64);
/// assert!(1u64.saturating_sub(1u64) == 0u64);
/// assert!(1u64.saturating_sub(18446744073709551614u64) == 0u64);
/// assert!(1u64.saturating_sub(18446744073709551615u64) == 0u64);
/// assert!(18446744073709551614u64.saturating_sub(0u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.saturating_sub(1u64) == 18446744073709551613u64);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_sub(0u64) == 0u64);
/// assert!(1u64.saturating_sub(0u64) == 1u64);
/// assert!(1u64.saturating_sub(1u64) == 0u64);
/// assert!(18446744073709551614u64.saturating_sub(0u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.saturating_sub(1u64) == 18446744073709551613u64);
/// assert!(18446744073709551614u64.saturating_sub(18446744073709551614u64) == 0u64);
/// assert!(18446744073709551615u64.saturating_sub(0u64) == 18446744073709551615u64);
/// assert!(18446744073709551615u64.saturating_sub(1u64) == 18446744073709551614u64);
/// assert!(18446744073709551615u64.saturating_sub(18446744073709551614u64) == 1u64);
/// assert!(18446744073709551615u64.saturating_sub(18446744073709551615u64) == 0u64);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_sub(1u64) == 0);
/// assert!(0u64.saturating_sub(18446744073709551614u64) == 0);
/// assert!(0u64.saturating_sub(18446744073709551615u64) == 0);
/// assert!(1u64.saturating_sub(18446744073709551614u64) == 0);
/// assert!(1u64.saturating_sub(18446744073709551615u64) == 0);
/// assert!(18446744073709551614u64.saturating_sub(18446744073709551615u64) == 0);
/// assert!(13238486017167303648u64.saturating_sub(18037820330502149770u64) == 0);
/// assert!(10556950143128431055u64.saturating_sub(15871205338462379579u64) == 0);
/// assert!(6179869513571376995u64.saturating_sub(14814611137434367897u64) == 0);
/// assert!(5756583463816967759u64.saturating_sub(16595678061184259053u64) == 0);
/// # }
/// ```
pub fn core_ops_sub_u64_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u64>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x + y == u64::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 + 0u64 == 0u64);
/// assert!(0u64 + 1u64 == 1u64);
/// assert!(0u64 + 18446744073709551614u64 == 18446744073709551614u64);
/// assert!(0u64 + 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(1u64 + 0u64 == 1u64);
/// assert!(1u64 + 1u64 == 2u64);
/// assert!(1u64 + 18446744073709551614u64 == 18446744073709551615u64);
/// assert!(18446744073709551614u64 + 0u64 == 18446744073709551614u64);
/// assert!(18446744073709551614u64 + 1u64 == 18446744073709551615u64);
/// assert!(18446744073709551615u64 + 0u64 == 18446744073709551615u64);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1u64 + 18446744073709551615u64));
/// assert!(panics!(18446744073709551614u64 + 18446744073709551614u64));
/// assert!(panics!(18446744073709551614u64 + 18446744073709551615u64));
/// assert!(panics!(18446744073709551615u64 + 1u64));
/// assert!(panics!(18446744073709551615u64 + 18446744073709551614u64));
/// assert!(panics!(18446744073709551615u64 + 18446744073709551615u64));
/// assert!(panics!(6893393409966827436u64 + 15423438448615291425u64));
/// assert!(panics!(16018578157876276551u64 + 18186996241134856597u64));
/// assert!(panics!(17018231180998817174u64 + 6681056814086010475u64));
/// assert!(panics!(12946848439981856205u64 + 10171789210195651729u64));
/// # }
/// ```
pub fn core_ops_add_u64_add() {}
/// # Properties for [`u64::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u64::down((x.up() + y.up()) % (u64::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_add(0u64) == 0u64);
/// assert!(0u64.wrapping_add(1u64) == 1u64);
/// assert!(0u64.wrapping_add(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(0u64.wrapping_add(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(1u64.wrapping_add(0u64) == 1u64);
/// assert!(1u64.wrapping_add(1u64) == 2u64);
/// assert!(1u64.wrapping_add(18446744073709551614u64) == 18446744073709551615u64);
/// assert!(1u64.wrapping_add(18446744073709551615u64) == 0u64);
/// assert!(18446744073709551614u64.wrapping_add(0u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.wrapping_add(1u64) == 18446744073709551615u64);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u64::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.wrapping_add(0u64) == 0u64);
/// assert!(0u64.wrapping_add(1u64) == 1u64);
/// assert!(0u64.wrapping_add(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(0u64.wrapping_add(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(1u64.wrapping_add(0u64) == 1u64);
/// assert!(1u64.wrapping_add(1u64) == 2u64);
/// assert!(1u64.wrapping_add(18446744073709551614u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.wrapping_add(0u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.wrapping_add(1u64) == 18446744073709551615u64);
/// assert!(18446744073709551615u64.wrapping_add(0u64) == 18446744073709551615u64);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u64::down(x.up() + y.up() - u64::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.wrapping_add(18446744073709551615u64) == 0u64);
/// assert!(18446744073709551614u64.wrapping_add(18446744073709551614u64)
///         == 18446744073709551612u64);
/// assert!(18446744073709551614u64.wrapping_add(18446744073709551615u64)
///         == 18446744073709551613u64);
/// assert!(18446744073709551615u64.wrapping_add(1u64) == 0u64);
/// assert!(18446744073709551615u64.wrapping_add(18446744073709551614u64)
///         == 18446744073709551613u64);
/// assert!(18446744073709551615u64.wrapping_add(18446744073709551615u64)
///         == 18446744073709551614u64);
/// assert!(16169412380638655792u64.wrapping_add(7553746068175034106u64)
///         == 5276414375104138282u64);
/// assert!(18110369949102128835u64.wrapping_add(5758695292733290984u64)
///         == 5422321168125868203u64);
/// assert!(5034225414382787550u64.wrapping_add(13762123097102891650u64) == 349604437776127584u64);
/// assert!(12929650550306515647u64.wrapping_add(15775877549990446634u64)
///         == 10258784026587410665u64);
/// # }
/// ```
pub fn core_ops_add_u64_wrapping_add() {}
/// # Properties for [`u64::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u64::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.checked_add(0u64) == Some(0u64));
/// assert!(0u64.checked_add(1u64) == Some(1u64));
/// assert!(0u64.checked_add(18446744073709551614u64) == Some(18446744073709551614u64));
/// assert!(0u64.checked_add(18446744073709551615u64) == Some(18446744073709551615u64));
/// assert!(1u64.checked_add(0u64) == Some(1u64));
/// assert!(1u64.checked_add(1u64) == Some(2u64));
/// assert!(1u64.checked_add(18446744073709551614u64) == Some(18446744073709551615u64));
/// assert!(18446744073709551614u64.checked_add(0u64) == Some(18446744073709551614u64));
/// assert!(18446744073709551614u64.checked_add(1u64) == Some(18446744073709551615u64));
/// assert!(18446744073709551615u64.checked_add(0u64) == Some(18446744073709551615u64));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.checked_add(18446744073709551615u64) == None);
/// assert!(18446744073709551614u64.checked_add(18446744073709551614u64) == None);
/// assert!(18446744073709551614u64.checked_add(18446744073709551615u64) == None);
/// assert!(18446744073709551615u64.checked_add(1u64) == None);
/// assert!(18446744073709551615u64.checked_add(18446744073709551614u64) == None);
/// assert!(18446744073709551615u64.checked_add(18446744073709551615u64) == None);
/// assert!(14222132645425439410u64.checked_add(11248999731035994990u64) == None);
/// assert!(16347620275190362092u64.checked_add(14360293286210606548u64) == None);
/// assert!(5427435255463426229u64.checked_add(14967389887151263664u64) == None);
/// assert!(2137645663744080727u64.checked_add(17656176057413913760u64) == None);
/// # }
/// ```
pub fn core_ops_add_u64_checked_add() {}
/// # Properties for [`u64::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u64::down((x.up() + y.up()).min(u64::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_add(0u64) == 0u64);
/// assert!(0u64.saturating_add(1u64) == 1u64);
/// assert!(0u64.saturating_add(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(0u64.saturating_add(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(1u64.saturating_add(0u64) == 1u64);
/// assert!(1u64.saturating_add(1u64) == 2u64);
/// assert!(1u64.saturating_add(18446744073709551614u64) == 18446744073709551615u64);
/// assert!(1u64.saturating_add(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.saturating_add(0u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.saturating_add(1u64) == 18446744073709551615u64);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() <= u64::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u64::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.saturating_add(0u64) == 0u64);
/// assert!(0u64.saturating_add(1u64) == 1u64);
/// assert!(0u64.saturating_add(18446744073709551614u64) == 18446744073709551614u64);
/// assert!(0u64.saturating_add(18446744073709551615u64) == 18446744073709551615u64);
/// assert!(1u64.saturating_add(0u64) == 1u64);
/// assert!(1u64.saturating_add(1u64) == 2u64);
/// assert!(1u64.saturating_add(18446744073709551614u64) == 18446744073709551615u64);
/// assert!(18446744073709551614u64.saturating_add(0u64) == 18446744073709551614u64);
/// assert!(18446744073709551614u64.saturating_add(1u64) == 18446744073709551615u64);
/// assert!(18446744073709551615u64.saturating_add(0u64) == 18446744073709551615u64);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `x.up() + y.up() > u64::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u64::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u64.saturating_add(18446744073709551615u64) == u64::MAX);
/// assert!(18446744073709551614u64.saturating_add(18446744073709551614u64) == u64::MAX);
/// assert!(18446744073709551614u64.saturating_add(18446744073709551615u64) == u64::MAX);
/// assert!(18446744073709551615u64.saturating_add(1u64) == u64::MAX);
/// assert!(18446744073709551615u64.saturating_add(18446744073709551614u64) == u64::MAX);
/// assert!(18446744073709551615u64.saturating_add(18446744073709551615u64) == u64::MAX);
/// assert!(12020636408201585899u64.saturating_add(17642979970036974151u64) == u64::MAX);
/// assert!(10824699074829269672u64.saturating_add(12725380823303972928u64) == u64::MAX);
/// assert!(8237074117337499592u64.saturating_add(14276725748061431755u64) == u64::MAX);
/// assert!(15043515233416467438u64.saturating_add(16682193489959846562u64) == u64::MAX);
/// # }
/// ```
pub fn core_ops_add_u64_saturating_add() {}
/// # Properties for [`core::ops::Rem::<u128>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == u128::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 % 1u128 == 0u128);
/// assert!(0u128 % 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(0u128 % 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(1u128 % 1u128 == 0u128);
/// assert!(1u128 % 340282366920938463463374607431768211454u128 == 1u128);
/// assert!(1u128 % 340282366920938463463374607431768211455u128 == 1u128);
/// assert!(340282366920938463463374607431768211454u128 % 1u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128
///         % 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128
///         % 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211455u128 % 1u128 == 0u128);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u128 % 0) } });
/// # }
/// ```
pub fn core_ops_rem_u128_rem() {}
/// # Properties for [`u128::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(u128::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_rem(1u128) == Some(0u128));
/// assert!(0u128.checked_rem(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(0u128.checked_rem(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(1u128.checked_rem(1u128) == Some(0u128));
/// assert!(1u128.checked_rem(340282366920938463463374607431768211454u128) == Some(1u128));
/// assert!(1u128.checked_rem(340282366920938463463374607431768211455u128) == Some(1u128));
/// assert!(340282366920938463463374607431768211454u128.checked_rem(1u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128
///         .checked_rem(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128
///         .checked_rem(340282366920938463463374607431768211455u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211455u128.checked_rem(1u128) == Some(0u128));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0u128.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_u128_checked_rem() {}
/// # Properties for [`u128::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `x == u128::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(u128::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_neg() == Some(0u128));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `x != u128::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128.checked_neg() == None);
/// assert!(340282366920938463463374607431768211454u128.checked_neg() == None);
/// assert!(340282366920938463463374607431768211455u128.checked_neg() == None);
/// assert!(138474940568389830637358467815261667412u128.checked_neg() == None);
/// assert!(329418651647205185287102883126906383346u128.checked_neg() == None);
/// assert!(185591526695770011102375654972997300044u128.checked_neg() == None);
/// assert!(98375658851382699622848366693414487756u128.checked_neg() == None);
/// assert!(272927189908345478454428737895513487591u128.checked_neg() == None);
/// assert!(166499632598454991651898827714385671375u128.checked_neg() == None);
/// assert!(269674048945404935227483592082200270236u128.checked_neg() == None);
/// # }
/// ```
pub fn t_u128_checked_neg() {}
/// # Properties for [`core::ops::Shl::<u128>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x << y == u128::down((x.up() << y) & u128::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 << 0u32 == 0u128);
/// assert!(0u128 << 1u32 == 0u128);
/// assert!(1u128 << 0u32 == 1u128);
/// assert!(1u128 << 1u32 == 2u128);
/// assert!(340282366920938463463374607431768211454u128 << 0u32
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128 << 1u32
///         == 340282366920938463463374607431768211452u128);
/// assert!(340282366920938463463374607431768211455u128 << 0u32
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128 << 1u32
///         == 340282366920938463463374607431768211454u128);
/// assert!(236874679604612805226843349983273168613u128 << 123u32
///         == 53169119831396634916152282411213783040u128);
/// assert!(267728748332624959855304654538945266398u128 << 115u32
///         == 30489167153316507834731074445180403712u128);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u128 << 4294967294u32));
/// assert!(panics!(0u128 << 4294967295u32));
/// assert!(panics!(1u128 << 4294967294u32));
/// assert!(panics!(1u128 << 4294967295u32));
/// assert!(panics!(340282366920938463463374607431768211454u128 << 4294967294u32));
/// assert!(panics!(340282366920938463463374607431768211454u128 << 4294967295u32));
/// assert!(panics!(340282366920938463463374607431768211455u128 << 4294967294u32));
/// assert!(panics!(340282366920938463463374607431768211455u128 << 4294967295u32));
/// assert!(panics!(214241830059530030967841527796753579484u128 << 137u32));
/// assert!(panics!(68999428739042363722058781434812394187u128 << 130u32));
/// # }
/// ```
pub fn core_ops_shl_u128_shl() {}
/// # Properties for [`core::ops::Shl::<u128>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(u128::down((x.up() << y) & u128::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_shl(0u32) == Some(0u128));
/// assert!(0u128.checked_shl(1u32) == Some(0u128));
/// assert!(1u128.checked_shl(0u32) == Some(1u128));
/// assert!(1u128.checked_shl(1u32) == Some(2u128));
/// assert!(340282366920938463463374607431768211454u128.checked_shl(0u32)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211454u128.checked_shl(1u32)
///         == Some(340282366920938463463374607431768211452u128));
/// assert!(340282366920938463463374607431768211455u128.checked_shl(0u32)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211455u128.checked_shl(1u32)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(99458653056135931001705141828900066903u128.checked_shl(75u32)
///         == Some(225648470958036513454680048052140507136u128));
/// assert!(327752596679603086725586765203600262309u128.checked_shl(22u32)
///         == Some(223869689642181400003312003404334628864u128));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_shl(4294967294u32) == None);
/// assert!(0u128.checked_shl(4294967295u32) == None);
/// assert!(1u128.checked_shl(4294967294u32) == None);
/// assert!(1u128.checked_shl(4294967295u32) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_shl(4294967294u32) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_shl(4294967295u32) == None);
/// assert!(340282366920938463463374607431768211455u128.checked_shl(4294967294u32) == None);
/// assert!(340282366920938463463374607431768211455u128.checked_shl(4294967295u32) == None);
/// assert!(256530846343781391073287964953310929928u128.checked_shl(133u32) == None);
/// assert!(79587231180793054207511194495346942002u128.checked_shl(135u32) == None);
/// # }
/// ```
pub fn core_ops_shl_u128_checked_shl() {}
/// # Properties for [`core::ops::Shl::<u128>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (u128::down((x.up() << y) & u128::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.overflowing_shl(0u32) == (0u128, false));
/// assert!(0u128.overflowing_shl(1u32) == (0u128, false));
/// assert!(1u128.overflowing_shl(0u32) == (1u128, false));
/// assert!(1u128.overflowing_shl(1u32) == (2u128, false));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shl(0u32)
///         == (340282366920938463463374607431768211454u128, false));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shl(1u32)
///         == (340282366920938463463374607431768211452u128, false));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shl(0u32)
///         == (340282366920938463463374607431768211455u128, false));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shl(1u32)
///         == (340282366920938463463374607431768211454u128, false));
/// assert!(311767090994655368604411815315976924074u128.overflowing_shl(30u32)
///         == (163560655895988147600567255405332267008u128, false));
/// assert!(134490828625379025413551471095210807982u128.overflowing_shl(32u32)
///         == (4575925631853198748385846866024071168u128, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (u128::down((x.up() << (y & (u128::BITS - 1)) & u128::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.overflowing_shl(4294967294u32) == (0u128, true));
/// assert!(0u128.overflowing_shl(4294967295u32) == (0u128, true));
/// assert!(1u128.overflowing_shl(4294967294u32)
///         == (85070591730234615865843651857942052864u128, true));
/// assert!(1u128.overflowing_shl(4294967295u32)
///         == (170141183460469231731687303715884105728u128, true));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shl(4294967294u32)
///         == (170141183460469231731687303715884105728u128, true));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shl(4294967295u32)
///         == (0u128, true));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shl(4294967294u32)
///         == (255211775190703847597530955573826158592u128, true));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shl(4294967295u32)
///         == (170141183460469231731687303715884105728u128, true));
/// assert!(139333737271630274446874898930230082366u128.overflowing_shl(139u32)
///         == (198870452552369684891871981289447485440u128, true));
/// assert!(61544771544160783430890872203498998270u128.overflowing_shl(130u32)
///         == (246179086176643133723563488813995993080u128, true));
/// # }
/// ```
pub fn core_ops_shl_u128_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<u128>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x >> y == u128::down((x.up() >> y) & u128::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 >> 0u32 == 0u128);
/// assert!(0u128 >> 1u32 == 0u128);
/// assert!(1u128 >> 0u32 == 1u128);
/// assert!(1u128 >> 1u32 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 >> 0u32
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128 >> 1u32
///         == 170141183460469231731687303715884105727u128);
/// assert!(340282366920938463463374607431768211455u128 >> 0u32
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128 >> 1u32
///         == 170141183460469231731687303715884105727u128);
/// assert!(134393416412467526968259325509813045566u128 >> 116u32 == 1617u128);
/// assert!(307086435482115175459951608857732372278u128 >> 32u32
///         == 71499132430673384913232086426u128);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u128 >> 4294967294u32));
/// assert!(panics!(0u128 >> 4294967295u32));
/// assert!(panics!(1u128 >> 4294967294u32));
/// assert!(panics!(1u128 >> 4294967295u32));
/// assert!(panics!(340282366920938463463374607431768211454u128 >> 4294967294u32));
/// assert!(panics!(340282366920938463463374607431768211454u128 >> 4294967295u32));
/// assert!(panics!(340282366920938463463374607431768211455u128 >> 4294967294u32));
/// assert!(panics!(340282366920938463463374607431768211455u128 >> 4294967295u32));
/// assert!(panics!(189912662630679158210173696604930771614u128 >> 134u32));
/// assert!(panics!(3379678869285564700775281549090400036u128 >> 137u32));
/// # }
/// ```
pub fn core_ops_shr_u128_shr() {}
/// # Properties for [`core::ops::Shr::<u128>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(u128::down((x.up() >> y) & u128::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_shr(0u32) == Some(0u128));
/// assert!(0u128.checked_shr(1u32) == Some(0u128));
/// assert!(1u128.checked_shr(0u32) == Some(1u128));
/// assert!(1u128.checked_shr(1u32) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128.checked_shr(0u32)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211454u128.checked_shr(1u32)
///         == Some(170141183460469231731687303715884105727u128));
/// assert!(340282366920938463463374607431768211455u128.checked_shr(0u32)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211455u128.checked_shr(1u32)
///         == Some(170141183460469231731687303715884105727u128));
/// assert!(163594850825552139051553903837263301833u128.checked_shr(111u32) == Some(63014u128));
/// assert!(160771083071170830471569018696904558575u128.checked_shr(42u32)
///         == Some(36555112062881296531388447u128));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_shr(4294967294u32) == None);
/// assert!(0u128.checked_shr(4294967295u32) == None);
/// assert!(1u128.checked_shr(4294967294u32) == None);
/// assert!(1u128.checked_shr(4294967295u32) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_shr(4294967294u32) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_shr(4294967295u32) == None);
/// assert!(340282366920938463463374607431768211455u128.checked_shr(4294967294u32) == None);
/// assert!(340282366920938463463374607431768211455u128.checked_shr(4294967295u32) == None);
/// assert!(199531002353215592194880001528437068698u128.checked_shr(135u32) == None);
/// assert!(303754403171291963163110228585564643149u128.checked_shr(129u32) == None);
/// # }
/// ```
pub fn core_ops_shr_u128_checked_shr() {}
/// # Properties for [`core::ops::Shr::<u128>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y < u128::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (u128::down((x.up() >> y) & u128::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.overflowing_shr(0u32) == (0u128, false));
/// assert!(0u128.overflowing_shr(1u32) == (0u128, false));
/// assert!(1u128.overflowing_shr(0u32) == (1u128, false));
/// assert!(1u128.overflowing_shr(1u32) == (0u128, false));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shr(0u32)
///         == (340282366920938463463374607431768211454u128, false));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shr(1u32)
///         == (170141183460469231731687303715884105727u128, false));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shr(0u32)
///         == (340282366920938463463374607431768211455u128, false));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shr(1u32)
///         == (170141183460469231731687303715884105727u128, false));
/// assert!(272856445613720225621324447799185709787u128.overflowing_shr(61u32)
///         == (118332620444427342324u128, false));
/// assert!(37501528119929969177921026281020832156u128.overflowing_shr(44u32)
///         == (2131715070841549344204453u128, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : u128, y : u32`  
/// __Precondition:__ `y >= u128::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (u128::down((x.up() >> (y & (u128::BITS - 1)) & u128::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.overflowing_shr(4294967294u32) == (0u128, true));
/// assert!(0u128.overflowing_shr(4294967295u32) == (0u128, true));
/// assert!(1u128.overflowing_shr(4294967294u32) == (0u128, true));
/// assert!(1u128.overflowing_shr(4294967295u32) == (0u128, true));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shr(4294967294u32)
///         == (3u128, true));
/// assert!(340282366920938463463374607431768211454u128.overflowing_shr(4294967295u32)
///         == (1u128, true));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shr(4294967294u32)
///         == (3u128, true));
/// assert!(340282366920938463463374607431768211455u128.overflowing_shr(4294967295u32)
///         == (1u128, true));
/// assert!(26743545863835572476552916318354585836u128.overflowing_shr(129u32)
///         == (13371772931917786238276458159177292918u128, true));
/// assert!(113803292843924092616552455990689082753u128.overflowing_shr(139u32)
///         == (55568014083947310847926003901703653u128, true));
/// # }
/// ```
pub fn core_ops_shr_u128_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<u128>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == u128::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 / 1u128 == 0u128);
/// assert!(0u128 / 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(0u128 / 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(1u128 / 1u128 == 1u128);
/// assert!(1u128 / 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(1u128 / 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 / 1u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128
///         / 340282366920938463463374607431768211454u128 == 1u128);
/// assert!(340282366920938463463374607431768211454u128
///         / 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 / 1u128
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u128 / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u128 / 0) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(340282366920938463463374607431768211454u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(340282366920938463463374607431768211455u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(275012819387112482453304633397141717299u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(152529964187822081817706612479662995499u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(175049362882125320478401232611828915371u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(231456877455561950859942629718955510967u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(240319901229856996058082260364546424734u128 / 0) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(27464845255787887764886524745774491745u128 / 0) }
///     });
/// # }
/// ```
pub fn core_ops_div_u128_div() {}
/// # Properties for [`u128::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == u128::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_div(1u128) == 0u128);
/// assert!(0u128.saturating_div(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(0u128.saturating_div(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(1u128.saturating_div(1u128) == 1u128);
/// assert!(1u128.saturating_div(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(1u128.saturating_div(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_div(1u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_div(340282366920938463463374607431768211454u128) == 1u128);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_div(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.saturating_div(1u128)
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0u128.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1u128.saturating_div(0)) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(340282366920938463463374607431768211454u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(340282366920938463463374607431768211455u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(55785049545772471192954540193472973582u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(98233633839899956083763853478476317059u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(65302386123225724116194643072819230637u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(336332166068324612151010359436927813030u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(149010380483451751087682932214265491768u128.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(251542770083679803763775234043278345736u128.saturating_div(0)) }
///     });
/// # }
/// ```
pub fn core_ops_div_u128_saturating_div() {}
/// # Properties for [`u128::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(u128::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_div(1u128) == Some(0u128));
/// assert!(0u128.checked_div(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(0u128.checked_div(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(1u128.checked_div(1u128) == Some(1u128));
/// assert!(1u128.checked_div(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(1u128.checked_div(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128.checked_div(1u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211454u128
///         .checked_div(340282366920938463463374607431768211454u128) == Some(1u128));
/// assert!(340282366920938463463374607431768211454u128
///         .checked_div(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211455u128.checked_div(1u128)
///         == Some(340282366920938463463374607431768211455u128));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_div(0) == None);
/// assert!(1u128.checked_div(0) == None);
/// assert!(340282366920938463463374607431768211454u128.checked_div(0) == None);
/// assert!(340282366920938463463374607431768211455u128.checked_div(0) == None);
/// assert!(333204663838938050456541908081459454212u128.checked_div(0) == None);
/// assert!(83947562064687258731327350094937670645u128.checked_div(0) == None);
/// assert!(256730501423313189585598789281040662945u128.checked_div(0) == None);
/// assert!(333501780472918816281820632831883474505u128.checked_div(0) == None);
/// assert!(215657401814658212020958570428697100455u128.checked_div(0) == None);
/// assert!(188701149141062770191057555406222408826u128.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_u128_checked_div() {}
/// # Properties for [`core::ops::Mul::<u128>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x * y == u128::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 * 0u128 == 0u128);
/// assert!(0u128 * 1u128 == 0u128);
/// assert!(0u128 * 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(0u128 * 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(1u128 * 0u128 == 0u128);
/// assert!(1u128 * 1u128 == 1u128);
/// assert!(1u128 * 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(1u128 * 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128 * 0u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 * 1u128
///         == 340282366920938463463374607431768211454u128);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() > u128::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 *
///         340282366920938463463374607431768211454u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 *
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211455u128 *
///         340282366920938463463374607431768211454u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211455u128 *
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(
///         149882902662976620118934916038003544342u128 *
///         202162041479609955244909838311184933383u128
///     ));
/// assert!(panics!(
///         55693163520385028171970434699325124475u128 *
///         101265277892061853406452691877378752249u128
///     ));
/// assert!(panics!(
///         102271793130338769973510625094578951014u128 *
///         337580251420691892141603570897471097708u128
///     ));
/// assert!(panics!(
///         269075652797081534457269180439538201531u128 *
///         272201855826873006881195630005864681011u128
///     ));
/// assert!(panics!(
///         6860403187591554593301922829284711715u128 *
///         121218187544161630917260768366478285837u128
///     ));
/// assert!(panics!(
///         191361031229009198443305008132439428304u128 *
///         4862618298087658813161706953119861432u128
///     ));
/// # }
/// ```
pub fn core_ops_mul_u128_mul() {}
/// # Properties for [`u128::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(u128::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_mul(0u128) == Some(0u128));
/// assert!(0u128.checked_mul(1u128) == Some(0u128));
/// assert!(0u128.checked_mul(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(0u128.checked_mul(340282366920938463463374607431768211455u128) == Some(0u128));
/// assert!(1u128.checked_mul(0u128) == Some(0u128));
/// assert!(1u128.checked_mul(1u128) == Some(1u128));
/// assert!(1u128.checked_mul(340282366920938463463374607431768211454u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(1u128.checked_mul(340282366920938463463374607431768211455u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211454u128.checked_mul(0u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128.checked_mul(1u128)
///         == Some(340282366920938463463374607431768211454u128));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128
///         .checked_mul(340282366920938463463374607431768211454u128) == None);
/// assert!(340282366920938463463374607431768211454u128
///         .checked_mul(340282366920938463463374607431768211455u128) == None);
/// assert!(340282366920938463463374607431768211455u128
///         .checked_mul(340282366920938463463374607431768211454u128) == None);
/// assert!(340282366920938463463374607431768211455u128
///         .checked_mul(340282366920938463463374607431768211455u128) == None);
/// assert!(83986192731364967255371384906620915430u128
///         .checked_mul(301005379867646432223940549653217006801u128) == None);
/// assert!(132145123220500236102832802004142857118u128
///         .checked_mul(290781247035955971507367219360115235397u128) == None);
/// assert!(47375551732540594898423331432896111836u128
///         .checked_mul(286445005567543643628770389355447743469u128) == None);
/// assert!(76171942705243134807864649500317153664u128
///         .checked_mul(248541258149928644783175960497109108923u128) == None);
/// assert!(201082694704065250270018979038698875697u128
///         .checked_mul(224701682906551679448413188341462427813u128) == None);
/// assert!(143369544602616970119164156220757045201u128
///         .checked_mul(259614584095699209779450772168161701028u128) == None);
/// # }
/// ```
pub fn core_ops_mul_u128_checked_mul() {}
/// # Properties for [`u128::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (u128::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.overflowing_mul(0u128) == (0u128, false));
/// assert!(0u128.overflowing_mul(1u128) == (0u128, false));
/// assert!(0u128.overflowing_mul(340282366920938463463374607431768211454u128) == (0u128, false));
/// assert!(0u128.overflowing_mul(340282366920938463463374607431768211455u128) == (0u128, false));
/// assert!(1u128.overflowing_mul(0u128) == (0u128, false));
/// assert!(1u128.overflowing_mul(1u128) == (1u128, false));
/// assert!(1u128.overflowing_mul(340282366920938463463374607431768211454u128)
///         == (340282366920938463463374607431768211454u128, false));
/// assert!(1u128.overflowing_mul(340282366920938463463374607431768211455u128)
///         == (340282366920938463463374607431768211455u128, false));
/// assert!(340282366920938463463374607431768211454u128.overflowing_mul(0u128) == (0u128, false));
/// assert!(340282366920938463463374607431768211454u128.overflowing_mul(1u128)
///         == (340282366920938463463374607431768211454u128, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128
///         .overflowing_mul(340282366920938463463374607431768211454u128) == (4u128, true));
/// assert!(340282366920938463463374607431768211454u128
///         .overflowing_mul(340282366920938463463374607431768211455u128) == (2u128, true));
/// assert!(340282366920938463463374607431768211455u128
///         .overflowing_mul(340282366920938463463374607431768211454u128) == (2u128, true));
/// assert!(340282366920938463463374607431768211455u128
///         .overflowing_mul(340282366920938463463374607431768211455u128) == (1u128, true));
/// assert!(202710526776821612376651727775976685436u128
///         .overflowing_mul(227281777036005480806578155919308562005u128)
///         == (336018835685535145147718485550848465964u128, true));
/// assert!(235887550856188465760845010052991805410u128
///         .overflowing_mul(155887035455639591643352860270705432704u128)
///         == (206480876155268889080486040635125440768u128, true));
/// assert!(328778598577936994384530909892157458448u128
///         .overflowing_mul(292536395194228716379804433946141806095u128)
///         == (214267568523147797115378604214973372656u128, true));
/// assert!(257901712983714713377055892131732080035u128
///         .overflowing_mul(329082346798086208949674514913657046958u128)
///         == (252161300280159637841622686788157082058u128, true));
/// assert!(317587301117762793646815636661184739267u128
///         .overflowing_mul(118769976190303526086643160229839482949u128)
///         == (110291638881610622275550017011903731599u128, true));
/// assert!(22556446852605643382733447774530207754u128
///         .overflowing_mul(18803453696967372894246851272535082844u128)
///         == (3883829826868789426400557085170289048u128, true));
/// # }
/// ```
pub fn core_ops_mul_u128_overflowing_mul() {}
/// # Properties for [`u128::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == u128::down((x.up() * y.up()).min(u128::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_mul(0u128) == 0u128);
/// assert!(0u128.saturating_mul(1u128) == 0u128);
/// assert!(0u128.saturating_mul(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(0u128.saturating_mul(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(1u128.saturating_mul(0u128) == 0u128);
/// assert!(1u128.saturating_mul(1u128) == 1u128);
/// assert!(1u128.saturating_mul(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(1u128.saturating_mul(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_mul(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_mul(1u128)
///         == 340282366920938463463374607431768211454u128);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u128::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_mul(0u128) == 0u128);
/// assert!(0u128.saturating_mul(1u128) == 0u128);
/// assert!(0u128.saturating_mul(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(0u128.saturating_mul(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(1u128.saturating_mul(0u128) == 0u128);
/// assert!(1u128.saturating_mul(1u128) == 1u128);
/// assert!(1u128.saturating_mul(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(1u128.saturating_mul(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_mul(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_mul(1u128)
///         == 340282366920938463463374607431768211454u128);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == u128::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_mul(340282366920938463463374607431768211454u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_mul(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_mul(340282366920938463463374607431768211454u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_mul(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(108865626697870544462235462392014442503u128
///         .saturating_mul(160484015167206425022002229947564915195u128) == u128::MAX);
/// assert!(212533656292065046214497212819649984684u128
///         .saturating_mul(192219731879265760054573880588872201115u128) == u128::MAX);
/// assert!(52191727681435494111745893055904531999u128
///         .saturating_mul(117907586350458190654469537484173366041u128) == u128::MAX);
/// assert!(15652500917196209974053586821863517998u128
///         .saturating_mul(172438897800237890023417068549656843125u128) == u128::MAX);
/// assert!(313204301664208190853155861414278512503u128
///         .saturating_mul(121072400299345306267381664062372119240u128) == u128::MAX);
/// assert!(216131861407551197989928883267478851534u128
///         .saturating_mul(130784265296077228702064419752487172934u128) == u128::MAX);
/// # }
/// ```
pub fn core_ops_mul_u128_saturating_mul() {}
/// # Properties for [`u128::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == u128::down((x.up() * y.up()) % (u128::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.wrapping_mul(0u128) == 0u128);
/// assert!(0u128.wrapping_mul(1u128) == 0u128);
/// assert!(0u128.wrapping_mul(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(0u128.wrapping_mul(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(1u128.wrapping_mul(0u128) == 0u128);
/// assert!(1u128.wrapping_mul(1u128) == 1u128);
/// assert!(1u128.wrapping_mul(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(1u128.wrapping_mul(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_mul(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_mul(1u128)
///         == 340282366920938463463374607431768211454u128);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() * y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == u128::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.wrapping_mul(0u128) == 0u128);
/// assert!(0u128.wrapping_mul(1u128) == 0u128);
/// assert!(0u128.wrapping_mul(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(0u128.wrapping_mul(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(1u128.wrapping_mul(0u128) == 0u128);
/// assert!(1u128.wrapping_mul(1u128) == 1u128);
/// assert!(1u128.wrapping_mul(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(1u128.wrapping_mul(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_mul(0u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_mul(1u128)
///         == 340282366920938463463374607431768211454u128);
/// # }
/// ```
pub fn core_ops_mul_u128_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<u128>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == u128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 - 0u128 == 0u128);
/// assert!(1u128 - 0u128 == 1u128);
/// assert!(1u128 - 1u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 - 0u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128 - 1u128
///         == 340282366920938463463374607431768211453u128);
/// assert!(340282366920938463463374607431768211454u128
///         - 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(340282366920938463463374607431768211455u128 - 0u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128 - 1u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211455u128
///         - 340282366920938463463374607431768211454u128 == 1u128);
/// assert!(340282366920938463463374607431768211455u128
///         - 340282366920938463463374607431768211455u128 == 0u128);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0u128 - 1u128));
/// assert!(panics!(0u128 - 340282366920938463463374607431768211454u128));
/// assert!(panics!(0u128 - 340282366920938463463374607431768211455u128));
/// assert!(panics!(1u128 - 340282366920938463463374607431768211454u128));
/// assert!(panics!(1u128 - 340282366920938463463374607431768211455u128));
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 -
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(
///         173335442983551848208445544467824981722u128 -
///         193194369060163904238092231996371934706u128
///     ));
/// assert!(panics!(
///         163329005353319356645062878468245074642u128 -
///         306640706726578553359129613804725384215u128
///     ));
/// assert!(panics!(
///         187820928096273628407369080600244857906u128 -
///         199237798684811334142674014794721622984u128
///     ));
/// assert!(panics!(
///         39192703242622176188272662953745193189u128 -
///         129848786047303952932728750671888392877u128
///     ));
/// # }
/// ```
pub fn core_ops_add_u128_sub() {}
/// # Properties for [`u128::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.wrapping_sub(0u128) == 0u128);
/// assert!(1u128.wrapping_sub(0u128) == 1u128);
/// assert!(1u128.wrapping_sub(1u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_sub(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_sub(1u128)
///         == 340282366920938463463374607431768211453u128);
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_sub(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_sub(0u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_sub(1u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211455u128
///         .wrapping_sub(340282366920938463463374607431768211454u128) == 1u128);
/// assert!(340282366920938463463374607431768211455u128
///         .wrapping_sub(340282366920938463463374607431768211455u128) == 0u128);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == u128::down(x.up() - y.up() + u128::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.wrapping_sub(1u128) == 340282366920938463463374607431768211455u128);
/// assert!(0u128.wrapping_sub(340282366920938463463374607431768211454u128) == 2u128);
/// assert!(0u128.wrapping_sub(340282366920938463463374607431768211455u128) == 1u128);
/// assert!(1u128.wrapping_sub(340282366920938463463374607431768211454u128) == 3u128);
/// assert!(1u128.wrapping_sub(340282366920938463463374607431768211455u128) == 2u128);
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_sub(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(42022178075940957402418275392870419800u128
///         .wrapping_sub(232521077444784075978496731338327395038u128)
///         == 149783467552095344887296151486311236218u128);
/// assert!(87482341232794209063943036177233957620u128
///         .wrapping_sub(124778826288021286200885310148781106391u128)
///         == 302985881865711386326432333460221062685u128);
/// assert!(102276865207819275345326162923108359491u128
///         .wrapping_sub(216469701706674408096694901239675767204u128)
///         == 226089530422083330712005869115200803743u128);
/// assert!(106571316489685418529281487195160529773u128
///         .wrapping_sub(266985841316245522516728348181861161612u128)
///         == 179867842094378359475927746445067579617u128);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y) == u128::down((x.up() - y.up()).rem_euclid(&(u128::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.wrapping_sub(0u128) == 0u128);
/// assert!(0u128.wrapping_sub(1u128) == 340282366920938463463374607431768211455u128);
/// assert!(0u128.wrapping_sub(340282366920938463463374607431768211454u128) == 2u128);
/// assert!(0u128.wrapping_sub(340282366920938463463374607431768211455u128) == 1u128);
/// assert!(1u128.wrapping_sub(0u128) == 1u128);
/// assert!(1u128.wrapping_sub(1u128) == 0u128);
/// assert!(1u128.wrapping_sub(340282366920938463463374607431768211454u128) == 3u128);
/// assert!(1u128.wrapping_sub(340282366920938463463374607431768211455u128) == 2u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_sub(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_sub(1u128)
///         == 340282366920938463463374607431768211453u128);
/// # }
/// ```
pub fn core_ops_add_u128_wrapping_sub() {}
/// # Properties for [`u128::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(u128::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_sub(0u128) == Some(0u128));
/// assert!(1u128.checked_sub(0u128) == Some(1u128));
/// assert!(1u128.checked_sub(1u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211454u128.checked_sub(0u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211454u128.checked_sub(1u128)
///         == Some(340282366920938463463374607431768211453u128));
/// assert!(340282366920938463463374607431768211454u128
///         .checked_sub(340282366920938463463374607431768211454u128) == Some(0u128));
/// assert!(340282366920938463463374607431768211455u128.checked_sub(0u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211455u128.checked_sub(1u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211455u128
///         .checked_sub(340282366920938463463374607431768211454u128) == Some(1u128));
/// assert!(340282366920938463463374607431768211455u128
///         .checked_sub(340282366920938463463374607431768211455u128) == Some(0u128));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_sub(1u128) == None);
/// assert!(0u128.checked_sub(340282366920938463463374607431768211454u128) == None);
/// assert!(0u128.checked_sub(340282366920938463463374607431768211455u128) == None);
/// assert!(1u128.checked_sub(340282366920938463463374607431768211454u128) == None);
/// assert!(1u128.checked_sub(340282366920938463463374607431768211455u128) == None);
/// assert!(340282366920938463463374607431768211454u128
///         .checked_sub(340282366920938463463374607431768211455u128) == None);
/// assert!(223084850748120244217618515063575618850u128
///         .checked_sub(257524323586146255696481756623772859313u128) == None);
/// assert!(259318522211408901781426562252083273483u128
///         .checked_sub(262589657307377944831789548959922001341u128) == None);
/// assert!(62403035161045869518487771757073272554u128
///         .checked_sub(68256595863038872644294905268032407130u128) == None);
/// assert!(119167304649820985459714122541467969180u128
///         .checked_sub(167979304337648529982724010353284516089u128) == None);
/// # }
/// ```
pub fn core_ops_add_u128_checked_sub() {}
/// # Properties for [`u128::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == u128::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_sub(0u128) == 0u128);
/// assert!(0u128.saturating_sub(1u128) == 0u128);
/// assert!(0u128.saturating_sub(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(0u128.saturating_sub(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(1u128.saturating_sub(0u128) == 1u128);
/// assert!(1u128.saturating_sub(1u128) == 0u128);
/// assert!(1u128.saturating_sub(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(1u128.saturating_sub(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_sub(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_sub(1u128)
///         == 340282366920938463463374607431768211453u128);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == u128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_sub(0u128) == 0u128);
/// assert!(1u128.saturating_sub(0u128) == 1u128);
/// assert!(1u128.saturating_sub(1u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_sub(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_sub(1u128)
///         == 340282366920938463463374607431768211453u128);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_sub(340282366920938463463374607431768211454u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128.saturating_sub(0u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128.saturating_sub(1u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_sub(340282366920938463463374607431768211454u128) == 1u128);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_sub(340282366920938463463374607431768211455u128) == 0u128);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_sub(1u128) == 0);
/// assert!(0u128.saturating_sub(340282366920938463463374607431768211454u128) == 0);
/// assert!(0u128.saturating_sub(340282366920938463463374607431768211455u128) == 0);
/// assert!(1u128.saturating_sub(340282366920938463463374607431768211454u128) == 0);
/// assert!(1u128.saturating_sub(340282366920938463463374607431768211455u128) == 0);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_sub(340282366920938463463374607431768211455u128) == 0);
/// assert!(308159493222008170212433516071637957991u128
///         .saturating_sub(336912958774975137111016713931993981058u128) == 0);
/// assert!(47047375312546278421773520970915229844u128
///         .saturating_sub(269933110999431756607483129968471252418u128) == 0);
/// assert!(210782027842437879508213648900258579303u128
///         .saturating_sub(282516503624699751817588154051848319960u128) == 0);
/// assert!(32501529328219165988604611100005045380u128
///         .saturating_sub(325604697242344190678852237326038363009u128) == 0);
/// # }
/// ```
pub fn core_ops_sub_u128_saturating_sub() {}
/// # Properties for [`core::ops::Add::<u128>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x + y == u128::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 + 0u128 == 0u128);
/// assert!(0u128 + 1u128 == 1u128);
/// assert!(0u128 + 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128 + 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128 + 0u128 == 1u128);
/// assert!(1u128 + 1u128 == 2u128);
/// assert!(1u128 + 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128 + 0u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128 + 1u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128 + 0u128
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1u128 + 340282366920938463463374607431768211455u128));
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 +
///         340282366920938463463374607431768211454u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211454u128 +
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(340282366920938463463374607431768211455u128 + 1u128));
/// assert!(panics!(
///         340282366920938463463374607431768211455u128 +
///         340282366920938463463374607431768211454u128
///     ));
/// assert!(panics!(
///         340282366920938463463374607431768211455u128 +
///         340282366920938463463374607431768211455u128
///     ));
/// assert!(panics!(
///         210109500062799975914838841707932531360u128 +
///         132805297844034673012835213123165235283u128
///     ));
/// assert!(panics!(
///         292527141025726170629434457517486133577u128 +
///         62549593203414806864999629999994590993u128
///     ));
/// assert!(panics!(
///         272189991663585483139520058414575216129u128 +
///         201934207247652074614492561532758375683u128
///     ));
/// assert!(panics!(
///         331838018723677017318974617936574615496u128 +
///         137934579005817069594299220790226433887u128
///     ));
/// # }
/// ```
pub fn core_ops_add_u128_add() {}
/// # Properties for [`u128::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == u128::down((x.up() + y.up()) % (u128::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.wrapping_add(0u128) == 0u128);
/// assert!(0u128.wrapping_add(1u128) == 1u128);
/// assert!(0u128.wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128.wrapping_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128.wrapping_add(0u128) == 1u128);
/// assert!(1u128.wrapping_add(1u128) == 2u128);
/// assert!(1u128.wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128.wrapping_add(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(1u128)
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u128::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.wrapping_add(0u128) == 0u128);
/// assert!(0u128.wrapping_add(1u128) == 1u128);
/// assert!(0u128.wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128.wrapping_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128.wrapping_add(0u128) == 1u128);
/// assert!(1u128.wrapping_add(1u128) == 2u128);
/// assert!(1u128.wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128.wrapping_add(1u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_add(0u128)
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == u128::down(x.up() + y.up() - u128::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128.wrapping_add(340282366920938463463374607431768211455u128) == 0u128);
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211452u128);
/// assert!(340282366920938463463374607431768211454u128
///         .wrapping_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211453u128);
/// assert!(340282366920938463463374607431768211455u128.wrapping_add(1u128) == 0u128);
/// assert!(340282366920938463463374607431768211455u128
///         .wrapping_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211453u128);
/// assert!(340282366920938463463374607431768211455u128
///         .wrapping_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(324095852810019033718900302256043190250u128
///         .wrapping_add(301666563555907743912651462542901642065u128)
///         == 285480049444988314168177157367176620859u128);
/// assert!(140186592719536264851285874341104468584u128
///         .wrapping_add(273652428684196331764332330094788144831u128)
///         == 73556654482794133152243597004124401959u128);
/// assert!(339901878841048909633284707826296378603u128
///         .wrapping_add(55346376919398270829707803157535194023u128)
///         == 54965888839508716999617903552063361170u128);
/// assert!(259399338196997942562433023447315451043u128
///         .wrapping_add(314051097974103186099354107262425275672u128)
///         == 233168069250162665198412523277972515259u128);
/// # }
/// ```
pub fn core_ops_add_u128_wrapping_add() {}
/// # Properties for [`u128::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(u128::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.checked_add(0u128) == Some(0u128));
/// assert!(0u128.checked_add(1u128) == Some(1u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211454u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(0u128.checked_add(340282366920938463463374607431768211455u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(1u128.checked_add(0u128) == Some(1u128));
/// assert!(1u128.checked_add(1u128) == Some(2u128));
/// assert!(1u128.checked_add(340282366920938463463374607431768211454u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211454u128.checked_add(0u128)
///         == Some(340282366920938463463374607431768211454u128));
/// assert!(340282366920938463463374607431768211454u128.checked_add(1u128)
///         == Some(340282366920938463463374607431768211455u128));
/// assert!(340282366920938463463374607431768211455u128.checked_add(0u128)
///         == Some(340282366920938463463374607431768211455u128));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128.checked_add(340282366920938463463374607431768211455u128) == None);
/// assert!(340282366920938463463374607431768211454u128
///         .checked_add(340282366920938463463374607431768211454u128) == None);
/// assert!(340282366920938463463374607431768211454u128
///         .checked_add(340282366920938463463374607431768211455u128) == None);
/// assert!(340282366920938463463374607431768211455u128.checked_add(1u128) == None);
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(340282366920938463463374607431768211454u128) == None);
/// assert!(340282366920938463463374607431768211455u128
///         .checked_add(340282366920938463463374607431768211455u128) == None);
/// assert!(223252351310797899371896571407078852238u128
///         .checked_add(198933678429125037099206705034159756030u128) == None);
/// assert!(256447879005283468649800694756386103934u128
///         .checked_add(141262044775507776552290659380803117353u128) == None);
/// assert!(278533717905506021149274765615925115088u128
///         .checked_add(128624499775899010657088658577178880972u128) == None);
/// assert!(182367566145357583407749308498687777468u128
///         .checked_add(336073980784426541441152610881749450375u128) == None);
/// # }
/// ```
pub fn core_ops_add_u128_checked_add() {}
/// # Properties for [`u128::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == u128::down((x.up() + y.up()).min(u128::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_add(0u128) == 0u128);
/// assert!(0u128.saturating_add(1u128) == 1u128);
/// assert!(0u128.saturating_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128.saturating_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128.saturating_add(0u128) == 1u128);
/// assert!(1u128.saturating_add(1u128) == 2u128);
/// assert!(1u128.saturating_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128.saturating_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_add(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_add(1u128)
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() <= u128::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u128::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.saturating_add(0u128) == 0u128);
/// assert!(0u128.saturating_add(1u128) == 1u128);
/// assert!(0u128.saturating_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128.saturating_add(340282366920938463463374607431768211455u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128.saturating_add(0u128) == 1u128);
/// assert!(1u128.saturating_add(1u128) == 2u128);
/// assert!(1u128.saturating_add(340282366920938463463374607431768211454u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_add(0u128)
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128.saturating_add(1u128)
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211455u128.saturating_add(0u128)
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `x.up() + y.up() > u128::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == u128::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1u128.saturating_add(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_add(340282366920938463463374607431768211454u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211454u128
///         .saturating_add(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211455u128.saturating_add(1u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_add(340282366920938463463374607431768211454u128) == u128::MAX);
/// assert!(340282366920938463463374607431768211455u128
///         .saturating_add(340282366920938463463374607431768211455u128) == u128::MAX);
/// assert!(289264163419910101806803528584714317220u128
///         .saturating_add(300948577925741324217620568742071014268u128) == u128::MAX);
/// assert!(262866761525365484006238052220884026037u128
///         .saturating_add(231671319992634450491778787559928171400u128) == u128::MAX);
/// assert!(303104661170160163368840990481556580352u128
///         .saturating_add(192325378959867363089690288603338439439u128) == u128::MAX);
/// assert!(48909424144176124352012893252146119824u128
///         .saturating_add(300768055918235410871680407688544204375u128) == u128::MAX);
/// # }
/// ```
pub fn core_ops_add_u128_saturating_add() {}
/// # Properties for [`core::ops::Rem::<usize>::rem`]
/// ## Semantics of rem
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x % y == usize::down(x.up() % y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize % 1usize == 0usize);
/// assert!(0usize % 18446744073709551614usize == 0usize);
/// assert!(0usize % 18446744073709551615usize == 0usize);
/// assert!(1usize % 1usize == 0usize);
/// assert!(1usize % 18446744073709551614usize == 1usize);
/// assert!(1usize % 18446744073709551615usize == 1usize);
/// assert!(18446744073709551614usize % 1usize == 0usize);
/// assert!(18446744073709551614usize % 18446744073709551614usize == 0usize);
/// assert!(18446744073709551614usize % 18446744073709551615usize == 18446744073709551614usize);
/// assert!(18446744073709551615usize % 1usize == 0usize);
/// # }
/// ```
/// ## Semantics of rem
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x % 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0usize % 0) } });
/// # }
/// ```
pub fn core_ops_rem_usize_rem() {}
/// # Properties for [`usize::checked_rem`]
/// ## Semantics of checked_rem
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_rem(y) == Some(usize::down(x.up() % y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_rem(1usize) == Some(0usize));
/// assert!(0usize.checked_rem(18446744073709551614usize) == Some(0usize));
/// assert!(0usize.checked_rem(18446744073709551615usize) == Some(0usize));
/// assert!(1usize.checked_rem(1usize) == Some(0usize));
/// assert!(1usize.checked_rem(18446744073709551614usize) == Some(1usize));
/// assert!(1usize.checked_rem(18446744073709551615usize) == Some(1usize));
/// assert!(18446744073709551614usize.checked_rem(1usize) == Some(0usize));
/// assert!(18446744073709551614usize.checked_rem(18446744073709551614usize) == Some(0usize));
/// assert!(18446744073709551614usize.checked_rem(18446744073709551615usize)
///         == Some(18446744073709551614usize));
/// assert!(18446744073709551615usize.checked_rem(1usize) == Some(0usize));
/// # }
/// ```
/// ## Semantics of checked_rem
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { x.checked_rem(0) == None } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { 0usize.checked_rem(0) == None } });
/// # }
/// ```
pub fn core_ops_rem_usize_checked_rem() {}
/// # Properties for [`usize::checked_neg`]
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `x == usize::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(usize::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_neg() == Some(0usize));
/// # }
/// ```
/// ## Semantics of checked neg
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `x != usize::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.checked_neg() == None);
/// assert!(18446744073709551614usize.checked_neg() == None);
/// assert!(18446744073709551615usize.checked_neg() == None);
/// assert!(2804256483167864291usize.checked_neg() == None);
/// assert!(7585775438330668386usize.checked_neg() == None);
/// assert!(11434774584719341435usize.checked_neg() == None);
/// assert!(1489667591549961925usize.checked_neg() == None);
/// assert!(17133221682642207108usize.checked_neg() == None);
/// assert!(4741369397168420480usize.checked_neg() == None);
/// assert!(1621878778272319306usize.checked_neg() == None);
/// # }
/// ```
pub fn t_usize_checked_neg() {}
/// # Properties for [`core::ops::Shl::<usize>::shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x << y == usize::down((x.up() << y) & usize::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize << 0u32 == 0usize);
/// assert!(0usize << 1u32 == 0usize);
/// assert!(1usize << 0u32 == 1usize);
/// assert!(1usize << 1u32 == 2usize);
/// assert!(18446744073709551614usize << 0u32 == 18446744073709551614usize);
/// assert!(18446744073709551614usize << 1u32 == 18446744073709551612usize);
/// assert!(18446744073709551615usize << 0u32 == 18446744073709551615usize);
/// assert!(18446744073709551615usize << 1u32 == 18446744073709551614usize);
/// assert!(4198485379628686927usize << 31u32 == 12085461446746439680usize);
/// assert!(4496529932537521230usize << 57u32 == 11240984669916758016usize);
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `panics!(x << y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0usize << 4294967294u32));
/// assert!(panics!(0usize << 4294967295u32));
/// assert!(panics!(1usize << 4294967294u32));
/// assert!(panics!(1usize << 4294967295u32));
/// assert!(panics!(18446744073709551614usize << 4294967294u32));
/// assert!(panics!(18446744073709551614usize << 4294967295u32));
/// assert!(panics!(18446744073709551615usize << 4294967294u32));
/// assert!(panics!(18446744073709551615usize << 4294967295u32));
/// assert!(panics!(9373288078329666115usize << 106u32));
/// assert!(panics!(18002805790537060194usize << 80u32));
/// # }
/// ```
pub fn core_ops_shl_usize_shl() {}
/// # Properties for [`core::ops::Shl::<usize>::checked_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == Some(usize::down((x.up() << y) & usize::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_shl(0u32) == Some(0usize));
/// assert!(0usize.checked_shl(1u32) == Some(0usize));
/// assert!(1usize.checked_shl(0u32) == Some(1usize));
/// assert!(1usize.checked_shl(1u32) == Some(2usize));
/// assert!(18446744073709551614usize.checked_shl(0u32) == Some(18446744073709551614usize));
/// assert!(18446744073709551614usize.checked_shl(1u32) == Some(18446744073709551612usize));
/// assert!(18446744073709551615usize.checked_shl(0u32) == Some(18446744073709551615usize));
/// assert!(18446744073709551615usize.checked_shl(1u32) == Some(18446744073709551614usize));
/// assert!(4002743157422775521usize.checked_shl(62u32) == Some(4611686018427387904usize));
/// assert!(4127163248930934875usize.checked_shl(6u32) == Some(5884030899646109376usize));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `x.checked_shl(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_shl(4294967294u32) == None);
/// assert!(0usize.checked_shl(4294967295u32) == None);
/// assert!(1usize.checked_shl(4294967294u32) == None);
/// assert!(1usize.checked_shl(4294967295u32) == None);
/// assert!(18446744073709551614usize.checked_shl(4294967294u32) == None);
/// assert!(18446744073709551614usize.checked_shl(4294967295u32) == None);
/// assert!(18446744073709551615usize.checked_shl(4294967294u32) == None);
/// assert!(18446744073709551615usize.checked_shl(4294967295u32) == None);
/// assert!(17341123163575432600usize.checked_shl(70u32) == None);
/// assert!(3649697719988853221usize.checked_shl(83u32) == None);
/// # }
/// ```
pub fn core_ops_shl_usize_checked_shl() {}
/// # Properties for [`core::ops::Shl::<usize>::overflowing_shl`]
/// ## Semantics of the left shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y) == (usize::down((x.up() << y) & usize::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.overflowing_shl(0u32) == (0usize, false));
/// assert!(0usize.overflowing_shl(1u32) == (0usize, false));
/// assert!(1usize.overflowing_shl(0u32) == (1usize, false));
/// assert!(1usize.overflowing_shl(1u32) == (2usize, false));
/// assert!(18446744073709551614usize.overflowing_shl(0u32) == (18446744073709551614usize, false));
/// assert!(18446744073709551614usize.overflowing_shl(1u32) == (18446744073709551612usize, false));
/// assert!(18446744073709551615usize.overflowing_shl(0u32) == (18446744073709551615usize, false));
/// assert!(18446744073709551615usize.overflowing_shl(1u32) == (18446744073709551614usize, false));
/// assert!(744606618841236746usize.overflowing_shl(1u32) == (1489213237682473492usize, false));
/// assert!(17319225806142551341usize.overflowing_shl(19u32) == (63119928850907136usize, false));
/// # }
/// ```
/// ## Semantics of the left shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `x.overflowing_shl(y)
///         == (usize::down((x.up() << (y & (usize::BITS - 1)) & usize::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.overflowing_shl(4294967294u32) == (0usize, true));
/// assert!(0usize.overflowing_shl(4294967295u32) == (0usize, true));
/// assert!(1usize.overflowing_shl(4294967294u32) == (4611686018427387904usize, true));
/// assert!(1usize.overflowing_shl(4294967295u32) == (9223372036854775808usize, true));
/// assert!(18446744073709551614usize.overflowing_shl(4294967294u32)
///         == (9223372036854775808usize, true));
/// assert!(18446744073709551614usize.overflowing_shl(4294967295u32) == (0usize, true));
/// assert!(18446744073709551615usize.overflowing_shl(4294967294u32)
///         == (13835058055282163712usize, true));
/// assert!(18446744073709551615usize.overflowing_shl(4294967295u32)
///         == (9223372036854775808usize, true));
/// assert!(16723328203647058273usize.overflowing_shl(115u32) == (794885334230892544usize, true));
/// assert!(13030765853001403397usize.overflowing_shl(97u32) == (13177699678403166208usize, true));
/// # }
/// ```
pub fn core_ops_shl_usize_overflowing_shl() {}
/// # Properties for [`core::ops::Shr::<usize>::shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x >> y == usize::down((x.up() >> y) & usize::MAX.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize >> 0u32 == 0usize);
/// assert!(0usize >> 1u32 == 0usize);
/// assert!(1usize >> 0u32 == 1usize);
/// assert!(1usize >> 1u32 == 0usize);
/// assert!(18446744073709551614usize >> 0u32 == 18446744073709551614usize);
/// assert!(18446744073709551614usize >> 1u32 == 9223372036854775807usize);
/// assert!(18446744073709551615usize >> 0u32 == 18446744073709551615usize);
/// assert!(18446744073709551615usize >> 1u32 == 9223372036854775807usize);
/// assert!(8453355734037312202usize >> 24u32 == 503859265687usize);
/// assert!(11794815530677549970usize >> 53u32 == 1309usize);
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `panics!(x >> y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0usize >> 4294967294u32));
/// assert!(panics!(0usize >> 4294967295u32));
/// assert!(panics!(1usize >> 4294967294u32));
/// assert!(panics!(1usize >> 4294967295u32));
/// assert!(panics!(18446744073709551614usize >> 4294967294u32));
/// assert!(panics!(18446744073709551614usize >> 4294967295u32));
/// assert!(panics!(18446744073709551615usize >> 4294967294u32));
/// assert!(panics!(18446744073709551615usize >> 4294967295u32));
/// assert!(panics!(6463195673427014317usize >> 77u32));
/// assert!(panics!(9492791727709979471usize >> 117u32));
/// # }
/// ```
pub fn core_ops_shr_usize_shr() {}
/// # Properties for [`core::ops::Shr::<usize>::checked_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == Some(usize::down((x.up() >> y) & usize::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_shr(0u32) == Some(0usize));
/// assert!(0usize.checked_shr(1u32) == Some(0usize));
/// assert!(1usize.checked_shr(0u32) == Some(1usize));
/// assert!(1usize.checked_shr(1u32) == Some(0usize));
/// assert!(18446744073709551614usize.checked_shr(0u32) == Some(18446744073709551614usize));
/// assert!(18446744073709551614usize.checked_shr(1u32) == Some(9223372036854775807usize));
/// assert!(18446744073709551615usize.checked_shr(0u32) == Some(18446744073709551615usize));
/// assert!(18446744073709551615usize.checked_shr(1u32) == Some(9223372036854775807usize));
/// assert!(7008447291125448975usize.checked_shr(21u32) == Some(3341888089716usize));
/// assert!(14631203246756600273usize.checked_shr(38u32) == Some(53228007usize));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `x.checked_shr(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_shr(4294967294u32) == None);
/// assert!(0usize.checked_shr(4294967295u32) == None);
/// assert!(1usize.checked_shr(4294967294u32) == None);
/// assert!(1usize.checked_shr(4294967295u32) == None);
/// assert!(18446744073709551614usize.checked_shr(4294967294u32) == None);
/// assert!(18446744073709551614usize.checked_shr(4294967295u32) == None);
/// assert!(18446744073709551615usize.checked_shr(4294967294u32) == None);
/// assert!(18446744073709551615usize.checked_shr(4294967295u32) == None);
/// assert!(17424996513998619702usize.checked_shr(117u32) == None);
/// assert!(13788294159629425758usize.checked_shr(72u32) == None);
/// # }
/// ```
pub fn core_ops_shr_usize_checked_shr() {}
/// # Properties for [`core::ops::Shr::<usize>::overflowing_shr`]
/// ## Semantics of the right shift when the number of bits is right
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y < usize::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y) == (usize::down((x.up() >> y) & usize::MAX.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.overflowing_shr(0u32) == (0usize, false));
/// assert!(0usize.overflowing_shr(1u32) == (0usize, false));
/// assert!(1usize.overflowing_shr(0u32) == (1usize, false));
/// assert!(1usize.overflowing_shr(1u32) == (0usize, false));
/// assert!(18446744073709551614usize.overflowing_shr(0u32) == (18446744073709551614usize, false));
/// assert!(18446744073709551614usize.overflowing_shr(1u32) == (9223372036854775807usize, false));
/// assert!(18446744073709551615usize.overflowing_shr(0u32) == (18446744073709551615usize, false));
/// assert!(18446744073709551615usize.overflowing_shr(1u32) == (9223372036854775807usize, false));
/// assert!(6419794158410640699usize.overflowing_shr(19u32) == (12244785610982usize, false));
/// assert!(14032898393155010433usize.overflowing_shr(40u32) == (12762846usize, false));
/// # }
/// ```
/// ## Semantics of the right shift otherwise
/// __Inputs:__ `x : usize, y : u32`  
/// __Precondition:__ `y >= usize::BITS`  
/// __Postcondition:__ `x.overflowing_shr(y)
///         == (usize::down((x.up() >> (y & (usize::BITS - 1)) & usize::MAX.up())), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.overflowing_shr(4294967294u32) == (0usize, true));
/// assert!(0usize.overflowing_shr(4294967295u32) == (0usize, true));
/// assert!(1usize.overflowing_shr(4294967294u32) == (0usize, true));
/// assert!(1usize.overflowing_shr(4294967295u32) == (0usize, true));
/// assert!(18446744073709551614usize.overflowing_shr(4294967294u32) == (3usize, true));
/// assert!(18446744073709551614usize.overflowing_shr(4294967295u32) == (1usize, true));
/// assert!(18446744073709551615usize.overflowing_shr(4294967294u32) == (3usize, true));
/// assert!(18446744073709551615usize.overflowing_shr(4294967295u32) == (1usize, true));
/// assert!(12873940137663621629usize.overflowing_shr(132u32) == (804621258603976351usize, true));
/// assert!(12790231478335222367usize.overflowing_shr(137u32) == (24980920856123481usize, true));
/// # }
/// ```
pub fn core_ops_shr_usize_overflowing_shr() {}
/// # Properties for [`core::ops::Div::<usize>::div`]
/// ## Semantics of the division by non-zero
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x / y == usize::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize / 1usize == 0usize);
/// assert!(0usize / 18446744073709551614usize == 0usize);
/// assert!(0usize / 18446744073709551615usize == 0usize);
/// assert!(1usize / 1usize == 1usize);
/// assert!(1usize / 18446744073709551614usize == 0usize);
/// assert!(1usize / 18446744073709551615usize == 0usize);
/// assert!(18446744073709551614usize / 1usize == 18446744073709551614usize);
/// assert!(18446744073709551614usize / 18446744073709551614usize == 1usize);
/// assert!(18446744073709551614usize / 18446744073709551615usize == 0usize);
/// assert!(18446744073709551615usize / 1usize == 18446744073709551615usize);
/// # }
/// ```
/// ## Semantics of the division by zero
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x / 0) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18446744073709551614usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18446744073709551615usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(7177219903555331539usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(2277125250100354123usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(18244672869508692365usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(10173311464430477581usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(201353183659338415usize / 0) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(10793719294718800765usize / 0) } });
/// # }
/// ```
pub fn core_ops_div_usize_div() {}
/// # Properties for [`usize::saturating_div`]
/// ## Semantics of the saturating division by non-zero
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.saturating_div(y) == usize::down(x.up() / y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_div(1usize) == 0usize);
/// assert!(0usize.saturating_div(18446744073709551614usize) == 0usize);
/// assert!(0usize.saturating_div(18446744073709551615usize) == 0usize);
/// assert!(1usize.saturating_div(1usize) == 1usize);
/// assert!(1usize.saturating_div(18446744073709551614usize) == 0usize);
/// assert!(1usize.saturating_div(18446744073709551615usize) == 0usize);
/// assert!(18446744073709551614usize.saturating_div(1usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.saturating_div(18446744073709551614usize) == 1usize);
/// assert!(18446744073709551614usize.saturating_div(18446744073709551615usize) == 0usize);
/// assert!(18446744073709551615usize.saturating_div(1usize) == 18446744073709551615usize);
/// # }
/// ```
/// ## Semantics of the saturating division by zero
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `{ #[allow(unconditional_panic)] { panics!(x.saturating_div(0)) } }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({ #[allow(unconditional_panic)] { panics!(0usize.saturating_div(0)) } });
/// assert!({ #[allow(unconditional_panic)] { panics!(1usize.saturating_div(0)) } });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(18446744073709551614usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(18446744073709551615usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(17331789349642637581usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(3151356618598880963usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(13425985371626048397usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(4614647675653282158usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(10881480130440921531usize.saturating_div(0)) }
///     });
/// assert!({
///         #[allow(unconditional_panic)]
///         { panics!(7984508094687010905usize.saturating_div(0)) }
///     });
/// # }
/// ```
pub fn core_ops_div_usize_saturating_div() {}
/// # Properties for [`usize::checked_div`]
/// ## Semantics of the checked division by non-zero
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `y != 0`  
/// __Postcondition:__ `x.checked_div(y) == Some(usize::down(x.up() / y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_div(1usize) == Some(0usize));
/// assert!(0usize.checked_div(18446744073709551614usize) == Some(0usize));
/// assert!(0usize.checked_div(18446744073709551615usize) == Some(0usize));
/// assert!(1usize.checked_div(1usize) == Some(1usize));
/// assert!(1usize.checked_div(18446744073709551614usize) == Some(0usize));
/// assert!(1usize.checked_div(18446744073709551615usize) == Some(0usize));
/// assert!(18446744073709551614usize.checked_div(1usize) == Some(18446744073709551614usize));
/// assert!(18446744073709551614usize.checked_div(18446744073709551614usize) == Some(1usize));
/// assert!(18446744073709551614usize.checked_div(18446744073709551615usize) == Some(0usize));
/// assert!(18446744073709551615usize.checked_div(1usize) == Some(18446744073709551615usize));
/// # }
/// ```
/// ## Semantics of the checked division by zero
/// __Inputs:__ `x : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.checked_div(0) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_div(0) == None);
/// assert!(1usize.checked_div(0) == None);
/// assert!(18446744073709551614usize.checked_div(0) == None);
/// assert!(18446744073709551615usize.checked_div(0) == None);
/// assert!(8382052278122782507usize.checked_div(0) == None);
/// assert!(6068471821273889991usize.checked_div(0) == None);
/// assert!(14906704407886571591usize.checked_div(0) == None);
/// assert!(4727273091749629466usize.checked_div(0) == None);
/// assert!(2496998586430605285usize.checked_div(0) == None);
/// assert!(9906709569436531661usize.checked_div(0) == None);
/// # }
/// ```
pub fn core_ops_div_usize_checked_div() {}
/// # Properties for [`core::ops::Mul::<usize>::mul`]
/// ## Semantics of non-overflowing multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x * y == usize::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize * 0usize == 0usize);
/// assert!(0usize * 1usize == 0usize);
/// assert!(0usize * 18446744073709551614usize == 0usize);
/// assert!(0usize * 18446744073709551615usize == 0usize);
/// assert!(1usize * 0usize == 0usize);
/// assert!(1usize * 1usize == 1usize);
/// assert!(1usize * 18446744073709551614usize == 18446744073709551614usize);
/// assert!(1usize * 18446744073709551615usize == 18446744073709551615usize);
/// assert!(18446744073709551614usize * 0usize == 0usize);
/// assert!(18446744073709551614usize * 1usize == 18446744073709551614usize);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() > usize::MAX.up()`  
/// __Postcondition:__ `panics!(x * y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(18446744073709551614usize * 18446744073709551614usize));
/// assert!(panics!(18446744073709551614usize * 18446744073709551615usize));
/// assert!(panics!(18446744073709551615usize * 18446744073709551614usize));
/// assert!(panics!(18446744073709551615usize * 18446744073709551615usize));
/// assert!(panics!(17903077435512809579usize * 8835285998654674202usize));
/// assert!(panics!(15930747705749709487usize * 1438359304808892483usize));
/// assert!(panics!(9460921690118579475usize * 273936620495114075usize));
/// assert!(panics!(8563712348939009824usize * 13696491322304248119usize));
/// assert!(panics!(3156708032895011001usize * 10210174684726516227usize));
/// assert!(panics!(6475317113245190952usize * 18032020102732042440usize));
/// # }
/// ```
pub fn core_ops_mul_usize_mul() {}
/// # Properties for [`usize::checked_mul`]
/// ## Semantics of the non-overflowing checked multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == Some(usize::down(x.up() * y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_mul(0usize) == Some(0usize));
/// assert!(0usize.checked_mul(1usize) == Some(0usize));
/// assert!(0usize.checked_mul(18446744073709551614usize) == Some(0usize));
/// assert!(0usize.checked_mul(18446744073709551615usize) == Some(0usize));
/// assert!(1usize.checked_mul(0usize) == Some(0usize));
/// assert!(1usize.checked_mul(1usize) == Some(1usize));
/// assert!(1usize.checked_mul(18446744073709551614usize) == Some(18446744073709551614usize));
/// assert!(1usize.checked_mul(18446744073709551615usize) == Some(18446744073709551615usize));
/// assert!(18446744073709551614usize.checked_mul(0usize) == Some(0usize));
/// assert!(18446744073709551614usize.checked_mul(1usize) == Some(18446744073709551614usize));
/// # }
/// ```
/// ## Semantics of the overflowing checked multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.checked_mul(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.checked_mul(18446744073709551614usize) == None);
/// assert!(18446744073709551614usize.checked_mul(18446744073709551615usize) == None);
/// assert!(18446744073709551615usize.checked_mul(18446744073709551614usize) == None);
/// assert!(18446744073709551615usize.checked_mul(18446744073709551615usize) == None);
/// assert!(13402162054652777749usize.checked_mul(16719724810344989561usize) == None);
/// assert!(17501871418825431508usize.checked_mul(2598871988993873506usize) == None);
/// assert!(4111730177039454392usize.checked_mul(9019577160733622203usize) == None);
/// assert!(15104279912333314477usize.checked_mul(18396173204373620125usize) == None);
/// assert!(17505153662600109477usize.checked_mul(7507461942573249606usize) == None);
/// assert!(14050982641261068715usize.checked_mul(8162696559809190175usize) == None);
/// # }
/// ```
pub fn core_ops_mul_usize_checked_mul() {}
/// # Properties for [`usize::overflowing_mul`]
/// ## Semantics of the overflowing multiplication when in bounds
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (usize::down(x.up() * y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.overflowing_mul(0usize) == (0usize, false));
/// assert!(0usize.overflowing_mul(1usize) == (0usize, false));
/// assert!(0usize.overflowing_mul(18446744073709551614usize) == (0usize, false));
/// assert!(0usize.overflowing_mul(18446744073709551615usize) == (0usize, false));
/// assert!(1usize.overflowing_mul(0usize) == (0usize, false));
/// assert!(1usize.overflowing_mul(1usize) == (1usize, false));
/// assert!(1usize.overflowing_mul(18446744073709551614usize)
///         == (18446744073709551614usize, false));
/// assert!(1usize.overflowing_mul(18446744073709551615usize)
///         == (18446744073709551615usize, false));
/// assert!(18446744073709551614usize.overflowing_mul(0usize) == (0usize, false));
/// assert!(18446744073709551614usize.overflowing_mul(1usize)
///         == (18446744073709551614usize, false));
/// # }
/// ```
/// ## Semantics of the overflowing multiplication when out of bounds
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.overflowing_mul(y) == (x.wrapping_mul(y), true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.overflowing_mul(18446744073709551614usize)
///         == (4usize, true));
/// assert!(18446744073709551614usize.overflowing_mul(18446744073709551615usize)
///         == (2usize, true));
/// assert!(18446744073709551615usize.overflowing_mul(18446744073709551614usize)
///         == (2usize, true));
/// assert!(18446744073709551615usize.overflowing_mul(18446744073709551615usize)
///         == (1usize, true));
/// assert!(10641328364506949378usize.overflowing_mul(13378019540258322952usize)
///         == (5807600512308947984usize, true));
/// assert!(10052814731475236489usize.overflowing_mul(16765056481422568357usize)
///         == (12421612466519959885usize, true));
/// assert!(15464872804716072760usize.overflowing_mul(4129458824260819679usize)
///         == (1016563076987990472usize, true));
/// assert!(3969558328221247719usize.overflowing_mul(9613602193248531277usize)
///         == (9503267555373077115usize, true));
/// assert!(8686025737837853553usize.overflowing_mul(1124133276319501778usize)
///         == (1764071868219047858usize, true));
/// assert!(17244477822553217784usize.overflowing_mul(3697603314042137741usize)
///         == (17764014115091069592usize, true));
/// # }
/// ```
pub fn core_ops_mul_usize_overflowing_mul() {}
/// # Properties for [`usize::saturating_mul`]
/// ## Semantics of the saturating multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_mul(y) == usize::down((x.up() * y.up()).min(usize::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_mul(0usize) == 0usize);
/// assert!(0usize.saturating_mul(1usize) == 0usize);
/// assert!(0usize.saturating_mul(18446744073709551614usize) == 0usize);
/// assert!(0usize.saturating_mul(18446744073709551615usize) == 0usize);
/// assert!(1usize.saturating_mul(0usize) == 0usize);
/// assert!(1usize.saturating_mul(1usize) == 1usize);
/// assert!(1usize.saturating_mul(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(1usize.saturating_mul(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.saturating_mul(0usize) == 0usize);
/// assert!(18446744073709551614usize.saturating_mul(1usize) == 18446744073709551614usize);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == usize::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_mul(0usize) == 0usize);
/// assert!(0usize.saturating_mul(1usize) == 0usize);
/// assert!(0usize.saturating_mul(18446744073709551614usize) == 0usize);
/// assert!(0usize.saturating_mul(18446744073709551615usize) == 0usize);
/// assert!(1usize.saturating_mul(0usize) == 0usize);
/// assert!(1usize.saturating_mul(1usize) == 1usize);
/// assert!(1usize.saturating_mul(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(1usize.saturating_mul(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.saturating_mul(0usize) == 0usize);
/// assert!(18446744073709551614usize.saturating_mul(1usize) == 18446744073709551614usize);
/// # }
/// ```
/// ## Semantics of the overflowing saturating multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.saturating_mul(y) == usize::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(18446744073709551614usize.saturating_mul(18446744073709551614usize) == usize::MAX);
/// assert!(18446744073709551614usize.saturating_mul(18446744073709551615usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_mul(18446744073709551614usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_mul(18446744073709551615usize) == usize::MAX);
/// assert!(12315406650880985854usize.saturating_mul(6190585726808029610usize) == usize::MAX);
/// assert!(2926452463504444294usize.saturating_mul(7784750665228454853usize) == usize::MAX);
/// assert!(9184578433191893884usize.saturating_mul(7925558975310578731usize) == usize::MAX);
/// assert!(5454875755313451192usize.saturating_mul(9569893605901549833usize) == usize::MAX);
/// assert!(10970357770137900327usize.saturating_mul(15794924281252106270usize) == usize::MAX);
/// assert!(8293222785003879485usize.saturating_mul(1452293063264026810usize) == usize::MAX);
/// # }
/// ```
pub fn core_ops_mul_usize_saturating_mul() {}
/// # Properties for [`usize::wrapping_mul`]
/// ## Semantics of the wrapping multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_mul(y) == usize::down((x.up() * y.up()) % (usize::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_mul(0usize) == 0usize);
/// assert!(0usize.wrapping_mul(1usize) == 0usize);
/// assert!(0usize.wrapping_mul(18446744073709551614usize) == 0usize);
/// assert!(0usize.wrapping_mul(18446744073709551615usize) == 0usize);
/// assert!(1usize.wrapping_mul(0usize) == 0usize);
/// assert!(1usize.wrapping_mul(1usize) == 1usize);
/// assert!(1usize.wrapping_mul(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(1usize.wrapping_mul(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.wrapping_mul(0usize) == 0usize);
/// assert!(18446744073709551614usize.wrapping_mul(1usize) == 18446744073709551614usize);
/// # }
/// ```
/// ## Semantics of the non-overflowing wrapping multiplication
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() * y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.wrapping_mul(y) == usize::down(x.up() * y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_mul(0usize) == 0usize);
/// assert!(0usize.wrapping_mul(1usize) == 0usize);
/// assert!(0usize.wrapping_mul(18446744073709551614usize) == 0usize);
/// assert!(0usize.wrapping_mul(18446744073709551615usize) == 0usize);
/// assert!(1usize.wrapping_mul(0usize) == 0usize);
/// assert!(1usize.wrapping_mul(1usize) == 1usize);
/// assert!(1usize.wrapping_mul(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(1usize.wrapping_mul(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.wrapping_mul(0usize) == 0usize);
/// assert!(18446744073709551614usize.wrapping_mul(1usize) == 18446744073709551614usize);
/// # }
/// ```
pub fn core_ops_mul_usize_wrapped_mul() {}
/// # Properties for [`core::ops::Sub::<usize>::sub`]
/// ## Semantics of non-underflowing subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x - y == usize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize - 0usize == 0usize);
/// assert!(1usize - 0usize == 1usize);
/// assert!(1usize - 1usize == 0usize);
/// assert!(18446744073709551614usize - 0usize == 18446744073709551614usize);
/// assert!(18446744073709551614usize - 1usize == 18446744073709551613usize);
/// assert!(18446744073709551614usize - 18446744073709551614usize == 0usize);
/// assert!(18446744073709551615usize - 0usize == 18446744073709551615usize);
/// assert!(18446744073709551615usize - 1usize == 18446744073709551614usize);
/// assert!(18446744073709551615usize - 18446744073709551614usize == 1usize);
/// assert!(18446744073709551615usize - 18446744073709551615usize == 0usize);
/// # }
/// ```
/// ## Panics when underflowing
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(0usize - 1usize));
/// assert!(panics!(0usize - 18446744073709551614usize));
/// assert!(panics!(0usize - 18446744073709551615usize));
/// assert!(panics!(1usize - 18446744073709551614usize));
/// assert!(panics!(1usize - 18446744073709551615usize));
/// assert!(panics!(18446744073709551614usize - 18446744073709551615usize));
/// assert!(panics!(463547751469030975usize - 5020659970757662054usize));
/// assert!(panics!(10909159953310655576usize - 14070458719243159851usize));
/// assert!(panics!(1750214835222116042usize - 3423429989789187965usize));
/// assert!(panics!(1854037786442063656usize - 7696036647940168270usize));
/// # }
/// ```
pub fn core_ops_add_usize_sub() {}
/// # Properties for [`usize::wrapping_sub`]
/// ## Semantics of non-underflowing wrapping subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == usize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_sub(0usize) == 0usize);
/// assert!(1usize.wrapping_sub(0usize) == 1usize);
/// assert!(1usize.wrapping_sub(1usize) == 0usize);
/// assert!(18446744073709551614usize.wrapping_sub(0usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.wrapping_sub(1usize) == 18446744073709551613usize);
/// assert!(18446744073709551614usize.wrapping_sub(18446744073709551614usize) == 0usize);
/// assert!(18446744073709551615usize.wrapping_sub(0usize) == 18446744073709551615usize);
/// assert!(18446744073709551615usize.wrapping_sub(1usize) == 18446744073709551614usize);
/// assert!(18446744073709551615usize.wrapping_sub(18446744073709551614usize) == 1usize);
/// assert!(18446744073709551615usize.wrapping_sub(18446744073709551615usize) == 0usize);
/// # }
/// ```
/// ## Semantics of underflowing wrapping subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == usize::down(x.up() - y.up() + usize::MAX + 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_sub(1usize) == 18446744073709551615usize);
/// assert!(0usize.wrapping_sub(18446744073709551614usize) == 2usize);
/// assert!(0usize.wrapping_sub(18446744073709551615usize) == 1usize);
/// assert!(1usize.wrapping_sub(18446744073709551614usize) == 3usize);
/// assert!(1usize.wrapping_sub(18446744073709551615usize) == 2usize);
/// assert!(18446744073709551614usize.wrapping_sub(18446744073709551615usize)
///         == 18446744073709551615usize);
/// assert!(4475744829315031623usize.wrapping_sub(9411892713793841930usize)
///         == 13510596189230741309usize);
/// assert!(4056473609565881219usize.wrapping_sub(18446001993785668029usize)
///         == 4057215689489764806usize);
/// assert!(4704288311423850653usize.wrapping_sub(13809174016516290320usize)
///         == 9341858368617111949usize);
/// assert!(6841801605528126176usize.wrapping_sub(11070906018381501746usize)
///         == 14217639660856176046usize);
/// # }
/// ```
/// ## Semantics of wrapping subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == usize::down((x.up() - y.up()).rem_euclid(&(usize::MAX.up() + 1)))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_sub(0usize) == 0usize);
/// assert!(0usize.wrapping_sub(1usize) == 18446744073709551615usize);
/// assert!(0usize.wrapping_sub(18446744073709551614usize) == 2usize);
/// assert!(0usize.wrapping_sub(18446744073709551615usize) == 1usize);
/// assert!(1usize.wrapping_sub(0usize) == 1usize);
/// assert!(1usize.wrapping_sub(1usize) == 0usize);
/// assert!(1usize.wrapping_sub(18446744073709551614usize) == 3usize);
/// assert!(1usize.wrapping_sub(18446744073709551615usize) == 2usize);
/// assert!(18446744073709551614usize.wrapping_sub(0usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.wrapping_sub(1usize) == 18446744073709551613usize);
/// # }
/// ```
pub fn core_ops_add_usize_wrapping_sub() {}
/// # Properties for [`usize::checked_sub`]
/// ## Semantics of non-underflowing checked subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(usize::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_sub(0usize) == Some(0usize));
/// assert!(1usize.checked_sub(0usize) == Some(1usize));
/// assert!(1usize.checked_sub(1usize) == Some(0usize));
/// assert!(18446744073709551614usize.checked_sub(0usize) == Some(18446744073709551614usize));
/// assert!(18446744073709551614usize.checked_sub(1usize) == Some(18446744073709551613usize));
/// assert!(18446744073709551614usize.checked_sub(18446744073709551614usize) == Some(0usize));
/// assert!(18446744073709551615usize.checked_sub(0usize) == Some(18446744073709551615usize));
/// assert!(18446744073709551615usize.checked_sub(1usize) == Some(18446744073709551614usize));
/// assert!(18446744073709551615usize.checked_sub(18446744073709551614usize) == Some(1usize));
/// assert!(18446744073709551615usize.checked_sub(18446744073709551615usize) == Some(0usize));
/// # }
/// ```
/// ## Semantics of underflowing checked subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_sub(1usize) == None);
/// assert!(0usize.checked_sub(18446744073709551614usize) == None);
/// assert!(0usize.checked_sub(18446744073709551615usize) == None);
/// assert!(1usize.checked_sub(18446744073709551614usize) == None);
/// assert!(1usize.checked_sub(18446744073709551615usize) == None);
/// assert!(18446744073709551614usize.checked_sub(18446744073709551615usize) == None);
/// assert!(304346397247500505usize.checked_sub(1688419109683707219usize) == None);
/// assert!(9010460634591928414usize.checked_sub(18301544572881018909usize) == None);
/// assert!(110366950250100461usize.checked_sub(11890136455652642831usize) == None);
/// assert!(7629033863907500328usize.checked_sub(8080975769941546249usize) == None);
/// # }
/// ```
pub fn core_ops_add_usize_checked_sub() {}
/// # Properties for [`usize::saturating_sub`]
/// ## Semantics of the saturating subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_sub(y) == usize::down((x.up() - y.up()).max(0u8.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_sub(0usize) == 0usize);
/// assert!(0usize.saturating_sub(1usize) == 0usize);
/// assert!(0usize.saturating_sub(18446744073709551614usize) == 0usize);
/// assert!(0usize.saturating_sub(18446744073709551615usize) == 0usize);
/// assert!(1usize.saturating_sub(0usize) == 1usize);
/// assert!(1usize.saturating_sub(1usize) == 0usize);
/// assert!(1usize.saturating_sub(18446744073709551614usize) == 0usize);
/// assert!(1usize.saturating_sub(18446744073709551615usize) == 0usize);
/// assert!(18446744073709551614usize.saturating_sub(0usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.saturating_sub(1usize) == 18446744073709551613usize);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() >= 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == usize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_sub(0usize) == 0usize);
/// assert!(1usize.saturating_sub(0usize) == 1usize);
/// assert!(1usize.saturating_sub(1usize) == 0usize);
/// assert!(18446744073709551614usize.saturating_sub(0usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.saturating_sub(1usize) == 18446744073709551613usize);
/// assert!(18446744073709551614usize.saturating_sub(18446744073709551614usize) == 0usize);
/// assert!(18446744073709551615usize.saturating_sub(0usize) == 18446744073709551615usize);
/// assert!(18446744073709551615usize.saturating_sub(1usize) == 18446744073709551614usize);
/// assert!(18446744073709551615usize.saturating_sub(18446744073709551614usize) == 1usize);
/// assert!(18446744073709551615usize.saturating_sub(18446744073709551615usize) == 0usize);
/// # }
/// ```
/// ## Semantics of the overflowing saturating subtraction
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() - y.up() < 0u8.up()`  
/// __Postcondition:__ `x.saturating_sub(y) == 0`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_sub(1usize) == 0);
/// assert!(0usize.saturating_sub(18446744073709551614usize) == 0);
/// assert!(0usize.saturating_sub(18446744073709551615usize) == 0);
/// assert!(1usize.saturating_sub(18446744073709551614usize) == 0);
/// assert!(1usize.saturating_sub(18446744073709551615usize) == 0);
/// assert!(18446744073709551614usize.saturating_sub(18446744073709551615usize) == 0);
/// assert!(16582986535031728551usize.saturating_sub(16829941720577328461usize) == 0);
/// assert!(557349064727656407usize.saturating_sub(10562293612504181372usize) == 0);
/// assert!(8775352247881139343usize.saturating_sub(9849558484776537017usize) == 0);
/// assert!(7504766850160443821usize.saturating_sub(13602578406991967439usize) == 0);
/// # }
/// ```
pub fn core_ops_sub_usize_saturating_sub() {}
/// # Properties for [`core::ops::Add::<usize>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x + y == usize::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize + 0usize == 0usize);
/// assert!(0usize + 1usize == 1usize);
/// assert!(0usize + 18446744073709551614usize == 18446744073709551614usize);
/// assert!(0usize + 18446744073709551615usize == 18446744073709551615usize);
/// assert!(1usize + 0usize == 1usize);
/// assert!(1usize + 1usize == 2usize);
/// assert!(1usize + 18446744073709551614usize == 18446744073709551615usize);
/// assert!(18446744073709551614usize + 0usize == 18446744073709551614usize);
/// assert!(18446744073709551614usize + 1usize == 18446744073709551615usize);
/// assert!(18446744073709551615usize + 0usize == 18446744073709551615usize);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() > usize::MAX.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(1usize + 18446744073709551615usize));
/// assert!(panics!(18446744073709551614usize + 18446744073709551614usize));
/// assert!(panics!(18446744073709551614usize + 18446744073709551615usize));
/// assert!(panics!(18446744073709551615usize + 1usize));
/// assert!(panics!(18446744073709551615usize + 18446744073709551614usize));
/// assert!(panics!(18446744073709551615usize + 18446744073709551615usize));
/// assert!(panics!(7985224470225369836usize + 13967774498916397289usize));
/// assert!(panics!(15203682681027028073usize + 15856495408349350213usize));
/// assert!(panics!(10743567839063766489usize + 17018294231191272335usize));
/// assert!(panics!(1120112595967320222usize + 17922677746076571600usize));
/// # }
/// ```
pub fn core_ops_add_usize_add() {}
/// # Properties for [`usize::wrapping_add`]
/// ## Semantics of the wrapping addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.wrapping_add(y) == usize::down((x.up() + y.up()) % (usize::MAX.up() + 1))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_add(0usize) == 0usize);
/// assert!(0usize.wrapping_add(1usize) == 1usize);
/// assert!(0usize.wrapping_add(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(0usize.wrapping_add(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(1usize.wrapping_add(0usize) == 1usize);
/// assert!(1usize.wrapping_add(1usize) == 2usize);
/// assert!(1usize.wrapping_add(18446744073709551614usize) == 18446744073709551615usize);
/// assert!(1usize.wrapping_add(18446744073709551615usize) == 0usize);
/// assert!(18446744073709551614usize.wrapping_add(0usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.wrapping_add(1usize) == 18446744073709551615usize);
/// # }
/// ```
/// ## Semantics of non-overflowing wrapping addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == usize::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.wrapping_add(0usize) == 0usize);
/// assert!(0usize.wrapping_add(1usize) == 1usize);
/// assert!(0usize.wrapping_add(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(0usize.wrapping_add(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(1usize.wrapping_add(0usize) == 1usize);
/// assert!(1usize.wrapping_add(1usize) == 2usize);
/// assert!(1usize.wrapping_add(18446744073709551614usize) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.wrapping_add(0usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.wrapping_add(1usize) == 18446744073709551615usize);
/// assert!(18446744073709551615usize.wrapping_add(0usize) == 18446744073709551615usize);
/// # }
/// ```
/// ## Semantics of the overflowing wrapping addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.wrapping_add(y) == usize::down(x.up() + y.up() - usize::MAX - 1)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.wrapping_add(18446744073709551615usize) == 0usize);
/// assert!(18446744073709551614usize.wrapping_add(18446744073709551614usize)
///         == 18446744073709551612usize);
/// assert!(18446744073709551614usize.wrapping_add(18446744073709551615usize)
///         == 18446744073709551613usize);
/// assert!(18446744073709551615usize.wrapping_add(1usize) == 0usize);
/// assert!(18446744073709551615usize.wrapping_add(18446744073709551614usize)
///         == 18446744073709551613usize);
/// assert!(18446744073709551615usize.wrapping_add(18446744073709551615usize)
///         == 18446744073709551614usize);
/// assert!(13235917199200992099usize.wrapping_add(8339715410169055009usize)
///         == 3128888535660495492usize);
/// assert!(16838437996241321379usize.wrapping_add(12996350293632658652usize)
///         == 11388044216164428415usize);
/// assert!(18090175919516931310usize.wrapping_add(8465335880407505150usize)
///         == 8108767726214884844usize);
/// assert!(9698234789024821577usize.wrapping_add(17388315672386635477usize)
///         == 8639806387701905438usize);
/// # }
/// ```
pub fn core_ops_add_usize_wrapping_add() {}
/// # Properties for [`usize::checked_add`]
/// ## Semantics of non-overflowing checked addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == Some(usize::down(x.up() + y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.checked_add(0usize) == Some(0usize));
/// assert!(0usize.checked_add(1usize) == Some(1usize));
/// assert!(0usize.checked_add(18446744073709551614usize) == Some(18446744073709551614usize));
/// assert!(0usize.checked_add(18446744073709551615usize) == Some(18446744073709551615usize));
/// assert!(1usize.checked_add(0usize) == Some(1usize));
/// assert!(1usize.checked_add(1usize) == Some(2usize));
/// assert!(1usize.checked_add(18446744073709551614usize) == Some(18446744073709551615usize));
/// assert!(18446744073709551614usize.checked_add(0usize) == Some(18446744073709551614usize));
/// assert!(18446744073709551614usize.checked_add(1usize) == Some(18446744073709551615usize));
/// assert!(18446744073709551615usize.checked_add(0usize) == Some(18446744073709551615usize));
/// # }
/// ```
/// ## None when overflowing
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.checked_add(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.checked_add(18446744073709551615usize) == None);
/// assert!(18446744073709551614usize.checked_add(18446744073709551614usize) == None);
/// assert!(18446744073709551614usize.checked_add(18446744073709551615usize) == None);
/// assert!(18446744073709551615usize.checked_add(1usize) == None);
/// assert!(18446744073709551615usize.checked_add(18446744073709551614usize) == None);
/// assert!(18446744073709551615usize.checked_add(18446744073709551615usize) == None);
/// assert!(17406199219358222500usize.checked_add(12057532573598720570usize) == None);
/// assert!(16652478412702907053usize.checked_add(13697805024481624470usize) == None);
/// assert!(14212389963904773965usize.checked_add(15828321440791249597usize) == None);
/// assert!(10305843512787464505usize.checked_add(8254422211664659454usize) == None);
/// # }
/// ```
pub fn core_ops_add_usize_checked_add() {}
/// # Properties for [`usize::saturating_add`]
/// ## Semantics of the saturating addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.saturating_add(y) == usize::down((x.up() + y.up()).min(usize::MAX.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_add(0usize) == 0usize);
/// assert!(0usize.saturating_add(1usize) == 1usize);
/// assert!(0usize.saturating_add(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(0usize.saturating_add(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(1usize.saturating_add(0usize) == 1usize);
/// assert!(1usize.saturating_add(1usize) == 2usize);
/// assert!(1usize.saturating_add(18446744073709551614usize) == 18446744073709551615usize);
/// assert!(1usize.saturating_add(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.saturating_add(0usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.saturating_add(1usize) == 18446744073709551615usize);
/// # }
/// ```
/// ## Semantics of the non-overflowing saturating addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() <= usize::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == usize::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.saturating_add(0usize) == 0usize);
/// assert!(0usize.saturating_add(1usize) == 1usize);
/// assert!(0usize.saturating_add(18446744073709551614usize) == 18446744073709551614usize);
/// assert!(0usize.saturating_add(18446744073709551615usize) == 18446744073709551615usize);
/// assert!(1usize.saturating_add(0usize) == 1usize);
/// assert!(1usize.saturating_add(1usize) == 2usize);
/// assert!(1usize.saturating_add(18446744073709551614usize) == 18446744073709551615usize);
/// assert!(18446744073709551614usize.saturating_add(0usize) == 18446744073709551614usize);
/// assert!(18446744073709551614usize.saturating_add(1usize) == 18446744073709551615usize);
/// assert!(18446744073709551615usize.saturating_add(0usize) == 18446744073709551615usize);
/// # }
/// ```
/// ## Semantics of the overflowing saturating addition
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `x.up() + y.up() > usize::MAX.up()`  
/// __Postcondition:__ `x.saturating_add(y) == usize::MAX`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(1usize.saturating_add(18446744073709551615usize) == usize::MAX);
/// assert!(18446744073709551614usize.saturating_add(18446744073709551614usize) == usize::MAX);
/// assert!(18446744073709551614usize.saturating_add(18446744073709551615usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_add(1usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_add(18446744073709551614usize) == usize::MAX);
/// assert!(18446744073709551615usize.saturating_add(18446744073709551615usize) == usize::MAX);
/// assert!(16974380191412572218usize.saturating_add(3130211604319668332usize) == usize::MAX);
/// assert!(15234787356361186187usize.saturating_add(6916969496393158625usize) == usize::MAX);
/// assert!(18059297269903942636usize.saturating_add(7774443985859826516usize) == usize::MAX);
/// assert!(16273117070416653403usize.saturating_add(17233689937496746826usize) == usize::MAX);
/// # }
/// ```
pub fn core_ops_add_usize_saturating_add() {}
/// # Properties for [`i8::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x != i8::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i8::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-127i8).checked_neg() == Some(127i8));
/// assert!((-1i8).checked_neg() == Some(1i8));
/// assert!(0i8.checked_neg() == Some(0i8));
/// assert!(1i8.checked_neg() == Some((-1i8)));
/// assert!(126i8.checked_neg() == Some((-126i8)));
/// assert!(127i8.checked_neg() == Some((-127i8)));
/// assert!((-111i8).checked_neg() == Some(111i8));
/// assert!((-13i8).checked_neg() == Some(13i8));
/// assert!(86i8.checked_neg() == Some((-86i8)));
/// assert!(55i8.checked_neg() == Some((-55i8)));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x == i8::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).checked_neg() == None);
/// # }
/// ```
pub fn t_i8_checked_neg() {}
/// # Properties for [`i8::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x != i8::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i8::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         (-127i8).neg() == 127i8
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i8).neg() == 1i8
///     });
/// assert!({
///         use std::ops::Neg;
///         0i8.neg() == 0i8
///     });
/// assert!({
///         use std::ops::Neg;
///         1i8.neg() == (-1i8)
///     });
/// assert!({
///         use std::ops::Neg;
///         126i8.neg() == (-126i8)
///     });
/// assert!({
///         use std::ops::Neg;
///         127i8.neg() == (-127i8)
///     });
/// assert!({
///         use std::ops::Neg;
///         25i8.neg() == (-25i8)
///     });
/// assert!({
///         use std::ops::Neg;
///         90i8.neg() == (-90i8)
///     });
/// assert!({
///         use std::ops::Neg;
///         98i8.neg() == (-98i8)
///     });
/// assert!({
///         use std::ops::Neg;
///         32i8.neg() == (-32i8)
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x == i8::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 128i8).neg())
///     });
/// # }
/// ```
pub fn t_i8_neg() {}
/// # Properties for [`i8::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x != i8::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i8::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-127i8).overflowing_neg() == (127i8, false));
/// assert!((-1i8).overflowing_neg() == (1i8, false));
/// assert!(0i8.overflowing_neg() == (0i8, false));
/// assert!(1i8.overflowing_neg() == ((-1i8), false));
/// assert!(126i8.overflowing_neg() == ((-126i8), false));
/// assert!(127i8.overflowing_neg() == ((-127i8), false));
/// assert!(9i8.overflowing_neg() == ((-9i8), false));
/// assert!(60i8.overflowing_neg() == ((-60i8), false));
/// assert!(23i8.overflowing_neg() == ((-23i8), false));
/// assert!(92i8.overflowing_neg() == ((-92i8), false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i8`  
/// __Precondition:__ `x == i8::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i8::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).overflowing_neg() == ((-128i8), true));
/// # }
/// ```
pub fn t_i8_overflowing_neg() {}
/// # Properties for [`i8::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() <= i8::MAX.up() && x.up() - y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x - y == i8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8) - (-128i8) == 0i8);
/// assert!((-128i8) - (-127i8) == (-1i8));
/// assert!((-128i8) - (-1i8) == (-127i8));
/// assert!((-128i8) - 0i8 == (-128i8));
/// assert!((-127i8) - (-128i8) == 1i8);
/// assert!((-127i8) - (-127i8) == 0i8);
/// assert!((-127i8) - (-1i8) == (-126i8));
/// assert!((-127i8) - 0i8 == (-127i8));
/// assert!((-127i8) - 1i8 == (-128i8));
/// assert!((-1i8) - (-128i8) == 127i8);
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() > i8::MAX.up() || x.up() - y.up() < i8::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 128i8) - 1i8));
/// assert!(panics!((- 128i8) - 126i8));
/// assert!(panics!((- 128i8) - 127i8));
/// assert!(panics!((- 127i8) - 126i8));
/// assert!(panics!((- 127i8) - 127i8));
/// assert!(panics!(0i8 - (- 128i8)));
/// assert!(panics!(1i8 - (- 128i8)));
/// assert!(panics!(1i8 - (- 127i8)));
/// assert!(panics!(126i8 - (- 128i8)));
/// assert!(panics!(126i8 - (- 127i8)));
/// # }
/// ```
pub fn core_ops_sub_i8_sub() {}
/// # Properties for [`i8::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() <= i8::MAX.up() && x.up() - y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i8::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).checked_sub((-128i8)) == Some(0i8));
/// assert!((-128i8).checked_sub((-127i8)) == Some((-1i8)));
/// assert!((-128i8).checked_sub((-1i8)) == Some((-127i8)));
/// assert!((-128i8).checked_sub(0i8) == Some((-128i8)));
/// assert!((-127i8).checked_sub((-128i8)) == Some(1i8));
/// assert!((-127i8).checked_sub((-127i8)) == Some(0i8));
/// assert!((-127i8).checked_sub((-1i8)) == Some((-126i8)));
/// assert!((-127i8).checked_sub(0i8) == Some((-127i8)));
/// assert!((-127i8).checked_sub(1i8) == Some((-128i8)));
/// assert!((-1i8).checked_sub((-128i8)) == Some(127i8));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() > i8::MAX.up() || x.up() - y.up() < i8::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).checked_sub(1i8) == None);
/// assert!((-128i8).checked_sub(126i8) == None);
/// assert!((-128i8).checked_sub(127i8) == None);
/// assert!((-127i8).checked_sub(126i8) == None);
/// assert!((-127i8).checked_sub(127i8) == None);
/// assert!(0i8.checked_sub((-128i8)) == None);
/// assert!(1i8.checked_sub((-128i8)) == None);
/// assert!(1i8.checked_sub((-127i8)) == None);
/// assert!(126i8.checked_sub((-128i8)) == None);
/// assert!(126i8.checked_sub((-127i8)) == None);
/// # }
/// ```
pub fn core_ops_sub_i8_checked_sub() {}
/// # Properties for [`i8::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() <= i8::MAX.up() && x.up() - y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).wrapping_sub((-128i8)) == 0i8);
/// assert!((-128i8).wrapping_sub((-127i8)) == (-1i8));
/// assert!((-128i8).wrapping_sub((-1i8)) == (-127i8));
/// assert!((-128i8).wrapping_sub(0i8) == (-128i8));
/// assert!((-127i8).wrapping_sub((-128i8)) == 1i8);
/// assert!((-127i8).wrapping_sub((-127i8)) == 0i8);
/// assert!((-127i8).wrapping_sub((-1i8)) == (-126i8));
/// assert!((-127i8).wrapping_sub(0i8) == (-127i8));
/// assert!((-127i8).wrapping_sub(1i8) == (-128i8));
/// assert!((-1i8).wrapping_sub((-128i8)) == 127i8);
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() > i8::MAX.up() || x.up() - y.up() < i8::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i8::down(
///             (-i8::MIN.up() + x.up() - y.up()).rem_euclid(&(-i8::MIN.up() * 2))
///                 + i8::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).wrapping_sub(1i8) == 127i8);
/// assert!((-128i8).wrapping_sub(126i8) == 2i8);
/// assert!((-128i8).wrapping_sub(127i8) == 1i8);
/// assert!((-127i8).wrapping_sub(126i8) == 3i8);
/// assert!((-127i8).wrapping_sub(127i8) == 2i8);
/// assert!(0i8.wrapping_sub((-128i8)) == (-128i8));
/// assert!(1i8.wrapping_sub((-128i8)) == (-127i8));
/// assert!(1i8.wrapping_sub((-127i8)) == (-128i8));
/// assert!(126i8.wrapping_sub((-128i8)) == (-2i8));
/// assert!(126i8.wrapping_sub((-127i8)) == (-3i8));
/// # }
/// ```
pub fn core_ops_sub_i8_wrapping_sub() {}
/// # Properties for [`i8::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() <= i8::MAX.up() && x.up() - y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i8::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).overflowing_sub((-128i8)) == (0i8, false));
/// assert!((-128i8).overflowing_sub((-127i8)) == ((-1i8), false));
/// assert!((-128i8).overflowing_sub((-1i8)) == ((-127i8), false));
/// assert!((-128i8).overflowing_sub(0i8) == ((-128i8), false));
/// assert!((-127i8).overflowing_sub((-128i8)) == (1i8, false));
/// assert!((-127i8).overflowing_sub((-127i8)) == (0i8, false));
/// assert!((-127i8).overflowing_sub((-1i8)) == ((-126i8), false));
/// assert!((-127i8).overflowing_sub(0i8) == ((-127i8), false));
/// assert!((-127i8).overflowing_sub(1i8) == ((-128i8), false));
/// assert!((-1i8).overflowing_sub((-128i8)) == (127i8, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() > i8::MAX.up() || x.up() - y.up() < i8::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i8::down(
///                 (-i8::MIN.up() + x.up() - y.up()).rem_euclid(&(-i8::MIN.up() * 2))
///                     + i8::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).overflowing_sub(1i8) == (127i8, true));
/// assert!((-128i8).overflowing_sub(126i8) == (2i8, true));
/// assert!((-128i8).overflowing_sub(127i8) == (1i8, true));
/// assert!((-127i8).overflowing_sub(126i8) == (3i8, true));
/// assert!((-127i8).overflowing_sub(127i8) == (2i8, true));
/// assert!(0i8.overflowing_sub((-128i8)) == ((-128i8), true));
/// assert!(1i8.overflowing_sub((-128i8)) == ((-127i8), true));
/// assert!(1i8.overflowing_sub((-127i8)) == ((-128i8), true));
/// assert!(126i8.overflowing_sub((-128i8)) == ((-2i8), true));
/// assert!(126i8.overflowing_sub((-127i8)) == ((-3i8), true));
/// # }
/// ```
pub fn core_ops_sub_i8_overflowing_sub() {}
/// # Properties for [`i8::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() + y.up() <= i8::MAX.up() && x.up() + y.up() >= i8::MIN.up()`  
/// __Postcondition:__ `x + y == i8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8) + 0i8 == (-128i8));
/// assert!((-128i8) + 1i8 == (-127i8));
/// assert!((-128i8) + 126i8 == (-2i8));
/// assert!((-128i8) + 127i8 == (-1i8));
/// assert!((-127i8) + (-1i8) == (-128i8));
/// assert!((-127i8) + 0i8 == (-127i8));
/// assert!((-127i8) + 1i8 == (-126i8));
/// assert!((-127i8) + 126i8 == (-1i8));
/// assert!((-127i8) + 127i8 == 0i8);
/// assert!((-1i8) + (-127i8) == (-128i8));
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() + y.up() > i8::MAX.up() || x.up() + y.up() < i8::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 128i8) + (- 128i8)));
/// assert!(panics!((- 128i8) + (- 127i8)));
/// assert!(panics!((- 128i8) + (- 1i8)));
/// assert!(panics!((- 127i8) + (- 128i8)));
/// assert!(panics!((- 127i8) + (- 127i8)));
/// assert!(panics!((- 1i8) + (- 128i8)));
/// assert!(panics!(1i8 + 127i8));
/// assert!(panics!(126i8 + 126i8));
/// assert!(panics!(126i8 + 127i8));
/// assert!(panics!(127i8 + 1i8));
/// # }
/// ```
pub fn core_ops_add_i8_add() {}
/// # Properties for [`i16::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x != i16::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i16::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32767i16).checked_neg() == Some(32767i16));
/// assert!((-1i16).checked_neg() == Some(1i16));
/// assert!(0i16.checked_neg() == Some(0i16));
/// assert!(1i16.checked_neg() == Some((-1i16)));
/// assert!(32766i16.checked_neg() == Some((-32766i16)));
/// assert!(32767i16.checked_neg() == Some((-32767i16)));
/// assert!(13070i16.checked_neg() == Some((-13070i16)));
/// assert!(28285i16.checked_neg() == Some((-28285i16)));
/// assert!(9343i16.checked_neg() == Some((-9343i16)));
/// assert!(23049i16.checked_neg() == Some((-23049i16)));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x == i16::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).checked_neg() == None);
/// # }
/// ```
pub fn t_i16_checked_neg() {}
/// # Properties for [`i16::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x != i16::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i16::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         (-32767i16).neg() == 32767i16
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i16).neg() == 1i16
///     });
/// assert!({
///         use std::ops::Neg;
///         0i16.neg() == 0i16
///     });
/// assert!({
///         use std::ops::Neg;
///         1i16.neg() == (-1i16)
///     });
/// assert!({
///         use std::ops::Neg;
///         32766i16.neg() == (-32766i16)
///     });
/// assert!({
///         use std::ops::Neg;
///         32767i16.neg() == (-32767i16)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-18227i16).neg() == 18227i16
///     });
/// assert!({
///         use std::ops::Neg;
///         (-12473i16).neg() == 12473i16
///     });
/// assert!({
///         use std::ops::Neg;
///         (-16636i16).neg() == 16636i16
///     });
/// assert!({
///         use std::ops::Neg;
///         (-6654i16).neg() == 6654i16
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x == i16::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 32768i16).neg())
///     });
/// # }
/// ```
pub fn t_i16_neg() {}
/// # Properties for [`i16::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x != i16::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i16::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32767i16).overflowing_neg() == (32767i16, false));
/// assert!((-1i16).overflowing_neg() == (1i16, false));
/// assert!(0i16.overflowing_neg() == (0i16, false));
/// assert!(1i16.overflowing_neg() == ((-1i16), false));
/// assert!(32766i16.overflowing_neg() == ((-32766i16), false));
/// assert!(32767i16.overflowing_neg() == ((-32767i16), false));
/// assert!((-21463i16).overflowing_neg() == (21463i16, false));
/// assert!((-21040i16).overflowing_neg() == (21040i16, false));
/// assert!(25137i16.overflowing_neg() == ((-25137i16), false));
/// assert!(22746i16.overflowing_neg() == ((-22746i16), false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i16`  
/// __Precondition:__ `x == i16::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i16::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).overflowing_neg() == ((-32768i16), true));
/// # }
/// ```
pub fn t_i16_overflowing_neg() {}
/// # Properties for [`i16::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() <= i16::MAX.up() && x.up() - y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x - y == i16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16) - (-32768i16) == 0i16);
/// assert!((-32768i16) - (-32767i16) == (-1i16));
/// assert!((-32768i16) - (-1i16) == (-32767i16));
/// assert!((-32768i16) - 0i16 == (-32768i16));
/// assert!((-32767i16) - (-32768i16) == 1i16);
/// assert!((-32767i16) - (-32767i16) == 0i16);
/// assert!((-32767i16) - (-1i16) == (-32766i16));
/// assert!((-32767i16) - 0i16 == (-32767i16));
/// assert!((-32767i16) - 1i16 == (-32768i16));
/// assert!((-1i16) - (-32768i16) == 32767i16);
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() > i16::MAX.up() || x.up() - y.up() < i16::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 32768i16) - 1i16));
/// assert!(panics!((- 32768i16) - 32766i16));
/// assert!(panics!((- 32768i16) - 32767i16));
/// assert!(panics!((- 32767i16) - 32766i16));
/// assert!(panics!((- 32767i16) - 32767i16));
/// assert!(panics!(0i16 - (- 32768i16)));
/// assert!(panics!(1i16 - (- 32768i16)));
/// assert!(panics!(1i16 - (- 32767i16)));
/// assert!(panics!(32766i16 - (- 32768i16)));
/// assert!(panics!(32766i16 - (- 32767i16)));
/// # }
/// ```
pub fn core_ops_sub_i16_sub() {}
/// # Properties for [`i16::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() <= i16::MAX.up() && x.up() - y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i16::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).checked_sub((-32768i16)) == Some(0i16));
/// assert!((-32768i16).checked_sub((-32767i16)) == Some((-1i16)));
/// assert!((-32768i16).checked_sub((-1i16)) == Some((-32767i16)));
/// assert!((-32768i16).checked_sub(0i16) == Some((-32768i16)));
/// assert!((-32767i16).checked_sub((-32768i16)) == Some(1i16));
/// assert!((-32767i16).checked_sub((-32767i16)) == Some(0i16));
/// assert!((-32767i16).checked_sub((-1i16)) == Some((-32766i16)));
/// assert!((-32767i16).checked_sub(0i16) == Some((-32767i16)));
/// assert!((-32767i16).checked_sub(1i16) == Some((-32768i16)));
/// assert!((-1i16).checked_sub((-32768i16)) == Some(32767i16));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() > i16::MAX.up() || x.up() - y.up() < i16::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).checked_sub(1i16) == None);
/// assert!((-32768i16).checked_sub(32766i16) == None);
/// assert!((-32768i16).checked_sub(32767i16) == None);
/// assert!((-32767i16).checked_sub(32766i16) == None);
/// assert!((-32767i16).checked_sub(32767i16) == None);
/// assert!(0i16.checked_sub((-32768i16)) == None);
/// assert!(1i16.checked_sub((-32768i16)) == None);
/// assert!(1i16.checked_sub((-32767i16)) == None);
/// assert!(32766i16.checked_sub((-32768i16)) == None);
/// assert!(32766i16.checked_sub((-32767i16)) == None);
/// # }
/// ```
pub fn core_ops_sub_i16_checked_sub() {}
/// # Properties for [`i16::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() <= i16::MAX.up() && x.up() - y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i16::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).wrapping_sub((-32768i16)) == 0i16);
/// assert!((-32768i16).wrapping_sub((-32767i16)) == (-1i16));
/// assert!((-32768i16).wrapping_sub((-1i16)) == (-32767i16));
/// assert!((-32768i16).wrapping_sub(0i16) == (-32768i16));
/// assert!((-32767i16).wrapping_sub((-32768i16)) == 1i16);
/// assert!((-32767i16).wrapping_sub((-32767i16)) == 0i16);
/// assert!((-32767i16).wrapping_sub((-1i16)) == (-32766i16));
/// assert!((-32767i16).wrapping_sub(0i16) == (-32767i16));
/// assert!((-32767i16).wrapping_sub(1i16) == (-32768i16));
/// assert!((-1i16).wrapping_sub((-32768i16)) == 32767i16);
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() > i16::MAX.up() || x.up() - y.up() < i16::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i16::down(
///             (-i16::MIN.up() + x.up() - y.up()).rem_euclid(&(-i16::MIN.up() * 2))
///                 + i16::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).wrapping_sub(1i16) == 32767i16);
/// assert!((-32768i16).wrapping_sub(32766i16) == 2i16);
/// assert!((-32768i16).wrapping_sub(32767i16) == 1i16);
/// assert!((-32767i16).wrapping_sub(32766i16) == 3i16);
/// assert!((-32767i16).wrapping_sub(32767i16) == 2i16);
/// assert!(0i16.wrapping_sub((-32768i16)) == (-32768i16));
/// assert!(1i16.wrapping_sub((-32768i16)) == (-32767i16));
/// assert!(1i16.wrapping_sub((-32767i16)) == (-32768i16));
/// assert!(32766i16.wrapping_sub((-32768i16)) == (-2i16));
/// assert!(32766i16.wrapping_sub((-32767i16)) == (-3i16));
/// # }
/// ```
pub fn core_ops_sub_i16_wrapping_sub() {}
/// # Properties for [`i16::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() <= i16::MAX.up() && x.up() - y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i16::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).overflowing_sub((-32768i16)) == (0i16, false));
/// assert!((-32768i16).overflowing_sub((-32767i16)) == ((-1i16), false));
/// assert!((-32768i16).overflowing_sub((-1i16)) == ((-32767i16), false));
/// assert!((-32768i16).overflowing_sub(0i16) == ((-32768i16), false));
/// assert!((-32767i16).overflowing_sub((-32768i16)) == (1i16, false));
/// assert!((-32767i16).overflowing_sub((-32767i16)) == (0i16, false));
/// assert!((-32767i16).overflowing_sub((-1i16)) == ((-32766i16), false));
/// assert!((-32767i16).overflowing_sub(0i16) == ((-32767i16), false));
/// assert!((-32767i16).overflowing_sub(1i16) == ((-32768i16), false));
/// assert!((-1i16).overflowing_sub((-32768i16)) == (32767i16, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() - y.up() > i16::MAX.up() || x.up() - y.up() < i16::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i16::down(
///                 (-i16::MIN.up() + x.up() - y.up()).rem_euclid(&(-i16::MIN.up() * 2))
///                     + i16::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).overflowing_sub(1i16) == (32767i16, true));
/// assert!((-32768i16).overflowing_sub(32766i16) == (2i16, true));
/// assert!((-32768i16).overflowing_sub(32767i16) == (1i16, true));
/// assert!((-32767i16).overflowing_sub(32766i16) == (3i16, true));
/// assert!((-32767i16).overflowing_sub(32767i16) == (2i16, true));
/// assert!(0i16.overflowing_sub((-32768i16)) == ((-32768i16), true));
/// assert!(1i16.overflowing_sub((-32768i16)) == ((-32767i16), true));
/// assert!(1i16.overflowing_sub((-32767i16)) == ((-32768i16), true));
/// assert!(32766i16.overflowing_sub((-32768i16)) == ((-2i16), true));
/// assert!(32766i16.overflowing_sub((-32767i16)) == ((-3i16), true));
/// # }
/// ```
pub fn core_ops_sub_i16_overflowing_sub() {}
/// # Properties for [`i16::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() + y.up() <= i16::MAX.up() && x.up() + y.up() >= i16::MIN.up()`  
/// __Postcondition:__ `x + y == i16::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16) + 0i16 == (-32768i16));
/// assert!((-32768i16) + 1i16 == (-32767i16));
/// assert!((-32768i16) + 32766i16 == (-2i16));
/// assert!((-32768i16) + 32767i16 == (-1i16));
/// assert!((-32767i16) + (-1i16) == (-32768i16));
/// assert!((-32767i16) + 0i16 == (-32767i16));
/// assert!((-32767i16) + 1i16 == (-32766i16));
/// assert!((-32767i16) + 32766i16 == (-1i16));
/// assert!((-32767i16) + 32767i16 == 0i16);
/// assert!((-1i16) + (-32767i16) == (-32768i16));
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `x.up() + y.up() > i16::MAX.up() || x.up() + y.up() < i16::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 32768i16) + (- 32768i16)));
/// assert!(panics!((- 32768i16) + (- 32767i16)));
/// assert!(panics!((- 32768i16) + (- 1i16)));
/// assert!(panics!((- 32767i16) + (- 32768i16)));
/// assert!(panics!((- 32767i16) + (- 32767i16)));
/// assert!(panics!((- 1i16) + (- 32768i16)));
/// assert!(panics!(1i16 + 32767i16));
/// assert!(panics!(32766i16 + 32766i16));
/// assert!(panics!(32766i16 + 32767i16));
/// assert!(panics!(32767i16 + 1i16));
/// # }
/// ```
pub fn core_ops_add_i16_add() {}
/// # Properties for [`i32::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x != i32::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i32::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483647i32).checked_neg() == Some(2147483647i32));
/// assert!((-1i32).checked_neg() == Some(1i32));
/// assert!(0i32.checked_neg() == Some(0i32));
/// assert!(1i32.checked_neg() == Some((-1i32)));
/// assert!(2147483646i32.checked_neg() == Some((-2147483646i32)));
/// assert!(2147483647i32.checked_neg() == Some((-2147483647i32)));
/// assert!(1188556621i32.checked_neg() == Some((-1188556621i32)));
/// assert!((-257186825i32).checked_neg() == Some(257186825i32));
/// assert!(406933217i32.checked_neg() == Some((-406933217i32)));
/// assert!(1212731973i32.checked_neg() == Some((-1212731973i32)));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x == i32::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).checked_neg() == None);
/// # }
/// ```
pub fn t_i32_checked_neg() {}
/// # Properties for [`i32::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x != i32::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i32::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         (-2147483647i32).neg() == 2147483647i32
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i32).neg() == 1i32
///     });
/// assert!({
///         use std::ops::Neg;
///         0i32.neg() == 0i32
///     });
/// assert!({
///         use std::ops::Neg;
///         1i32.neg() == (-1i32)
///     });
/// assert!({
///         use std::ops::Neg;
///         2147483646i32.neg() == (-2147483646i32)
///     });
/// assert!({
///         use std::ops::Neg;
///         2147483647i32.neg() == (-2147483647i32)
///     });
/// assert!({
///         use std::ops::Neg;
///         1972895171i32.neg() == (-1972895171i32)
///     });
/// assert!({
///         use std::ops::Neg;
///         2051109421i32.neg() == (-2051109421i32)
///     });
/// assert!({
///         use std::ops::Neg;
///         103059754i32.neg() == (-103059754i32)
///     });
/// assert!({
///         use std::ops::Neg;
///         2065644367i32.neg() == (-2065644367i32)
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x == i32::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 2147483648i32).neg())
///     });
/// # }
/// ```
pub fn t_i32_neg() {}
/// # Properties for [`i32::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x != i32::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i32::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483647i32).overflowing_neg() == (2147483647i32, false));
/// assert!((-1i32).overflowing_neg() == (1i32, false));
/// assert!(0i32.overflowing_neg() == (0i32, false));
/// assert!(1i32.overflowing_neg() == ((-1i32), false));
/// assert!(2147483646i32.overflowing_neg() == ((-2147483646i32), false));
/// assert!(2147483647i32.overflowing_neg() == ((-2147483647i32), false));
/// assert!(1998419589i32.overflowing_neg() == ((-1998419589i32), false));
/// assert!(206309489i32.overflowing_neg() == ((-206309489i32), false));
/// assert!(2011324096i32.overflowing_neg() == ((-2011324096i32), false));
/// assert!((-689596157i32).overflowing_neg() == (689596157i32, false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i32`  
/// __Precondition:__ `x == i32::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i32::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).overflowing_neg() == ((-2147483648i32), true));
/// # }
/// ```
pub fn t_i32_overflowing_neg() {}
/// # Properties for [`i32::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() <= i32::MAX.up() && x.up() - y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x - y == i32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32) - (-2147483648i32) == 0i32);
/// assert!((-2147483648i32) - (-2147483647i32) == (-1i32));
/// assert!((-2147483648i32) - (-1i32) == (-2147483647i32));
/// assert!((-2147483648i32) - 0i32 == (-2147483648i32));
/// assert!((-2147483647i32) - (-2147483648i32) == 1i32);
/// assert!((-2147483647i32) - (-2147483647i32) == 0i32);
/// assert!((-2147483647i32) - (-1i32) == (-2147483646i32));
/// assert!((-2147483647i32) - 0i32 == (-2147483647i32));
/// assert!((-2147483647i32) - 1i32 == (-2147483648i32));
/// assert!((-1i32) - (-2147483648i32) == 2147483647i32);
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() > i32::MAX.up() || x.up() - y.up() < i32::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 2147483648i32) - 1i32));
/// assert!(panics!((- 2147483648i32) - 2147483646i32));
/// assert!(panics!((- 2147483648i32) - 2147483647i32));
/// assert!(panics!((- 2147483647i32) - 2147483646i32));
/// assert!(panics!((- 2147483647i32) - 2147483647i32));
/// assert!(panics!(0i32 - (- 2147483648i32)));
/// assert!(panics!(1i32 - (- 2147483648i32)));
/// assert!(panics!(1i32 - (- 2147483647i32)));
/// assert!(panics!(2147483646i32 - (- 2147483648i32)));
/// assert!(panics!(2147483646i32 - (- 2147483647i32)));
/// # }
/// ```
pub fn core_ops_sub_i32_sub() {}
/// # Properties for [`i32::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() <= i32::MAX.up() && x.up() - y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i32::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).checked_sub((-2147483648i32)) == Some(0i32));
/// assert!((-2147483648i32).checked_sub((-2147483647i32)) == Some((-1i32)));
/// assert!((-2147483648i32).checked_sub((-1i32)) == Some((-2147483647i32)));
/// assert!((-2147483648i32).checked_sub(0i32) == Some((-2147483648i32)));
/// assert!((-2147483647i32).checked_sub((-2147483648i32)) == Some(1i32));
/// assert!((-2147483647i32).checked_sub((-2147483647i32)) == Some(0i32));
/// assert!((-2147483647i32).checked_sub((-1i32)) == Some((-2147483646i32)));
/// assert!((-2147483647i32).checked_sub(0i32) == Some((-2147483647i32)));
/// assert!((-2147483647i32).checked_sub(1i32) == Some((-2147483648i32)));
/// assert!((-1i32).checked_sub((-2147483648i32)) == Some(2147483647i32));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() > i32::MAX.up() || x.up() - y.up() < i32::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).checked_sub(1i32) == None);
/// assert!((-2147483648i32).checked_sub(2147483646i32) == None);
/// assert!((-2147483648i32).checked_sub(2147483647i32) == None);
/// assert!((-2147483647i32).checked_sub(2147483646i32) == None);
/// assert!((-2147483647i32).checked_sub(2147483647i32) == None);
/// assert!(0i32.checked_sub((-2147483648i32)) == None);
/// assert!(1i32.checked_sub((-2147483648i32)) == None);
/// assert!(1i32.checked_sub((-2147483647i32)) == None);
/// assert!(2147483646i32.checked_sub((-2147483648i32)) == None);
/// assert!(2147483646i32.checked_sub((-2147483647i32)) == None);
/// # }
/// ```
pub fn core_ops_sub_i32_checked_sub() {}
/// # Properties for [`i32::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() <= i32::MAX.up() && x.up() - y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i32::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).wrapping_sub((-2147483648i32)) == 0i32);
/// assert!((-2147483648i32).wrapping_sub((-2147483647i32)) == (-1i32));
/// assert!((-2147483648i32).wrapping_sub((-1i32)) == (-2147483647i32));
/// assert!((-2147483648i32).wrapping_sub(0i32) == (-2147483648i32));
/// assert!((-2147483647i32).wrapping_sub((-2147483648i32)) == 1i32);
/// assert!((-2147483647i32).wrapping_sub((-2147483647i32)) == 0i32);
/// assert!((-2147483647i32).wrapping_sub((-1i32)) == (-2147483646i32));
/// assert!((-2147483647i32).wrapping_sub(0i32) == (-2147483647i32));
/// assert!((-2147483647i32).wrapping_sub(1i32) == (-2147483648i32));
/// assert!((-1i32).wrapping_sub((-2147483648i32)) == 2147483647i32);
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() > i32::MAX.up() || x.up() - y.up() < i32::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i32::down(
///             (-i32::MIN.up() + x.up() - y.up()).rem_euclid(&(-i32::MIN.up() * 2))
///                 + i32::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).wrapping_sub(1i32) == 2147483647i32);
/// assert!((-2147483648i32).wrapping_sub(2147483646i32) == 2i32);
/// assert!((-2147483648i32).wrapping_sub(2147483647i32) == 1i32);
/// assert!((-2147483647i32).wrapping_sub(2147483646i32) == 3i32);
/// assert!((-2147483647i32).wrapping_sub(2147483647i32) == 2i32);
/// assert!(0i32.wrapping_sub((-2147483648i32)) == (-2147483648i32));
/// assert!(1i32.wrapping_sub((-2147483648i32)) == (-2147483647i32));
/// assert!(1i32.wrapping_sub((-2147483647i32)) == (-2147483648i32));
/// assert!(2147483646i32.wrapping_sub((-2147483648i32)) == (-2i32));
/// assert!(2147483646i32.wrapping_sub((-2147483647i32)) == (-3i32));
/// # }
/// ```
pub fn core_ops_sub_i32_wrapping_sub() {}
/// # Properties for [`i32::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() <= i32::MAX.up() && x.up() - y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i32::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).overflowing_sub((-2147483648i32)) == (0i32, false));
/// assert!((-2147483648i32).overflowing_sub((-2147483647i32)) == ((-1i32), false));
/// assert!((-2147483648i32).overflowing_sub((-1i32)) == ((-2147483647i32), false));
/// assert!((-2147483648i32).overflowing_sub(0i32) == ((-2147483648i32), false));
/// assert!((-2147483647i32).overflowing_sub((-2147483648i32)) == (1i32, false));
/// assert!((-2147483647i32).overflowing_sub((-2147483647i32)) == (0i32, false));
/// assert!((-2147483647i32).overflowing_sub((-1i32)) == ((-2147483646i32), false));
/// assert!((-2147483647i32).overflowing_sub(0i32) == ((-2147483647i32), false));
/// assert!((-2147483647i32).overflowing_sub(1i32) == ((-2147483648i32), false));
/// assert!((-1i32).overflowing_sub((-2147483648i32)) == (2147483647i32, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() - y.up() > i32::MAX.up() || x.up() - y.up() < i32::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i32::down(
///                 (-i32::MIN.up() + x.up() - y.up()).rem_euclid(&(-i32::MIN.up() * 2))
///                     + i32::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).overflowing_sub(1i32) == (2147483647i32, true));
/// assert!((-2147483648i32).overflowing_sub(2147483646i32) == (2i32, true));
/// assert!((-2147483648i32).overflowing_sub(2147483647i32) == (1i32, true));
/// assert!((-2147483647i32).overflowing_sub(2147483646i32) == (3i32, true));
/// assert!((-2147483647i32).overflowing_sub(2147483647i32) == (2i32, true));
/// assert!(0i32.overflowing_sub((-2147483648i32)) == ((-2147483648i32), true));
/// assert!(1i32.overflowing_sub((-2147483648i32)) == ((-2147483647i32), true));
/// assert!(1i32.overflowing_sub((-2147483647i32)) == ((-2147483648i32), true));
/// assert!(2147483646i32.overflowing_sub((-2147483648i32)) == ((-2i32), true));
/// assert!(2147483646i32.overflowing_sub((-2147483647i32)) == ((-3i32), true));
/// # }
/// ```
pub fn core_ops_sub_i32_overflowing_sub() {}
/// # Properties for [`i32::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() + y.up() <= i32::MAX.up() && x.up() + y.up() >= i32::MIN.up()`  
/// __Postcondition:__ `x + y == i32::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32) + 0i32 == (-2147483648i32));
/// assert!((-2147483648i32) + 1i32 == (-2147483647i32));
/// assert!((-2147483648i32) + 2147483646i32 == (-2i32));
/// assert!((-2147483648i32) + 2147483647i32 == (-1i32));
/// assert!((-2147483647i32) + (-1i32) == (-2147483648i32));
/// assert!((-2147483647i32) + 0i32 == (-2147483647i32));
/// assert!((-2147483647i32) + 1i32 == (-2147483646i32));
/// assert!((-2147483647i32) + 2147483646i32 == (-1i32));
/// assert!((-2147483647i32) + 2147483647i32 == 0i32);
/// assert!((-1i32) + (-2147483647i32) == (-2147483648i32));
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `x.up() + y.up() > i32::MAX.up() || x.up() + y.up() < i32::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 2147483648i32) + (- 2147483648i32)));
/// assert!(panics!((- 2147483648i32) + (- 2147483647i32)));
/// assert!(panics!((- 2147483648i32) + (- 1i32)));
/// assert!(panics!((- 2147483647i32) + (- 2147483648i32)));
/// assert!(panics!((- 2147483647i32) + (- 2147483647i32)));
/// assert!(panics!((- 1i32) + (- 2147483648i32)));
/// assert!(panics!(1i32 + 2147483647i32));
/// assert!(panics!(2147483646i32 + 2147483646i32));
/// assert!(panics!(2147483646i32 + 2147483647i32));
/// assert!(panics!(2147483647i32 + 1i32));
/// # }
/// ```
pub fn core_ops_add_i32_add() {}
/// # Properties for [`i64::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x != i64::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i64::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775807i64).checked_neg() == Some(9223372036854775807i64));
/// assert!((-1i64).checked_neg() == Some(1i64));
/// assert!(0i64.checked_neg() == Some(0i64));
/// assert!(1i64.checked_neg() == Some((-1i64)));
/// assert!(9223372036854775806i64.checked_neg() == Some((-9223372036854775806i64)));
/// assert!(9223372036854775807i64.checked_neg() == Some((-9223372036854775807i64)));
/// assert!(4353663488066752935i64.checked_neg() == Some((-4353663488066752935i64)));
/// assert!(3476733529313598090i64.checked_neg() == Some((-3476733529313598090i64)));
/// assert!((-7056530715366628844i64).checked_neg() == Some(7056530715366628844i64));
/// assert!((-2307308282106980276i64).checked_neg() == Some(2307308282106980276i64));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x == i64::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).checked_neg() == None);
/// # }
/// ```
pub fn t_i64_checked_neg() {}
/// # Properties for [`i64::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x != i64::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i64::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         (-9223372036854775807i64).neg() == 9223372036854775807i64
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i64).neg() == 1i64
///     });
/// assert!({
///         use std::ops::Neg;
///         0i64.neg() == 0i64
///     });
/// assert!({
///         use std::ops::Neg;
///         1i64.neg() == (-1i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         9223372036854775806i64.neg() == (-9223372036854775806i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         9223372036854775807i64.neg() == (-9223372036854775807i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         3380262809433411823i64.neg() == (-3380262809433411823i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-2309480948683311672i64).neg() == 2309480948683311672i64
///     });
/// assert!({
///         use std::ops::Neg;
///         5610565728402143740i64.neg() == (-5610565728402143740i64)
///     });
/// assert!({
///         use std::ops::Neg;
///         2036429713825017967i64.neg() == (-2036429713825017967i64)
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x == i64::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 9223372036854775808i64).neg())
///     });
/// # }
/// ```
pub fn t_i64_neg() {}
/// # Properties for [`i64::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x != i64::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i64::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775807i64).overflowing_neg() == (9223372036854775807i64, false));
/// assert!((-1i64).overflowing_neg() == (1i64, false));
/// assert!(0i64.overflowing_neg() == (0i64, false));
/// assert!(1i64.overflowing_neg() == ((-1i64), false));
/// assert!(9223372036854775806i64.overflowing_neg() == ((-9223372036854775806i64), false));
/// assert!(9223372036854775807i64.overflowing_neg() == ((-9223372036854775807i64), false));
/// assert!((-8815779951093358336i64).overflowing_neg() == (8815779951093358336i64, false));
/// assert!((-6802248454276055698i64).overflowing_neg() == (6802248454276055698i64, false));
/// assert!(6554505005600417268i64.overflowing_neg() == ((-6554505005600417268i64), false));
/// assert!(1767940940917087685i64.overflowing_neg() == ((-1767940940917087685i64), false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i64`  
/// __Precondition:__ `x == i64::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i64::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).overflowing_neg() == ((-9223372036854775808i64), true));
/// # }
/// ```
pub fn t_i64_overflowing_neg() {}
/// # Properties for [`i64::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() <= i64::MAX.up() && x.up() - y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x - y == i64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64) - (-9223372036854775808i64) == 0i64);
/// assert!((-9223372036854775808i64) - (-9223372036854775807i64) == (-1i64));
/// assert!((-9223372036854775808i64) - (-1i64) == (-9223372036854775807i64));
/// assert!((-9223372036854775808i64) - 0i64 == (-9223372036854775808i64));
/// assert!((-9223372036854775807i64) - (-9223372036854775808i64) == 1i64);
/// assert!((-9223372036854775807i64) - (-9223372036854775807i64) == 0i64);
/// assert!((-9223372036854775807i64) - (-1i64) == (-9223372036854775806i64));
/// assert!((-9223372036854775807i64) - 0i64 == (-9223372036854775807i64));
/// assert!((-9223372036854775807i64) - 1i64 == (-9223372036854775808i64));
/// assert!((-1i64) - (-9223372036854775808i64) == 9223372036854775807i64);
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() > i64::MAX.up() || x.up() - y.up() < i64::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 9223372036854775808i64) - 1i64));
/// assert!(panics!((- 9223372036854775808i64) - 9223372036854775806i64));
/// assert!(panics!((- 9223372036854775808i64) - 9223372036854775807i64));
/// assert!(panics!((- 9223372036854775807i64) - 9223372036854775806i64));
/// assert!(panics!((- 9223372036854775807i64) - 9223372036854775807i64));
/// assert!(panics!(0i64 - (- 9223372036854775808i64)));
/// assert!(panics!(1i64 - (- 9223372036854775808i64)));
/// assert!(panics!(1i64 - (- 9223372036854775807i64)));
/// assert!(panics!(9223372036854775806i64 - (- 9223372036854775808i64)));
/// assert!(panics!(9223372036854775806i64 - (- 9223372036854775807i64)));
/// # }
/// ```
pub fn core_ops_sub_i64_sub() {}
/// # Properties for [`i64::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() <= i64::MAX.up() && x.up() - y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i64::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).checked_sub((-9223372036854775808i64)) == Some(0i64));
/// assert!((-9223372036854775808i64).checked_sub((-9223372036854775807i64)) == Some((-1i64)));
/// assert!((-9223372036854775808i64).checked_sub((-1i64)) == Some((-9223372036854775807i64)));
/// assert!((-9223372036854775808i64).checked_sub(0i64) == Some((-9223372036854775808i64)));
/// assert!((-9223372036854775807i64).checked_sub((-9223372036854775808i64)) == Some(1i64));
/// assert!((-9223372036854775807i64).checked_sub((-9223372036854775807i64)) == Some(0i64));
/// assert!((-9223372036854775807i64).checked_sub((-1i64)) == Some((-9223372036854775806i64)));
/// assert!((-9223372036854775807i64).checked_sub(0i64) == Some((-9223372036854775807i64)));
/// assert!((-9223372036854775807i64).checked_sub(1i64) == Some((-9223372036854775808i64)));
/// assert!((-1i64).checked_sub((-9223372036854775808i64)) == Some(9223372036854775807i64));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() > i64::MAX.up() || x.up() - y.up() < i64::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).checked_sub(1i64) == None);
/// assert!((-9223372036854775808i64).checked_sub(9223372036854775806i64) == None);
/// assert!((-9223372036854775808i64).checked_sub(9223372036854775807i64) == None);
/// assert!((-9223372036854775807i64).checked_sub(9223372036854775806i64) == None);
/// assert!((-9223372036854775807i64).checked_sub(9223372036854775807i64) == None);
/// assert!(0i64.checked_sub((-9223372036854775808i64)) == None);
/// assert!(1i64.checked_sub((-9223372036854775808i64)) == None);
/// assert!(1i64.checked_sub((-9223372036854775807i64)) == None);
/// assert!(9223372036854775806i64.checked_sub((-9223372036854775808i64)) == None);
/// assert!(9223372036854775806i64.checked_sub((-9223372036854775807i64)) == None);
/// # }
/// ```
pub fn core_ops_sub_i64_checked_sub() {}
/// # Properties for [`i64::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() <= i64::MAX.up() && x.up() - y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i64::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).wrapping_sub((-9223372036854775808i64)) == 0i64);
/// assert!((-9223372036854775808i64).wrapping_sub((-9223372036854775807i64)) == (-1i64));
/// assert!((-9223372036854775808i64).wrapping_sub((-1i64)) == (-9223372036854775807i64));
/// assert!((-9223372036854775808i64).wrapping_sub(0i64) == (-9223372036854775808i64));
/// assert!((-9223372036854775807i64).wrapping_sub((-9223372036854775808i64)) == 1i64);
/// assert!((-9223372036854775807i64).wrapping_sub((-9223372036854775807i64)) == 0i64);
/// assert!((-9223372036854775807i64).wrapping_sub((-1i64)) == (-9223372036854775806i64));
/// assert!((-9223372036854775807i64).wrapping_sub(0i64) == (-9223372036854775807i64));
/// assert!((-9223372036854775807i64).wrapping_sub(1i64) == (-9223372036854775808i64));
/// assert!((-1i64).wrapping_sub((-9223372036854775808i64)) == 9223372036854775807i64);
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() > i64::MAX.up() || x.up() - y.up() < i64::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i64::down(
///             (-i64::MIN.up() + x.up() - y.up()).rem_euclid(&(-i64::MIN.up() * 2))
///                 + i64::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).wrapping_sub(1i64) == 9223372036854775807i64);
/// assert!((-9223372036854775808i64).wrapping_sub(9223372036854775806i64) == 2i64);
/// assert!((-9223372036854775808i64).wrapping_sub(9223372036854775807i64) == 1i64);
/// assert!((-9223372036854775807i64).wrapping_sub(9223372036854775806i64) == 3i64);
/// assert!((-9223372036854775807i64).wrapping_sub(9223372036854775807i64) == 2i64);
/// assert!(0i64.wrapping_sub((-9223372036854775808i64)) == (-9223372036854775808i64));
/// assert!(1i64.wrapping_sub((-9223372036854775808i64)) == (-9223372036854775807i64));
/// assert!(1i64.wrapping_sub((-9223372036854775807i64)) == (-9223372036854775808i64));
/// assert!(9223372036854775806i64.wrapping_sub((-9223372036854775808i64)) == (-2i64));
/// assert!(9223372036854775806i64.wrapping_sub((-9223372036854775807i64)) == (-3i64));
/// # }
/// ```
pub fn core_ops_sub_i64_wrapping_sub() {}
/// # Properties for [`i64::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() <= i64::MAX.up() && x.up() - y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i64::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).overflowing_sub((-9223372036854775808i64)) == (0i64, false));
/// assert!((-9223372036854775808i64).overflowing_sub((-9223372036854775807i64))
///         == ((-1i64), false));
/// assert!((-9223372036854775808i64).overflowing_sub((-1i64))
///         == ((-9223372036854775807i64), false));
/// assert!((-9223372036854775808i64).overflowing_sub(0i64) == ((-9223372036854775808i64), false));
/// assert!((-9223372036854775807i64).overflowing_sub((-9223372036854775808i64)) == (1i64, false));
/// assert!((-9223372036854775807i64).overflowing_sub((-9223372036854775807i64)) == (0i64, false));
/// assert!((-9223372036854775807i64).overflowing_sub((-1i64))
///         == ((-9223372036854775806i64), false));
/// assert!((-9223372036854775807i64).overflowing_sub(0i64) == ((-9223372036854775807i64), false));
/// assert!((-9223372036854775807i64).overflowing_sub(1i64) == ((-9223372036854775808i64), false));
/// assert!((-1i64).overflowing_sub((-9223372036854775808i64)) == (9223372036854775807i64, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() - y.up() > i64::MAX.up() || x.up() - y.up() < i64::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i64::down(
///                 (-i64::MIN.up() + x.up() - y.up()).rem_euclid(&(-i64::MIN.up() * 2))
///                     + i64::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).overflowing_sub(1i64) == (9223372036854775807i64, true));
/// assert!((-9223372036854775808i64).overflowing_sub(9223372036854775806i64) == (2i64, true));
/// assert!((-9223372036854775808i64).overflowing_sub(9223372036854775807i64) == (1i64, true));
/// assert!((-9223372036854775807i64).overflowing_sub(9223372036854775806i64) == (3i64, true));
/// assert!((-9223372036854775807i64).overflowing_sub(9223372036854775807i64) == (2i64, true));
/// assert!(0i64.overflowing_sub((-9223372036854775808i64)) == ((-9223372036854775808i64), true));
/// assert!(1i64.overflowing_sub((-9223372036854775808i64)) == ((-9223372036854775807i64), true));
/// assert!(1i64.overflowing_sub((-9223372036854775807i64)) == ((-9223372036854775808i64), true));
/// assert!(9223372036854775806i64.overflowing_sub((-9223372036854775808i64)) == ((-2i64), true));
/// assert!(9223372036854775806i64.overflowing_sub((-9223372036854775807i64)) == ((-3i64), true));
/// # }
/// ```
pub fn core_ops_sub_i64_overflowing_sub() {}
/// # Properties for [`i64::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() + y.up() <= i64::MAX.up() && x.up() + y.up() >= i64::MIN.up()`  
/// __Postcondition:__ `x + y == i64::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64) + 0i64 == (-9223372036854775808i64));
/// assert!((-9223372036854775808i64) + 1i64 == (-9223372036854775807i64));
/// assert!((-9223372036854775808i64) + 9223372036854775806i64 == (-2i64));
/// assert!((-9223372036854775808i64) + 9223372036854775807i64 == (-1i64));
/// assert!((-9223372036854775807i64) + (-1i64) == (-9223372036854775808i64));
/// assert!((-9223372036854775807i64) + 0i64 == (-9223372036854775807i64));
/// assert!((-9223372036854775807i64) + 1i64 == (-9223372036854775806i64));
/// assert!((-9223372036854775807i64) + 9223372036854775806i64 == (-1i64));
/// assert!((-9223372036854775807i64) + 9223372036854775807i64 == 0i64);
/// assert!((-1i64) + (-9223372036854775807i64) == (-9223372036854775808i64));
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `x.up() + y.up() > i64::MAX.up() || x.up() + y.up() < i64::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 9223372036854775808i64) + (- 9223372036854775808i64)));
/// assert!(panics!((- 9223372036854775808i64) + (- 9223372036854775807i64)));
/// assert!(panics!((- 9223372036854775808i64) + (- 1i64)));
/// assert!(panics!((- 9223372036854775807i64) + (- 9223372036854775808i64)));
/// assert!(panics!((- 9223372036854775807i64) + (- 9223372036854775807i64)));
/// assert!(panics!((- 1i64) + (- 9223372036854775808i64)));
/// assert!(panics!(1i64 + 9223372036854775807i64));
/// assert!(panics!(9223372036854775806i64 + 9223372036854775806i64));
/// assert!(panics!(9223372036854775806i64 + 9223372036854775807i64));
/// assert!(panics!(9223372036854775807i64 + 1i64));
/// # }
/// ```
pub fn core_ops_add_i64_add() {}
/// # Properties for [`i128::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x != i128::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(i128::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105727i128).checked_neg()
///         == Some(170141183460469231731687303715884105727i128));
/// assert!((-1i128).checked_neg() == Some(1i128));
/// assert!(0i128.checked_neg() == Some(0i128));
/// assert!(1i128.checked_neg() == Some((-1i128)));
/// assert!(170141183460469231731687303715884105726i128.checked_neg()
///         == Some((-170141183460469231731687303715884105726i128)));
/// assert!(170141183460469231731687303715884105727i128.checked_neg()
///         == Some((-170141183460469231731687303715884105727i128)));
/// assert!(4443526550899695848864685213695735410i128.checked_neg()
///         == Some((-4443526550899695848864685213695735410i128)));
/// assert!(89983046691539104091212351332571083884i128.checked_neg()
///         == Some((-89983046691539104091212351332571083884i128)));
/// assert!(157033005170615327237629801662667437696i128.checked_neg()
///         == Some((-157033005170615327237629801662667437696i128)));
/// assert!((-68452167488008918508926081952391400531i128).checked_neg()
///         == Some(68452167488008918508926081952391400531i128));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x == i128::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128).checked_neg() == None);
/// # }
/// ```
pub fn t_i128_checked_neg() {}
/// # Properties for [`i128::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x != i128::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == i128::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         (-170141183460469231731687303715884105727i128).neg()
///             == 170141183460469231731687303715884105727i128
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1i128).neg() == 1i128
///     });
/// assert!({
///         use std::ops::Neg;
///         0i128.neg() == 0i128
///     });
/// assert!({
///         use std::ops::Neg;
///         1i128.neg() == (-1i128)
///     });
/// assert!({
///         use std::ops::Neg;
///         170141183460469231731687303715884105726i128.neg()
///             == (-170141183460469231731687303715884105726i128)
///     });
/// assert!({
///         use std::ops::Neg;
///         170141183460469231731687303715884105727i128.neg()
///             == (-170141183460469231731687303715884105727i128)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-44218845834060431073417620021160931837i128).neg()
///             == 44218845834060431073417620021160931837i128
///     });
/// assert!({
///         use std::ops::Neg;
///         22063174368917564432144834016789279349i128.neg()
///             == (-22063174368917564432144834016789279349i128)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-25307357476739349708078327272883921843i128).neg()
///             == 25307357476739349708078327272883921843i128
///     });
/// assert!({
///         use std::ops::Neg;
///         122194466512687741927882042981427409958i128.neg()
///             == (-122194466512687741927882042981427409958i128)
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x == i128::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 170141183460469231731687303715884105728i128).neg())
///     });
/// # }
/// ```
pub fn t_i128_neg() {}
/// # Properties for [`i128::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x != i128::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i128::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105727i128).overflowing_neg()
///         == (170141183460469231731687303715884105727i128, false));
/// assert!((-1i128).overflowing_neg() == (1i128, false));
/// assert!(0i128.overflowing_neg() == (0i128, false));
/// assert!(1i128.overflowing_neg() == ((-1i128), false));
/// assert!(170141183460469231731687303715884105726i128.overflowing_neg()
///         == ((-170141183460469231731687303715884105726i128), false));
/// assert!(170141183460469231731687303715884105727i128.overflowing_neg()
///         == ((-170141183460469231731687303715884105727i128), false));
/// assert!(80702799090855744584094085980453386678i128.overflowing_neg()
///         == ((-80702799090855744584094085980453386678i128), false));
/// assert!((-29079516468604951734499855108652283431i128).overflowing_neg()
///         == (29079516468604951734499855108652283431i128, false));
/// assert!(41346973316414323899529452013530373549i128.overflowing_neg()
///         == ((-41346973316414323899529452013530373549i128), false));
/// assert!(139411089024669258240116948326023335869i128.overflowing_neg()
///         == ((-139411089024669258240116948326023335869i128), false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : i128`  
/// __Precondition:__ `x == i128::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (i128::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128).overflowing_neg()
///         == ((-170141183460469231731687303715884105728i128), true));
/// # }
/// ```
pub fn t_i128_overflowing_neg() {}
/// # Properties for [`i128::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() <= i128::MAX.up() && x.up() - y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x - y == i128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         - (-170141183460469231731687303715884105728i128) == 0i128);
/// assert!((-170141183460469231731687303715884105728i128)
///         - (-170141183460469231731687303715884105727i128) == (-1i128));
/// assert!((-170141183460469231731687303715884105728i128) - (-1i128)
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105728i128) - 0i128
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         - (-170141183460469231731687303715884105728i128) == 1i128);
/// assert!((-170141183460469231731687303715884105727i128)
///         - (-170141183460469231731687303715884105727i128) == 0i128);
/// assert!((-170141183460469231731687303715884105727i128) - (-1i128)
///         == (-170141183460469231731687303715884105726i128));
/// assert!((-170141183460469231731687303715884105727i128) - 0i128
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105727i128) - 1i128
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-1i128) - (-170141183460469231731687303715884105728i128)
///         == 170141183460469231731687303715884105727i128);
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() > i128::MAX.up() || x.up() - y.up() < i128::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 170141183460469231731687303715884105728i128) - 1i128));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105728i128) -
///         170141183460469231731687303715884105726i128
///     ));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105728i128) -
///         170141183460469231731687303715884105727i128
///     ));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105727i128) -
///         170141183460469231731687303715884105726i128
///     ));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105727i128) -
///         170141183460469231731687303715884105727i128
///     ));
/// assert!(panics!(0i128 - (- 170141183460469231731687303715884105728i128)));
/// assert!(panics!(1i128 - (- 170141183460469231731687303715884105728i128)));
/// assert!(panics!(1i128 - (- 170141183460469231731687303715884105727i128)));
/// assert!(panics!(
///         170141183460469231731687303715884105726i128 - (-
///         170141183460469231731687303715884105728i128)
///     ));
/// assert!(panics!(
///         170141183460469231731687303715884105726i128 - (-
///         170141183460469231731687303715884105727i128)
///     ));
/// # }
/// ```
pub fn core_ops_sub_i128_sub() {}
/// # Properties for [`i128::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() <= i128::MAX.up() && x.up() - y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(i128::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         .checked_sub((-170141183460469231731687303715884105728i128)) == Some(0i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         .checked_sub((-170141183460469231731687303715884105727i128)) == Some((-1i128)));
/// assert!((-170141183460469231731687303715884105728i128).checked_sub((-1i128))
///         == Some((-170141183460469231731687303715884105727i128)));
/// assert!((-170141183460469231731687303715884105728i128).checked_sub(0i128)
///         == Some((-170141183460469231731687303715884105728i128)));
/// assert!((-170141183460469231731687303715884105727i128)
///         .checked_sub((-170141183460469231731687303715884105728i128)) == Some(1i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         .checked_sub((-170141183460469231731687303715884105727i128)) == Some(0i128));
/// assert!((-170141183460469231731687303715884105727i128).checked_sub((-1i128))
///         == Some((-170141183460469231731687303715884105726i128)));
/// assert!((-170141183460469231731687303715884105727i128).checked_sub(0i128)
///         == Some((-170141183460469231731687303715884105727i128)));
/// assert!((-170141183460469231731687303715884105727i128).checked_sub(1i128)
///         == Some((-170141183460469231731687303715884105728i128)));
/// assert!((-1i128).checked_sub((-170141183460469231731687303715884105728i128))
///         == Some(170141183460469231731687303715884105727i128));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() > i128::MAX.up() || x.up() - y.up() < i128::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128).checked_sub(1i128) == None);
/// assert!((-170141183460469231731687303715884105728i128)
///         .checked_sub(170141183460469231731687303715884105726i128) == None);
/// assert!((-170141183460469231731687303715884105728i128)
///         .checked_sub(170141183460469231731687303715884105727i128) == None);
/// assert!((-170141183460469231731687303715884105727i128)
///         .checked_sub(170141183460469231731687303715884105726i128) == None);
/// assert!((-170141183460469231731687303715884105727i128)
///         .checked_sub(170141183460469231731687303715884105727i128) == None);
/// assert!(0i128.checked_sub((-170141183460469231731687303715884105728i128)) == None);
/// assert!(1i128.checked_sub((-170141183460469231731687303715884105728i128)) == None);
/// assert!(1i128.checked_sub((-170141183460469231731687303715884105727i128)) == None);
/// assert!(170141183460469231731687303715884105726i128
///         .checked_sub((-170141183460469231731687303715884105728i128)) == None);
/// assert!(170141183460469231731687303715884105726i128
///         .checked_sub((-170141183460469231731687303715884105727i128)) == None);
/// # }
/// ```
pub fn core_ops_sub_i128_checked_sub() {}
/// # Properties for [`i128::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() <= i128::MAX.up() && x.up() - y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == i128::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         .wrapping_sub((-170141183460469231731687303715884105728i128)) == 0i128);
/// assert!((-170141183460469231731687303715884105728i128)
///         .wrapping_sub((-170141183460469231731687303715884105727i128)) == (-1i128));
/// assert!((-170141183460469231731687303715884105728i128).wrapping_sub((-1i128))
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105728i128).wrapping_sub(0i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         .wrapping_sub((-170141183460469231731687303715884105728i128)) == 1i128);
/// assert!((-170141183460469231731687303715884105727i128)
///         .wrapping_sub((-170141183460469231731687303715884105727i128)) == 0i128);
/// assert!((-170141183460469231731687303715884105727i128).wrapping_sub((-1i128))
///         == (-170141183460469231731687303715884105726i128));
/// assert!((-170141183460469231731687303715884105727i128).wrapping_sub(0i128)
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105727i128).wrapping_sub(1i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-1i128).wrapping_sub((-170141183460469231731687303715884105728i128))
///         == 170141183460469231731687303715884105727i128);
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() > i128::MAX.up() || x.up() - y.up() < i128::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == i128::down(
///             (-i128::MIN.up() + x.up() - y.up()).rem_euclid(&(-i128::MIN.up() * 2))
///                 + i128::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128).wrapping_sub(1i128)
///         == 170141183460469231731687303715884105727i128);
/// assert!((-170141183460469231731687303715884105728i128)
///         .wrapping_sub(170141183460469231731687303715884105726i128) == 2i128);
/// assert!((-170141183460469231731687303715884105728i128)
///         .wrapping_sub(170141183460469231731687303715884105727i128) == 1i128);
/// assert!((-170141183460469231731687303715884105727i128)
///         .wrapping_sub(170141183460469231731687303715884105726i128) == 3i128);
/// assert!((-170141183460469231731687303715884105727i128)
///         .wrapping_sub(170141183460469231731687303715884105727i128) == 2i128);
/// assert!(0i128.wrapping_sub((-170141183460469231731687303715884105728i128))
///         == (-170141183460469231731687303715884105728i128));
/// assert!(1i128.wrapping_sub((-170141183460469231731687303715884105728i128))
///         == (-170141183460469231731687303715884105727i128));
/// assert!(1i128.wrapping_sub((-170141183460469231731687303715884105727i128))
///         == (-170141183460469231731687303715884105728i128));
/// assert!(170141183460469231731687303715884105726i128
///         .wrapping_sub((-170141183460469231731687303715884105728i128)) == (-2i128));
/// assert!(170141183460469231731687303715884105726i128
///         .wrapping_sub((-170141183460469231731687303715884105727i128)) == (-3i128));
/// # }
/// ```
pub fn core_ops_sub_i128_wrapping_sub() {}
/// # Properties for [`i128::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() <= i128::MAX.up() && x.up() - y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (i128::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         .overflowing_sub((-170141183460469231731687303715884105728i128))
///         == (0i128, false));
/// assert!((-170141183460469231731687303715884105728i128)
///         .overflowing_sub((-170141183460469231731687303715884105727i128))
///         == ((-1i128), false));
/// assert!((-170141183460469231731687303715884105728i128).overflowing_sub((-1i128))
///         == ((-170141183460469231731687303715884105727i128), false));
/// assert!((-170141183460469231731687303715884105728i128).overflowing_sub(0i128)
///         == ((-170141183460469231731687303715884105728i128), false));
/// assert!((-170141183460469231731687303715884105727i128)
///         .overflowing_sub((-170141183460469231731687303715884105728i128))
///         == (1i128, false));
/// assert!((-170141183460469231731687303715884105727i128)
///         .overflowing_sub((-170141183460469231731687303715884105727i128))
///         == (0i128, false));
/// assert!((-170141183460469231731687303715884105727i128).overflowing_sub((-1i128))
///         == ((-170141183460469231731687303715884105726i128), false));
/// assert!((-170141183460469231731687303715884105727i128).overflowing_sub(0i128)
///         == ((-170141183460469231731687303715884105727i128), false));
/// assert!((-170141183460469231731687303715884105727i128).overflowing_sub(1i128)
///         == ((-170141183460469231731687303715884105728i128), false));
/// assert!((-1i128).overflowing_sub((-170141183460469231731687303715884105728i128))
///         == (170141183460469231731687303715884105727i128, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() - y.up() > i128::MAX.up() || x.up() - y.up() < i128::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             i128::down(
///                 (-i128::MIN.up() + x.up() - y.up()).rem_euclid(&(-i128::MIN.up() * 2))
///                     + i128::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128).overflowing_sub(1i128)
///         == (170141183460469231731687303715884105727i128, true));
/// assert!((-170141183460469231731687303715884105728i128)
///         .overflowing_sub(170141183460469231731687303715884105726i128) == (2i128, true));
/// assert!((-170141183460469231731687303715884105728i128)
///         .overflowing_sub(170141183460469231731687303715884105727i128) == (1i128, true));
/// assert!((-170141183460469231731687303715884105727i128)
///         .overflowing_sub(170141183460469231731687303715884105726i128) == (3i128, true));
/// assert!((-170141183460469231731687303715884105727i128)
///         .overflowing_sub(170141183460469231731687303715884105727i128) == (2i128, true));
/// assert!(0i128.overflowing_sub((-170141183460469231731687303715884105728i128))
///         == ((-170141183460469231731687303715884105728i128), true));
/// assert!(1i128.overflowing_sub((-170141183460469231731687303715884105728i128))
///         == ((-170141183460469231731687303715884105727i128), true));
/// assert!(1i128.overflowing_sub((-170141183460469231731687303715884105727i128))
///         == ((-170141183460469231731687303715884105728i128), true));
/// assert!(170141183460469231731687303715884105726i128
///         .overflowing_sub((-170141183460469231731687303715884105728i128))
///         == ((-2i128), true));
/// assert!(170141183460469231731687303715884105726i128
///         .overflowing_sub((-170141183460469231731687303715884105727i128))
///         == ((-3i128), true));
/// # }
/// ```
pub fn core_ops_sub_i128_overflowing_sub() {}
/// # Properties for [`i128::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() + y.up() <= i128::MAX.up() && x.up() + y.up() >= i128::MIN.up()`  
/// __Postcondition:__ `x + y == i128::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128) + 0i128
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105728i128) + 1i128
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         + 170141183460469231731687303715884105726i128 == (-2i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         + 170141183460469231731687303715884105727i128 == (-1i128));
/// assert!((-170141183460469231731687303715884105727i128) + (-1i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105727i128) + 0i128
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105727i128) + 1i128
///         == (-170141183460469231731687303715884105726i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         + 170141183460469231731687303715884105726i128 == (-1i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         + 170141183460469231731687303715884105727i128 == 0i128);
/// assert!((-1i128) + (-170141183460469231731687303715884105727i128)
///         == (-170141183460469231731687303715884105728i128));
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `x.up() + y.up() > i128::MAX.up() || x.up() + y.up() < i128::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(
///         (- 170141183460469231731687303715884105728i128) + (-
///         170141183460469231731687303715884105728i128)
///     ));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105728i128) + (-
///         170141183460469231731687303715884105727i128)
///     ));
/// assert!(panics!((- 170141183460469231731687303715884105728i128) + (- 1i128)));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105727i128) + (-
///         170141183460469231731687303715884105728i128)
///     ));
/// assert!(panics!(
///         (- 170141183460469231731687303715884105727i128) + (-
///         170141183460469231731687303715884105727i128)
///     ));
/// assert!(panics!((- 1i128) + (- 170141183460469231731687303715884105728i128)));
/// assert!(panics!(1i128 + 170141183460469231731687303715884105727i128));
/// assert!(panics!(
///         170141183460469231731687303715884105726i128 +
///         170141183460469231731687303715884105726i128
///     ));
/// assert!(panics!(
///         170141183460469231731687303715884105726i128 +
///         170141183460469231731687303715884105727i128
///     ));
/// assert!(panics!(170141183460469231731687303715884105727i128 + 1i128));
/// # }
/// ```
pub fn core_ops_add_i128_add() {}
/// # Properties for [`isize::checked_neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x != isize::MIN`  
/// __Postcondition:__ `x.checked_neg() == Some(isize::down(-x.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775807isize).checked_neg() == Some(9223372036854775807isize));
/// assert!((-1isize).checked_neg() == Some(1isize));
/// assert!(0isize.checked_neg() == Some(0isize));
/// assert!(1isize.checked_neg() == Some((-1isize)));
/// assert!(9223372036854775806isize.checked_neg() == Some((-9223372036854775806isize)));
/// assert!(9223372036854775807isize.checked_neg() == Some((-9223372036854775807isize)));
/// assert!((-2468253669518672069isize).checked_neg() == Some(2468253669518672069isize));
/// assert!((-2849936932971652588isize).checked_neg() == Some(2849936932971652588isize));
/// assert!(5720514286039556912isize.checked_neg() == Some((-5720514286039556912isize)));
/// assert!(556703848178133930isize.checked_neg() == Some((-556703848178133930isize)));
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x == isize::MIN`  
/// __Postcondition:__ `x.checked_neg() == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).checked_neg() == None);
/// # }
/// ```
pub fn t_isize_checked_neg() {}
/// # Properties for [`isize::neg`]
/// ## Semantics of checked neg
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x != isize::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         x.neg() == isize::down(-x.up())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         (-9223372036854775807isize).neg() == 9223372036854775807isize
///     });
/// assert!({
///         use std::ops::Neg;
///         (-1isize).neg() == 1isize
///     });
/// assert!({
///         use std::ops::Neg;
///         0isize.neg() == 0isize
///     });
/// assert!({
///         use std::ops::Neg;
///         1isize.neg() == (-1isize)
///     });
/// assert!({
///         use std::ops::Neg;
///         9223372036854775806isize.neg() == (-9223372036854775806isize)
///     });
/// assert!({
///         use std::ops::Neg;
///         9223372036854775807isize.neg() == (-9223372036854775807isize)
///     });
/// assert!({
///         use std::ops::Neg;
///         1754184058210101207isize.neg() == (-1754184058210101207isize)
///     });
/// assert!({
///         use std::ops::Neg;
///         (-4924702195596593612isize).neg() == 4924702195596593612isize
///     });
/// assert!({
///         use std::ops::Neg;
///         (-3192340132157188785isize).neg() == 3192340132157188785isize
///     });
/// assert!({
///         use std::ops::Neg;
///         (-8404963545860953688isize).neg() == 8404963545860953688isize
///     });
/// # }
/// ```
/// ## Semantics of checked neg when out of bounds
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x == isize::MIN`  
/// __Postcondition:__ `{
///         use std::ops::Neg;
///         panics!(x.neg())
///     }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         use std::ops::Neg;
///         panics!((- 9223372036854775808isize).neg())
///     });
/// # }
/// ```
pub fn t_isize_neg() {}
/// # Properties for [`isize::overflowing_neg`]
/// ## Semantics of overflowing neg
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x != isize::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (isize::down(-x.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775807isize).overflowing_neg() == (9223372036854775807isize, false));
/// assert!((-1isize).overflowing_neg() == (1isize, false));
/// assert!(0isize.overflowing_neg() == (0isize, false));
/// assert!(1isize.overflowing_neg() == ((-1isize), false));
/// assert!(9223372036854775806isize.overflowing_neg() == ((-9223372036854775806isize), false));
/// assert!(9223372036854775807isize.overflowing_neg() == ((-9223372036854775807isize), false));
/// assert!(2089716406878315418isize.overflowing_neg() == ((-2089716406878315418isize), false));
/// assert!(4291223543272356792isize.overflowing_neg() == ((-4291223543272356792isize), false));
/// assert!((-7411973784467903162isize).overflowing_neg() == (7411973784467903162isize, false));
/// assert!((-4121371156157905624isize).overflowing_neg() == (4121371156157905624isize, false));
/// # }
/// ```
/// ## Semantics of overflowing neg when out of bounds
/// __Inputs:__ `x : isize`  
/// __Precondition:__ `x == isize::MIN`  
/// __Postcondition:__ `x.overflowing_neg() == (isize::MIN, true)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).overflowing_neg() == ((-9223372036854775808isize), true));
/// # }
/// ```
pub fn t_isize_overflowing_neg() {}
/// # Properties for [`isize::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() <= isize::MAX.up() && x.up() - y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x - y == isize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize) - (-9223372036854775808isize) == 0isize);
/// assert!((-9223372036854775808isize) - (-9223372036854775807isize) == (-1isize));
/// assert!((-9223372036854775808isize) - (-1isize) == (-9223372036854775807isize));
/// assert!((-9223372036854775808isize) - 0isize == (-9223372036854775808isize));
/// assert!((-9223372036854775807isize) - (-9223372036854775808isize) == 1isize);
/// assert!((-9223372036854775807isize) - (-9223372036854775807isize) == 0isize);
/// assert!((-9223372036854775807isize) - (-1isize) == (-9223372036854775806isize));
/// assert!((-9223372036854775807isize) - 0isize == (-9223372036854775807isize));
/// assert!((-9223372036854775807isize) - 1isize == (-9223372036854775808isize));
/// assert!((-1isize) - (-9223372036854775808isize) == 9223372036854775807isize);
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() > isize::MAX.up() || x.up() - y.up() < isize::MIN.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 9223372036854775808isize) - 1isize));
/// assert!(panics!((- 9223372036854775808isize) - 9223372036854775806isize));
/// assert!(panics!((- 9223372036854775808isize) - 9223372036854775807isize));
/// assert!(panics!((- 9223372036854775807isize) - 9223372036854775806isize));
/// assert!(panics!((- 9223372036854775807isize) - 9223372036854775807isize));
/// assert!(panics!(0isize - (- 9223372036854775808isize)));
/// assert!(panics!(1isize - (- 9223372036854775808isize)));
/// assert!(panics!(1isize - (- 9223372036854775807isize)));
/// assert!(panics!(9223372036854775806isize - (- 9223372036854775808isize)));
/// assert!(panics!(9223372036854775806isize - (- 9223372036854775807isize)));
/// # }
/// ```
pub fn core_ops_sub_isize_sub() {}
/// # Properties for [`isize::checked_sub`]
/// ## Semantics of non-overflowing checked subtraction
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() <= isize::MAX.up() && x.up() - y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == Some(isize::down(x.up() - y.up()))`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).checked_sub((-9223372036854775808isize)) == Some(0isize));
/// assert!((-9223372036854775808isize).checked_sub((-9223372036854775807isize))
///         == Some((-1isize)));
/// assert!((-9223372036854775808isize).checked_sub((-1isize))
///         == Some((-9223372036854775807isize)));
/// assert!((-9223372036854775808isize).checked_sub(0isize) == Some((-9223372036854775808isize)));
/// assert!((-9223372036854775807isize).checked_sub((-9223372036854775808isize)) == Some(1isize));
/// assert!((-9223372036854775807isize).checked_sub((-9223372036854775807isize)) == Some(0isize));
/// assert!((-9223372036854775807isize).checked_sub((-1isize))
///         == Some((-9223372036854775806isize)));
/// assert!((-9223372036854775807isize).checked_sub(0isize) == Some((-9223372036854775807isize)));
/// assert!((-9223372036854775807isize).checked_sub(1isize) == Some((-9223372036854775808isize)));
/// assert!((-1isize).checked_sub((-9223372036854775808isize)) == Some(9223372036854775807isize));
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() > isize::MAX.up() || x.up() - y.up() < isize::MIN.up()`  
/// __Postcondition:__ `x.checked_sub(y) == None`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).checked_sub(1isize) == None);
/// assert!((-9223372036854775808isize).checked_sub(9223372036854775806isize) == None);
/// assert!((-9223372036854775808isize).checked_sub(9223372036854775807isize) == None);
/// assert!((-9223372036854775807isize).checked_sub(9223372036854775806isize) == None);
/// assert!((-9223372036854775807isize).checked_sub(9223372036854775807isize) == None);
/// assert!(0isize.checked_sub((-9223372036854775808isize)) == None);
/// assert!(1isize.checked_sub((-9223372036854775808isize)) == None);
/// assert!(1isize.checked_sub((-9223372036854775807isize)) == None);
/// assert!(9223372036854775806isize.checked_sub((-9223372036854775808isize)) == None);
/// assert!(9223372036854775806isize.checked_sub((-9223372036854775807isize)) == None);
/// # }
/// ```
pub fn core_ops_sub_isize_checked_sub() {}
/// # Properties for [`isize::wrapping_sub`]
/// ## Semantics of non-overflowing wrapping subtraction
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() <= isize::MAX.up() && x.up() - y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y) == isize::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).wrapping_sub((-9223372036854775808isize)) == 0isize);
/// assert!((-9223372036854775808isize).wrapping_sub((-9223372036854775807isize)) == (-1isize));
/// assert!((-9223372036854775808isize).wrapping_sub((-1isize)) == (-9223372036854775807isize));
/// assert!((-9223372036854775808isize).wrapping_sub(0isize) == (-9223372036854775808isize));
/// assert!((-9223372036854775807isize).wrapping_sub((-9223372036854775808isize)) == 1isize);
/// assert!((-9223372036854775807isize).wrapping_sub((-9223372036854775807isize)) == 0isize);
/// assert!((-9223372036854775807isize).wrapping_sub((-1isize)) == (-9223372036854775806isize));
/// assert!((-9223372036854775807isize).wrapping_sub(0isize) == (-9223372036854775807isize));
/// assert!((-9223372036854775807isize).wrapping_sub(1isize) == (-9223372036854775808isize));
/// assert!((-1isize).wrapping_sub((-9223372036854775808isize)) == 9223372036854775807isize);
/// # }
/// ```
/// ## Semantics of overflowing wrapping subtraction
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() > isize::MAX.up() || x.up() - y.up() < isize::MIN.up()`  
/// __Postcondition:__ `x.wrapping_sub(y)
///         == isize::down(
///             (-isize::MIN.up() + x.up() - y.up()).rem_euclid(&(-isize::MIN.up() * 2))
///                 + isize::MIN.up(),
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).wrapping_sub(1isize) == 9223372036854775807isize);
/// assert!((-9223372036854775808isize).wrapping_sub(9223372036854775806isize) == 2isize);
/// assert!((-9223372036854775808isize).wrapping_sub(9223372036854775807isize) == 1isize);
/// assert!((-9223372036854775807isize).wrapping_sub(9223372036854775806isize) == 3isize);
/// assert!((-9223372036854775807isize).wrapping_sub(9223372036854775807isize) == 2isize);
/// assert!(0isize.wrapping_sub((-9223372036854775808isize)) == (-9223372036854775808isize));
/// assert!(1isize.wrapping_sub((-9223372036854775808isize)) == (-9223372036854775807isize));
/// assert!(1isize.wrapping_sub((-9223372036854775807isize)) == (-9223372036854775808isize));
/// assert!(9223372036854775806isize.wrapping_sub((-9223372036854775808isize)) == (-2isize));
/// assert!(9223372036854775806isize.wrapping_sub((-9223372036854775807isize)) == (-3isize));
/// # }
/// ```
pub fn core_ops_sub_isize_wrapping_sub() {}
/// # Properties for [`isize::overflowing_sub`]
/// ## Semantics of overflowing subtraction when in bounds
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() <= isize::MAX.up() && x.up() - y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y) == (isize::down(x.up() - y.up()), false)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).overflowing_sub((-9223372036854775808isize))
///         == (0isize, false));
/// assert!((-9223372036854775808isize).overflowing_sub((-9223372036854775807isize))
///         == ((-1isize), false));
/// assert!((-9223372036854775808isize).overflowing_sub((-1isize))
///         == ((-9223372036854775807isize), false));
/// assert!((-9223372036854775808isize).overflowing_sub(0isize)
///         == ((-9223372036854775808isize), false));
/// assert!((-9223372036854775807isize).overflowing_sub((-9223372036854775808isize))
///         == (1isize, false));
/// assert!((-9223372036854775807isize).overflowing_sub((-9223372036854775807isize))
///         == (0isize, false));
/// assert!((-9223372036854775807isize).overflowing_sub((-1isize))
///         == ((-9223372036854775806isize), false));
/// assert!((-9223372036854775807isize).overflowing_sub(0isize)
///         == ((-9223372036854775807isize), false));
/// assert!((-9223372036854775807isize).overflowing_sub(1isize)
///         == ((-9223372036854775808isize), false));
/// assert!((-1isize).overflowing_sub((-9223372036854775808isize))
///         == (9223372036854775807isize, false));
/// # }
/// ```
/// ## Semantics of overflowing subtraction when not in bounds
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() - y.up() > isize::MAX.up() || x.up() - y.up() < isize::MIN.up()`  
/// __Postcondition:__ `x.overflowing_sub(y)
///         == (
///             isize::down(
///                 (-isize::MIN.up() + x.up() - y.up()).rem_euclid(&(-isize::MIN.up() * 2))
///                     + isize::MIN.up(),
///             ),
///             true,
///         )`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).overflowing_sub(1isize)
///         == (9223372036854775807isize, true));
/// assert!((-9223372036854775808isize).overflowing_sub(9223372036854775806isize)
///         == (2isize, true));
/// assert!((-9223372036854775808isize).overflowing_sub(9223372036854775807isize)
///         == (1isize, true));
/// assert!((-9223372036854775807isize).overflowing_sub(9223372036854775806isize)
///         == (3isize, true));
/// assert!((-9223372036854775807isize).overflowing_sub(9223372036854775807isize)
///         == (2isize, true));
/// assert!(0isize.overflowing_sub((-9223372036854775808isize))
///         == ((-9223372036854775808isize), true));
/// assert!(1isize.overflowing_sub((-9223372036854775808isize))
///         == ((-9223372036854775807isize), true));
/// assert!(1isize.overflowing_sub((-9223372036854775807isize))
///         == ((-9223372036854775808isize), true));
/// assert!(9223372036854775806isize.overflowing_sub((-9223372036854775808isize))
///         == ((-2isize), true));
/// assert!(9223372036854775806isize.overflowing_sub((-9223372036854775807isize))
///         == ((-3isize), true));
/// # }
/// ```
pub fn core_ops_sub_isize_overflowing_sub() {}
/// # Properties for [`isize::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() + y.up() <= isize::MAX.up() && x.up() + y.up() >= isize::MIN.up()`  
/// __Postcondition:__ `x + y == isize::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize) + 0isize == (-9223372036854775808isize));
/// assert!((-9223372036854775808isize) + 1isize == (-9223372036854775807isize));
/// assert!((-9223372036854775808isize) + 9223372036854775806isize == (-2isize));
/// assert!((-9223372036854775808isize) + 9223372036854775807isize == (-1isize));
/// assert!((-9223372036854775807isize) + (-1isize) == (-9223372036854775808isize));
/// assert!((-9223372036854775807isize) + 0isize == (-9223372036854775807isize));
/// assert!((-9223372036854775807isize) + 1isize == (-9223372036854775806isize));
/// assert!((-9223372036854775807isize) + 9223372036854775806isize == (-1isize));
/// assert!((-9223372036854775807isize) + 9223372036854775807isize == 0isize);
/// assert!((-1isize) + (-9223372036854775807isize) == (-9223372036854775808isize));
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `x.up() + y.up() > isize::MAX.up() || x.up() + y.up() < isize::MIN.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!((- 9223372036854775808isize) + (- 9223372036854775808isize)));
/// assert!(panics!((- 9223372036854775808isize) + (- 9223372036854775807isize)));
/// assert!(panics!((- 9223372036854775808isize) + (- 1isize)));
/// assert!(panics!((- 9223372036854775807isize) + (- 9223372036854775808isize)));
/// assert!(panics!((- 9223372036854775807isize) + (- 9223372036854775807isize)));
/// assert!(panics!((- 1isize) + (- 9223372036854775808isize)));
/// assert!(panics!(1isize + 9223372036854775807isize));
/// assert!(panics!(9223372036854775806isize + 9223372036854775806isize));
/// assert!(panics!(9223372036854775806isize + 9223372036854775807isize));
/// assert!(panics!(9223372036854775807isize + 1isize));
/// # }
/// ```
pub fn core_ops_add_isize_add() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.cmp(&(0u8)) == core::cmp::Ordering::Equal);
/// assert!(0u8.cmp(&(1u8)) == core::cmp::Ordering::Less);
/// assert!(0u8.cmp(&(254u8)) == core::cmp::Ordering::Less);
/// assert!(0u8.cmp(&(255u8)) == core::cmp::Ordering::Less);
/// assert!(1u8.cmp(&(0u8)) == core::cmp::Ordering::Greater);
/// assert!(1u8.cmp(&(1u8)) == core::cmp::Ordering::Equal);
/// assert!(1u8.cmp(&(254u8)) == core::cmp::Ordering::Less);
/// assert!(1u8.cmp(&(255u8)) == core::cmp::Ordering::Less);
/// assert!(254u8.cmp(&(0u8)) == core::cmp::Ordering::Greater);
/// assert!(254u8.cmp(&(1u8)) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.lt(&(0u8)) == false);
/// assert!(0u8.lt(&(1u8)) == true);
/// assert!(0u8.lt(&(254u8)) == true);
/// assert!(0u8.lt(&(255u8)) == true);
/// assert!(1u8.lt(&(0u8)) == false);
/// assert!(1u8.lt(&(1u8)) == false);
/// assert!(1u8.lt(&(254u8)) == true);
/// assert!(1u8.lt(&(255u8)) == true);
/// assert!(254u8.lt(&(0u8)) == false);
/// assert!(254u8.lt(&(1u8)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.gt(&(0u8)) == false);
/// assert!(0u8.gt(&(1u8)) == false);
/// assert!(0u8.gt(&(254u8)) == false);
/// assert!(0u8.gt(&(255u8)) == false);
/// assert!(1u8.gt(&(0u8)) == true);
/// assert!(1u8.gt(&(1u8)) == false);
/// assert!(1u8.gt(&(254u8)) == false);
/// assert!(1u8.gt(&(255u8)) == false);
/// assert!(254u8.gt(&(0u8)) == true);
/// assert!(254u8.gt(&(1u8)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.ge(&(0u8)) == true);
/// assert!(0u8.ge(&(1u8)) == false);
/// assert!(0u8.ge(&(254u8)) == false);
/// assert!(0u8.ge(&(255u8)) == false);
/// assert!(1u8.ge(&(0u8)) == true);
/// assert!(1u8.ge(&(1u8)) == true);
/// assert!(1u8.ge(&(254u8)) == false);
/// assert!(1u8.ge(&(255u8)) == false);
/// assert!(254u8.ge(&(0u8)) == true);
/// assert!(254u8.ge(&(1u8)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u8>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8.le(&(0u8)) == true);
/// assert!(0u8.le(&(1u8)) == true);
/// assert!(0u8.le(&(254u8)) == true);
/// assert!(0u8.le(&(255u8)) == true);
/// assert!(1u8.le(&(0u8)) == false);
/// assert!(1u8.le(&(1u8)) == true);
/// assert!(1u8.le(&(254u8)) == true);
/// assert!(1u8.le(&(255u8)) == true);
/// assert!(254u8.le(&(0u8)) == false);
/// assert!(254u8.le(&(1u8)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u8_le() {}
/// # Properties for [`core::ops::BitXor::<u8>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u8::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 ^ 0u8 == 0u8);
/// assert!(0u8 ^ 1u8 == 1u8);
/// assert!(0u8 ^ 254u8 == 254u8);
/// assert!(0u8 ^ 255u8 == 255u8);
/// assert!(1u8 ^ 0u8 == 1u8);
/// assert!(1u8 ^ 1u8 == 0u8);
/// assert!(1u8 ^ 254u8 == 255u8);
/// assert!(1u8 ^ 255u8 == 254u8);
/// assert!(254u8 ^ 0u8 == 254u8);
/// assert!(254u8 ^ 1u8 == 255u8);
/// # }
/// ```
pub fn core_ops_bit_xor_u8_xor() {}
/// # Properties for [`core::ops::BitAnd::<u8>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u8::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 & 0u8 == 0u8);
/// assert!(0u8 & 1u8 == 0u8);
/// assert!(0u8 & 254u8 == 0u8);
/// assert!(0u8 & 255u8 == 0u8);
/// assert!(1u8 & 0u8 == 0u8);
/// assert!(1u8 & 1u8 == 1u8);
/// assert!(1u8 & 254u8 == 0u8);
/// assert!(1u8 & 255u8 == 1u8);
/// assert!(254u8 & 0u8 == 0u8);
/// assert!(254u8 & 1u8 == 0u8);
/// # }
/// ```
pub fn core_ops_bit_and_u8_and() {}
/// # Properties for [`core::ops::BitAnd::<u8>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u8::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 ^ 0u8 == 0u8);
/// assert!(0u8 ^ 1u8 == 1u8);
/// assert!(0u8 ^ 254u8 == 254u8);
/// assert!(0u8 ^ 255u8 == 255u8);
/// assert!(1u8 ^ 0u8 == 1u8);
/// assert!(1u8 ^ 1u8 == 1u8);
/// assert!(1u8 ^ 254u8 == 255u8);
/// assert!(1u8 ^ 255u8 == 255u8);
/// assert!(254u8 ^ 0u8 == 254u8);
/// assert!(254u8 ^ 1u8 == 255u8);
/// # }
/// ```
pub fn core_ops_bit_and_u8_or() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.cmp(&(0u16)) == core::cmp::Ordering::Equal);
/// assert!(0u16.cmp(&(1u16)) == core::cmp::Ordering::Less);
/// assert!(0u16.cmp(&(65534u16)) == core::cmp::Ordering::Less);
/// assert!(0u16.cmp(&(65535u16)) == core::cmp::Ordering::Less);
/// assert!(1u16.cmp(&(0u16)) == core::cmp::Ordering::Greater);
/// assert!(1u16.cmp(&(1u16)) == core::cmp::Ordering::Equal);
/// assert!(1u16.cmp(&(65534u16)) == core::cmp::Ordering::Less);
/// assert!(1u16.cmp(&(65535u16)) == core::cmp::Ordering::Less);
/// assert!(65534u16.cmp(&(0u16)) == core::cmp::Ordering::Greater);
/// assert!(65534u16.cmp(&(1u16)) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.lt(&(0u16)) == false);
/// assert!(0u16.lt(&(1u16)) == true);
/// assert!(0u16.lt(&(65534u16)) == true);
/// assert!(0u16.lt(&(65535u16)) == true);
/// assert!(1u16.lt(&(0u16)) == false);
/// assert!(1u16.lt(&(1u16)) == false);
/// assert!(1u16.lt(&(65534u16)) == true);
/// assert!(1u16.lt(&(65535u16)) == true);
/// assert!(65534u16.lt(&(0u16)) == false);
/// assert!(65534u16.lt(&(1u16)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.gt(&(0u16)) == false);
/// assert!(0u16.gt(&(1u16)) == false);
/// assert!(0u16.gt(&(65534u16)) == false);
/// assert!(0u16.gt(&(65535u16)) == false);
/// assert!(1u16.gt(&(0u16)) == true);
/// assert!(1u16.gt(&(1u16)) == false);
/// assert!(1u16.gt(&(65534u16)) == false);
/// assert!(1u16.gt(&(65535u16)) == false);
/// assert!(65534u16.gt(&(0u16)) == true);
/// assert!(65534u16.gt(&(1u16)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.ge(&(0u16)) == true);
/// assert!(0u16.ge(&(1u16)) == false);
/// assert!(0u16.ge(&(65534u16)) == false);
/// assert!(0u16.ge(&(65535u16)) == false);
/// assert!(1u16.ge(&(0u16)) == true);
/// assert!(1u16.ge(&(1u16)) == true);
/// assert!(1u16.ge(&(65534u16)) == false);
/// assert!(1u16.ge(&(65535u16)) == false);
/// assert!(65534u16.ge(&(0u16)) == true);
/// assert!(65534u16.ge(&(1u16)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u16>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16.le(&(0u16)) == true);
/// assert!(0u16.le(&(1u16)) == true);
/// assert!(0u16.le(&(65534u16)) == true);
/// assert!(0u16.le(&(65535u16)) == true);
/// assert!(1u16.le(&(0u16)) == false);
/// assert!(1u16.le(&(1u16)) == true);
/// assert!(1u16.le(&(65534u16)) == true);
/// assert!(1u16.le(&(65535u16)) == true);
/// assert!(65534u16.le(&(0u16)) == false);
/// assert!(65534u16.le(&(1u16)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u16_le() {}
/// # Properties for [`core::ops::BitXor::<u16>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u16::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 ^ 0u16 == 0u16);
/// assert!(0u16 ^ 1u16 == 1u16);
/// assert!(0u16 ^ 65534u16 == 65534u16);
/// assert!(0u16 ^ 65535u16 == 65535u16);
/// assert!(1u16 ^ 0u16 == 1u16);
/// assert!(1u16 ^ 1u16 == 0u16);
/// assert!(1u16 ^ 65534u16 == 65535u16);
/// assert!(1u16 ^ 65535u16 == 65534u16);
/// assert!(65534u16 ^ 0u16 == 65534u16);
/// assert!(65534u16 ^ 1u16 == 65535u16);
/// # }
/// ```
pub fn core_ops_bit_xor_u16_xor() {}
/// # Properties for [`core::ops::BitAnd::<u16>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u16::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 & 0u16 == 0u16);
/// assert!(0u16 & 1u16 == 0u16);
/// assert!(0u16 & 65534u16 == 0u16);
/// assert!(0u16 & 65535u16 == 0u16);
/// assert!(1u16 & 0u16 == 0u16);
/// assert!(1u16 & 1u16 == 1u16);
/// assert!(1u16 & 65534u16 == 0u16);
/// assert!(1u16 & 65535u16 == 1u16);
/// assert!(65534u16 & 0u16 == 0u16);
/// assert!(65534u16 & 1u16 == 0u16);
/// # }
/// ```
pub fn core_ops_bit_and_u16_and() {}
/// # Properties for [`core::ops::BitAnd::<u16>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u16, y : u16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u16::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u16 ^ 0u16 == 0u16);
/// assert!(0u16 ^ 1u16 == 1u16);
/// assert!(0u16 ^ 65534u16 == 65534u16);
/// assert!(0u16 ^ 65535u16 == 65535u16);
/// assert!(1u16 ^ 0u16 == 1u16);
/// assert!(1u16 ^ 1u16 == 1u16);
/// assert!(1u16 ^ 65534u16 == 65535u16);
/// assert!(1u16 ^ 65535u16 == 65535u16);
/// assert!(65534u16 ^ 0u16 == 65534u16);
/// assert!(65534u16 ^ 1u16 == 65535u16);
/// # }
/// ```
pub fn core_ops_bit_and_u16_or() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.cmp(&(0u32)) == core::cmp::Ordering::Equal);
/// assert!(0u32.cmp(&(1u32)) == core::cmp::Ordering::Less);
/// assert!(0u32.cmp(&(4294967294u32)) == core::cmp::Ordering::Less);
/// assert!(0u32.cmp(&(4294967295u32)) == core::cmp::Ordering::Less);
/// assert!(1u32.cmp(&(0u32)) == core::cmp::Ordering::Greater);
/// assert!(1u32.cmp(&(1u32)) == core::cmp::Ordering::Equal);
/// assert!(1u32.cmp(&(4294967294u32)) == core::cmp::Ordering::Less);
/// assert!(1u32.cmp(&(4294967295u32)) == core::cmp::Ordering::Less);
/// assert!(4294967294u32.cmp(&(0u32)) == core::cmp::Ordering::Greater);
/// assert!(4294967294u32.cmp(&(1u32)) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.lt(&(0u32)) == false);
/// assert!(0u32.lt(&(1u32)) == true);
/// assert!(0u32.lt(&(4294967294u32)) == true);
/// assert!(0u32.lt(&(4294967295u32)) == true);
/// assert!(1u32.lt(&(0u32)) == false);
/// assert!(1u32.lt(&(1u32)) == false);
/// assert!(1u32.lt(&(4294967294u32)) == true);
/// assert!(1u32.lt(&(4294967295u32)) == true);
/// assert!(4294967294u32.lt(&(0u32)) == false);
/// assert!(4294967294u32.lt(&(1u32)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.gt(&(0u32)) == false);
/// assert!(0u32.gt(&(1u32)) == false);
/// assert!(0u32.gt(&(4294967294u32)) == false);
/// assert!(0u32.gt(&(4294967295u32)) == false);
/// assert!(1u32.gt(&(0u32)) == true);
/// assert!(1u32.gt(&(1u32)) == false);
/// assert!(1u32.gt(&(4294967294u32)) == false);
/// assert!(1u32.gt(&(4294967295u32)) == false);
/// assert!(4294967294u32.gt(&(0u32)) == true);
/// assert!(4294967294u32.gt(&(1u32)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.ge(&(0u32)) == true);
/// assert!(0u32.ge(&(1u32)) == false);
/// assert!(0u32.ge(&(4294967294u32)) == false);
/// assert!(0u32.ge(&(4294967295u32)) == false);
/// assert!(1u32.ge(&(0u32)) == true);
/// assert!(1u32.ge(&(1u32)) == true);
/// assert!(1u32.ge(&(4294967294u32)) == false);
/// assert!(1u32.ge(&(4294967295u32)) == false);
/// assert!(4294967294u32.ge(&(0u32)) == true);
/// assert!(4294967294u32.ge(&(1u32)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u32>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32.le(&(0u32)) == true);
/// assert!(0u32.le(&(1u32)) == true);
/// assert!(0u32.le(&(4294967294u32)) == true);
/// assert!(0u32.le(&(4294967295u32)) == true);
/// assert!(1u32.le(&(0u32)) == false);
/// assert!(1u32.le(&(1u32)) == true);
/// assert!(1u32.le(&(4294967294u32)) == true);
/// assert!(1u32.le(&(4294967295u32)) == true);
/// assert!(4294967294u32.le(&(0u32)) == false);
/// assert!(4294967294u32.le(&(1u32)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u32_le() {}
/// # Properties for [`core::ops::BitXor::<u32>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u32::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 ^ 0u32 == 0u32);
/// assert!(0u32 ^ 1u32 == 1u32);
/// assert!(0u32 ^ 4294967294u32 == 4294967294u32);
/// assert!(0u32 ^ 4294967295u32 == 4294967295u32);
/// assert!(1u32 ^ 0u32 == 1u32);
/// assert!(1u32 ^ 1u32 == 0u32);
/// assert!(1u32 ^ 4294967294u32 == 4294967295u32);
/// assert!(1u32 ^ 4294967295u32 == 4294967294u32);
/// assert!(4294967294u32 ^ 0u32 == 4294967294u32);
/// assert!(4294967294u32 ^ 1u32 == 4294967295u32);
/// # }
/// ```
pub fn core_ops_bit_xor_u32_xor() {}
/// # Properties for [`core::ops::BitAnd::<u32>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u32::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 & 0u32 == 0u32);
/// assert!(0u32 & 1u32 == 0u32);
/// assert!(0u32 & 4294967294u32 == 0u32);
/// assert!(0u32 & 4294967295u32 == 0u32);
/// assert!(1u32 & 0u32 == 0u32);
/// assert!(1u32 & 1u32 == 1u32);
/// assert!(1u32 & 4294967294u32 == 0u32);
/// assert!(1u32 & 4294967295u32 == 1u32);
/// assert!(4294967294u32 & 0u32 == 0u32);
/// assert!(4294967294u32 & 1u32 == 0u32);
/// # }
/// ```
pub fn core_ops_bit_and_u32_and() {}
/// # Properties for [`core::ops::BitAnd::<u32>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u32, y : u32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u32::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u32 ^ 0u32 == 0u32);
/// assert!(0u32 ^ 1u32 == 1u32);
/// assert!(0u32 ^ 4294967294u32 == 4294967294u32);
/// assert!(0u32 ^ 4294967295u32 == 4294967295u32);
/// assert!(1u32 ^ 0u32 == 1u32);
/// assert!(1u32 ^ 1u32 == 1u32);
/// assert!(1u32 ^ 4294967294u32 == 4294967295u32);
/// assert!(1u32 ^ 4294967295u32 == 4294967295u32);
/// assert!(4294967294u32 ^ 0u32 == 4294967294u32);
/// assert!(4294967294u32 ^ 1u32 == 4294967295u32);
/// # }
/// ```
pub fn core_ops_bit_and_u32_or() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.cmp(&(0u64)) == core::cmp::Ordering::Equal);
/// assert!(0u64.cmp(&(1u64)) == core::cmp::Ordering::Less);
/// assert!(0u64.cmp(&(18446744073709551614u64)) == core::cmp::Ordering::Less);
/// assert!(0u64.cmp(&(18446744073709551615u64)) == core::cmp::Ordering::Less);
/// assert!(1u64.cmp(&(0u64)) == core::cmp::Ordering::Greater);
/// assert!(1u64.cmp(&(1u64)) == core::cmp::Ordering::Equal);
/// assert!(1u64.cmp(&(18446744073709551614u64)) == core::cmp::Ordering::Less);
/// assert!(1u64.cmp(&(18446744073709551615u64)) == core::cmp::Ordering::Less);
/// assert!(18446744073709551614u64.cmp(&(0u64)) == core::cmp::Ordering::Greater);
/// assert!(18446744073709551614u64.cmp(&(1u64)) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.lt(&(0u64)) == false);
/// assert!(0u64.lt(&(1u64)) == true);
/// assert!(0u64.lt(&(18446744073709551614u64)) == true);
/// assert!(0u64.lt(&(18446744073709551615u64)) == true);
/// assert!(1u64.lt(&(0u64)) == false);
/// assert!(1u64.lt(&(1u64)) == false);
/// assert!(1u64.lt(&(18446744073709551614u64)) == true);
/// assert!(1u64.lt(&(18446744073709551615u64)) == true);
/// assert!(18446744073709551614u64.lt(&(0u64)) == false);
/// assert!(18446744073709551614u64.lt(&(1u64)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.gt(&(0u64)) == false);
/// assert!(0u64.gt(&(1u64)) == false);
/// assert!(0u64.gt(&(18446744073709551614u64)) == false);
/// assert!(0u64.gt(&(18446744073709551615u64)) == false);
/// assert!(1u64.gt(&(0u64)) == true);
/// assert!(1u64.gt(&(1u64)) == false);
/// assert!(1u64.gt(&(18446744073709551614u64)) == false);
/// assert!(1u64.gt(&(18446744073709551615u64)) == false);
/// assert!(18446744073709551614u64.gt(&(0u64)) == true);
/// assert!(18446744073709551614u64.gt(&(1u64)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.ge(&(0u64)) == true);
/// assert!(0u64.ge(&(1u64)) == false);
/// assert!(0u64.ge(&(18446744073709551614u64)) == false);
/// assert!(0u64.ge(&(18446744073709551615u64)) == false);
/// assert!(1u64.ge(&(0u64)) == true);
/// assert!(1u64.ge(&(1u64)) == true);
/// assert!(1u64.ge(&(18446744073709551614u64)) == false);
/// assert!(1u64.ge(&(18446744073709551615u64)) == false);
/// assert!(18446744073709551614u64.ge(&(0u64)) == true);
/// assert!(18446744073709551614u64.ge(&(1u64)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u64>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64.le(&(0u64)) == true);
/// assert!(0u64.le(&(1u64)) == true);
/// assert!(0u64.le(&(18446744073709551614u64)) == true);
/// assert!(0u64.le(&(18446744073709551615u64)) == true);
/// assert!(1u64.le(&(0u64)) == false);
/// assert!(1u64.le(&(1u64)) == true);
/// assert!(1u64.le(&(18446744073709551614u64)) == true);
/// assert!(1u64.le(&(18446744073709551615u64)) == true);
/// assert!(18446744073709551614u64.le(&(0u64)) == false);
/// assert!(18446744073709551614u64.le(&(1u64)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u64_le() {}
/// # Properties for [`core::ops::BitXor::<u64>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u64::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 ^ 0u64 == 0u64);
/// assert!(0u64 ^ 1u64 == 1u64);
/// assert!(0u64 ^ 18446744073709551614u64 == 18446744073709551614u64);
/// assert!(0u64 ^ 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(1u64 ^ 0u64 == 1u64);
/// assert!(1u64 ^ 1u64 == 0u64);
/// assert!(1u64 ^ 18446744073709551614u64 == 18446744073709551615u64);
/// assert!(1u64 ^ 18446744073709551615u64 == 18446744073709551614u64);
/// assert!(18446744073709551614u64 ^ 0u64 == 18446744073709551614u64);
/// assert!(18446744073709551614u64 ^ 1u64 == 18446744073709551615u64);
/// # }
/// ```
pub fn core_ops_bit_xor_u64_xor() {}
/// # Properties for [`core::ops::BitAnd::<u64>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u64::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 & 0u64 == 0u64);
/// assert!(0u64 & 1u64 == 0u64);
/// assert!(0u64 & 18446744073709551614u64 == 0u64);
/// assert!(0u64 & 18446744073709551615u64 == 0u64);
/// assert!(1u64 & 0u64 == 0u64);
/// assert!(1u64 & 1u64 == 1u64);
/// assert!(1u64 & 18446744073709551614u64 == 0u64);
/// assert!(1u64 & 18446744073709551615u64 == 1u64);
/// assert!(18446744073709551614u64 & 0u64 == 0u64);
/// assert!(18446744073709551614u64 & 1u64 == 0u64);
/// # }
/// ```
pub fn core_ops_bit_and_u64_and() {}
/// # Properties for [`core::ops::BitAnd::<u64>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u64, y : u64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u64::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u64 ^ 0u64 == 0u64);
/// assert!(0u64 ^ 1u64 == 1u64);
/// assert!(0u64 ^ 18446744073709551614u64 == 18446744073709551614u64);
/// assert!(0u64 ^ 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(1u64 ^ 0u64 == 1u64);
/// assert!(1u64 ^ 1u64 == 1u64);
/// assert!(1u64 ^ 18446744073709551614u64 == 18446744073709551615u64);
/// assert!(1u64 ^ 18446744073709551615u64 == 18446744073709551615u64);
/// assert!(18446744073709551614u64 ^ 0u64 == 18446744073709551614u64);
/// assert!(18446744073709551614u64 ^ 1u64 == 18446744073709551615u64);
/// # }
/// ```
pub fn core_ops_bit_and_u64_or() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.cmp(&(0u128)) == core::cmp::Ordering::Equal);
/// assert!(0u128.cmp(&(1u128)) == core::cmp::Ordering::Less);
/// assert!(0u128.cmp(&(340282366920938463463374607431768211454u128))
///         == core::cmp::Ordering::Less);
/// assert!(0u128.cmp(&(340282366920938463463374607431768211455u128))
///         == core::cmp::Ordering::Less);
/// assert!(1u128.cmp(&(0u128)) == core::cmp::Ordering::Greater);
/// assert!(1u128.cmp(&(1u128)) == core::cmp::Ordering::Equal);
/// assert!(1u128.cmp(&(340282366920938463463374607431768211454u128))
///         == core::cmp::Ordering::Less);
/// assert!(1u128.cmp(&(340282366920938463463374607431768211455u128))
///         == core::cmp::Ordering::Less);
/// assert!(340282366920938463463374607431768211454u128.cmp(&(0u128))
///         == core::cmp::Ordering::Greater);
/// assert!(340282366920938463463374607431768211454u128.cmp(&(1u128))
///         == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.lt(&(0u128)) == false);
/// assert!(0u128.lt(&(1u128)) == true);
/// assert!(0u128.lt(&(340282366920938463463374607431768211454u128)) == true);
/// assert!(0u128.lt(&(340282366920938463463374607431768211455u128)) == true);
/// assert!(1u128.lt(&(0u128)) == false);
/// assert!(1u128.lt(&(1u128)) == false);
/// assert!(1u128.lt(&(340282366920938463463374607431768211454u128)) == true);
/// assert!(1u128.lt(&(340282366920938463463374607431768211455u128)) == true);
/// assert!(340282366920938463463374607431768211454u128.lt(&(0u128)) == false);
/// assert!(340282366920938463463374607431768211454u128.lt(&(1u128)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.gt(&(0u128)) == false);
/// assert!(0u128.gt(&(1u128)) == false);
/// assert!(0u128.gt(&(340282366920938463463374607431768211454u128)) == false);
/// assert!(0u128.gt(&(340282366920938463463374607431768211455u128)) == false);
/// assert!(1u128.gt(&(0u128)) == true);
/// assert!(1u128.gt(&(1u128)) == false);
/// assert!(1u128.gt(&(340282366920938463463374607431768211454u128)) == false);
/// assert!(1u128.gt(&(340282366920938463463374607431768211455u128)) == false);
/// assert!(340282366920938463463374607431768211454u128.gt(&(0u128)) == true);
/// assert!(340282366920938463463374607431768211454u128.gt(&(1u128)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.ge(&(0u128)) == true);
/// assert!(0u128.ge(&(1u128)) == false);
/// assert!(0u128.ge(&(340282366920938463463374607431768211454u128)) == false);
/// assert!(0u128.ge(&(340282366920938463463374607431768211455u128)) == false);
/// assert!(1u128.ge(&(0u128)) == true);
/// assert!(1u128.ge(&(1u128)) == true);
/// assert!(1u128.ge(&(340282366920938463463374607431768211454u128)) == false);
/// assert!(1u128.ge(&(340282366920938463463374607431768211455u128)) == false);
/// assert!(340282366920938463463374607431768211454u128.ge(&(0u128)) == true);
/// assert!(340282366920938463463374607431768211454u128.ge(&(1u128)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<u128>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128.le(&(0u128)) == true);
/// assert!(0u128.le(&(1u128)) == true);
/// assert!(0u128.le(&(340282366920938463463374607431768211454u128)) == true);
/// assert!(0u128.le(&(340282366920938463463374607431768211455u128)) == true);
/// assert!(1u128.le(&(0u128)) == false);
/// assert!(1u128.le(&(1u128)) == true);
/// assert!(1u128.le(&(340282366920938463463374607431768211454u128)) == true);
/// assert!(1u128.le(&(340282366920938463463374607431768211455u128)) == true);
/// assert!(340282366920938463463374607431768211454u128.le(&(0u128)) == false);
/// assert!(340282366920938463463374607431768211454u128.le(&(1u128)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_u128_le() {}
/// # Properties for [`core::ops::BitXor::<u128>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u128::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 ^ 0u128 == 0u128);
/// assert!(0u128 ^ 1u128 == 1u128);
/// assert!(0u128 ^ 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128 ^ 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128 ^ 0u128 == 1u128);
/// assert!(1u128 ^ 1u128 == 0u128);
/// assert!(1u128 ^ 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128 ^ 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128 ^ 0u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128 ^ 1u128
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
pub fn core_ops_bit_xor_u128_xor() {}
/// # Properties for [`core::ops::BitAnd::<u128>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == u128::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 & 0u128 == 0u128);
/// assert!(0u128 & 1u128 == 0u128);
/// assert!(0u128 & 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(0u128 & 340282366920938463463374607431768211455u128 == 0u128);
/// assert!(1u128 & 0u128 == 0u128);
/// assert!(1u128 & 1u128 == 1u128);
/// assert!(1u128 & 340282366920938463463374607431768211454u128 == 0u128);
/// assert!(1u128 & 340282366920938463463374607431768211455u128 == 1u128);
/// assert!(340282366920938463463374607431768211454u128 & 0u128 == 0u128);
/// assert!(340282366920938463463374607431768211454u128 & 1u128 == 0u128);
/// # }
/// ```
pub fn core_ops_bit_and_u128_and() {}
/// # Properties for [`core::ops::BitAnd::<u128>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : u128, y : u128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == u128::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u128 ^ 0u128 == 0u128);
/// assert!(0u128 ^ 1u128 == 1u128);
/// assert!(0u128 ^ 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(0u128 ^ 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128 ^ 0u128 == 1u128);
/// assert!(1u128 ^ 1u128 == 1u128);
/// assert!(1u128 ^ 340282366920938463463374607431768211454u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(1u128 ^ 340282366920938463463374607431768211455u128
///         == 340282366920938463463374607431768211455u128);
/// assert!(340282366920938463463374607431768211454u128 ^ 0u128
///         == 340282366920938463463374607431768211454u128);
/// assert!(340282366920938463463374607431768211454u128 ^ 1u128
///         == 340282366920938463463374607431768211455u128);
/// # }
/// ```
pub fn core_ops_bit_and_u128_or() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.cmp(&(0usize)) == core::cmp::Ordering::Equal);
/// assert!(0usize.cmp(&(1usize)) == core::cmp::Ordering::Less);
/// assert!(0usize.cmp(&(18446744073709551614usize)) == core::cmp::Ordering::Less);
/// assert!(0usize.cmp(&(18446744073709551615usize)) == core::cmp::Ordering::Less);
/// assert!(1usize.cmp(&(0usize)) == core::cmp::Ordering::Greater);
/// assert!(1usize.cmp(&(1usize)) == core::cmp::Ordering::Equal);
/// assert!(1usize.cmp(&(18446744073709551614usize)) == core::cmp::Ordering::Less);
/// assert!(1usize.cmp(&(18446744073709551615usize)) == core::cmp::Ordering::Less);
/// assert!(18446744073709551614usize.cmp(&(0usize)) == core::cmp::Ordering::Greater);
/// assert!(18446744073709551614usize.cmp(&(1usize)) == core::cmp::Ordering::Greater);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.lt(&(0usize)) == false);
/// assert!(0usize.lt(&(1usize)) == true);
/// assert!(0usize.lt(&(18446744073709551614usize)) == true);
/// assert!(0usize.lt(&(18446744073709551615usize)) == true);
/// assert!(1usize.lt(&(0usize)) == false);
/// assert!(1usize.lt(&(1usize)) == false);
/// assert!(1usize.lt(&(18446744073709551614usize)) == true);
/// assert!(1usize.lt(&(18446744073709551615usize)) == true);
/// assert!(18446744073709551614usize.lt(&(0usize)) == false);
/// assert!(18446744073709551614usize.lt(&(1usize)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.gt(&(0usize)) == false);
/// assert!(0usize.gt(&(1usize)) == false);
/// assert!(0usize.gt(&(18446744073709551614usize)) == false);
/// assert!(0usize.gt(&(18446744073709551615usize)) == false);
/// assert!(1usize.gt(&(0usize)) == true);
/// assert!(1usize.gt(&(1usize)) == false);
/// assert!(1usize.gt(&(18446744073709551614usize)) == false);
/// assert!(1usize.gt(&(18446744073709551615usize)) == false);
/// assert!(18446744073709551614usize.gt(&(0usize)) == true);
/// assert!(18446744073709551614usize.gt(&(1usize)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.ge(&(0usize)) == true);
/// assert!(0usize.ge(&(1usize)) == false);
/// assert!(0usize.ge(&(18446744073709551614usize)) == false);
/// assert!(0usize.ge(&(18446744073709551615usize)) == false);
/// assert!(1usize.ge(&(0usize)) == true);
/// assert!(1usize.ge(&(1usize)) == true);
/// assert!(1usize.ge(&(18446744073709551614usize)) == false);
/// assert!(1usize.ge(&(18446744073709551615usize)) == false);
/// assert!(18446744073709551614usize.ge(&(0usize)) == true);
/// assert!(18446744073709551614usize.ge(&(1usize)) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<usize>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize.le(&(0usize)) == true);
/// assert!(0usize.le(&(1usize)) == true);
/// assert!(0usize.le(&(18446744073709551614usize)) == true);
/// assert!(0usize.le(&(18446744073709551615usize)) == true);
/// assert!(1usize.le(&(0usize)) == false);
/// assert!(1usize.le(&(1usize)) == true);
/// assert!(1usize.le(&(18446744073709551614usize)) == true);
/// assert!(1usize.le(&(18446744073709551615usize)) == true);
/// assert!(18446744073709551614usize.le(&(0usize)) == false);
/// assert!(18446744073709551614usize.le(&(1usize)) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_usize_le() {}
/// # Properties for [`core::ops::BitXor::<usize>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == usize::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize ^ 0usize == 0usize);
/// assert!(0usize ^ 1usize == 1usize);
/// assert!(0usize ^ 18446744073709551614usize == 18446744073709551614usize);
/// assert!(0usize ^ 18446744073709551615usize == 18446744073709551615usize);
/// assert!(1usize ^ 0usize == 1usize);
/// assert!(1usize ^ 1usize == 0usize);
/// assert!(1usize ^ 18446744073709551614usize == 18446744073709551615usize);
/// assert!(1usize ^ 18446744073709551615usize == 18446744073709551614usize);
/// assert!(18446744073709551614usize ^ 0usize == 18446744073709551614usize);
/// assert!(18446744073709551614usize ^ 1usize == 18446744073709551615usize);
/// # }
/// ```
pub fn core_ops_bit_xor_usize_xor() {}
/// # Properties for [`core::ops::BitAnd::<usize>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == usize::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize & 0usize == 0usize);
/// assert!(0usize & 1usize == 0usize);
/// assert!(0usize & 18446744073709551614usize == 0usize);
/// assert!(0usize & 18446744073709551615usize == 0usize);
/// assert!(1usize & 0usize == 0usize);
/// assert!(1usize & 1usize == 1usize);
/// assert!(1usize & 18446744073709551614usize == 0usize);
/// assert!(1usize & 18446744073709551615usize == 1usize);
/// assert!(18446744073709551614usize & 0usize == 0usize);
/// assert!(18446744073709551614usize & 1usize == 0usize);
/// # }
/// ```
pub fn core_ops_bit_and_usize_and() {}
/// # Properties for [`core::ops::BitAnd::<usize>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : usize, y : usize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == usize::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0usize ^ 0usize == 0usize);
/// assert!(0usize ^ 1usize == 1usize);
/// assert!(0usize ^ 18446744073709551614usize == 18446744073709551614usize);
/// assert!(0usize ^ 18446744073709551615usize == 18446744073709551615usize);
/// assert!(1usize ^ 0usize == 1usize);
/// assert!(1usize ^ 1usize == 1usize);
/// assert!(1usize ^ 18446744073709551614usize == 18446744073709551615usize);
/// assert!(1usize ^ 18446744073709551615usize == 18446744073709551615usize);
/// assert!(18446744073709551614usize ^ 0usize == 18446744073709551614usize);
/// assert!(18446744073709551614usize ^ 1usize == 18446744073709551615usize);
/// # }
/// ```
pub fn core_ops_bit_and_usize_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).cmp(&((-128i8))) == core::cmp::Ordering::Equal);
/// assert!((-128i8).cmp(&((-127i8))) == core::cmp::Ordering::Less);
/// assert!((-128i8).cmp(&((-1i8))) == core::cmp::Ordering::Less);
/// assert!((-128i8).cmp(&(0i8)) == core::cmp::Ordering::Less);
/// assert!((-128i8).cmp(&(1i8)) == core::cmp::Ordering::Less);
/// assert!((-128i8).cmp(&(126i8)) == core::cmp::Ordering::Less);
/// assert!((-128i8).cmp(&(127i8)) == core::cmp::Ordering::Less);
/// assert!((-127i8).cmp(&((-128i8))) == core::cmp::Ordering::Greater);
/// assert!((-127i8).cmp(&((-127i8))) == core::cmp::Ordering::Equal);
/// assert!((-127i8).cmp(&((-1i8))) == core::cmp::Ordering::Less);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).lt(&((-128i8))) == false);
/// assert!((-128i8).lt(&((-127i8))) == true);
/// assert!((-128i8).lt(&((-1i8))) == true);
/// assert!((-128i8).lt(&(0i8)) == true);
/// assert!((-128i8).lt(&(1i8)) == true);
/// assert!((-128i8).lt(&(126i8)) == true);
/// assert!((-128i8).lt(&(127i8)) == true);
/// assert!((-127i8).lt(&((-128i8))) == false);
/// assert!((-127i8).lt(&((-127i8))) == false);
/// assert!((-127i8).lt(&((-1i8))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).gt(&((-128i8))) == false);
/// assert!((-128i8).gt(&((-127i8))) == false);
/// assert!((-128i8).gt(&((-1i8))) == false);
/// assert!((-128i8).gt(&(0i8)) == false);
/// assert!((-128i8).gt(&(1i8)) == false);
/// assert!((-128i8).gt(&(126i8)) == false);
/// assert!((-128i8).gt(&(127i8)) == false);
/// assert!((-127i8).gt(&((-128i8))) == true);
/// assert!((-127i8).gt(&((-127i8))) == false);
/// assert!((-127i8).gt(&((-1i8))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).ge(&((-128i8))) == true);
/// assert!((-128i8).ge(&((-127i8))) == false);
/// assert!((-128i8).ge(&((-1i8))) == false);
/// assert!((-128i8).ge(&(0i8)) == false);
/// assert!((-128i8).ge(&(1i8)) == false);
/// assert!((-128i8).ge(&(126i8)) == false);
/// assert!((-128i8).ge(&(127i8)) == false);
/// assert!((-127i8).ge(&((-128i8))) == true);
/// assert!((-127i8).ge(&((-127i8))) == true);
/// assert!((-127i8).ge(&((-1i8))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i8>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8).le(&((-128i8))) == true);
/// assert!((-128i8).le(&((-127i8))) == true);
/// assert!((-128i8).le(&((-1i8))) == true);
/// assert!((-128i8).le(&(0i8)) == true);
/// assert!((-128i8).le(&(1i8)) == true);
/// assert!((-128i8).le(&(126i8)) == true);
/// assert!((-128i8).le(&(127i8)) == true);
/// assert!((-127i8).le(&((-128i8))) == false);
/// assert!((-127i8).le(&((-127i8))) == true);
/// assert!((-127i8).le(&((-1i8))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i8_le() {}
/// # Properties for [`core::ops::BitXor::<i8>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i8::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8) ^ (-128i8) == 0i8);
/// assert!((-128i8) ^ (-127i8) == 1i8);
/// assert!((-128i8) ^ (-1i8) == 127i8);
/// assert!((-128i8) ^ 0i8 == (-128i8));
/// assert!((-128i8) ^ 1i8 == (-127i8));
/// assert!((-128i8) ^ 126i8 == (-2i8));
/// assert!((-128i8) ^ 127i8 == (-1i8));
/// assert!((-127i8) ^ (-128i8) == 1i8);
/// assert!((-127i8) ^ (-127i8) == 0i8);
/// assert!((-127i8) ^ (-1i8) == 126i8);
/// # }
/// ```
pub fn core_ops_bit_xor_i8_xor() {}
/// # Properties for [`core::ops::BitAnd::<i8>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i8::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8) & (-128i8) == (-128i8));
/// assert!((-128i8) & (-127i8) == (-128i8));
/// assert!((-128i8) & (-1i8) == (-128i8));
/// assert!((-128i8) & 0i8 == 0i8);
/// assert!((-128i8) & 1i8 == 0i8);
/// assert!((-128i8) & 126i8 == 0i8);
/// assert!((-128i8) & 127i8 == 0i8);
/// assert!((-127i8) & (-128i8) == (-128i8));
/// assert!((-127i8) & (-127i8) == (-127i8));
/// assert!((-127i8) & (-1i8) == (-127i8));
/// # }
/// ```
pub fn core_ops_bit_and_i8_and() {}
/// # Properties for [`core::ops::BitAnd::<i8>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i8::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-128i8) ^ (-128i8) == (-128i8));
/// assert!((-128i8) ^ (-127i8) == (-127i8));
/// assert!((-128i8) ^ (-1i8) == (-1i8));
/// assert!((-128i8) ^ 0i8 == (-128i8));
/// assert!((-128i8) ^ 1i8 == (-127i8));
/// assert!((-128i8) ^ 126i8 == (-2i8));
/// assert!((-128i8) ^ 127i8 == (-1i8));
/// assert!((-127i8) ^ (-128i8) == (-127i8));
/// assert!((-127i8) ^ (-127i8) == (-127i8));
/// assert!((-127i8) ^ (-1i8) == (-1i8));
/// # }
/// ```
pub fn core_ops_bit_and_i8_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).cmp(&((-32768i16))) == core::cmp::Ordering::Equal);
/// assert!((-32768i16).cmp(&((-32767i16))) == core::cmp::Ordering::Less);
/// assert!((-32768i16).cmp(&((-1i16))) == core::cmp::Ordering::Less);
/// assert!((-32768i16).cmp(&(0i16)) == core::cmp::Ordering::Less);
/// assert!((-32768i16).cmp(&(1i16)) == core::cmp::Ordering::Less);
/// assert!((-32768i16).cmp(&(32766i16)) == core::cmp::Ordering::Less);
/// assert!((-32768i16).cmp(&(32767i16)) == core::cmp::Ordering::Less);
/// assert!((-32767i16).cmp(&((-32768i16))) == core::cmp::Ordering::Greater);
/// assert!((-32767i16).cmp(&((-32767i16))) == core::cmp::Ordering::Equal);
/// assert!((-32767i16).cmp(&((-1i16))) == core::cmp::Ordering::Less);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).lt(&((-32768i16))) == false);
/// assert!((-32768i16).lt(&((-32767i16))) == true);
/// assert!((-32768i16).lt(&((-1i16))) == true);
/// assert!((-32768i16).lt(&(0i16)) == true);
/// assert!((-32768i16).lt(&(1i16)) == true);
/// assert!((-32768i16).lt(&(32766i16)) == true);
/// assert!((-32768i16).lt(&(32767i16)) == true);
/// assert!((-32767i16).lt(&((-32768i16))) == false);
/// assert!((-32767i16).lt(&((-32767i16))) == false);
/// assert!((-32767i16).lt(&((-1i16))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).gt(&((-32768i16))) == false);
/// assert!((-32768i16).gt(&((-32767i16))) == false);
/// assert!((-32768i16).gt(&((-1i16))) == false);
/// assert!((-32768i16).gt(&(0i16)) == false);
/// assert!((-32768i16).gt(&(1i16)) == false);
/// assert!((-32768i16).gt(&(32766i16)) == false);
/// assert!((-32768i16).gt(&(32767i16)) == false);
/// assert!((-32767i16).gt(&((-32768i16))) == true);
/// assert!((-32767i16).gt(&((-32767i16))) == false);
/// assert!((-32767i16).gt(&((-1i16))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).ge(&((-32768i16))) == true);
/// assert!((-32768i16).ge(&((-32767i16))) == false);
/// assert!((-32768i16).ge(&((-1i16))) == false);
/// assert!((-32768i16).ge(&(0i16)) == false);
/// assert!((-32768i16).ge(&(1i16)) == false);
/// assert!((-32768i16).ge(&(32766i16)) == false);
/// assert!((-32768i16).ge(&(32767i16)) == false);
/// assert!((-32767i16).ge(&((-32768i16))) == true);
/// assert!((-32767i16).ge(&((-32767i16))) == true);
/// assert!((-32767i16).ge(&((-1i16))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i16>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16).le(&((-32768i16))) == true);
/// assert!((-32768i16).le(&((-32767i16))) == true);
/// assert!((-32768i16).le(&((-1i16))) == true);
/// assert!((-32768i16).le(&(0i16)) == true);
/// assert!((-32768i16).le(&(1i16)) == true);
/// assert!((-32768i16).le(&(32766i16)) == true);
/// assert!((-32768i16).le(&(32767i16)) == true);
/// assert!((-32767i16).le(&((-32768i16))) == false);
/// assert!((-32767i16).le(&((-32767i16))) == true);
/// assert!((-32767i16).le(&((-1i16))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i16_le() {}
/// # Properties for [`core::ops::BitXor::<i16>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i16::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16) ^ (-32768i16) == 0i16);
/// assert!((-32768i16) ^ (-32767i16) == 1i16);
/// assert!((-32768i16) ^ (-1i16) == 32767i16);
/// assert!((-32768i16) ^ 0i16 == (-32768i16));
/// assert!((-32768i16) ^ 1i16 == (-32767i16));
/// assert!((-32768i16) ^ 32766i16 == (-2i16));
/// assert!((-32768i16) ^ 32767i16 == (-1i16));
/// assert!((-32767i16) ^ (-32768i16) == 1i16);
/// assert!((-32767i16) ^ (-32767i16) == 0i16);
/// assert!((-32767i16) ^ (-1i16) == 32766i16);
/// # }
/// ```
pub fn core_ops_bit_xor_i16_xor() {}
/// # Properties for [`core::ops::BitAnd::<i16>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i16::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16) & (-32768i16) == (-32768i16));
/// assert!((-32768i16) & (-32767i16) == (-32768i16));
/// assert!((-32768i16) & (-1i16) == (-32768i16));
/// assert!((-32768i16) & 0i16 == 0i16);
/// assert!((-32768i16) & 1i16 == 0i16);
/// assert!((-32768i16) & 32766i16 == 0i16);
/// assert!((-32768i16) & 32767i16 == 0i16);
/// assert!((-32767i16) & (-32768i16) == (-32768i16));
/// assert!((-32767i16) & (-32767i16) == (-32767i16));
/// assert!((-32767i16) & (-1i16) == (-32767i16));
/// # }
/// ```
pub fn core_ops_bit_and_i16_and() {}
/// # Properties for [`core::ops::BitAnd::<i16>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i16, y : i16`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i16::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-32768i16) ^ (-32768i16) == (-32768i16));
/// assert!((-32768i16) ^ (-32767i16) == (-32767i16));
/// assert!((-32768i16) ^ (-1i16) == (-1i16));
/// assert!((-32768i16) ^ 0i16 == (-32768i16));
/// assert!((-32768i16) ^ 1i16 == (-32767i16));
/// assert!((-32768i16) ^ 32766i16 == (-2i16));
/// assert!((-32768i16) ^ 32767i16 == (-1i16));
/// assert!((-32767i16) ^ (-32768i16) == (-32767i16));
/// assert!((-32767i16) ^ (-32767i16) == (-32767i16));
/// assert!((-32767i16) ^ (-1i16) == (-1i16));
/// # }
/// ```
pub fn core_ops_bit_and_i16_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).cmp(&((-2147483648i32))) == core::cmp::Ordering::Equal);
/// assert!((-2147483648i32).cmp(&((-2147483647i32))) == core::cmp::Ordering::Less);
/// assert!((-2147483648i32).cmp(&((-1i32))) == core::cmp::Ordering::Less);
/// assert!((-2147483648i32).cmp(&(0i32)) == core::cmp::Ordering::Less);
/// assert!((-2147483648i32).cmp(&(1i32)) == core::cmp::Ordering::Less);
/// assert!((-2147483648i32).cmp(&(2147483646i32)) == core::cmp::Ordering::Less);
/// assert!((-2147483648i32).cmp(&(2147483647i32)) == core::cmp::Ordering::Less);
/// assert!((-2147483647i32).cmp(&((-2147483648i32))) == core::cmp::Ordering::Greater);
/// assert!((-2147483647i32).cmp(&((-2147483647i32))) == core::cmp::Ordering::Equal);
/// assert!((-2147483647i32).cmp(&((-1i32))) == core::cmp::Ordering::Less);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).lt(&((-2147483648i32))) == false);
/// assert!((-2147483648i32).lt(&((-2147483647i32))) == true);
/// assert!((-2147483648i32).lt(&((-1i32))) == true);
/// assert!((-2147483648i32).lt(&(0i32)) == true);
/// assert!((-2147483648i32).lt(&(1i32)) == true);
/// assert!((-2147483648i32).lt(&(2147483646i32)) == true);
/// assert!((-2147483648i32).lt(&(2147483647i32)) == true);
/// assert!((-2147483647i32).lt(&((-2147483648i32))) == false);
/// assert!((-2147483647i32).lt(&((-2147483647i32))) == false);
/// assert!((-2147483647i32).lt(&((-1i32))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).gt(&((-2147483648i32))) == false);
/// assert!((-2147483648i32).gt(&((-2147483647i32))) == false);
/// assert!((-2147483648i32).gt(&((-1i32))) == false);
/// assert!((-2147483648i32).gt(&(0i32)) == false);
/// assert!((-2147483648i32).gt(&(1i32)) == false);
/// assert!((-2147483648i32).gt(&(2147483646i32)) == false);
/// assert!((-2147483648i32).gt(&(2147483647i32)) == false);
/// assert!((-2147483647i32).gt(&((-2147483648i32))) == true);
/// assert!((-2147483647i32).gt(&((-2147483647i32))) == false);
/// assert!((-2147483647i32).gt(&((-1i32))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).ge(&((-2147483648i32))) == true);
/// assert!((-2147483648i32).ge(&((-2147483647i32))) == false);
/// assert!((-2147483648i32).ge(&((-1i32))) == false);
/// assert!((-2147483648i32).ge(&(0i32)) == false);
/// assert!((-2147483648i32).ge(&(1i32)) == false);
/// assert!((-2147483648i32).ge(&(2147483646i32)) == false);
/// assert!((-2147483648i32).ge(&(2147483647i32)) == false);
/// assert!((-2147483647i32).ge(&((-2147483648i32))) == true);
/// assert!((-2147483647i32).ge(&((-2147483647i32))) == true);
/// assert!((-2147483647i32).ge(&((-1i32))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i32>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32).le(&((-2147483648i32))) == true);
/// assert!((-2147483648i32).le(&((-2147483647i32))) == true);
/// assert!((-2147483648i32).le(&((-1i32))) == true);
/// assert!((-2147483648i32).le(&(0i32)) == true);
/// assert!((-2147483648i32).le(&(1i32)) == true);
/// assert!((-2147483648i32).le(&(2147483646i32)) == true);
/// assert!((-2147483648i32).le(&(2147483647i32)) == true);
/// assert!((-2147483647i32).le(&((-2147483648i32))) == false);
/// assert!((-2147483647i32).le(&((-2147483647i32))) == true);
/// assert!((-2147483647i32).le(&((-1i32))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i32_le() {}
/// # Properties for [`core::ops::BitXor::<i32>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i32::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32) ^ (-2147483648i32) == 0i32);
/// assert!((-2147483648i32) ^ (-2147483647i32) == 1i32);
/// assert!((-2147483648i32) ^ (-1i32) == 2147483647i32);
/// assert!((-2147483648i32) ^ 0i32 == (-2147483648i32));
/// assert!((-2147483648i32) ^ 1i32 == (-2147483647i32));
/// assert!((-2147483648i32) ^ 2147483646i32 == (-2i32));
/// assert!((-2147483648i32) ^ 2147483647i32 == (-1i32));
/// assert!((-2147483647i32) ^ (-2147483648i32) == 1i32);
/// assert!((-2147483647i32) ^ (-2147483647i32) == 0i32);
/// assert!((-2147483647i32) ^ (-1i32) == 2147483646i32);
/// # }
/// ```
pub fn core_ops_bit_xor_i32_xor() {}
/// # Properties for [`core::ops::BitAnd::<i32>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i32::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32) & (-2147483648i32) == (-2147483648i32));
/// assert!((-2147483648i32) & (-2147483647i32) == (-2147483648i32));
/// assert!((-2147483648i32) & (-1i32) == (-2147483648i32));
/// assert!((-2147483648i32) & 0i32 == 0i32);
/// assert!((-2147483648i32) & 1i32 == 0i32);
/// assert!((-2147483648i32) & 2147483646i32 == 0i32);
/// assert!((-2147483648i32) & 2147483647i32 == 0i32);
/// assert!((-2147483647i32) & (-2147483648i32) == (-2147483648i32));
/// assert!((-2147483647i32) & (-2147483647i32) == (-2147483647i32));
/// assert!((-2147483647i32) & (-1i32) == (-2147483647i32));
/// # }
/// ```
pub fn core_ops_bit_and_i32_and() {}
/// # Properties for [`core::ops::BitAnd::<i32>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i32, y : i32`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i32::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-2147483648i32) ^ (-2147483648i32) == (-2147483648i32));
/// assert!((-2147483648i32) ^ (-2147483647i32) == (-2147483647i32));
/// assert!((-2147483648i32) ^ (-1i32) == (-1i32));
/// assert!((-2147483648i32) ^ 0i32 == (-2147483648i32));
/// assert!((-2147483648i32) ^ 1i32 == (-2147483647i32));
/// assert!((-2147483648i32) ^ 2147483646i32 == (-2i32));
/// assert!((-2147483648i32) ^ 2147483647i32 == (-1i32));
/// assert!((-2147483647i32) ^ (-2147483648i32) == (-2147483647i32));
/// assert!((-2147483647i32) ^ (-2147483647i32) == (-2147483647i32));
/// assert!((-2147483647i32) ^ (-1i32) == (-1i32));
/// # }
/// ```
pub fn core_ops_bit_and_i32_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).cmp(&((-9223372036854775808i64)))
///         == core::cmp::Ordering::Equal);
/// assert!((-9223372036854775808i64).cmp(&((-9223372036854775807i64)))
///         == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808i64).cmp(&((-1i64))) == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808i64).cmp(&(0i64)) == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808i64).cmp(&(1i64)) == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808i64).cmp(&(9223372036854775806i64)) == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808i64).cmp(&(9223372036854775807i64)) == core::cmp::Ordering::Less);
/// assert!((-9223372036854775807i64).cmp(&((-9223372036854775808i64)))
///         == core::cmp::Ordering::Greater);
/// assert!((-9223372036854775807i64).cmp(&((-9223372036854775807i64)))
///         == core::cmp::Ordering::Equal);
/// assert!((-9223372036854775807i64).cmp(&((-1i64))) == core::cmp::Ordering::Less);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).lt(&((-9223372036854775808i64))) == false);
/// assert!((-9223372036854775808i64).lt(&((-9223372036854775807i64))) == true);
/// assert!((-9223372036854775808i64).lt(&((-1i64))) == true);
/// assert!((-9223372036854775808i64).lt(&(0i64)) == true);
/// assert!((-9223372036854775808i64).lt(&(1i64)) == true);
/// assert!((-9223372036854775808i64).lt(&(9223372036854775806i64)) == true);
/// assert!((-9223372036854775808i64).lt(&(9223372036854775807i64)) == true);
/// assert!((-9223372036854775807i64).lt(&((-9223372036854775808i64))) == false);
/// assert!((-9223372036854775807i64).lt(&((-9223372036854775807i64))) == false);
/// assert!((-9223372036854775807i64).lt(&((-1i64))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).gt(&((-9223372036854775808i64))) == false);
/// assert!((-9223372036854775808i64).gt(&((-9223372036854775807i64))) == false);
/// assert!((-9223372036854775808i64).gt(&((-1i64))) == false);
/// assert!((-9223372036854775808i64).gt(&(0i64)) == false);
/// assert!((-9223372036854775808i64).gt(&(1i64)) == false);
/// assert!((-9223372036854775808i64).gt(&(9223372036854775806i64)) == false);
/// assert!((-9223372036854775808i64).gt(&(9223372036854775807i64)) == false);
/// assert!((-9223372036854775807i64).gt(&((-9223372036854775808i64))) == true);
/// assert!((-9223372036854775807i64).gt(&((-9223372036854775807i64))) == false);
/// assert!((-9223372036854775807i64).gt(&((-1i64))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).ge(&((-9223372036854775808i64))) == true);
/// assert!((-9223372036854775808i64).ge(&((-9223372036854775807i64))) == false);
/// assert!((-9223372036854775808i64).ge(&((-1i64))) == false);
/// assert!((-9223372036854775808i64).ge(&(0i64)) == false);
/// assert!((-9223372036854775808i64).ge(&(1i64)) == false);
/// assert!((-9223372036854775808i64).ge(&(9223372036854775806i64)) == false);
/// assert!((-9223372036854775808i64).ge(&(9223372036854775807i64)) == false);
/// assert!((-9223372036854775807i64).ge(&((-9223372036854775808i64))) == true);
/// assert!((-9223372036854775807i64).ge(&((-9223372036854775807i64))) == true);
/// assert!((-9223372036854775807i64).ge(&((-1i64))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i64>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64).le(&((-9223372036854775808i64))) == true);
/// assert!((-9223372036854775808i64).le(&((-9223372036854775807i64))) == true);
/// assert!((-9223372036854775808i64).le(&((-1i64))) == true);
/// assert!((-9223372036854775808i64).le(&(0i64)) == true);
/// assert!((-9223372036854775808i64).le(&(1i64)) == true);
/// assert!((-9223372036854775808i64).le(&(9223372036854775806i64)) == true);
/// assert!((-9223372036854775808i64).le(&(9223372036854775807i64)) == true);
/// assert!((-9223372036854775807i64).le(&((-9223372036854775808i64))) == false);
/// assert!((-9223372036854775807i64).le(&((-9223372036854775807i64))) == true);
/// assert!((-9223372036854775807i64).le(&((-1i64))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i64_le() {}
/// # Properties for [`core::ops::BitXor::<i64>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i64::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64) ^ (-9223372036854775808i64) == 0i64);
/// assert!((-9223372036854775808i64) ^ (-9223372036854775807i64) == 1i64);
/// assert!((-9223372036854775808i64) ^ (-1i64) == 9223372036854775807i64);
/// assert!((-9223372036854775808i64) ^ 0i64 == (-9223372036854775808i64));
/// assert!((-9223372036854775808i64) ^ 1i64 == (-9223372036854775807i64));
/// assert!((-9223372036854775808i64) ^ 9223372036854775806i64 == (-2i64));
/// assert!((-9223372036854775808i64) ^ 9223372036854775807i64 == (-1i64));
/// assert!((-9223372036854775807i64) ^ (-9223372036854775808i64) == 1i64);
/// assert!((-9223372036854775807i64) ^ (-9223372036854775807i64) == 0i64);
/// assert!((-9223372036854775807i64) ^ (-1i64) == 9223372036854775806i64);
/// # }
/// ```
pub fn core_ops_bit_xor_i64_xor() {}
/// # Properties for [`core::ops::BitAnd::<i64>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i64::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64) & (-9223372036854775808i64) == (-9223372036854775808i64));
/// assert!((-9223372036854775808i64) & (-9223372036854775807i64) == (-9223372036854775808i64));
/// assert!((-9223372036854775808i64) & (-1i64) == (-9223372036854775808i64));
/// assert!((-9223372036854775808i64) & 0i64 == 0i64);
/// assert!((-9223372036854775808i64) & 1i64 == 0i64);
/// assert!((-9223372036854775808i64) & 9223372036854775806i64 == 0i64);
/// assert!((-9223372036854775808i64) & 9223372036854775807i64 == 0i64);
/// assert!((-9223372036854775807i64) & (-9223372036854775808i64) == (-9223372036854775808i64));
/// assert!((-9223372036854775807i64) & (-9223372036854775807i64) == (-9223372036854775807i64));
/// assert!((-9223372036854775807i64) & (-1i64) == (-9223372036854775807i64));
/// # }
/// ```
pub fn core_ops_bit_and_i64_and() {}
/// # Properties for [`core::ops::BitAnd::<i64>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i64, y : i64`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i64::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808i64) ^ (-9223372036854775808i64) == (-9223372036854775808i64));
/// assert!((-9223372036854775808i64) ^ (-9223372036854775807i64) == (-9223372036854775807i64));
/// assert!((-9223372036854775808i64) ^ (-1i64) == (-1i64));
/// assert!((-9223372036854775808i64) ^ 0i64 == (-9223372036854775808i64));
/// assert!((-9223372036854775808i64) ^ 1i64 == (-9223372036854775807i64));
/// assert!((-9223372036854775808i64) ^ 9223372036854775806i64 == (-2i64));
/// assert!((-9223372036854775808i64) ^ 9223372036854775807i64 == (-1i64));
/// assert!((-9223372036854775807i64) ^ (-9223372036854775808i64) == (-9223372036854775807i64));
/// assert!((-9223372036854775807i64) ^ (-9223372036854775807i64) == (-9223372036854775807i64));
/// assert!((-9223372036854775807i64) ^ (-1i64) == (-1i64));
/// # }
/// ```
pub fn core_ops_bit_and_i64_or() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         .cmp(&((-170141183460469231731687303715884105728i128)))
///         == core::cmp::Ordering::Equal);
/// assert!((-170141183460469231731687303715884105728i128)
///         .cmp(&((-170141183460469231731687303715884105727i128)))
///         == core::cmp::Ordering::Less);
/// assert!((-170141183460469231731687303715884105728i128).cmp(&((-1i128)))
///         == core::cmp::Ordering::Less);
/// assert!((-170141183460469231731687303715884105728i128).cmp(&(0i128))
///         == core::cmp::Ordering::Less);
/// assert!((-170141183460469231731687303715884105728i128).cmp(&(1i128))
///         == core::cmp::Ordering::Less);
/// assert!((-170141183460469231731687303715884105728i128)
///         .cmp(&(170141183460469231731687303715884105726i128)) == core::cmp::Ordering::Less);
/// assert!((-170141183460469231731687303715884105728i128)
///         .cmp(&(170141183460469231731687303715884105727i128)) == core::cmp::Ordering::Less);
/// assert!((-170141183460469231731687303715884105727i128)
///         .cmp(&((-170141183460469231731687303715884105728i128)))
///         == core::cmp::Ordering::Greater);
/// assert!((-170141183460469231731687303715884105727i128)
///         .cmp(&((-170141183460469231731687303715884105727i128)))
///         == core::cmp::Ordering::Equal);
/// assert!((-170141183460469231731687303715884105727i128).cmp(&((-1i128)))
///         == core::cmp::Ordering::Less);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         .lt(&((-170141183460469231731687303715884105728i128))) == false);
/// assert!((-170141183460469231731687303715884105728i128)
///         .lt(&((-170141183460469231731687303715884105727i128))) == true);
/// assert!((-170141183460469231731687303715884105728i128).lt(&((-1i128))) == true);
/// assert!((-170141183460469231731687303715884105728i128).lt(&(0i128)) == true);
/// assert!((-170141183460469231731687303715884105728i128).lt(&(1i128)) == true);
/// assert!((-170141183460469231731687303715884105728i128)
///         .lt(&(170141183460469231731687303715884105726i128)) == true);
/// assert!((-170141183460469231731687303715884105728i128)
///         .lt(&(170141183460469231731687303715884105727i128)) == true);
/// assert!((-170141183460469231731687303715884105727i128)
///         .lt(&((-170141183460469231731687303715884105728i128))) == false);
/// assert!((-170141183460469231731687303715884105727i128)
///         .lt(&((-170141183460469231731687303715884105727i128))) == false);
/// assert!((-170141183460469231731687303715884105727i128).lt(&((-1i128))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         .gt(&((-170141183460469231731687303715884105728i128))) == false);
/// assert!((-170141183460469231731687303715884105728i128)
///         .gt(&((-170141183460469231731687303715884105727i128))) == false);
/// assert!((-170141183460469231731687303715884105728i128).gt(&((-1i128))) == false);
/// assert!((-170141183460469231731687303715884105728i128).gt(&(0i128)) == false);
/// assert!((-170141183460469231731687303715884105728i128).gt(&(1i128)) == false);
/// assert!((-170141183460469231731687303715884105728i128)
///         .gt(&(170141183460469231731687303715884105726i128)) == false);
/// assert!((-170141183460469231731687303715884105728i128)
///         .gt(&(170141183460469231731687303715884105727i128)) == false);
/// assert!((-170141183460469231731687303715884105727i128)
///         .gt(&((-170141183460469231731687303715884105728i128))) == true);
/// assert!((-170141183460469231731687303715884105727i128)
///         .gt(&((-170141183460469231731687303715884105727i128))) == false);
/// assert!((-170141183460469231731687303715884105727i128).gt(&((-1i128))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         .ge(&((-170141183460469231731687303715884105728i128))) == true);
/// assert!((-170141183460469231731687303715884105728i128)
///         .ge(&((-170141183460469231731687303715884105727i128))) == false);
/// assert!((-170141183460469231731687303715884105728i128).ge(&((-1i128))) == false);
/// assert!((-170141183460469231731687303715884105728i128).ge(&(0i128)) == false);
/// assert!((-170141183460469231731687303715884105728i128).ge(&(1i128)) == false);
/// assert!((-170141183460469231731687303715884105728i128)
///         .ge(&(170141183460469231731687303715884105726i128)) == false);
/// assert!((-170141183460469231731687303715884105728i128)
///         .ge(&(170141183460469231731687303715884105727i128)) == false);
/// assert!((-170141183460469231731687303715884105727i128)
///         .ge(&((-170141183460469231731687303715884105728i128))) == true);
/// assert!((-170141183460469231731687303715884105727i128)
///         .ge(&((-170141183460469231731687303715884105727i128))) == true);
/// assert!((-170141183460469231731687303715884105727i128).ge(&((-1i128))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<i128>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         .le(&((-170141183460469231731687303715884105728i128))) == true);
/// assert!((-170141183460469231731687303715884105728i128)
///         .le(&((-170141183460469231731687303715884105727i128))) == true);
/// assert!((-170141183460469231731687303715884105728i128).le(&((-1i128))) == true);
/// assert!((-170141183460469231731687303715884105728i128).le(&(0i128)) == true);
/// assert!((-170141183460469231731687303715884105728i128).le(&(1i128)) == true);
/// assert!((-170141183460469231731687303715884105728i128)
///         .le(&(170141183460469231731687303715884105726i128)) == true);
/// assert!((-170141183460469231731687303715884105728i128)
///         .le(&(170141183460469231731687303715884105727i128)) == true);
/// assert!((-170141183460469231731687303715884105727i128)
///         .le(&((-170141183460469231731687303715884105728i128))) == false);
/// assert!((-170141183460469231731687303715884105727i128)
///         .le(&((-170141183460469231731687303715884105727i128))) == true);
/// assert!((-170141183460469231731687303715884105727i128).le(&((-1i128))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_i128_le() {}
/// # Properties for [`core::ops::BitXor::<i128>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i128::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         ^ (-170141183460469231731687303715884105728i128) == 0i128);
/// assert!((-170141183460469231731687303715884105728i128)
///         ^ (-170141183460469231731687303715884105727i128) == 1i128);
/// assert!((-170141183460469231731687303715884105728i128) ^ (-1i128)
///         == 170141183460469231731687303715884105727i128);
/// assert!((-170141183460469231731687303715884105728i128) ^ 0i128
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105728i128) ^ 1i128
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         ^ 170141183460469231731687303715884105726i128 == (-2i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         ^ 170141183460469231731687303715884105727i128 == (-1i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         ^ (-170141183460469231731687303715884105728i128) == 1i128);
/// assert!((-170141183460469231731687303715884105727i128)
///         ^ (-170141183460469231731687303715884105727i128) == 0i128);
/// assert!((-170141183460469231731687303715884105727i128) ^ (-1i128)
///         == 170141183460469231731687303715884105726i128);
/// # }
/// ```
pub fn core_ops_bit_xor_i128_xor() {}
/// # Properties for [`core::ops::BitAnd::<i128>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == i128::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         & (-170141183460469231731687303715884105728i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         & (-170141183460469231731687303715884105727i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105728i128) & (-1i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105728i128) & 0i128 == 0i128);
/// assert!((-170141183460469231731687303715884105728i128) & 1i128 == 0i128);
/// assert!((-170141183460469231731687303715884105728i128)
///         & 170141183460469231731687303715884105726i128 == 0i128);
/// assert!((-170141183460469231731687303715884105728i128)
///         & 170141183460469231731687303715884105727i128 == 0i128);
/// assert!((-170141183460469231731687303715884105727i128)
///         & (-170141183460469231731687303715884105728i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         & (-170141183460469231731687303715884105727i128)
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105727i128) & (-1i128)
///         == (-170141183460469231731687303715884105727i128));
/// # }
/// ```
pub fn core_ops_bit_and_i128_and() {}
/// # Properties for [`core::ops::BitAnd::<i128>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : i128, y : i128`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == i128::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-170141183460469231731687303715884105728i128)
///         ^ (-170141183460469231731687303715884105728i128)
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         ^ (-170141183460469231731687303715884105727i128)
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105728i128) ^ (-1i128) == (-1i128));
/// assert!((-170141183460469231731687303715884105728i128) ^ 0i128
///         == (-170141183460469231731687303715884105728i128));
/// assert!((-170141183460469231731687303715884105728i128) ^ 1i128
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         ^ 170141183460469231731687303715884105726i128 == (-2i128));
/// assert!((-170141183460469231731687303715884105728i128)
///         ^ 170141183460469231731687303715884105727i128 == (-1i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         ^ (-170141183460469231731687303715884105728i128)
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105727i128)
///         ^ (-170141183460469231731687303715884105727i128)
///         == (-170141183460469231731687303715884105727i128));
/// assert!((-170141183460469231731687303715884105727i128) ^ (-1i128) == (-1i128));
/// # }
/// ```
pub fn core_ops_bit_and_i128_or() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::cmp`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.cmp(&(y)) == x.up().cmp(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).cmp(&((-9223372036854775808isize)))
///         == core::cmp::Ordering::Equal);
/// assert!((-9223372036854775808isize).cmp(&((-9223372036854775807isize)))
///         == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808isize).cmp(&((-1isize))) == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808isize).cmp(&(0isize)) == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808isize).cmp(&(1isize)) == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808isize).cmp(&(9223372036854775806isize))
///         == core::cmp::Ordering::Less);
/// assert!((-9223372036854775808isize).cmp(&(9223372036854775807isize))
///         == core::cmp::Ordering::Less);
/// assert!((-9223372036854775807isize).cmp(&((-9223372036854775808isize)))
///         == core::cmp::Ordering::Greater);
/// assert!((-9223372036854775807isize).cmp(&((-9223372036854775807isize)))
///         == core::cmp::Ordering::Equal);
/// assert!((-9223372036854775807isize).cmp(&((-1isize))) == core::cmp::Ordering::Less);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_cmp() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::lt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.lt(&(y)) == x.up().lt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).lt(&((-9223372036854775808isize))) == false);
/// assert!((-9223372036854775808isize).lt(&((-9223372036854775807isize))) == true);
/// assert!((-9223372036854775808isize).lt(&((-1isize))) == true);
/// assert!((-9223372036854775808isize).lt(&(0isize)) == true);
/// assert!((-9223372036854775808isize).lt(&(1isize)) == true);
/// assert!((-9223372036854775808isize).lt(&(9223372036854775806isize)) == true);
/// assert!((-9223372036854775808isize).lt(&(9223372036854775807isize)) == true);
/// assert!((-9223372036854775807isize).lt(&((-9223372036854775808isize))) == false);
/// assert!((-9223372036854775807isize).lt(&((-9223372036854775807isize))) == false);
/// assert!((-9223372036854775807isize).lt(&((-1isize))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_lt() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::gt`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.gt(&(y)) == x.up().gt(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).gt(&((-9223372036854775808isize))) == false);
/// assert!((-9223372036854775808isize).gt(&((-9223372036854775807isize))) == false);
/// assert!((-9223372036854775808isize).gt(&((-1isize))) == false);
/// assert!((-9223372036854775808isize).gt(&(0isize)) == false);
/// assert!((-9223372036854775808isize).gt(&(1isize)) == false);
/// assert!((-9223372036854775808isize).gt(&(9223372036854775806isize)) == false);
/// assert!((-9223372036854775808isize).gt(&(9223372036854775807isize)) == false);
/// assert!((-9223372036854775807isize).gt(&((-9223372036854775808isize))) == true);
/// assert!((-9223372036854775807isize).gt(&((-9223372036854775807isize))) == false);
/// assert!((-9223372036854775807isize).gt(&((-1isize))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_gt() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::ge`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.ge(&(y)) == x.up().ge(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).ge(&((-9223372036854775808isize))) == true);
/// assert!((-9223372036854775808isize).ge(&((-9223372036854775807isize))) == false);
/// assert!((-9223372036854775808isize).ge(&((-1isize))) == false);
/// assert!((-9223372036854775808isize).ge(&(0isize)) == false);
/// assert!((-9223372036854775808isize).ge(&(1isize)) == false);
/// assert!((-9223372036854775808isize).ge(&(9223372036854775806isize)) == false);
/// assert!((-9223372036854775808isize).ge(&(9223372036854775807isize)) == false);
/// assert!((-9223372036854775807isize).ge(&((-9223372036854775808isize))) == true);
/// assert!((-9223372036854775807isize).ge(&((-9223372036854775807isize))) == true);
/// assert!((-9223372036854775807isize).ge(&((-1isize))) == false);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_ge() {}
/// # Properties for [`core::cmp::PartialOrd::<isize>::le`]
/// ## Semantics of comparaison
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x.le(&(y)) == x.up().le(&(y).up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize).le(&((-9223372036854775808isize))) == true);
/// assert!((-9223372036854775808isize).le(&((-9223372036854775807isize))) == true);
/// assert!((-9223372036854775808isize).le(&((-1isize))) == true);
/// assert!((-9223372036854775808isize).le(&(0isize)) == true);
/// assert!((-9223372036854775808isize).le(&(1isize)) == true);
/// assert!((-9223372036854775808isize).le(&(9223372036854775806isize)) == true);
/// assert!((-9223372036854775808isize).le(&(9223372036854775807isize)) == true);
/// assert!((-9223372036854775807isize).le(&((-9223372036854775808isize))) == false);
/// assert!((-9223372036854775807isize).le(&((-9223372036854775807isize))) == true);
/// assert!((-9223372036854775807isize).le(&((-1isize))) == true);
/// # }
/// ```
pub fn core_cmp_partial_ord_isize_le() {}
/// # Properties for [`core::ops::BitXor::<isize>::bitxor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == isize::down(x.up() ^ y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize) ^ (-9223372036854775808isize) == 0isize);
/// assert!((-9223372036854775808isize) ^ (-9223372036854775807isize) == 1isize);
/// assert!((-9223372036854775808isize) ^ (-1isize) == 9223372036854775807isize);
/// assert!((-9223372036854775808isize) ^ 0isize == (-9223372036854775808isize));
/// assert!((-9223372036854775808isize) ^ 1isize == (-9223372036854775807isize));
/// assert!((-9223372036854775808isize) ^ 9223372036854775806isize == (-2isize));
/// assert!((-9223372036854775808isize) ^ 9223372036854775807isize == (-1isize));
/// assert!((-9223372036854775807isize) ^ (-9223372036854775808isize) == 1isize);
/// assert!((-9223372036854775807isize) ^ (-9223372036854775807isize) == 0isize);
/// assert!((-9223372036854775807isize) ^ (-1isize) == 9223372036854775806isize);
/// # }
/// ```
pub fn core_ops_bit_xor_isize_xor() {}
/// # Properties for [`core::ops::BitAnd::<isize>::bitand`]
/// ## Semantics of bitwise and
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x & y == isize::down(x.up() & y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize) & (-9223372036854775808isize)
///         == (-9223372036854775808isize));
/// assert!((-9223372036854775808isize) & (-9223372036854775807isize)
///         == (-9223372036854775808isize));
/// assert!((-9223372036854775808isize) & (-1isize) == (-9223372036854775808isize));
/// assert!((-9223372036854775808isize) & 0isize == 0isize);
/// assert!((-9223372036854775808isize) & 1isize == 0isize);
/// assert!((-9223372036854775808isize) & 9223372036854775806isize == 0isize);
/// assert!((-9223372036854775808isize) & 9223372036854775807isize == 0isize);
/// assert!((-9223372036854775807isize) & (-9223372036854775808isize)
///         == (-9223372036854775808isize));
/// assert!((-9223372036854775807isize) & (-9223372036854775807isize)
///         == (-9223372036854775807isize));
/// assert!((-9223372036854775807isize) & (-1isize) == (-9223372036854775807isize));
/// # }
/// ```
pub fn core_ops_bit_and_isize_and() {}
/// # Properties for [`core::ops::BitAnd::<isize>::bitor`]
/// ## Semantics of bitwise or
/// __Inputs:__ `x : isize, y : isize`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x ^ y == isize::down(x.up() | y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # use num_traits::Euclid;
/// # #[allow(arithmetic_overflow)] {
/// assert!((-9223372036854775808isize) ^ (-9223372036854775808isize)
///         == (-9223372036854775808isize));
/// assert!((-9223372036854775808isize) ^ (-9223372036854775807isize)
///         == (-9223372036854775807isize));
/// assert!((-9223372036854775808isize) ^ (-1isize) == (-1isize));
/// assert!((-9223372036854775808isize) ^ 0isize == (-9223372036854775808isize));
/// assert!((-9223372036854775808isize) ^ 1isize == (-9223372036854775807isize));
/// assert!((-9223372036854775808isize) ^ 9223372036854775806isize == (-2isize));
/// assert!((-9223372036854775808isize) ^ 9223372036854775807isize == (-1isize));
/// assert!((-9223372036854775807isize) ^ (-9223372036854775808isize)
///         == (-9223372036854775807isize));
/// assert!((-9223372036854775807isize) ^ (-9223372036854775807isize)
///         == (-9223372036854775807isize));
/// assert!((-9223372036854775807isize) ^ (-1isize) == (-1isize));
/// # }
/// ```
pub fn core_ops_bit_and_isize_or() {}
