use crate::helpers::*;

pub fn specs() {
    tests! {
        header: "Properties for [`Vec`]",
        ident: slice_get,
        contract! {
            header: "Indexing",
            inputs: [v: Vec<u8>, i: usize],
            precondition: v.len() > 0,// && i < v.len(),
            postcondition: v.get(eval(i % v.len())) == Some(&eval(v[i % v.len()])),
        },
    }
}
