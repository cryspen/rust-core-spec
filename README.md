# Rust core specification
This project is about adding specifications to Rust's [`core`
lib](https://doc.rust-lang.org/nightly/core/index.html), by decorating
it with thorough functional tests, item-wise. This project aims at
generating tests for the `core` library and at facilitating
verification of Rust programs.

# Approach
For every function `f`, we want to write a comprehensive and
documented list of _contracts_ that are satisfied by `f`. Each
contract exercises one aspect of the behavior of a function. Ideally,
the set of contracts for a given function should be complete,
i.e. every input falls into at least one contract.

Contracts are universally quantified predicates about functions: for
each of them, we _generate_ a list of concrete tests, exercising the
validity of each contract.

We need to write contracts manually: contracts define the
specifications of functions.

Independenlty, we generate automatically test cases for those contracts.

## Contracts
A contract is defined by the following data.
 - Some metadata (title, english description).
 - A number of generic parameters, possibly with bounds: `T, U where T: Foo`.
 - A number of inputs (typed variables) `x₁: T₁`...`xᵢ: Tᵢ`.
 - A precondition: a boolean predicate on the contract's inputs.
 - A postcondition: a boolean predicate on the contract's inputs.
 
## Test vectors
A contract can be instantiated with generic arguments and inputs:
picking generic arguments and inputs, we end up with a concrete test
case for a bunch of specific values.

Multiple instantiations for a same contract produces a test vector.

# Problems
## 🚩 Abstractions in pre- and post-conditions
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

**Contracts should be written in an abstract way, but all the
abstraction should collapse when concretizing contracts into tests.**

## 💡✅ Partial computations: have your cake and eat it too
Having abstract contracts and very concrete tests seems a bit
contradictory.

We need a way of collapsing our abstractions, but only the
abstractions. It is the job of the proc-macro crate
`partial_compute`. Given a function of type `(T₁, …, Tᵢ, …, Tⱼ) → U`
it is able to produce a function `(T₁, …, Tᵢ) → Expr`: given the `i`
first arguments, it produces a rust expression (a string) where the
`i` first arguments are inlined and where some parts are pre-computed.

Through this mechanics, we can concretize a contract in a flexible
way.

**✅ This is implemented in `./partial_compute/`.**

## 🚩 Generic arguments

A contract is a way of specifying and testing a Rust piece of
code. Contracts should thus embrace the generic nature of Rust:
contracts are quanitifed not only over values, but also over
(possibly constrainted) generic arguments.

At the level of contract, being very generic is a good thing: thanks
to genericity, we're able to state one property once, and exercise it
one a whole bunch of concrete types and functions.

However, when concretizing contracts into plain tests, this genericity
has to go away. We need to come up with a concrete list of concrete
types and concrete associated inputs.

To concretize the generic arguments of a contract, we need to explore
the types available in the context, and choose some that satisfy the
constraints.

**We need to be able to pick concrete types out of generic
constraints.**

## 💡📋 Search for types and filter them upon constraints

There is two orthogonal problem here:
 1. explore the available types and traits in `core`, what type
    implements what trait, and super-trait relations;
 2. from this trait/type graph, generate randomly concrete types that
    satisfies some constraints.
    
The first problem is mostly already solved by hax. A first simple
strategy for the second is to generate concrete types, and then check
wether bounds are met.

**📋 This is not implemented yet.**

## Inputs and test vectors generation

Similarly to generic arguments, contracts are quantified universally
over a number of inputs. We thus need to generate concrete values for
those inputs.

### 🚩 Generating random values inhabiting a given type

After concretizing its generic arguments, a contract have a number of
inputs, that is, variables of a certain concrete type. To obtain a
test vector, we need to generate inhabitants of each of those concrete
types.

While some type have tangible values (i.e. you can inspect or print
them), some don't: mutable references, interned values or `dyn`
objects for instance. For those, we need to "cook" the value using an
inlined Rust expression. Hopefully, those constructs are somehow
primitive and come into a limited of number.

**We need a way of generating values for any type.**

### 💡✅ (Option A) Use [`arbitrary`](https://docs.rs/arbitrary/1.3.2/arbitrary)

Using the crate `arbitrary`, it is easy to derive a random generator
per type. It is then easy to generate random values.

The downside is that we need to derive an implementation of
`arbitrary` for every single type in `core`.

### 💡📋 (Option B) Leverage hax to construct values

Rust types --omitting unions-- fall into three categories: primitive
ones (`u8`, `bool`, etc.), structs and enums.

Primitive types are the easiest to generate: they come in a limited
number and are simple.

Structs and enums are always built using their constructors. Also, we
only want --and only can-- run test (and thus generate) about public
types.

While a public enum can always be constructed via one of it's variant
constructor, a public struct might declare private fields. Then,
functions or methods in the same visibility scope than such a struct
acts as "smart constructors".

Hax already exports (1) enum and struct definitions (2) the type
signature of any function or method. Leveraging this two pieces of
information, given a type, we can ask a solver (i.e. Z3) to find
different paths toward the construction of a value.

### 🚩 Generating inputs validating preconditions

While a generic argument have a number of bounds that are constraining
what concrete type we can instantiate, the set of input of a contract
is constrained by the precondition of the contract.

**How to generate values satisfying a precondition?**

### ✅ (Option A) Filtering values

A simple brute-force approach consists in generating values regardless
the precondition, and filter them out afterward.

### 📋 (Option B) Using Z3

If the brute-force approach is too slow, we could use Z3 to find
values that matches certain predicates. This is non-trivial, as this
requires to plunge every Rust function in Z3.

## Different shades of completeness

The goal is this project is to add specifications and tests to `core`:
the quality of the specifications and tests are conditioned upon their
completeness: a specification is not very useful if too partial.

Tests usually cannot be complete: the domains of the functions we are
testing are huge. Though, we want tests to have good coverage.

### Crate-level or module-level completeness

We should be able to track how complete we are in term of item
specified within a given namespace.

Each function of a namespace and every method of every implementation
block of a namespace should be specified. Each such a method or
function should count as one computational item.

The goal is to add a complete set of contract for each such item (see
the [next section](fn-completeness) for what completeness means in
this context).

**How do we count items in a namespace?**

### 📋 Leveraging hax

The frontend of hax is already able to export all items that belongs
to a namespace. We just need to consume this information and
re-contextualize it.


### 🚩 Function/method-level contract completeness {#fn-completeness}

A function `f` --or method-- is specified with a set of
contracts. Each contracts comes with its own set of inputs and its own
precondition, potentially exercising only a small portion of the
domain of `f`.

Also, there a two kinds of contracts: models and properties. The
former has its post-condition in the shape `f(...) == <expr>`, where
`<expr>` doesn't use `f`: we give a very concrete expected behavior
for `f` of a certain range of inputs. Any other contracts falls in the
latter kind.

A property model can be, for instance, the commutativity of the
addition: `x + y == y + x`.

The notion of completeness is trivial to define on the contracts that
belongs to the model kind: for every instantiation of `f` (that is,
for every set of generic types and set of inputs), there should exists
an instantiation of a model contract that exercise that precise
instantiation of `f`.

**We need to check for completeness of model contracts.**

### 📋 Using hax and F*

Using hax and its translation to F*, we can easily get:
 - A precondition function for each model contract `C₁`, `…`, `Cₙ` of
   `f`.
 - A lemma for each function `f` we specify, encoding our completeness
   requirement: for all (generic or not) input `v₁`, `…`, `vₘ` (`m` is
   the arity of `f`) of `f`, either `C₁`, `…` or `Cₙ` gives a model to
   `f(v₁, …, vₘ)`.

F* should be pretty good at proving automatically such lemmas.

### 🚩 Function/method-level test vector coverage

The set of test vectors of every contract of a function together
should exercise all the path and edge-cases in the control-flow of a
function.

### 📋 Feedback loop with a code coverage tool

Using a code coverage tool, we should be able to return a metric that
informs on how "complete" our tests are. When this metric is below a
certain minimal threshold, we could re-generate tests and re-iterate
the process until the metric is beyond that threshold.

## Usability of the tool

### Stable test vectors
### Rustdoc output
### Plain rust output (no rustdoc)

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

Tests are being generated via `cargo build` using a build script
located in `build/main.rs` by instantiating the contracts with
generated test vectors. More information is available in the
["Contracts" section](#contracts).

Tests can be run in two different ways:
 - `cargo test` will ask `rustdoc` to test all the documentation
examples from `generated_tests/doc.rs`. The downside is that you need
to trust that `rustdoc` indeed executes all the tests.
 - `cargo run` will run the binary built from
   `generated_tests/bin.rs`, which runs all the tests in a very simple
   and understandable way.
