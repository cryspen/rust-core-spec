use crate::helpers::*;

pub fn specs() {
    tests! {
        header: "Properties for [`Option::is_some`]",
        ident: option_is_some,
        contract! {
            header: "`is_some` is a shorthand to pattern matching",
            inputs: <T>[v: Option<T>],
            precondition: true,
            postcondition: v.is_some() == (match v { Some(_) => true, None => false}),
        },
    }
    tests! {
        header: "Properties for [`Option::is_none`]",
        ident: option_is_none,
        contract! {
            header: "`is_none` is a shorthand to pattern matching",
            inputs: <T>[v: Option<T>],
            precondition: true,
            postcondition: v.is_none() == (match v { Some(_) => false, None => true}),
        },
    }
    tests! {
        header: "Properties for [`Option::expect`]",
        ident: option_expect,
        contract! {
            header: "Unwrapping a [`None`] with `expect` always panic",
            inputs: <T>[v: Option<T>],
            precondition: v.is_none(),
            postcondition: panics!(v.expect("message")),
            n: 1,
        },
        contract! {
            header: "Unwrapping a [`Some(_)`] with `expect` always succeeds",
            inputs: <T>[v: Option<T>],
            precondition: v.is_some(),
            postcondition: doesn_t_panic!(v.expect("message")),
        },
        contract! {
            header: "Wrapping a value in a `Some` and unwrapping is identity",
            inputs: <T>[v: T],
            precondition: true,
            postcondition: Some(v).unwrap() == v,
        },
    }
    tests! {
        header: "Properties for [`Option::map`]",
        ident: option_map,
        contract! {
            header: "Applying `f` on `Some(v)` via `map` is equal to wrapping in `Some` the application of `v` to `f`",
            inputs: <T>[v: T, f: Fn1<T, T>],
            precondition: true,
            postcondition: Some(v).map(f) == Some((f)(v)),
        },
        contract! {
            header: "Mapping a `None` is the identity",
            inputs: <T>[v: Option<T>, f: Fn1<T, T>],
            precondition: v.is_none(),
            postcondition: v.map(f) == v,
            n: 1,
        },
    }
    tests! {
        header: "Properties for [`Option::filter`]",
        ident: option_filter,
        contract! {
            header: "The filtering of `Some(v)` with a predicate `f` being non-empty is equivalent to applying a predicate `f` on `v`",
            inputs: <T>[v: T, f: FnR1<T, bool>],
            precondition: true,
            postcondition: Some(v).filter(f).is_some() == f(&v),
        },
        contract! {
            header: "Filtering a `None` is the identity",
            inputs: <T>[v: Option<T>, f: FnR1<T, bool>],
            precondition: v.is_none(),
            postcondition: v.filter(f) == v,
            n: 1,
        },
    }
    tests! {
        header: "Properties for [`Option::flatten`]",
        ident: option_flatten,
        contract! {
            header: "Nested `Some`s",
            inputs: <T>[x: T],
            precondition: true,
            postcondition: Some(Some(x)).flatten() == Some(x),
        },
        contract! {
            header: "Nested or direct `None` flattens to None",
            inputs: <T>[x: Option<Option<T>>],
            precondition: x.is_none() || x.unwrap().is_none(),
            postcondition: x.flatten() == None,
            n: 2,
        },
    }
    tests! {
        header: "Properties for [`Option::take`]",
        ident: option_take,
        contract! {
            header: "Take steals a value",
            inputs: <T>[x: Option<T>],
            precondition: true,
            postcondition: {
                let mut y = x.clone();
                y.take() == x && y.is_none()
            },
        },
    }
    tests! {
        header: "Properties for [`Option::zip`]",
        ident: option_zip,
        contract! {
            header: "Zipping two non-empty options",
            inputs: <T>[x: T, y: T],
            precondition: true,
            postcondition: Some(x).zip(Some(y)) == Some((x, y)),
        },
        contract! {
            header: "Zipping two options when one is a `None` makes `None`",
            inputs: <T>[x: Option<T>, y: Option<T>],
            precondition: x.is_none() || y.is_none(),
            postcondition: x.zip(y).is_none(),
        },
    }
    tests! {
        header: "Properties for [`Option::unwrap`]",
        ident: option_unwrap,
        contract! {
            header: "Unwrapping a [`None`] always panic",
            inputs: <T>[v: Option<T>],
            precondition: v.is_none(),
            postcondition: panics!(v.unwrap()),
            n: 1,
        },
        contract! {
            header: "Unwrapping a [`Some(_)`] always succeeds",
            inputs: <T>[v: Option<T>],
            precondition: v.is_some(),
            postcondition: doesn_t_panic!(v.unwrap()),
        },
    }
    tests! {
        header: "Properties for [`Option::as_mut`]",
        ident: option_as_mut,
        contract! {
            header: "In place update via `as_mut` is equivalent to functional update",
            inputs: [v: Option<u8>],
            precondition: v.is_some() && v.unwrap() < 50,
            postcondition: {
                let (v_unwrapped, mut v_mut) = (v.unwrap().clone(), v);
                *v_mut.as_mut().unwrap() += 10;
                v_mut.unwrap() == v_unwrapped + 10
            },
        },
    }
    tests! {
        header: "Properties for [`Option::as_slice`]",
        ident: option_as_slice,
        contract! {
            header: "[`None.as_slice()`] should always result in an empty slice",
            inputs: <T>[v: Option<T>],
            precondition: v.is_none(),
            postcondition: {v.as_slice().is_empty()},
            n: 1
        },
        contract! {
            header: "[`Some(v).as_slice()`] should always result in a slice containing exactly `v`",
            inputs: <T>[v: T],
            precondition: true,
            postcondition: {Some(v).as_slice() == [v]},
        },
    }
}
