// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! value_type {
    ($name:ident { $($fields:ident : $types:ty,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {
            $($fields: $types,)*
        }

        impl $name {
            pub fn new($($fields: $types,)*) -> Self {
                Self { $($fields,)* }
            }

            $(
                pub fn $fields(&self) -> &$types {
                    &self.$fields
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! value_wrapper {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pub $type);

        impl $name {
            pub fn new(value: $type) -> Self {
                Self(value)
            }

            pub fn get(&self) -> &$type {
                &self.0
            }

            pub fn get_mut(&mut self) -> &mut $type {
                &mut self.0
            }

            pub fn into_inner(self) -> $type {
                self.0
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }
    };
}

/// `flame_guard!` - creates a flamegraph scope guard using the `flame` crate.
/// ### Input(s):
/// ```ignore
/// flame_guard!("feature_name", ["path", "segments", "for", "fn"]);
/// flame_guard!(["path", "segments", "for", "fn"]);
/// flame_guard!("path", "segments", "for", "fn");
/// ```
/// (The last two are equivalent, and use the default value for `feature_name` aka `flame_on`.)
///
/// ### Output:
/// - `flame_guard!("feature_name", ["path1", "path2", ...]);`:
/// ```ignore
/// #[cfg(feature = "feature_name")]
/// const FG_PATH: &[&str] = &["path1", "path2", ...];
/// #[cfg(feature = "feature_name")]
/// let _fg = ::flame::start_guard(FG_PATH.join("::"));
/// ```
/// - `flame_guard!(["path1", "path2", ...]);` or `flame_guard!("path1", "path2", ...);`:
/// ```ignore
/// #[cfg(feature = "flame_on")]
/// const FG_PATH: &[&str] = &["path1", "path2", ...];
/// #[cfg(feature = "flame_on")]
/// let _fg = ::flame::start_guard(FG_PATH.join("::"));
/// ```
#[macro_export]
macro_rules! flame_guard {
    // ($feat:expr, [ $($path_seg:expr),+ $(,)? ]) => {
    //     #[cfg(feature = $feat)]
    //     const FG_PATH: &[&str] = &[$($path_seg,)*];
    //     #[cfg(feature = $feat)]
    //     let _fg = ::flame::start_guard(FG_PATH.join("::"));
    // };
    // ([ $($path_seg:expr),+ $(,)? ]) => {
    //     #[cfg(feature = "flame_on")]
    //     const FG_PATH: &[&str] = &[$($path_seg,)*];
    //     #[cfg(feature = "flame_on")]
    //     let _fg = ::flame::start_guard(FG_PATH.join("::"));
    // };
    ($($path_seg:expr),+ $(,)?) => {
        #[cfg(feature = "flame_on")]
        const FG_PATH: &[&str] = &[$($path_seg,)*];
        #[cfg(feature = "flame_on")]
        let _fg = ::flame::start_guard(FG_PATH.join("::"));
    };
}

/// `flame_dump!` - creates a call to dump the current flamegraph data to the indicated output.
/// ### Input(s):
/// ```ignore
/// flame_dump!();
/// flame_dump!(html);
/// flame_dump!(html, "filename");
/// flame_dump!(json);
/// flame_dump!(json, "filename");
/// flame_dump!(stdout);
/// ```
/// `html`, `json`, and `stdout` are literals. The first two statements are equivalent (aka the default is `html`).
///
/// ### Output:
/// - `flame_dump!();` or `flame_dump!(html);` or `flame_dump!(html, "filename");`:
/// ```ignore
/// #[cfg(feature = "flame_on")]
/// {
///     let secs = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
///     ::flame::dump_html(
///         ::std::fs::File::create(
///             format!("flames/{}.{}.html", FG_PATH.join(".") /* or $filename */, secs)
///         ).unwrap()
///     ).unwrap();
/// }
/// ```
/// - `flame_dump!(json);` or `flame_dump!(json, "filename");`:
/// ```ignore
/// #[cfg(feature = "flame_on")]
/// {
///     let secs = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
///     ::flame::dump_json(
///         &mut ::std::fs::File::create(
///             format!("flames/{}.{}.json", FG_PATH.join(".") /* or $filename */, secs)
///         ).unwrap()
///     ).unwrap();
/// }
/// ```
/// - `flame_dump!(stdout);`:
/// ```ignore
/// #[cfg(feature = "flame_on")]
/// {
///     ::flame::dump_stdout();
/// }
/// ```
#[macro_export]
macro_rules! flame_dump {
    () => {
        #[cfg(feature = "flame_on")]
        {
            ::flame::dump_html(
                std::fs::File::create(format!(
                    "flames/{}.{}.html",
                    FG_PATH.join("."),
                    std::time::UNIX_EPOCH
                        .elapsed()
                        .expect("Unable to get time since epoch")
                        .as_secs()
                ))
                .expect("Unable to create new html file"),
            )
            .expect("Unable to dump flamegraph data to html");
        }
    };
    (html) => {
        #[cfg(feature = "flame_on")]
        {
            let secs = std::time::UNIX_EPOCH
                        .elapsed()
                        .expect("Unable to get time since epoch")
                        .as_secs();
            ::flame::dump_html(
                std::fs::File::create(format!(
                    "flames/{}.{}.html",
                    FG_PATH.join("."),
                    secs
                ))
                .expect("Unable to create new html file"),
            )
            .expect("Unable to dump flamegraph data to html");
        }
    };
    (html, $filename:expr) => {
        #[cfg(feature = "flame_on")]
        {
            let secs = std::time::UNIX_EPOCH
                        .elapsed()
                        .expect("Unable to get time since epoch")
                        .as_secs();
            ::flame::dump_html(
                std::fs::File::create(format!(
                    "flames/{}.{}.html",
                    $filename,
                    secs
                ))
                .expect("Unable to create new html file"),
            )
            .expect("Unable to dump flamegraph data to html");
        }
    };
    (json) => {
        #[cfg(feature = "flame_on")]
        {
            let secs = std::time::UNIX_EPOCH
                        .elapsed()
                        .expect("Unable to get time since epoch")
                        .as_secs();
            ::flame::dump_json(
                &mut std::fs::File::create(format!(
                    "flames/{}.{}.json",
                    FG_PATH.join("."),
                    secs,
                ))
                .expect("Unable to create new json file"),
            )
            .expect("Unable to dump flamegraph data to json");
        }
    };
    (json, $filename:expr) => {
        #[cfg(feature = "flame_on")]
        {
            let secs = std::time::UNIX_EPOCH
                        .elapsed()
                        .expect("Unable to get time since epoch")
                        .as_secs();
            ::flame::dump_json(
                &mut std::fs::File::create(format!(
                    "flames/{}.{}.json",
                    $filename,
                    secs,
                ))
                .expect("Unable to create new json file"),
            )
            .expect("Unable to dump flamegraph data to json");
        }
    };
    (stdout) => {
        #[cfg(feature = "flame_on")]
        {
            ::flame::dump_stdout();
        }
    };
}

mod tester {
    value_type!(Test {
        a: u32,
        b: u32,
        c: u32,
    });

    value_wrapper!(TestWrapper, Test);

    fn flame_guard_usage() {
        /// These three invocations output the same code:
        // flame_guard!("flame_on", ["flame_guard_usage", "sub_flame_guard_usage"]);
        // flame_guard!(["tester", "flame_guard_usage"]);
        flame_guard!("tester", "flame_guard_usage");
        flame_dump!();
        flame_dump!(html);
        flame_dump!(html, "manual_filename");
        flame_dump!(json);
        flame_dump!(json, "manual_filename");
        flame_dump!(stdout);
    }

    #[test]
    fn quicky() {
        const STRS: &[&str] = &["a", "b", "c"];
        let s = format!(
            "flames/{}.{:?}.html",
            STRS.join("."),
            std::time::UNIX_EPOCH
                .elapsed()
                .expect("Unable to get time since epoch")
                .as_secs()
        );
        println!("{}", s);
    }
}
