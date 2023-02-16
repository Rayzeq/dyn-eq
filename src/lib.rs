// Copyright (c) 2023 Zacharie Dubrulle
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! This crate provides a [`DynEq`] trait that can be used in trait objects,
//! which permit comparing trait objects. If the two objects are instances of
//! different structs, they will always be not equal. If they are instances
//! of the same struct, the struct's [`Eq`] will be used.
//!
//! # Example
//!
//! ```
//! use dyn_eq::DynEq;
//!
//! trait MyTrait: DynEq {}
//! dyn_eq::eq_trait_object!(MyTrait);
//!
//! impl MyTrait for u8 {}
//! impl MyTrait for u16 {}
//!
//! let a: Box<dyn MyTrait> = Box::new(5u8);
//! let a_bis: Box<dyn MyTrait> = Box::new(5u8);
//! let b: Box<dyn MyTrait> = Box::new(10u8);
//! let c: Box<dyn MyTrait> = Box::new(5u16);
//! let d: Box<dyn MyTrait> = Box::new(10u16);
//!
//! // Same type, same value
//! assert!(a == a_bis);
//! // Same type, different value
//! assert!(a != b);
//! // Different type, different value
//! assert!(a != d);
//! // Different type, same value
//! // Even if the value is the same, the fact that it's a diffrent type means it's not equal
//! assert!(a != c);
//! ```
#![no_std]

extern crate alloc;

/// Re-export of [`alloc::boxed::Box`] for the macro.
///
pub use alloc::boxed::Box;
use core::any::Any;

mod macros;

/// This trait is implemented by any type that implements [`PartialEq`] and [`Eq`].
///
/// [`PartialEq`]: ::core::cmp::PartialEq
/// [`Eq`]: ::core::cmp::PartialEq
pub trait DynEq: Any + private::Sealed {
	/// Compare if this object is equal to `other`, which a pointer to a struct of the same
	/// type
	///
	/// # Safety
	///
	/// This function assumes that the object pointed to by `other` is of the same type as `self`.
	/// Not respecting this contract is Undefined Behavior.
	#[doc(hidden)]
	unsafe fn dyn_eq(&self, other: *const ()) -> bool;
}

impl<T: PartialEq + Eq + 'static> DynEq for T {
	unsafe fn dyn_eq(&self, other: *const ()) -> bool {
		self == &*other.cast::<Self>()
	}
}

/// Private module to seal the [`DynEq`] trait
mod private {
	/// Sealing trait
	pub trait Sealed {}
	impl<T> Sealed for T where T: PartialEq {}
}
