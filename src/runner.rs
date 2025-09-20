// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::__private::{FAILPOINTS, State};
use crate::common::{Branch, Label};
use crate::error::Error;
use crate::options::Options;
use crate::tree::{ExecutionStatus, Tree};

/// Runner for code instrumented with failpoints
///
/// Construct this with [`default()`], tune with [`with_`] methods, and
/// call [`run()`] to run and trace the tested code.
///
/// [`default()`]: Self::default
/// [`with_`]: Self::with_branch_preference
/// [`run()`]: Self::run
///
/// # Example for running from a test
///
/// ```
/// # use faine::{Branch, Runner};
/// # fn foobar() {}
/// #[test]
/// # fn dummy() {}
/// fn test_foobar() {
///     Runner::default()
///         .with_branch_preference(Branch::Activate)
///         .run(|| {
///             // <optional setup code>
///             foobar();
///             // <asserts>
///         })
///         .unwrap();
/// }
/// # test_foobar();
/// ```
#[derive(Default)]
pub struct Runner {
    options: Options,
}

impl Runner {
    /// Select execution order preference
    ///
    /// By default, the runner first tries paths passing through an activated
    /// failpoint. You may instead chose to first try paths skipping it.
    ///
    /// Currently, this does not have any effects apart from execution order.
    pub fn with_branch_preference(mut self, branch_preference: Branch) -> Self {
        self.options.branch_preference = branch_preference;
        self
    }

    /// Run the provided code with failpoint handling
    ///
    /// Runs the provided code, being aware of failpoints defined in it.
    /// The code will be ran multiple times with different failpoint
    /// combinations activated.
    ///
    /// Currently, returns nothing useful and never returns an error (but this
    /// will change in the future), but you can run asserts from the code.
    ///
    /// You can treat a code you pass to it as a regular test.
    pub fn run(self, mut func: impl FnMut()) -> Result<(), Error> {
        FAILPOINTS.with_borrow_mut(|state| {
            assert!(state.is_none(), "failpoints state double initialization");
            *state = Some(Box::new(State {
                enabled: true,
                tree: Tree::new(self.options),
            }));
        });

        loop {
            FAILPOINTS.with_borrow_mut(|state| {
                state
                    .as_mut()
                    .expect("failpoints state must be initialized")
                    .tree
                    .start()
            });

            // TODO: catch panics (but not asserts?)
            func();

            let mut status = ExecutionStatus::Continue;
            FAILPOINTS.with_borrow_mut(|state| {
                status = state
                    .as_mut()
                    .expect("failpoints state must be initialized")
                    .tree
                    .finalize(Label::Finished);
            });

            match status {
                ExecutionStatus::Continue => {}
                ExecutionStatus::Stop => {
                    break;
                }
            }
        }

        FAILPOINTS.with_borrow_mut(|state| {
            let _state = state.take().expect("failpoints state must be initialized");
        });

        Ok(())
    }
}
