// TODO At some point I should probably update this with the features I'm actually using.
#![feature(
    associated_type_defaults,
    backtrace,
    box_patterns,
    box_syntax,
    inline_const,
    // inline_const_pat,
    const_trait_impl,
    control_flow_enum,
    concat_idents,
    const_option,
    const_slice_index,
    const_type_name,
    crate_visibility_modifier,
    default_free_fn,
    exclusive_range_pattern,
    explicit_generic_args_with_impl_trait,
    half_open_range_patterns,
    let_chains,
    let_else,
    lint_reasons,
    no_coverage,
    once_cell,
    pattern,
    proc_macro_hygiene,
    round_char_boundary,
    test,
    try_blocks
)]
// Activate ALL THE WARNINGS. I want clippy to be as absolutely annoying as fucking possible.
#![warn(
    clippy::pedantic,
    clippy::all,
    // this is adding too many weird false positives unfortunately
    // clippy::allow_attributes_without_reason,
    // missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    // rustdoc::all
)]
#![allow(
    // This is a library so there's going to be a lot of unused
    unused,
    // I will remove this later on, but for now it's less pointlessly annoying
    dead_code,
    // I hate this lint
    clippy::module_inception,
    // I also hate this lint
    clippy::module_name_repetitions,
    // I am undecided on this lint
    clippy::unnecessary_wraps,
    // I use them sparingly and only when appropriate
    clippy::wildcard_imports,
    reason = "these lints are annoying and somewhat pointless"
)]

//! ## Profiling Attributes
//!
//! Ignore for coverage if `no_coverage` feature is not globally enabled:
//!   `#[cfg_attr(coverage, feature(no_coverage))]`
//! Otherwise:
//!   `#[no_coverage]`
//!
//! Flamegraph function (or any other item, modules, etc.) if feature-gated:
//!   `#[cfg_attr(feature = "flame", flame)]`
//! Otherwise:
//!   `#[flame]`
//!
//! Only run block of code when flamegraphing:
//!  `#[cfg(feature = "flame")]`

pub mod db;
mod macros;
mod server;
mod services;
mod term_ui;

pub mod bins;
pub mod types;
pub mod util;

pub use types::{DatabaseError, Error, Result};
pub use util::persist::{Method, Persistence};

#[doc(hidden)]
#[allow(clippy::inline_always, reason = "I know what im about son.")]
#[inline(always)]
#[must_use]
pub const fn type_name_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

shadow_rs::shadow!(build_info);
