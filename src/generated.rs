//! This module contains placeholder functions decorated with contracts and concrete tests.
/// # Properties for [`Option`]
/// ## Wrapping in a value in a [`Some`] and then unwrapping the result is the identity function
/// __Inputs:__ `v : T`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `Some(v).unwrap() == v`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(Some(0u8).unwrap() == 0u8);
/// assert!(Some(255u8).unwrap() == 255u8);
/// assert!(Some(20u8).unwrap() == 20u8);
/// assert!(Some(237u8).unwrap() == 237u8);
/// assert!(Some(112u8).unwrap() == 112u8);
/// assert!(Some(89u8).unwrap() == 89u8);
/// # }
/// ```
/// ## Unwrapping a [`None`] always panic
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `panics!(v.unwrap())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
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
/// # #[allow(arithmetic_overflow)] {
/// assert!(doesn_t_panic!(Some(0u8).unwrap()));
/// assert!(doesn_t_panic!(Some(255u8).unwrap()));
/// assert!(doesn_t_panic!(Some(3u8).unwrap()));
/// assert!(doesn_t_panic!(Some(78u8).unwrap()));
/// assert!(doesn_t_panic!(Some(32u8).unwrap()));
/// assert!(doesn_t_panic!(Some(216u8).unwrap()));
/// # }
/// ```
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
/// # #[allow(arithmetic_overflow)] {
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(0u8).unwrap().clone(), Some(0u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(39u8).unwrap().clone(), Some(39u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(46u8).unwrap().clone(), Some(46u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(47u8).unwrap().clone(), Some(47u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// assert!({
///         let (v_unwrapped, mut v_mut) = (Some(2u8).unwrap().clone(), Some(2u8));
///         *v_mut.as_mut().unwrap() += 10;
///         v_mut.unwrap() == v_unwrapped + 10
///     });
/// # }
/// ```
/// ## [`None.as_slice()`] should always result in an empty slice
/// __Inputs:__ `v : Option<T>`  
/// __Precondition:__ `v.is_none()`  
/// __Postcondition:__ `{ v.as_slice().is_empty() }`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
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
/// # #[allow(arithmetic_overflow)] {
/// assert!({ Some(0u8).as_slice() == [0u8] });
/// assert!({ Some(255u8).as_slice() == [255u8] });
/// assert!({ Some(101u8).as_slice() == [101u8] });
/// assert!({ Some(218u8).as_slice() == [218u8] });
/// assert!({ Some(9u8).as_slice() == [9u8] });
/// assert!({ Some(139u8).as_slice() == [139u8] });
/// # }
/// ```
pub fn option(){}
/// # Properties for [`Vec`]
/// ## Indexing
/// __Inputs:__ `v : Vec<u8>, i : usize`  
/// __Precondition:__ `v.len() > 0`  
/// __Postcondition:__ `v.get(i % v.len()) == Some(&v[i % v.len()])`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(vec![0u8].get(0usize) == Some(&0u8));
/// assert!(vec![0u8].get(0usize) == Some(&0u8));
/// assert!(vec![255u8].get(0usize) == Some(&255u8));
/// assert!(vec![255u8].get(0usize) == Some(&255u8));
/// assert!(vec![224u8, 116u8].get(1usize) == Some(&116u8));
/// assert!(vec![173u8].get(0usize) == Some(&173u8));
/// assert!(vec![199u8, 169u8, 173u8].get(1usize) == Some(&169u8));
/// assert!(vec![182u8].get(0usize) == Some(&182u8));
/// assert!(vec![52u8].get(0usize) == Some(&52u8));
/// assert!(vec![212u8, 234u8, 189u8, 202u8, 116u8].get(0usize) == Some(&212u8));
/// assert!(vec![41u8, 87u8].get(0usize) == Some(&41u8));
/// assert!(vec![10u8, 163u8, 176u8].get(2usize) == Some(&176u8));
/// # }
/// ```
pub fn slice_get(){}
/// # Properties for [`core::ops::Sub::<i8>::sub`]
/// ## Semantics of non-overflowing subtraction
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() < 128.up() && x.up() - y.up() >= -128i32.up()`  
/// __Postcondition:__ `x - y == i8::down(x.up() - y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(-128i8 - -128i8 == 0i8);
/// assert!(-128i8 - 0i8 == -128i8);
/// assert!(0i8 - 0i8 == 0i8);
/// assert!(0i8 - 127i8 == -127i8);
/// assert!(127i8 - 0i8 == 127i8);
/// assert!(127i8 - 127i8 == 0i8);
/// assert!(-14i8 - -70i8 == 56i8);
/// assert!(11i8 - 63i8 == -52i8);
/// assert!(27i8 - 101i8 == -74i8);
/// # }
/// ```
/// ## Overflowing subtraction panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() - y.up() >= 128.up() || x.up() - y.up() < -128i32.up()`  
/// __Postcondition:__ `panics!(x - y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(- 128i8 - 127i8));
/// assert!(panics!(0i8 - - 128i8));
/// assert!(panics!(127i8 - - 128i8));
/// assert!(panics!(- 127i8 - 26i8));
/// assert!(panics!(32i8 - - 98i8));
/// assert!(panics!(99i8 - - 97i8));
/// # }
/// ```
pub fn core_ops_sub_i8_sub(){}
/// # Properties for [`core::ops::Add::<i8>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() + y.up() < 128.up() && x.up() + y.up() >= -128i32.up()`  
/// __Postcondition:__ `x + y == i8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(-128i8 + 0i8 == -128i8);
/// assert!(-128i8 + 127i8 == -1i8);
/// assert!(0i8 + -128i8 == -128i8);
/// assert!(0i8 + 0i8 == 0i8);
/// assert!(0i8 + 127i8 == 127i8);
/// assert!(127i8 + -128i8 == -1i8);
/// assert!(127i8 + 0i8 == 127i8);
/// assert!(50i8 + -41i8 == 9i8);
/// assert!(-33i8 + 63i8 == 30i8);
/// assert!(46i8 + 38i8 == 84i8);
/// # }
/// ```
/// ## Overflowing addition panics
/// __Inputs:__ `x : i8, y : i8`  
/// __Precondition:__ `x.up() + y.up() >= 128.up() || x.up() + y.up() < -128i32.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(- 128i8 + - 128i8));
/// assert!(panics!(127i8 + 127i8));
/// assert!(panics!(- 120i8 + - 37i8));
/// assert!(panics!(13i8 + 117i8));
/// assert!(panics!(- 49i8 + - 121i8));
/// # }
/// ```
pub fn core_ops_add_i8_add(){}
/// # Properties for [`core::ops::Add::<u8>::add`]
/// ## Semantics of non-overflowing addition
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() < 256.up()`  
/// __Postcondition:__ `x + y == u8::down(x.up() + y.up())`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 + 0u8 == 0u8);
/// assert!(0u8 + 255u8 == 255u8);
/// assert!(255u8 + 0u8 == 255u8);
/// assert!(107u8 + 74u8 == 181u8);
/// assert!(79u8 + 131u8 == 210u8);
/// assert!(34u8 + 158u8 == 192u8);
/// assert!(15u8 + 10u8 == 25u8);
/// assert!(101u8 + 89u8 == 190u8);
/// assert!(33u8 + 109u8 == 142u8);
/// assert!(128u8 + 125u8 == 253u8);
/// assert!(41u8 + 174u8 == 215u8);
/// # }
/// ```
/// ## Panics when overflowing
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() >= 256.up()`  
/// __Postcondition:__ `panics!(x + y)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(panics!(255u8 + 255u8));
/// assert!(panics!(201u8 + 177u8));
/// assert!(panics!(24u8 + 251u8));
/// assert!(panics!(217u8 + 47u8));
/// assert!(panics!(50u8 + 229u8));
/// # }
/// ```
/// ## Commutativity
/// __Inputs:__ `x : u8, y : u8`  
/// __Precondition:__ `x.up() + y.up() < 256.up()`  
/// __Postcondition:__ `x + y == y + x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 + 0u8 == 0u8 + 0u8);
/// assert!(0u8 + 255u8 == 255u8 + 0u8);
/// assert!(255u8 + 0u8 == 0u8 + 255u8);
/// assert!(150u8 + 19u8 == 19u8 + 150u8);
/// assert!(77u8 + 61u8 == 61u8 + 77u8);
/// assert!(14u8 + 148u8 == 148u8 + 14u8);
/// assert!(101u8 + 136u8 == 136u8 + 101u8);
/// assert!(217u8 + 26u8 == 26u8 + 217u8);
/// assert!(230u8 + 20u8 == 20u8 + 230u8);
/// assert!(35u8 + 159u8 == 159u8 + 35u8);
/// assert!(25u8 + 82u8 == 82u8 + 25u8);
/// # }
/// ```
/// ## Left identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `x + 0 == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0u8 + 0 == 0u8);
/// assert!(255u8 + 0 == 255u8);
/// assert!(36u8 + 0 == 36u8);
/// assert!(136u8 + 0 == 136u8);
/// assert!(12u8 + 0 == 12u8);
/// assert!(175u8 + 0 == 175u8);
/// # }
/// ```
/// ## Right identity
/// __Inputs:__ `x : u8`  
/// __Precondition:__ `true`  
/// __Postcondition:__ `0 + x == x`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!(0 + 0u8 == 0u8);
/// assert!(0 + 255u8 == 255u8);
/// assert!(0 + 162u8 == 162u8);
/// assert!(0 + 56u8 == 56u8);
/// assert!(0 + 225u8 == 225u8);
/// assert!(0 + 37u8 == 37u8);
/// # }
/// ```
/// ## Associativity
/// __Inputs:__ `x : u8, y : u8, z : u8`  
/// __Precondition:__ `x.up() + y.up() + z.up() < 256.up()`  
/// __Postcondition:__ `(x + y) + z == x + (y + z)`  
/// ```
/// # use core_spec::lifts::*;
/// # use core_spec::*;
/// # #[allow(arithmetic_overflow)] {
/// assert!((0u8 + 0u8) + 0u8 == 0u8 + (0u8 + 0u8));
/// assert!((0u8 + 0u8) + 255u8 == 0u8 + (0u8 + 255u8));
/// assert!((0u8 + 255u8) + 0u8 == 0u8 + (255u8 + 0u8));
/// assert!((255u8 + 0u8) + 0u8 == 255u8 + (0u8 + 0u8));
/// assert!((17u8 + 9u8) + 125u8 == 17u8 + (9u8 + 125u8));
/// assert!((112u8 + 12u8) + 56u8 == 112u8 + (12u8 + 56u8));
/// assert!((14u8 + 202u8) + 5u8 == 14u8 + (202u8 + 5u8));
/// assert!((139u8 + 27u8) + 74u8 == 139u8 + (27u8 + 74u8));
/// assert!((108u8 + 39u8) + 61u8 == 108u8 + (39u8 + 61u8));
/// assert!((105u8 + 37u8) + 34u8 == 105u8 + (37u8 + 34u8));
/// assert!((153u8 + 28u8) + 28u8 == 153u8 + (28u8 + 28u8));
/// assert!((69u8 + 58u8) + 121u8 == 69u8 + (58u8 + 121u8));
/// assert!((28u8 + 104u8) + 52u8 == 28u8 + (104u8 + 52u8));
/// assert!((27u8 + 152u8) + 73u8 == 27u8 + (152u8 + 73u8));
/// assert!((154u8 + 28u8) + 20u8 == 154u8 + (28u8 + 20u8));
/// assert!((118u8 + 37u8) + 30u8 == 118u8 + (37u8 + 30u8));
/// assert!((3u8 + 27u8) + 199u8 == 3u8 + (27u8 + 199u8));
/// assert!((0u8 + 62u8) + 53u8 == 0u8 + (62u8 + 53u8));
/// assert!((7u8 + 15u8) + 151u8 == 7u8 + (15u8 + 151u8));
/// assert!((13u8 + 131u8) + 100u8 == 13u8 + (131u8 + 100u8));
/// assert!((4u8 + 17u8) + 210u8 == 4u8 + (17u8 + 210u8));
/// assert!((35u8 + 146u8) + 32u8 == 35u8 + (146u8 + 32u8));
/// assert!((101u8 + 34u8) + 72u8 == 101u8 + (34u8 + 72u8));
/// # }
/// ```
pub fn core_ops_add_u8_add(){}