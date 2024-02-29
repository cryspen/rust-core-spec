use crate::helpers::*;

pub fn specs() {
    tests! {
        header: "Properties for [`Option`]",
        ident: option,
        contract! {
            header: "Wrapping in a value in a [`Some`] and then unwrapping the result is the identity function",
            inputs: <T>[v: T],
            precondition: true,
            postcondition: Some(v).unwrap() == v,
        },
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
