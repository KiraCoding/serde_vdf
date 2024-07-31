#![no_std]

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! {
    "serde_vdf requires that either `std` (default) or `alloc` feature is enabled"
}

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod error;
pub mod lexer;
pub mod parser;
pub mod ser;

mod io;

// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}
