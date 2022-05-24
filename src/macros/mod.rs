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

/// Returns the name of the calling function without a long module path prefix.
#[macro_export]
macro_rules! type_name_of {
    ($ex:expr) => {{
        type_name_of(ex)
    }};
    ($t:ty) => {{
        std::any::type_name::<$t>()
    }};
}

/// Returns the name of the calling function without a long module path prefix.
#[macro_export]
macro_rules! current_function_name {
    () => {{
        fn f() {}
        let name = $crate::type_name_of(&f);
        // Remove "::f" from the name:
        let name = &name.get(..name.len() - 3).unwrap();
        name
    }};
}

/// `flame_guard!` - creates a flamegraph scope guard using the `flame` crate.
/// ### Input(s):
/// ```ignore
/// flame_guard!();
/// flame_guard!("this_function");
/// flame_guard!("this_function", "this_function_path");
/// flame_guard!("this_function", "this", "function", "path");
/// ```
/// The first version will automatically generate the function name but will probably be more performance overhead
///
/// ### Output:
/// ```ignore
/// fn some_function() {
///     flame_guard!();
/// }
/// ```
/// ```ignore
/// #[cfg(feature = "flame")]
/// let _fg = ::flame::start_guard("<MODULE_PATH>::some_function");
/// ```
/// - `flame_guard!("this_function");`:
/// ```ignore
/// #[cfg(feature = "flame")]
/// let _fg = ::flame::start_guard("<MODULE_PATH>::this_function");
/// ```
/// - `flame_guard!("this_function", "this_module");`
/// ```ignore
/// #[cfg(feature = "flame")]
/// let _fg = ::flame::start_guard("this_module::this_function");
/// ```
/// - `flame_guard!("this_function", "this", "module", "path");`
/// ```ignore
/// #[cfg(feature = "flame")]
/// let _fg = ::flame::start_guard("this::module::path::this_function");
/// ```
#[macro_export]
macro_rules! flame_guard {
    () => {
        #[cfg(feature = "flame")]
        static _THIS_FUNC: &str = $crate::current_function_name!();
        #[cfg(feature = "flame")]
        let _fg = ::flame::start_guard(format!("{}::{}", module_path!(), _THIS_FUNC));
    };
    ($single:expr) => {
        #[cfg(feature = "flame")]
        let _fg = ::flame::start_guard(concat!(file!(), "::", $single));
    };
    ($this:expr, $path:expr $(,)?) => {
        #[cfg(feature = "flame")]
        let _fg = ::flame::start_guard(concat!($path, "::", $this));
    };
    ($this:expr, $($path_segs:expr),+ $(,)?) => {
        #[cfg(feature = "flame")]
        let _fg = ::flame::start_guard(concat!(concat_with::concat!(with "::", $($path_segs,)*), "::", $this));
    };
}

/// `profile_guard!` - like `flame_guard!` but better!
/// ### Input(s):
/// ```ignore
/// profile_guard!("path", "segments", "for", "fn");
/// ```
///
/// ### Output:
/// ```ignore
/// fn some_function() {
///     profile_guard!();
/// }
/// ```
/// ```ignore
/// #[cfg(feature = "flame")]
/// let _fg = ::flame::start_guard("<MODULE_PATH>::some_function");
/// #[cfg(feature = "puffin")]
/// ::puffin::profile_guard!("<MODULE_PATH>::some_function");
/// ```
/// ```ignore
/// fn some_function() {
///     profile_guard!("this_function");
/// }
/// ```
/// ```ignore
/// #[cfg(feature = "flame")]
/// let _fg = ::flame::start_guard("<MODULE_PATH>::this_function");
/// #[cfg(feature = "puffin")]
/// ::puffin::profile_guard!("<MODULE_PATH>::this_function");
/// ```
/// ```ignore
/// fn some_function() {
///     profile_guard!("this_function", "this::module::path");
/// }
/// ```
/// ```ignore
/// #[cfg(feature = "flame")]
/// let _fg = ::flame::start_guard("this::module::path::this_function");
/// #[cfg(feature = "puffin")]
/// ::puffin::profile_guard!("this::module::path::this_function");
/// ```
/// ```ignore
/// fn some_function() {
///     profile_guard!("this_function", "this", "module", "path");
/// }
/// ```
/// ```ignore
/// #[cfg(feature = "flame")]
/// let _fg = ::flame::start_guard("this::module::path::this_function");
/// #[cfg(feature = "puffin")]
/// ::puffin::profile_guard!("this::module::path::this_function");
/// ```
#[macro_export]
macro_rules! profile_guard {
    () => {
        $crate::profile_guard!([""])
    };
    ([$data:expr]) => {
        #[cfg(any(feature = "flame", feature = "puffin"))]
        static _THIS_FUNC: &str = $crate::current_function_name!();
        #[cfg(feature = "flame")]
        let _fg = ::flame::start_guard(format!("{}::{}", module_path!(), _THIS_FUNC));
        #[cfg(feature = "puffin")]
        let _ps = if ::puffin::are_scopes_on() {
            Some(::puffin::ProfilerScope::new(
                _THIS_FUNC,
                module_path!(),
                ToString::to_string(&$data),
            ))
        } else {
            None
        };
    };
    ($single:expr) => {
        $crate::profile_guard!($single, [""])
    };
    ($single:expr, [$data:expr]) => {
        #[cfg(feature = "flame")]
        let _fg = ::flame::start_guard(concat!(file!(), "::", $single));
        #[cfg(feature = "puffin")]
        let _ps = if ::puffin::are_scopes_on() {
            Some(::puffin::ProfilerScope::new(
                $single,
                file!(),
                ToString::to_string(&$data),
            ))
        } else {
            None
        };
    };
    ($this:expr, $path:expr $(,)?) => {
        $crate::profile_guard!($this, $path, [""])
    };
    ($this:expr, $path:expr, [$data:expr]) => {
        #[cfg(feature = "flame")]
        let _fg = ::flame::start_guard(concat!($path, "::", $this));
        #[cfg(feature = "puffin")]
        let _ps = if ::puffin::are_scopes_on() {
            Some(::puffin::ProfilerScope::new(
                $this,
                $path,
                ToString::to_string(&$data),
            ))
        } else {
            None
        };
    };
}

