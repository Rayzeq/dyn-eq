// Copyright (c) 2023 Zacharie Dubrulle
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [![github]](https://github.com/Rayzeq/dyn-eq)
//! [![crates.io]](https://crates.io/crates/dyn-eq)
//! [![license]](https://www.mozilla.org/en-US/MPL/2.0/)
//! [![passively-maintained]](https://github.com/Rayzeq/dyn-eq/issues)
//!
//! [github]: https://img.shields.io/badge/github-rayzeq/dyn--eq-a?style=for-the-badge&logo=github
//! [crates.io]: https://img.shields.io/crates/v/dyn-eq?style=for-the-badge&logo=rust
//! [license]: https://img.shields.io/crates/l/dyn-eq?style=for-the-badge
//! [passively-maintained]: https://img.shields.io/badge/maintenance-passively--maintained-brightgreen?style=for-the-badge
//!
//! This crate provides a [`DynEq`] trait which permit comparing trait objects.
//! If the two objects are instances of different structs, they will always be
//! not equal. If they are instances of the same struct, the struct's [`Eq`]
//! will be used.
//!
//! ###### Todos
//!
//! Here's a list of things that could be done and could be nice to have, but I'll implement them only if someone ask:
//!   - [ ] Permit having `PartialEq` without `Eq` (implementation on `dyn Trait` will follow)
//!
//! # Features
//!
//! This crate has one feature: `alloc`, which is enabled by default. Disabling
//! this feature removes the dependency on the [`alloc`] crate, but you won't be
//! able to use [`DynEq`] for `Box<dyn Trait>`.
//!
//! [`alloc`]: https://doc.rust-lang.org/alloc/
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
//! let a: &dyn MyTrait = &5u8;
//! let a_bis: &dyn MyTrait = &5u8;
//! let b: &dyn MyTrait = &10u8;
//! let c: &dyn MyTrait = &5u16;
//! let d: &dyn MyTrait = &10u16;
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
//!
//! // Now data structures containing Box<dyn MyTrait> can derive Eq (only when `alloc`
//! // feature is enabled).
//! # #[cfg(feature = "alloc")]
//! #[derive(PartialEq, Eq)]
//! struct Container {
//!     field: Box<dyn MyTrait>
//! }
//! ```
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

/// Re-export of [`alloc::boxed::Box`] for the macro.
#[cfg(feature = "alloc")]
#[doc(hidden)]
pub use alloc::boxed::Box;
use core::any::Any;

mod macros;

/// This trait is implemented by any type that implements [`Eq`].
pub trait DynEq: Any + private::Sealed {
	/// Upcast this reference to a `&dyn Any`, which can then be passed to [`dyn_eq`](DynEq::dyn_eq).
	#[doc(hidden)]
	fn as_any(&self) -> &dyn Any;

	/// This method tests for self and other values to be equal.
	#[doc(hidden)]
	fn dyn_eq(&self, other: &dyn Any) -> bool;
}

impl<T: Eq + 'static> DynEq for T {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn dyn_eq(&self, other: &dyn Any) -> bool {
		other.downcast_ref().map_or(false, |other| self == other)
	}
}

/// Private module to seal the [`DynEq`] trait
mod private {
	/// Sealing trait
	pub trait Sealed {}
	impl<T> Sealed for T where T: PartialEq {}
}
