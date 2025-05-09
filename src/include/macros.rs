/// cfg_if MACRO is copied from libc crate.
/// A macro for defining #[cfg] if-else statements.
///
/// This is similar to the `if/elif` C preprocessor macro by allowing definition
/// of a cascade of `#[cfg]` cases, emitting the implementation which matches
/// first.
///
/// This allows you to conveniently provide a long list #[cfg]'d blocks of code
/// without having to rewrite each clause multiple times.
#[macro_export]
macro_rules! cfg_if {
    // match if/else chains with a final `else`
    ($(
        if #[cfg($($meta:meta),*)] { $($it:item)* }
    ) else * else {
        $($it2:item)*
    }) => {
        cfg_if! {
            @__items
            () ;
            $( ( ($($meta),*) ($($it)*) ), )*
            ( () ($($it2)*) ),
        }
    };

    // match if/else chains lacking a final `else`
    (
        if #[cfg($($i_met:meta),*)] { $($i_it:item)* }
        $(
            else if #[cfg($($e_met:meta),*)] { $($e_it:item)* }
        )*
    ) => {
        cfg_if! {
            @__items
            () ;
            ( ($($i_met),*) ($($i_it)*) ),
            $( ( ($($e_met),*) ($($e_it)*) ), )*
            ( () () ),
        }
    };

    // Internal and recursive macro to emit all the items
    //
    // Collects all the negated `cfg`s in a list at the beginning and after the
    // semicolon is all the remaining items
    (@__items ($($not:meta,)*) ; ) => {};
    (@__items ($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ),
     $($rest:tt)*) => {
        // Emit all items within one block, applying an appropriate #[cfg]. The
        // #[cfg] will require all `$m` matchers specified and must also negate
        // all previous matchers.
        cfg_if! { @__apply cfg(all($($m,)* not(any($($not),*)))), $($it)* }

        // Recurse to emit all other items in `$rest`, and when we do so add all
        // our `$m` matchers to the list of `$not` matchers as future emissions
        // will have to negate everything we just matched as well.
        cfg_if! { @__items ($($not,)* $($m,)*) ; $($rest)* }
    };

    // Internal macro to Apply a cfg attribute to a list of items
    (@__apply $m:meta, $($it:item)*) => {
        $(#[$m] $it)*
    };
}

/// ### weak_alias! MACRO
/// 
/// This macro creates a weak alias for a symbol.
/// 
/// It should **`use core::arch::global_asm;`**
/// 
/// **Uasge**: `weak_alias!(__foo, foo)`
#[macro_export]
macro_rules! weak_alias {
    ($src:ident, $alias:ident) => {
        global_asm!(
            concat!(".weak ", stringify!($alias)),
            concat!(".equ ", stringify!($alias), ", ", stringify!($src))
        );
    };
}

#[macro_export]
macro_rules! __syscall {
    ($nr:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { __syscall0($nr as c_long) }
    }};
    ($nr:expr, $arg1:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { __syscall1($nr as c_long, $arg1 as c_long) }
    }};
    ($nr:expr, $arg1:expr, $arg2:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { __syscall2($nr as c_long, $arg1 as c_long, $arg2 as c_long) }
    }};
    ($nr:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { __syscall3($nr as c_long, $arg1 as c_long, $arg2 as c_long, $arg3 as c_long) }
    }};
    ($nr:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { __syscall4($nr as c_long, $arg1 as c_long, $arg2 as c_long, $arg3 as c_long, $arg4 as c_long) }
    }};
    ($nr:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        #[allow(unused_unsafe)]
        unsafe { __syscall5($nr as c_long, $arg1 as c_long, $arg2 as c_long, $arg3 as c_long, $arg4 as c_long, $arg5 as c_long) }
    }};
    ($nr:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            __syscall6(
                $nr as c_long,
                $arg1 as c_long,
                $arg2 as c_long,
                $arg3 as c_long,
                $arg4 as c_long,
                $arg5 as c_long,
                $arg6 as c_long,
            )
        }
    }};
}