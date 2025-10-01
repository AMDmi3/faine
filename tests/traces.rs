// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use faine::{
    Branch, Report, Runner, Step, Trace, enable_failpoints, inject_override, inject_return,
};

struct TraceBuilder {
    trace: Trace,
}

impl TraceBuilder {
    pub fn new() -> Self {
        Self {
            trace: Trace { path: vec![] },
        }
    }

    pub fn step(mut self, name: &'static str, branch: Branch) -> Self {
        self.trace.path.push(Step { name, branch });
        self
    }

    pub fn finish(self) -> Trace {
        self.trace
    }
}

#[test]
fn test_linear() {
    fn foo() -> Result<(), usize> {
        inject_return!("first", Err(1));
        inject_return!("second", Err(2));
        Ok(())
    }

    let mut manual_report = Report {
        is_exhaustive: true,
        traces: vec![],
    };

    let final_report = Runner::default()
        .run(|trace| {
            _ = foo();
            manual_report.traces.push(trace.trace());
        })
        .unwrap();

    use Branch::{Activate, Skip};
    let expected_report = Report {
        traces: vec![
            TraceBuilder::new().step("first", Activate).finish(),
            TraceBuilder::new()
                .step("first", Skip)
                .step("second", Activate)
                .finish(),
            TraceBuilder::new()
                .step("first", Skip)
                .step("second", Skip)
                .finish(),
        ],
        is_exhaustive: true,
    };

    assert_eq!(final_report, expected_report);
    assert_eq!(manual_report, expected_report);
}

#[test]
fn test_branching() {
    fn foo() -> Result<(), usize> {
        inject_return!("top", Err(1));
        if inject_override!(true, "fork", false) {
            inject_return!("left", Err(3));
        } else {
            inject_return!("right", Err(5));
        }
        inject_return!("bottom", Err(7));
        Ok(())
    }

    let mut manual_report = Report {
        is_exhaustive: true,
        traces: vec![],
    };

    let final_report = Runner::default()
        .run(|trace| {
            _ = foo();
            manual_report.traces.push(trace.trace());
        })
        .unwrap();

    use Branch::{Activate, Skip};
    let expected_report = Report {
        traces: vec![
            TraceBuilder::new().step("top", Activate).finish(),
            // left at fork (if (true))
            TraceBuilder::new()
                .step("top", Skip)
                .step("fork", Skip)
                .step("left", Activate)
                .finish(),
            TraceBuilder::new()
                .step("top", Skip)
                .step("fork", Skip)
                .step("left", Skip)
                .step("bottom", Skip)
                .finish(),
            TraceBuilder::new()
                .step("top", Skip)
                .step("fork", Skip)
                .step("left", Skip)
                .step("bottom", Activate)
                .finish(),
            // right at fork (if (false))
            TraceBuilder::new()
                .step("top", Skip)
                .step("fork", Activate)
                .step("right", Activate)
                .finish(),
            TraceBuilder::new()
                .step("top", Skip)
                .step("fork", Activate)
                .step("right", Skip)
                .step("bottom", Skip)
                .finish(),
            TraceBuilder::new()
                .step("top", Skip)
                .step("fork", Activate)
                .step("right", Skip)
                .step("bottom", Activate)
                .finish(),
        ],
        is_exhaustive: true,
    };

    assert_eq!(final_report, expected_report);
    assert_eq!(manual_report, expected_report);
}

#[test]
fn test_disabled() {
    fn foo() -> Result<(), usize> {
        inject_return!("first", Err(1));
        enable_failpoints(false);
        inject_return!("disabled", Err(3));
        enable_failpoints(true);
        inject_return!("second", Err(2));
        Ok(())
    }

    let mut manual_report = Report {
        is_exhaustive: true,
        traces: vec![],
    };

    let final_report = Runner::default()
        .run(|trace| {
            _ = foo();
            manual_report.traces.push(trace.trace());
        })
        .unwrap();

    use Branch::{Activate, Skip};
    let expected_report = Report {
        traces: vec![
            TraceBuilder::new().step("first", Activate).finish(),
            TraceBuilder::new()
                .step("first", Skip)
                .step("second", Activate)
                .finish(),
            TraceBuilder::new()
                .step("first", Skip)
                .step("second", Skip)
                .finish(),
        ],
        is_exhaustive: true,
    };

    assert_eq!(final_report, expected_report);
    assert_eq!(manual_report, expected_report);
}
