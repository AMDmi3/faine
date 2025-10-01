// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::fmt;

use crate::common::Branch;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Step {
    pub name: &'static str,
    pub branch: Branch,
}

#[derive(Eq, PartialEq, Hash)]
pub struct Trace {
    pub path: Vec<Step>,
}

impl fmt::Debug for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (n, step) in self.path.iter().enumerate() {
            write!(f, "{}", if n == 0 { "" } else { "→" })?;
            write!(
                f,
                "{}{}",
                match step.branch {
                    Branch::Skip => "",
                    Branch::Activate => "!",
                },
                step.name,
            )?;
        }
        Ok(())
    }
}

impl Trace {
    pub fn failpoint_status(&self, failpoint_name: &str) -> Option<Branch> {
        for step in &self.path {
            if step.name == failpoint_name {
                return Some(step.branch);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Report {
    pub traces: Vec<Trace>,
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
