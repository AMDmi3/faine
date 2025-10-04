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
    /// Get information on first visit of a named failpoint
    ///
    /// A single failpoint may be visited multiple times, for instance in loops
    /// or repeated subroutine calls. This method returns information on a first visit.
    ///
    /// Returns `None` if a named failpoint was never visited.
    ///
    /// See also [`failpoint_status_last`](Self::failpoint_status_last)
    pub fn failpoint_status_first(&self, failpoint_name: &str) -> Option<Branch> {
        for step in &self.path {
            if step.name == failpoint_name {
                return Some(step.branch);
            }
        }
        None
    }

    /// Get information on last visit of a named failpoint
    ///
    /// A single failpoint may be visited multiple times, for instance in loops
    /// or repeated subroutine calls. This method returns information on a last visit.
    ///
    /// Returns `None` if a named failpoint was never visited.
    ///
    /// See also [`failpoint_status_first`](Self::failpoint_status_first)
    pub fn failpoint_status_last(&self, failpoint_name: &str) -> Option<Branch> {
        for step in self.path.iter().rev() {
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
