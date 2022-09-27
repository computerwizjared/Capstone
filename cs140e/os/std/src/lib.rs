#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(dead_code))]
#![allow(unused_features)]

// Language features:
#![feature(alloc_error_handler)]
#![feature(allocator_internals)]
#![feature(allow_internal_unsafe)]
#![feature(allow_internal_unstable)]
#![feature(box_syntax)]
#![feature(c_unwind)]
#![feature(cfg_accessible)]
#![feature(cfg_eval)]
#![feature(cfg_target_thread_local)]
#![feature(concat_bytes)]
#![feature(concat_idents)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(decl_macro)]
#![feature(deprecated_suggestion)]
#![feature(doc_cfg)]
#![feature(doc_cfg_hide)]
#![feature(doc_masked)]
#![feature(doc_notable_trait)]
#![feature(dropck_eyepatch)]
#![feature(exhaustive_patterns)]
#![feature(intra_doc_pointers)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(link_cfg)]
#![feature(min_specialization)]
#![feature(must_not_suspend)]
#![feature(needs_panic_runtime)]
#![feature(negative_impls)]
#![feature(never_type)]
#![feature(platform_intrinsics)]
#![feature(prelude_import)]
#![feature(rustc_attrs)]
#![feature(rustdoc_internals)]
#![feature(thread_local)]
#![feature(try_blocks)]
#![feature(rustc_private)]
//
// Library features (core):
#![feature(array_error_internals)]
#![feature(atomic_mut_ptr)]
#![feature(char_error_internals)]
#![feature(char_internals)]
#![feature(core_intrinsics)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(cstr_internals)]
#![feature(duration_checked_float)]
#![feature(duration_constants)]
#![feature(exact_size_is_empty)]
#![feature(exclusive_wrapper)]
#![feature(extend_one)]
#![feature(float_minimum_maximum)]
#![feature(hasher_prefixfree_extras)]
#![feature(hashmap_internals)]
#![feature(int_error_internals)]
#![feature(is_some_with)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_write_slice)]
#![feature(mixed_integer_ops)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(panic_can_unwind)]
#![feature(panic_info_message)]
#![feature(panic_internals)]
#![feature(pointer_byte_offsets)]
#![feature(pointer_is_aligned)]
#![feature(portable_simd)]
#![feature(prelude_2024)]
#![feature(provide_any)]
#![feature(ptr_as_uninit)]
#![feature(raw_os_nonzero)]
#![feature(slice_internals)]
#![feature(slice_ptr_get)]
#![feature(std_internals)]
#![feature(str_internals)]
#![feature(strict_provenance)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_uninit_array)]
//
// Library features (alloc):
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
//
// Only for re-exporting:
#![feature(const_format_args)]
#![feature(core_panic)]
#![feature(custom_test_frameworks)]
#![feature(edition_panic)]
#![feature(format_args_nl)]
#![feature(log_syntax)]
#![feature(once_cell)]
#![feature(saturating_int_impl)]
#![feature(stdsimd)]
#![feature(test)]
#![feature(trace_macros)]
//
// Only used in tests/benchmarks:
#![feature(bench_black_box)]
//
#![default_lib_allocator]

#[cfg(not(feature = "std"))]
pub mod error;

#[cfg(feature = "std")]
pub use std::error as error;

#[cfg(not(feature = "std"))]
pub mod io;

#[cfg(feature = "std")]
pub use std::io as io;

#[cfg(feature = "alloc")]
extern crate alloc;

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::rust_2021::*;

// The Rust prelude
pub mod prelude;

pub mod sync;