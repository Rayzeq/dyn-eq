// Copyright (c) 2023 Zacharie Dubrulle
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Macros to automatically implement [`PartialEq`] and [`Eq`] on `Box<TraitObject>`.
//!
//! Almost everything here has been taken from [dyn-clone] by David Tolnay.
//!
//! [`PartialEq`]: ::core::cmp::PartialEq
//! [`Eq`]: ::core::cmp::Eq
//! [dyn-clone]: https://github.com/dtolnay/dyn-clone

/// Implement [`PartialEq`] and [`Eq`] for a trait object that has [`DynEq`] as a supertrait.
///
/// ```
/// use dyn_eq::DynEq;
///
/// trait MyTrait: DynEq {
///     /* ... */
/// }
///
/// dyn_eq::eq_trait_object!(MyTrait);
///
/// #[derive(PartialEq, Eq)]
/// struct MyEq {}
///
/// impl MyTrait for MyEq {
///     /* ... */
/// }
///
/// // Now data structures containing Box<dyn MyTrait> can derive Eq.
/// # #[cfg(feature = "alloc")]
/// #[derive(PartialEq, Eq)]
/// struct Container {
///     trait_object: Box<dyn MyTrait>,
/// }
/// ```
///
/// The macro supports traits that have type parameters and/or where clauses.
///
/// ```
/// use dyn_eq::DynEq;
/// use std::io::Read;
///
/// trait Difficult<R>: DynEq where R: Read {
///     /* ... */
/// }
///
/// dyn_eq::eq_trait_object!(<R> Difficult<R> where R: Read + 'static);
/// ```
///
/// [`PartialEq`]: ::core::cmp::PartialEq
/// [`Eq`]: ::core::cmp::Eq
/// [`DynEq`]: super::DynEq
#[macro_export]
macro_rules! eq_trait_object {
	($($path:tt)+) => {
		$crate::__internal_eq_trait_object!(begin $($path)+);
	};
}

/// Internal implementation of [`eq_trait_object`].
#[doc(hidden)]
#[macro_export]
macro_rules! __internal_eq_trait_object {
	// Invocation started with `<`, parse generics.
	(begin < $($rest:tt)*) => {
		$crate::__internal_eq_trait_object!(generics () () $($rest)*);
	};

	// Invocation did not start with `<`.
	(begin $first:tt $($rest:tt)*) => {
		$crate::__internal_eq_trait_object!(path () ($first) $($rest)*);
	};

	// End of generics.
	(generics ($($generics:tt)*) () > $($rest:tt)*) => {
		$crate::__internal_eq_trait_object!(path ($($generics)*) () $($rest)*);
	};

	// Generics open bracket.
	(generics ($($generics:tt)*) ($($brackets:tt)*) < $($rest:tt)*) => {
		$crate::__internal_eq_trait_object!(generics ($($generics)* <) ($($brackets)* <) $($rest)*);
	};

	// Generics close bracket.
	(generics ($($generics:tt)*) (< $($brackets:tt)*) > $($rest:tt)*) => {
		$crate::__internal_eq_trait_object!(generics ($($generics)* >) ($($brackets)*) $($rest)*);
	};

	// Token inside of generics.
	(generics ($($generics:tt)*) ($($brackets:tt)*) $first:tt $($rest:tt)*) => {
		$crate::__internal_eq_trait_object!(generics ($($generics)* $first) ($($brackets)*) $($rest)*);
	};

	// End with `where` clause.
	(path ($($generics:tt)*) ($($path:tt)*) where $($rest:tt)*) => {
		$crate::__internal_eq_trait_object!(impl ($($generics)*) ($($path)*) ($($rest)*));
	};

	// End without `where` clause.
	(path ($($generics:tt)*) ($($path:tt)*)) => {
		$crate::__internal_eq_trait_object!(impl ($($generics)*) ($($path)*) ());
	};

	// Token inside of path.
	(path ($($generics:tt)*) ($($path:tt)*) $first:tt $($rest:tt)*) => {
		$crate::__internal_eq_trait_object!(path ($($generics)*) ($($path)* $first) $($rest)*);
	};

	// The impl.
	(impl ($($generics:tt)*) ($($path:tt)*) ($($bound:tt)*)) => {
		impl<'eq, $($generics)*> ::core::cmp::PartialEq for (dyn $($path)* + 'eq) where $($bound)* {
			$crate::__internal_eq_trait_object!(func ($($path)*));
		}
		impl<'eq, $($generics)*> ::core::cmp::PartialEq for (dyn $($path)* + ::core::marker::Send + 'eq) where $($bound)* {
			$crate::__internal_eq_trait_object!(func ($($path)*));
		}
		impl<'eq, $($generics)*> ::core::cmp::PartialEq for (dyn $($path)* + ::core::marker::Sync + 'eq) where $($bound)* {
			$crate::__internal_eq_trait_object!(func ($($path)*));
		}
		impl<'eq, $($generics)*> ::core::cmp::PartialEq for (dyn $($path)* + ::core::marker::Send + ::core::marker::Sync + 'eq) where $($bound)* {
			$crate::__internal_eq_trait_object!(func ($($path)*));
		}

		// Needed because of [this issue](https://github.com/rust-lang/rust/issues/31740)
		#[cfg(feature = "alloc")]
		impl<'eq, $($generics)*> ::core::cmp::PartialEq<&Self> for $crate::Box<dyn $($path)* + 'eq> where $($bound)* {
			fn eq(&self, other: &&Self) -> bool {
				self == *other
			}
		}
		#[cfg(feature = "alloc")]
		impl<'eq, $($generics)*> ::core::cmp::PartialEq<&Self> for $crate::Box<dyn $($path)* + ::core::marker::Send + 'eq> where $($bound)* {
			fn eq(&self, other: &&Self) -> bool {
				self == *other
			}
		}
		#[cfg(feature = "alloc")]
		impl<'eq, $($generics)*> ::core::cmp::PartialEq<&Self> for $crate::Box<dyn $($path)* + ::core::marker::Sync + 'eq> where $($bound)* {
			fn eq(&self, other: &&Self) -> bool {
				self == *other
			}
		}
		#[cfg(feature = "alloc")]
		impl<'eq, $($generics)*> ::core::cmp::PartialEq<&Self> for $crate::Box<dyn $($path)* + ::core::marker::Send + ::core::marker::Sync + 'eq> where $($bound)* {
			fn eq(&self, other: &&Self) -> bool {
				self == *other
			}
		}

		impl<'eq, $($generics)*> ::core::cmp::Eq for (dyn $($path)* + 'eq) where $($bound)* {}
		impl<'eq, $($generics)*> ::core::cmp::Eq for (dyn $($path)* + ::core::marker::Send + 'eq) where $($bound)* {}
		impl<'eq, $($generics)*> ::core::cmp::Eq for (dyn $($path)* + ::core::marker::Sync + 'eq) where $($bound)* {}
		impl<'eq, $($generics)*> ::core::cmp::Eq for (dyn $($path)* + ::core::marker::Send + ::core::marker::Sync + 'eq) where $($bound)* {}
	};

	// The implementation of the `eq` function.
	(func ($($path:tt)*)) => {
		fn eq(&self, other: &Self) -> bool {
			if self.type_id() == other.type_id() {
				unsafe { self.dyn_eq(other as *const _ as *const ()) }
			} else {
				false
			}
		}
	}
}
