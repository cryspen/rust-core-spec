# Rust core specification
This project is about adding specifications to Rust's [`core`
lib](https://doc.rust-lang.org/nightly/core/index.html), by decorating
it with thorough functional tests, item-wise. This project aims at
generating tests for the `core` library and at facilitating
verification of Rust programs.

## Approach
For every function `f`, we want to write a comprehensive and
documented list of _contracts_ that are satisfied by `f`. Each
contract exercises one aspect of the behavior of a function. Ideally,
the set of contracts for a given function should be complete,
i.e. every input falls into at least one contract.

Contracts are universally quantified predicates about functions: for
each of them, we _generate_ a list of concrete tests, exercising the
validity of each contract.

### Reduced manual effort and clarity
We need to write contracts manually: contracts define the
specifications of functions.

Independenlty, we generate automatically test cases for those contracts.

# Quickstart
The various contracts are defined in the `src/specs` folder.

Running `cargo build` generates the tests under two different formats.
 1. `generated_tests/doc.rs`: a "literate" Rust module which contains
    tests as documentation examples. This format is especially useful
    for browsing the tests: just run `cargo doc --no-deps --open` and
    browse the `tests` module.
 2. `generated_tests/bin.rs`: a binary that packs all the tests as
    plain `asserts!`, running them in sequence. No TCB involved
    (e.g. `rustdoc`), just vanilla Rust.

Tests can be run in two different ways:
 - `cargo test` will ask `rustdoc` to test all the documentation
examples from `generated_tests/doc.rs`. The downside is that you need
to trust that `rustdoc` indeed executes all the tests.
 - `cargo run` will run the binary built from
   `generated_tests/bin.rs`, which runs all the tests in a very simple
   and understandable way.

# Contracts
## Definition
A contract is defined by the following data.
 - Some metadata (title, english description).
 - A sequence of typed input variables `x₁: T₁`...`xᵢ: Tᵢ`.
   _Note: let's call **domain** the tuple type `(T₁, ..., Tᵢ)`_
 - A precondition (a boolean predicate on the contract's domain);
 - A postcondition (a boolean predicate on the contract's domain).
 
## Test vector
A contract can be instantiated with values of its domain.

## Abstractions
To define contracts, we usually use abstractions.

For instance, we model the addition on machine integer of some size
with the addition on mathematical integers: whenever `x↑ + y↑ < 256`
holds, then `u8::add(x, y) == int::add(x↑, y↑)↓` holds. Note ↑ and ↓
lift up and down machine integers to mathematical ones.

While such abstractions are great at the level of contracts, when we
instantiate them with test vector, the resulting tests should be very
concrete and self contained.

Let's continue with addition example. Instantiating our contract with
`(x, y) = (40, 2)`, we would like to test that `u8::add(40, 2) == 42`
holds, not `u8::add(40, 2) == int::add(40↑, 2↑)↓`.

The final tests we will run against `core` should be as concrete as
possible: we don't want to test `core` modulo the semantics of our
abstraction. We want to test `core` itself, and the valididty of the
generated tests should be obvious to the reader.

## Partial computations: have your cake and eat it too
Here, we want both to have abstract contracts and very concrete tests:
it seems a bit contradictory.

We need a way of collapsing our abstractions, but only the
abstractions. It is the job of the proc-macro crate
`partial_compute`. Given a function of type `(T₁, …, Tᵢ, …, Tⱼ) → U`
it is able to produce a function `(T₁, …, Tᵢ) → Expr`: given the `i`
first arguments, it produces a rust expression (a string) where the
`i` first arguments are inlined and where some parts are pre-computed.

Through this mechanics, we can concretize a contract in a flexible
way.

# Test vector generation
Currently test vectors are a combination of (1) an optional,
user-defined list of tests and (2) randomly generated values.

gWe want to leverage the hax toolchain to automatically find edge
cases, given the actual `core` implementation and the specification.
