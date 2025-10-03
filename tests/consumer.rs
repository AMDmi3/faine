// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::process::Command;

#[test]
fn test_consumer_unit() {
    let success = Command::new("cargo")
        .args(["test", "test_unit"])
        .current_dir("tests/consumer-test-project")
        .status()
        .expect("failed to build or run test project")
        .success();
    if !success {
        panic!("failed to build or run test project");
    }
}

#[test]
fn test_consumer_integration() {
    let success = Command::new("cargo")
        .args(["test", "test_integration"])
        .current_dir("tests/consumer-test-project")
        .status()
        .expect("failed to build or run test project")
        .success();
    if !success {
        panic!("failed to build or run test project");
    }
}
