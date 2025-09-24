// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::process::Command;

#[test]
fn test_consumer() {
    let success = Command::new("cargo")
        .args(["test"])
        .current_dir("tests/consumer-test-project")
        .status()
        .expect("failed to build or run test project")
        .success();
    if !success {
        panic!("failed to build or run test project");
    }
}