/// `flame_dump!` - creates a call to dump the current flamegraph data to the indicated output.
/// ### Input(s):
/// ```ignore
/// flame_dump!("filename");
/// flame_dump!(html, "filename");
/// flame_dump!(json, "filename");
/// flame_dump!(stdout);
/// ```
/// `html`, `json`, and `stdout` are literals. The first two statements are equivalent (aka the default is `html`).
///
/// ### Output:
/// - `flame_dump!("filename");` or `flame_dump!(html, "filename");`:
/// ```ignore
/// #[cfg(feature = "flame")]
/// {
///     let secs = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
///     ::flame::dump_html(
///         ::std::fs::File::create(
///             format!("flames/{}.{}.html", "filename", secs)
///         ).unwrap()
///     ).unwrap();
/// }
/// ```
/// - `flame_dump!(json, "filename");`:
/// ```ignore
/// #[cfg(feature = "flame")]
/// {
///     let secs = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
///     ::flame::dump_json(
///         &mut ::std::fs::File::create(
///             format!("flames/{}.{}.json", "filename", secs)
///         ).unwrap()
///     ).unwrap();
/// }
/// ```
/// - `flame_dump!(stdout);`:
/// ```ignore
/// #[cfg(feature = "flame")]
/// {
///     ::flame::dump_stdout();
/// }
/// ```
#[macro_export]
macro_rules! flame_dump {
    (stdout) => {
        #[cfg(feature = "flame")]
        {
            ::flame::dump_stdout();
        }
    };
    (html, $filename:expr) => {
        #[cfg(feature = "flame")]
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
    (json, $filename:expr) => {
        #[cfg(feature = "flame")]
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
    ($filename:expr) => {
        #[cfg(feature = "flame")]
        {
            ::flame::dump_html(
                std::fs::File::create(format!(
                    "flames/{}.{}.html",
                    $filename,
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
}

/// `flame_all_tests!(["Path1", "Path2", .., "PathN"], Test1, Test2, .. TestN)`
///
/// Creates a test function named `flame_all_tests` which will call the listed test functions within
/// their own flamegraph scope, and dump the resulting html. Easier than adding a scope or attribute to each test function.
/// **Does not support `should_panic` tests and test failures will abort the function call!**
///
/// ### Input(s)
/// - `Path1 ... PathN` - A bracketed list of strings that will make up the output filename and **main** flamegraph scope.
/// - `Test1 ... TestN` - Any number of test functions that will be invoked with their own flamegraph scope within the main test scope.
///
/// ### Example Usage
/// ```ignore
/// #[cfg(test)]
/// mod tests {
///     #[test]
///     fn test_function_1() { assert!(true); }
///     fn test_function_2() { assert!(true); }
///     fn test_function_3() { assert!(true); }
///     
///     flame_all_tests!(["my_mod", "SubjectUnderTest"], test_function_1, test_function_2, ..., test_function_n);
/// }
/// ```
///
/// ### Output
/// ```ignore
/// // For the example invocation
/// flame_all_tests!(["example", "SomeStruct", "tests"], test_function_1, test_function_2, test_function_3);
/// // Output would be:
/// #[cfg(feature = "flame")]
/// #[cfg_attr(feature = "flame", test)]
/// fn flame_all_tests() {
///     flame_guard!("example", "SomeStruct", "tests");
///     {
///         flame_guard!("test", "test_function_1");
///         test_function_1();
///     }
///     {
///         flame_guard!("test", "test_function_2");
///         test_function_2();
///     }
///     {
///         flame_guard!("test", "test_function_3");
///         test_function_3();
///     }
///     flame_dump!(html, "example.SomeStruct");
/// }
/// ```
#[macro_export]
macro_rules! flame_all_tests {
    ([$($path_seg:expr),+ $(,)?], $($test_fns:ident),+ $(,)?) => {
        #[cfg(feature = "flame")]
        #[cfg_attr(feature = "flame", test)]
        #[cfg_attr(coverage, ignore)]
        fn flame_all_tests() {
            $crate::flame_guard!($($path_seg),+);
            $({
                let _this_test = stringify!($test_fns);
                let _fgt = ::flame::start_guard(format!("test::{}", _this_test));
                $test_fns();
            })*
            $crate::flame_dump!(html, concat_with::concat!(with ".", $($path_seg,)*));
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
        flame_guard!("tester", "flame_guard_usage");
        flame_dump!("some_file");
        flame_dump!(html, "manual_filename");
        flame_dump!(json, "manual_filename");
        flame_dump!(stdout);
    }

    fn profile_scope_usage0() {
        profile_guard!();
    }

    fn profile_scope_usage0b() {
        profile_guard!(["some_data"]);
    }

    fn profile_scope_usage1() {
        profile_guard!("single");
    }

    fn profile_scope_usage1b() {
        profile_guard!("single", ["some_data"]);
    }

    fn profile_scope_usage2() {
        profile_guard!("one", "two::three::four", ["arg_data"]);
    }

    fn profile_scope_usage3() {}

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

    flame_all_tests!(["macro", "usage", "tests"], quicky, flame_guard_usage);
}
