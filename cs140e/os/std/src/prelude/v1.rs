//! The first version of the prelude of The Rust Standard Library.
//!
//! See the [module-level documentation](super) for more.

// Re-exported core operators
#[doc(no_inline)]
pub use core::marker::{Send, Sized, Sync, Unpin};
#[doc(no_inline)]
pub use core::ops::{Drop, Fn, FnMut, FnOnce};

// Re-exported functions
#[doc(no_inline)]
pub use core::mem::drop;

// Re-exported types and traits
#[doc(no_inline)]
pub use core::convert::{AsMut, AsRef, From, Into};
#[doc(no_inline)]
pub use core::iter::{DoubleEndedIterator, ExactSizeIterator};
#[doc(no_inline)]
pub use core::iter::{Extend, IntoIterator, Iterator};
#[doc(no_inline)]
pub use core::option::Option::{self, None, Some};
#[doc(no_inline)]
pub use core::result::Result::{self, Err, Ok};

// Re-exported built-in macros
#[allow(deprecated)]
#[doc(no_inline)]
pub use core::prelude::v1::{
    assert, cfg, column, compile_error, concat, concat_idents, env, file, format_args,
    format_args_nl, include, include_bytes, include_str, line, log_syntax, module_path, option_env,
    stringify, trace_macros, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
};

#[doc(no_inline)]
pub use core::prelude::v1::concat_bytes;

// Do not `doc(inline)` these `doc(hidden)` items.
#[allow(deprecated)]
pub use core::prelude::v1::{RustcDecodable, RustcEncodable};

// Do not `doc(no_inline)` so that they become doc items on their own
// (no public module for them to be re-exported from).
pub use core::prelude::v1::{bench, derive, global_allocator, test, test_case};

// Do not `doc(no_inline)` either.
pub use core::prelude::v1::cfg_accessible;

// Do not `doc(no_inline)` either.
pub use core::prelude::v1::cfg_eval;