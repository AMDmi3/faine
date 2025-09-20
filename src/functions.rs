// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::__private::FAILPOINTS;

/// Enable or disable failpoints
///
/// This is primarily indented to be used in the test code which is run under
/// [`Runner::run()`]. For instance, you may want to run the tested code twice,
/// first time with failpoints enabled to simulate all possible failures, then
/// with failpoints disabled to simulate eventual successful execution and check
/// how it recovers from previous errors.
///
/// It can be run from the instrumented code as well, though.
///
/// Like `inject_*` macros, it does nothing outside of [`Runner::run()`].
///
/// [`Runner::run()`]: crate::Runner::run
pub fn enable_failpoints(enable: bool) {
    FAILPOINTS.with_borrow_mut(|state| {
        if let Some(state) = state {
            state.enabled = enable;
        }
    });
}
