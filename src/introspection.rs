// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::fmt;

use crate::common::Branch;

/// Record of a single failpoint passage
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Step {
    /// Failpoint name
    pub name: &'static str,

    /// Branch taken at the failpoint, e.g. whether the failpoint
    /// was activated or skipped
    pub branch: Branch,
}

/// Execution trace
///
/// A record of completed execution of a tested code fragment,
/// consisting of a sequence of execution steps where the execution
/// has passed through a failpoint
#[derive(Eq, PartialEq, Hash)]
pub struct Trace {
    /// Sequence of execution steps in the order of execution
    pub path: Vec<Step>,
}

impl fmt::Debug for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (n, step) in self.path.iter().enumerate() {
            #[cfg(feature = "fancy-traces")]
            {
                if n > 0 {
                    write!(f, "→")?;
                }

                use colored::Colorize;
                write!(
                    f,
                    "({}{})",
                    match step.branch {
                        Branch::Skip => "",
                        Branch::Activate => "💥",
                    },
                    match step.branch {
                        Branch::Skip => step.name.green(),
                        Branch::Activate => step.name.red(),
                    }
                )?;
            }
            #[cfg(not(feature = "fancy-traces"))]
            {
                if n > 0 {
                    write!(f, "->")?;
                }

                write!(
                    f,
                    "({}{})",
                    match step.branch {
                        Branch::Skip => "",
                        Branch::Activate => "!",
                    },
                    step.name,
                )?;
            }
        }
        Ok(())
    }
}

impl Trace {
    /// Get branch taken in a named failpoint
    ///
    /// Note: execution may pass through a single failpoint multiple times in
    /// case of loops, this returns a first status.
    ///
    /// Returns `None` if a named failpoint was not visited.
    pub fn failpoint_status(&self, failpoint_name: &str) -> Option<Branch> {
        for step in &self.path {
            if step.name == failpoint_name {
                return Some(step.branch);
            }
        }
        None
    }
}

/// Execution report
///
/// Contains cumulative data on series of tested code runs with
/// different failpoint paths.
#[derive(Debug)]
pub struct Report {
    /// Execution traces
    pub traces: Vec<Trace>,

    /// Whether all known execution paths were visited
    pub is_exhaustive: bool,
}

impl PartialEq for Report {
    fn eq(&self, other: &Self) -> bool {
        use std::collections::HashSet;
        self.traces.iter().collect::<HashSet<_>>() == other.traces.iter().collect::<HashSet<_>>()
            && self.is_exhaustive == other.is_exhaustive
    }
}

impl Eq for Report {}
