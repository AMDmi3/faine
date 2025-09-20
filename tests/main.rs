// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: GPL-3.0-or-later

use faine::{Branch, Runner, enable_failpoints, inject_return};

#[test]
fn test_runner_with_no_failpoints() {
    Runner::default().run(|| {}).unwrap();
}

#[test]
fn test_failpoints_outside_of_runner() {
    fn foo() -> Result<(), usize> {
        inject_return!("1", Err(1));
        Ok(())
    }
    assert_eq!(foo(), Ok(()));
}

#[test]
#[ignore] // TODO: handle panics
fn test_panic() {
    // runner should catch this panic (or should it, how do we handle asserts?)
    Runner::default()
        .run(|| {
            panic!("this panic should be caught");
        })
        .unwrap();
    // repeatable run should pass fine, e.g. there should be no
    // leftover state from the panicked run
    Runner::default().run(|| {}).unwrap();
}

#[test]
fn test_simple() {
    fn foo() -> Result<(), usize> {
        inject_return!("1", Err(1));
        inject_return!("2", Err(2));
        inject_return!("3", Err(3));
        Ok(())
    }

    let mut results = vec![];
    Runner::default()
        .run(|| {
            results.push(foo());
        })
        .unwrap();
    results.sort();

    assert_eq!(results, vec![Ok(()), Err(1), Err(2), Err(3)]);
}

#[test]
fn test_enable_disable() {
    fn foo() -> Result<(), usize> {
        inject_return!("1", Err(1));
        enable_failpoints(false);
        inject_return!("2", Err(2));
        enable_failpoints(true);
        inject_return!("3", Err(3));
        Ok(())
    }

    let mut results = vec![];
    Runner::default()
        .run(|| {
            results.push(foo());
        })
        .unwrap();
    results.sort();

    assert_eq!(results, vec![Ok(()), Err(1), Err(3)]);
}

#[test]
fn test_branch_preference_default() {
    fn foo() -> Result<(), usize> {
        inject_return!("1", Err(1));
        Ok(())
    }

    let mut results = vec![];
    Runner::default()
        .run(|| {
            results.push(foo());
        })
        .unwrap();

    assert_eq!(results, vec![Err(1), Ok(())]);
}

#[test]
fn test_branch_preference_custom() {
    fn foo() -> Result<(), usize> {
        inject_return!("1", Err(1));
        Ok(())
    }

    let mut results = vec![];
    Runner::default()
        .with_branch_preference(Branch::Skip)
        .run(|| {
            results.push(foo());
        })
        .unwrap();

    assert_eq!(results, vec![Ok(()), Err(1)]);
}
