//! A minimal library for experimenting with automated releases.

/// Returns a friendly greeting.
pub fn greeting() -> &'static str {
    "Hello from the release-plz sandbox!"
}

/// Returns a greeting for a name, trimming surrounding whitespace.
///
/// Empty names fall back to the default greeting.
///
/// ```
/// use petaxp_release_plz_sandbox::{greeting, greeting_for};
///
/// assert_eq!(greeting_for("  Rust  "), "Hello, Rust!");
/// assert_eq!(greeting_for(" "), greeting());
/// ```
pub fn greeting_for(name: &str) -> String {
    let name = name.trim();
    if name.is_empty() {
        greeting().to_owned()
    } else {
        format!("Hello, {name}!")
    }
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_is_friendly() {
        assert_eq!(greeting(), "Hello from the release-plz sandbox!");
    }
}
