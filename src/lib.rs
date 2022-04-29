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
    crate_visibility_modifier,
    default_free_fn,
    exclusive_range_pattern,
    explicit_generic_args_with_impl_trait,
    half_open_range_patterns,
    let_else,
    once_cell,
    pattern,
    round_char_boundary,
    test,
    try_blocks
)]
// Activate ALL THE WARNINGS. I want clippy to be as absolutely annoying as fucking possible.
#![warn(
    clippy::pedantic,
    clippy::all,
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
    clippy::unnecessary_wraps
)]

mod db;
mod macros;
mod server;
mod services;
mod term_ui;

pub mod bins;
pub mod types;
pub mod util;

pub use types::{DatabaseError, Error, Result};
pub use util::{
    id::{ShortId, ShortIdError},
    persist::{Method, Persistence},
};

shadow_rs::shadow!(build_info);
