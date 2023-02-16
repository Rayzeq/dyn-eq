// Copyright (c) 2023 Zacharie Dubrulle
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dyn_eq::DynEq;

trait MyTrait: DynEq {}
dyn_eq::eq_trait_object!(MyTrait);

// This works
#[derive(PartialEq, Eq)]
struct Container {
	field: Box<dyn MyTrait>,
}

#[derive(Debug, PartialEq, Eq)]
struct A {
	value: u32,
}
impl MyTrait for A {}

#[derive(Debug, PartialEq, Eq)]
struct B {
	value: u32,
}
impl MyTrait for B {}

#[test]
fn a_and_a_same_value_equal() {
	let a1: Box<dyn MyTrait> = Box::new(A { value: 5 });
	let a2: Box<dyn MyTrait> = Box::new(A { value: 5 });

	assert!(a1 == a2);
}

#[test]
fn a_and_a_different_value_not_equal() {
	let a1: Box<dyn MyTrait> = Box::new(A { value: 5 });
	let a2: Box<dyn MyTrait> = Box::new(A { value: 6 });

	assert!(a1 != a2);
}

#[test]
fn b_and_b_same_value_equal() {
	let b1: Box<dyn MyTrait> = Box::new(B { value: 5 });
	let b2: Box<dyn MyTrait> = Box::new(B { value: 5 });

	assert!(b1 == b2);
}

#[test]
fn b_and_b_different_value_not_equal() {
	let b1: Box<dyn MyTrait> = Box::new(B { value: 5 });
	let b2: Box<dyn MyTrait> = Box::new(B { value: 6 });

	assert!(b1 != b2);
}

#[test]
fn a_and_b_same_value_not_equal() {
	let a: Box<dyn MyTrait> = Box::new(A { value: 5 });
	let b: Box<dyn MyTrait> = Box::new(B { value: 5 });

	assert!(a != b);
}

#[test]
fn a_and_b_different_value_not_equal() {
	let a: Box<dyn MyTrait> = Box::new(A { value: 5 });
	let b: Box<dyn MyTrait> = Box::new(B { value: 6 });

	assert!(a != b);
}
