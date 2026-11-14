//! A minimal library for experimenting with automated releases.

/// Returns a friendly greeting.
pub fn greeting() -> &'static str {
    "Hello from the release-plz sandbox!"
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_is_friendly() {
        assert_eq!(greeting(), "Hello from the release-plz sandbox!");
    }
}
