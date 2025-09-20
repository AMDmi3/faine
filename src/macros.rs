// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Define failpoint which returns from an enclosing function
#[macro_export]
macro_rules! inject_return {
    ($ret:expr) => {{
        const NAME: &str = concat!(file!(), ":", line!(), ":", column!());
        let mut branch = $crate::Branch::Skip;
        $crate::__private::FAILPOINTS.with_borrow_mut(|state| {
            if let Some(state) = state
                && state.enabled
            {
                branch = state.tree.visit($crate::Label::Failpoint(NAME));
            }
        });
        match branch {
            $crate::Branch::Activate => {
                return $ret;
            }
            $crate::Branch::Skip => {}
        }
    }};
    ($name:expr, $ret:expr) => {{
        let mut branch = $crate::Branch::Skip;
        $crate::__private::FAILPOINTS.with_borrow_mut(|state| {
            if let Some(state) = state
                && state.enabled
            {
                branch = state.tree.visit($crate::Label::Failpoint($name));
            }
        });
        match branch {
            $crate::Branch::Activate => {
                return $ret;
            }
            $crate::Branch::Skip => {}
        }
    }};
}

/// Define failpoint which returns [`std::io::Error`] from an enclosing function
#[macro_export]
macro_rules! inject_return_io_error {
    () => {{
        const NAME: &str = concat!(file!(), ":", line!(), ":", column!());
        $crate::inject_return!(NAME, Err(std::io::Error::other(NAME)));
    }};
    ($name:literal) => {{
        $crate::inject_return!($name, Err(std::io::Error::other($name)));
    }};
}

/// Define failpoint which overrides an expression
///
/// When the failpoint is activated, the expression is not executed. If you
/// want to execute is never the less, use `inject_override_with_side_effect!`
#[macro_export]
macro_rules! inject_override {
    ($input:expr, $ret:expr) => {{
        const NAME: &str = concat!(file!(), ":", line!(), ":", column!());
        let mut branch = $crate::Branch::Skip;
        $crate::__private::FAILPOINTS.with_borrow_mut(|state| {
            if let Some(state) = state
                && state.enabled
            {
                branch = state.tree.visit($crate::Label::Failpoint(NAME));
            }
        });
        match branch {
            $crate::Branch::Activate => $ret,
            $crate::Branch::Skip => $input,
        }
    }};
    ($input:expr, $name:expr, $ret:expr) => {{
        let mut branch = $crate::Branch::Skip;
        $crate::__private::FAILPOINTS.with_borrow_mut(|state| {
            if let Some(state) = state
                && state.enabled
            {
                branch = state.tree.visit($crate::Label::Failpoint($name));
            }
        });
        match branch {
            $crate::Branch::Activate => $ret,
            $crate::Branch::Skip => $input,
        }
    }};
}

/// Define failpoint which overrides an expression with [`std::io::Error`]
///
/// When the failpoint is activated, the expression is not executed. If you
/// want to execute is never the less, use `inject_override_with_side_effect_io_error!`
#[macro_export]
macro_rules! inject_override_io_error {
    ($input:expr) => {{
        const NAME: &str = concat!(file!(), ":", line!(), ":", column!());
        $crate::inject_override!($input, NAME, Err(std::io::Error::other(NAME)))
    }};
    ($input:expr, $name:expr) => {{ $crate::inject_override!($input, $name, Err(std::io::Error::other($name))) }};
}

/// Define failpoint which overrides an expression (which is still executed)
///
/// When the failpoint is activated, the expression is never the less executed.
/// You may want this if an expression has a side effect you want to be applied.
///
/// Otherwise, use plain [`inject_override!`]
#[macro_export]
macro_rules! inject_override_with_side_effect {
    ($input:expr, $ret:expr) => {{
        const NAME: &str = concat!(file!(), ":", line!(), ":", column!());
        let mut branch = $crate::Branch::Skip;
        $crate::__private::FAILPOINTS.with_borrow_mut(|state| {
            if let Some(state) = state
                && state.enabled
            {
                branch = state.tree.visit($crate::Label::Failpoint(NAME));
            }
        });
        let res = $input;
        match branch {
            $crate::Branch::Activate => $ret,
            $crate::Branch::Skip => res,
        }
    }};
    ($input:expr, $name:expr, $ret:expr) => {{
        let mut branch = $crate::Branch::Skip;
        $crate::__private::FAILPOINTS.with_borrow_mut(|state| {
            if let Some(state) = state
                && state.enabled
            {
                branch = state.tree.visit($crate::Label::Failpoint($name));
            }
        });
        let res = $input;
        match branch {
            $crate::Branch::Activate => $ret,
            $crate::Branch::Skip => res,
        }
    }};
}

/// Define failpoint which overrides an expression (which is still executed) with [`std::io::Error`]
///
/// When the failpoint is activated, the expression is never the less executed.
/// You may want this if an expression has a side effect you want to be applied.
///
/// Otherwise, use plain [`inject_override_io_error!`]
#[macro_export]
macro_rules! inject_override_with_side_effect_io_error {
    ($input:expr) => {{
        const NAME: &str = concat!(file!(), ":", line!(), ":", column!());
        $crate::inject_override_with_side_effect!($input, NAME, Err(std::io::Error::other(NAME)))
    }};
    ($input:expr, $name:expr) => {{ $crate::inject_override_with_side_effect!($input, $name, Err(std::io::Error::other($name))) }};
}
