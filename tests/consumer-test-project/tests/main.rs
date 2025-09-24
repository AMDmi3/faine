use consumer_test_project::tested_function;
use faine::Runner;

#[test]
fn test_integration() {
    let mut res = true;
    Runner::default()
        .run(|| {
            res &= tested_function();
        })
        .unwrap();
    assert!(!res);
}
