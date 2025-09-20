// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::ops::{Index, IndexMut};

use crate::common::Branch;

#[derive(Default)]
pub struct BranchVec<T>([T; 2]);

impl<T> Index<Branch> for BranchVec<T> {
    type Output = T;

    fn index(&self, branch: Branch) -> &Self::Output {
        match branch {
            Branch::Skip => &self.0[0],
            Branch::Activate => &self.0[1],
        }
    }
}

impl<T> IndexMut<Branch> for BranchVec<T> {
    fn index_mut(&mut self, branch: Branch) -> &mut Self::Output {
        match branch {
            Branch::Skip => &mut self.0[0],
            Branch::Activate => &mut self.0[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let mut v = BranchVec::default();
        assert_eq!(v[Branch::Skip], 0);
        assert_eq!(v[Branch::Activate], 0);
        v[Branch::Skip] = 1;
        v[Branch::Activate] = 2;
        assert_eq!(v[Branch::Skip], 1);
        assert_eq!(v[Branch::Activate], 2);
    }

    #[test]
    fn test_default() {
        let v: BranchVec<usize> = Default::default();
        assert_eq!(v[Branch::Skip], 0);
        assert_eq!(v[Branch::Activate], 0);
    }
}
