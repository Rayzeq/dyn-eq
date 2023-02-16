Test equality between trait objects
===================================

[![github](https://img.shields.io/badge/github-rayzeq/dyn--eq-a?style=flat-square&logo=github)](https://github.com/Rayzeq/dyn-eq)
[![crates.io](https://img.shields.io/crates/v/dyn-eq?style=flat-square&logo=rust)](https://crates.io/crates/dyn-eq)
[![doc.rs](https://img.shields.io/badge/docs.rs-dyn--eq-a?style=flat-square&logo=docs.rs)](https://docs.rs/dyn-eq)
![license](https://img.shields.io/crates/l/dyn-eq?style=flat-square)
![build](https://img.shields.io/github/actions/workflow/status/Rayzeq/dyn-eq/rust.yml?style=flat-square)
![passively-maintained](https://img.shields.io/badge/maintenance-passively--maintained-brightgreen?style=flat-square)

This crate provides a `DynEq` trait that can be used in trait objects,
which permit comparing trait objects. If the two objects are instances of
different structs, they will always be not equal. If they are instances
of the same struct, the struct's `Eq` will be used.

## Example

```rust
use dyn_eq::DynEq;

trait MyTrait: DynEq {}
dyn_eq::eq_trait_object!(MyTrait);

impl MyTrait for u8 {}
impl MyTrait for u16 {}

let a: Box<dyn MyTrait> = Box::new(5u8);
let a_bis: Box<dyn MyTrait> = Box::new(5u8);
let b: Box<dyn MyTrait> = Box::new(10u8);
let c: Box<dyn MyTrait> = Box::new(5u16);
let d: Box<dyn MyTrait> = Box::new(10u16);

// Same type, same value
assert!(a == a_bis);
// Same type, different value
assert!(a != b);
// Different type, different value
assert!(a != d);
// Different type, same value
// Even if the value is the same, the fact that it's a diffrent type means it's not equal
assert!(a != c);
```