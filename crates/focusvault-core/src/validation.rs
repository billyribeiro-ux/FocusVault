use crate::error::{DomainError, DomainResult};
use url::Url;

/// Validates the "Why it matters" field.
/// Must be at least 12 characters and contain at least 3 words.
pub fn validate_why(why: &str) -> DomainResult<()> {
    let trimmed = why.trim();

    if trimmed.len() < 12 {
        return Err(DomainError::validation(
            "Why must be at least 12 characters",
        ));
    }

    let word_count = trimmed.split_whitespace().count();
    if word_count < 3 {
        return Err(DomainError::validation("Why must contain at least 3 words"));
    }

    Ok(())
}

/// Normalizes a URL: trims whitespace, ensures scheme, strips known tracking params.
pub fn normalize_url(raw: &str) -> DomainResult<String> {
    let trimmed = raw.trim();

    if trimmed.is_empty() {
        return Err(DomainError::validation("URL cannot be empty"));
    }

    // Add https:// if no scheme present
    let with_scheme = if !trimmed.contains("://") {
        format!("https://{trimmed}")
    } else {
        trimmed.to_string()
    };

    let mut parsed = Url::parse(&with_scheme)
        .map_err(|e| DomainError::validation(format!("Invalid URL: {e}")))?;

    // Strip common tracking parameters
    let tracking_params = [
        "utm_source",
        "utm_medium",
        "utm_campaign",
        "utm_term",
        "utm_content",
        "fbclid",
        "gclid",
        "ref",
    ];

    let filtered_pairs: Vec<(String, String)> = parsed
        .query_pairs()
        .filter(|(key, _)| !tracking_params.contains(&key.as_ref()))
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    if filtered_pairs.is_empty() {
        parsed.set_query(None);
    } else {
        let query_string: Vec<String> = filtered_pairs
            .iter()
            .map(|(k, v)| {
                if v.is_empty() {
                    k.clone()
                } else {
                    format!("{k}={v}")
                }
            })
            .collect();
        parsed.set_query(Some(&query_string.join("&")));
    }

    // Remove trailing slash for cleanliness (unless it's just the root path)
    let result = parsed.to_string();
    Ok(result)
}

/// Extracts the hostname from a URL string.
pub fn extract_hostname(url: &str) -> Option<String> {
    Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(String::from))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── validate_why tests ──

    #[test]
    fn why_valid_basic() {
        assert!(validate_why("This is important for my project").is_ok());
    }

    #[test]
    fn why_exactly_12_chars_3_words() {
        // "abc defg hij" = 12 chars, 3 words
        assert!(validate_why("abc defg hij").is_ok());
    }

    #[test]
    fn why_rejects_too_short() {
        assert!(validate_why("short").is_err());
    }

    #[test]
    fn why_rejects_11_chars() {
        assert!(validate_why("abc def ghi").is_err()); // 11 chars
    }

    #[test]
    fn why_rejects_two_words() {
        assert!(validate_why("longword anotherlong").is_err()); // only 2 words
    }

    #[test]
    fn why_rejects_empty() {
        assert!(validate_why("").is_err());
    }

    #[test]
    fn why_rejects_whitespace_only() {
        assert!(validate_why("              ").is_err());
    }

    #[test]
    fn why_trims_before_validating() {
        // "  a b c d e f  " trimmed = "a b c d e f" = 11 chars → fail
        assert!(validate_why("  a b c d e f  ").is_err());
    }

    #[test]
    fn why_unicode_chars_counted_by_byte_len() {
        // Unicode: "日本語 テスト データ" = 3 words, > 12 bytes
        assert!(validate_why("日本語 テスト データ").is_ok());
    }

    #[test]
    fn why_single_long_word_rejected() {
        assert!(validate_why("abcdefghijklmnop").is_err()); // 1 word
    }

    // ── normalize_url tests ──

    #[test]
    fn url_basic_normalization() {
        let result = normalize_url("https://example.com").unwrap();
        assert_eq!(result, "https://example.com/");
    }

    #[test]
    fn url_adds_https_scheme() {
        let result = normalize_url("example.com/page").unwrap();
        assert!(result.starts_with("https://"));
    }

    #[test]
    fn url_trims_whitespace() {
        let result = normalize_url("  https://example.com  ").unwrap();
        assert_eq!(result, "https://example.com/");
    }

    #[test]
    fn url_strips_utm_params() {
        let result =
            normalize_url("https://example.com/page?utm_source=twitter&utm_medium=social&q=test")
                .unwrap();
        assert!(result.contains("q=test"));
        assert!(!result.contains("utm_source"));
        assert!(!result.contains("utm_medium"));
    }

    #[test]
    fn url_strips_fbclid() {
        let result = normalize_url("https://example.com?fbclid=abc123&keep=yes").unwrap();
        assert!(!result.contains("fbclid"));
        assert!(result.contains("keep=yes"));
    }

    #[test]
    fn url_removes_query_if_only_tracking() {
        let result = normalize_url("https://example.com?utm_source=test").unwrap();
        assert!(!result.contains('?'));
    }

    #[test]
    fn url_rejects_empty() {
        assert!(normalize_url("").is_err());
    }

    #[test]
    fn url_rejects_whitespace_only() {
        assert!(normalize_url("   ").is_err());
    }

    #[test]
    fn url_preserves_http_scheme() {
        let result = normalize_url("http://example.com").unwrap();
        assert!(result.starts_with("http://"));
    }

    #[test]
    fn url_preserves_path_and_fragment() {
        let result = normalize_url("https://example.com/docs/api#section").unwrap();
        assert!(result.contains("/docs/api"));
        assert!(result.contains("#section"));
    }

    // ── extract_hostname tests ──

    #[test]
    fn hostname_basic() {
        assert_eq!(
            extract_hostname("https://www.example.com/path"),
            Some("www.example.com".to_string())
        );
    }

    #[test]
    fn hostname_invalid_url() {
        assert_eq!(extract_hostname("not a url"), None);
    }
}
