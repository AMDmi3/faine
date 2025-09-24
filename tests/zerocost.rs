// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::process::Command;

fn run_test_project(faine: bool) -> u64 {
    let mut args = vec!["run", "--release"];
    if faine {
        args.push("--features=faine");
    }

    let output = Command::new("cargo")
        .args(&args)
        .current_dir("tests/zerocost-test-project")
        .output()
        .expect("failed to build or run test project");
    if !output.status.success() {
        panic!("failed to build or run test project");
    }
    str::from_utf8(&output.stdout)
        .expect("failed to parse test project output as utf-8 string")
        .trim()
        .parse()
        .expect("failed to parse test project output as integer")
}

#[test]
#[ignore]
fn test_zerocost() {
    let size_without_faine = run_test_project(false);
    let size_with_faine = run_test_project(true);

    assert!(size_without_faine > 0, "cannot get binary size");
    assert!(
        size_with_faine >= size_without_faine,
        "binary size decrease after addinf faine dependency is absolutely not expected"
    );
    assert_eq!(
        size_without_faine,
        size_with_faine,
        "binary size increase of {} bytes after adding faine",
        size_with_faine - size_without_faine
    );
}
