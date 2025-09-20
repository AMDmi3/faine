// SPDX-FileCopyrightText: Copyright 2025 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Error when executing tested code
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "impossible error")
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    fn returns_error() -> Result<(), Error> {
        Ok(())
    }

    fn returns_anyhow_error() -> Result<(), anyhow::Error> {
        returns_error()?;
        Ok(())
    }

    #[test]
    fn test_error() {
        // only tests compilation in fact
        returns_anyhow_error().unwrap();
    }
}
