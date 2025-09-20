// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::common::Branch;

pub struct Options {
    pub branch_preference: Branch,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            branch_preference: Branch::Activate,
        }
    }
}

impl Options {
    pub fn branch_preference(mut self, branch_preference: Branch) -> Self {
        self.branch_preference = branch_preference;
        self
    }
}
