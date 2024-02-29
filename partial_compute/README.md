# Partial compute
This crate provides the proc-macro `partial_compute`. This proc-macro
can be attched to function items, and transforms a function of type
`(x1: T1, ..., xN: TN, y1: U1, ..., yM: UM) -> V` into a function of
type `(x1: T1, ..., xN: TN) -> RustFn`.

`RustFn` is Rust function of type `(y1: U1, ..., yM: UM) -> V`
represented as a pretty-printed string.
