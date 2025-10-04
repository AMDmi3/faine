// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Path chosen when execution passes through a failpoint
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Branch {
    /// Failpoint is skipped lile it never existed
    Skip,

    /// Failpoint is activated, and whatever a given failpoint
    /// was supposed to do happens instead of normal code flow
    Activate,
}

/// Label used when describing code execution path
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Label {
    /// Code execution passes through a named failpont
    Failpoint(&'static str),

    /// Code execution has finished
    Finished,
    // TODO: Panic,
}
