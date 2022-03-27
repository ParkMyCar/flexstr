#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! A flexible, simple to use, immutable, clone-efficient [String] replacement for Rust

extern crate alloc;

pub mod custom;
mod storage;
mod string;

pub use crate::string::std_str::{
    BoxedStr, BoxedStrRef, FlexStr, LocalStr, LocalStrRef, SharedStr, SharedStrRef,
};

/// Provides support for [BStr](bstr::BStr)-based [FlexStr] strings
#[cfg(feature = "bstr")]
pub mod b_str {
    pub use crate::string::b_str::{
        BoxedBStr, BoxedBStrRef, FlexBStr, LocalBStr, LocalBStrRef, SharedBStr, SharedBStrRef,
    };
}

/// Provides support for [CStr](std::ffi::CStr)-based [FlexStr] strings
#[cfg(feature = "std")]
pub mod c_str {
    pub use crate::string::c_str::{
        BoxedCStr, BoxedCStrRef, CStrNullError, FlexCStr, LocalCStr, LocalCStrRef, SharedCStr,
        SharedCStrRef,
    };
}

/// Provides support for [OsStr](std::ffi::OsStr)-based [FlexStr] strings
#[cfg(feature = "std")]
pub mod os_str {
    pub use crate::string::os_str::{
        BoxedOsStr, BoxedOsStrRef, FlexOsStr, LocalOsStr, LocalOsStrRef, SharedOsStr,
        SharedOsStrRef,
    };
}

/// Provides support for raw [`[u8]`](slice)-based [FlexStr] strings
pub mod raw_str {
    pub use crate::string::raw_str::{
        BoxedRawStr, BoxedRawStrRef, FlexRawStr, LocalRawStr, LocalRawStrRef, SharedRawStr,
        SharedRawStrRef,
    };
}

use core::mem;

use crate::storage::{BorrowStr, HeapStr, InlineStr};
use crate::string::Str;

// Cannot yet reference associated types from a generic param (impl trait) for const generic params,
// so we are forced to work with raw const generics for now. Also, cannot call const fn functions
// with a trait that has bounds other than `Size` atm.
union FlexStrInner<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>
where
    STR: Str + ?Sized + 'static,
{
    static_str: mem::ManuallyDrop<BorrowStr<BPAD, &'static STR>>,
    inline_str: mem::ManuallyDrop<InlineStr<SIZE, STR>>,
    heap_str: mem::ManuallyDrop<HeapStr<HPAD, HEAP, STR>>,
    borrow_str: mem::ManuallyDrop<BorrowStr<BPAD, &'str STR>>,
}

impl<'str, const SIZE: usize, const BPAD: usize, const HPAD: usize, HEAP, STR>
    FlexStrInner<'str, SIZE, BPAD, HPAD, HEAP, STR>
where
    STR: Str,
{
}
