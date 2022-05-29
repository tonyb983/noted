// TODO At some point I should probably update this with the features I'm actually using.
#![feature(
    associated_type_defaults,
    backtrace,
    box_patterns,
    box_syntax,
    inline_const,
    const_convert,
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
    io_error_other,
    mixed_integer_ops,
    no_coverage,
    once_cell,
    pattern,
    proc_macro_hygiene,
    round_char_boundary,
    stmt_expr_attributes,
    test,
    try_blocks
)]
// Activate ALL THE WARNINGS. I want clippy to be as absolutely annoying as fucking possible.
#![warn(
    clippy::pedantic,
    clippy::all,
    nonstandard_style,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    // This is working but I prefer to use `cargo-udeps` or `cargo-machete`
    // unused_crate_dependencies,
    // missing_docs,
    // rustdoc::all
    // this is adding too many warnings to out-of-crate code unfortunately
    // clippy::allow_attributes_without_reason,
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

const TRACE_FEATURE: &str = "trace";

shadow_rs::shadow!(build_info);
