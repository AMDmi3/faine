// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::tree::Tree;
use std::cell::RefCell;

pub struct State {
    pub enabled: bool,
    pub tree: Tree,
}

thread_local! {
    pub static FAILPOINTS: RefCell<Option<Box<State>>> = const { RefCell::new(None) };
}
