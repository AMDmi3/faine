use faine::inject_return;

pub fn tested_function() -> bool {
    inject_return!(false);
    true
}

#[cfg(test)]
mod tests {
    use faine::Runner;

    use super::*;

    #[test]
    fn test_unit() {
        let mut res = true;
        Runner::default()
            .run(|| {
                res &= tested_function();
            })
            .unwrap();
        assert!(!res);
    }
}
