Test equality between trait objects
===================================

[![github]](https://github.com/Rayzeq/dyn-eq)
[![crates.io]](https://crates.io/crates/dyn-eq)
[![doc.rs]](https://docs.rs/dyn-eq)
[![license]](https://www.mozilla.org/en-US/MPL/2.0/)
[![build]](https://github.com/Rayzeq/dyn-eq/actions?query=branch%3Amain)
[![passively-maintained]](https://github.com/Rayzeq/dyn-eq/issues)

[github]: https://img.shields.io/badge/github-rayzeq/dyn--eq-a?style=flat-square&logo=github
[crates.io]: https://img.shields.io/crates/v/dyn-eq?style=flat-square&logo=rust
[doc.rs]: https://img.shields.io/badge/docs.rs-dyn--eq-a?style=flat-square&logo=docs.rs
[license]: https://img.shields.io/crates/l/dyn-eq?style=flat-square
[build]: https://img.shields.io/github/actions/workflow/status/Rayzeq/dyn-eq/rust.yml?style=flat-square
[passively-maintained]: https://img.shields.io/badge/maintenance-passively--maintained-brightgreen?style=flat-square

This crate provides a `DynEq` trait which permit comparing trait objects.
If the two objects are instances of different structs, they will always be
not equal. If they are instances of the same struct, the struct's `Eq`
will be used.


## Example

```rust
use dyn_eq::DynEq;

trait MyTrait: DynEq {}
dyn_eq::eq_trait_object!(MyTrait);

impl MyTrait for u8 {}
impl MyTrait for u16 {}

let a: &dyn MyTrait = &5u8;
let a_bis: &dyn MyTrait = &5u8;
let b: &dyn MyTrait = &10u8;
let c: &dyn MyTrait = &5u16;
let d: &dyn MyTrait = &10u16;

// Same type, same value
assert!(a == a_bis);
// Same type, different value
assert!(a != b);
// Different type, different value
assert!(a != d);
// Different type, same value
// Even if the value is the same, the fact that it's a diffrent type means it's not equal
assert!(a != c);

// Now data structures containing Box<dyn MyTrait> can derive Eq.
#[derive(PartialEq, Eq)]
struct Container {
    field: Box<dyn MyTrait>
}
```