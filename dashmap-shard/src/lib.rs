//! This crate is a Rust port of Google's high-performance [SwissTable] hash
//! map, adapted to make it a drop-in replacement for Rust's standard `HashMap`
//! and `HashSet` types.
//!
//! The original C++ version of [SwissTable] can be found [here], and this
//! [CppCon talk] gives an overview of how the algorithm works.
//!
//! [SwissTable]: https://abseil.io/blog/20180927-swisstables
//! [here]: https://github.com/abseil/abseil-cpp/blob/master/absl/container/internal/raw_hash_set.h
//! [CppCon talk]: https://www.youtube.com/watch?v=ncHmEUmJZf4

#![no_std]
#![cfg_attr(
    feature = "nightly",
    feature(
        alloc_layout_extra,
        allocator_api,
        ptr_offset_from,
        test,
        core_intrinsics,
        dropck_eyepatch,
        cfg_doctest,
    )
)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]
#![warn(rust_2018_idioms)]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(has_extern_crate_alloc)]
#[cfg_attr(test, macro_use)]
extern crate alloc;
#[cfg(not(has_extern_crate_alloc))]
extern crate std as alloc;

#[cfg(feature = "nightly")]
#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[macro_use]
mod macros;

#[cfg(feature = "raw")]
/// Experimental and unsafe `RawTable` API. This module is only available if the
/// `raw` feature is enabled.
pub mod raw {
    // The RawTable API is still experimental and is not properly documented yet.
    #[allow(missing_docs)]
    #[path = "mod.rs"]
    mod inner;
    pub use inner::*;

    #[cfg(feature = "rayon")]
    pub mod rayon {
        pub use super::external_trait_impls::rayon::raw::*;
    }
}
#[cfg(not(feature = "raw"))]
mod raw;

mod external_trait_impls;
mod map;
#[cfg(feature = "rustc-internal-api")]
mod rustc_entry;
mod scopeguard;
mod set;

pub mod hash_map {
    //! A hash map implemented with quadratic probing and SIMD lookup.
    pub use super::map::*;

    #[cfg(feature = "rustc-internal-api")]
    pub use super::rustc_entry::*;

    #[cfg(feature = "rayon")]
    /// [rayon]-based parallel iterator types for hash maps.
    /// You will rarely need to interact with it directly unless you have need
    /// to name one of the iterator types.
    ///
    /// [rayon]: https://docs.rs/rayon/1.0/rayon
    pub mod rayon {
        pub use super::super::external_trait_impls::rayon::map::*;
    }
}
pub mod hash_set {
    //! A hash set implemented as a `HashMap` where the value is `()`.
    pub use super::set::*;

    #[cfg(feature = "rayon")]
    /// [rayon]-based parallel iterator types for hash sets.
    /// You will rarely need to interact with it directly unless you have need
    /// to name one of the iterator types.
    ///
    /// [rayon]: https://docs.rs/rayon/1.0/rayon
    pub mod rayon {
        pub use super::super::external_trait_impls::rayon::set::*;
    }
}

pub use map::HashMap;
pub use set::HashSet;

/// Augments `AllocErr` with a `CapacityOverflow` variant.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CollectionAllocErr {
    /// Error due to the computed capacity exceeding the collection's maximum
    /// (usually `isize::MAX` bytes).
    CapacityOverflow,
    /// Error due to the allocator.
    AllocErr {
        /// The layout of the allocation request that failed.
        layout: alloc::alloc::Layout,
    },
}
