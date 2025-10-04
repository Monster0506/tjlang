//! Tests for STRING module

#[cfg(test)]
mod tests {
    use crate::stdlib::string::*;
    use std::collections::HashMap;

    #[test]
    fn test_length() {
        let result = STRING::length("Hello");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_char_count() {
        let result = STRING::char_count("Hello");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_byte_count() {
        let result = STRING::byte_count("Hello");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_to_uppercase() {
        let result = STRING::to_uppercase("hello");
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_to_lowercase() {
        let result = STRING::to_lowercase("HELLO");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_to_titlecase() {
        let result = STRING::to_titlecase("hello world");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_capitalize() {
        let result = STRING::capitalize("hello");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_reverse() {
        let result = STRING::reverse("hello");
        assert_eq!(result, "olleh");
    }

    #[test]
    fn test_trim() {
        let result = STRING::trim("  hello  ");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_trim_start() {
        let result = STRING::trim_start("  hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_trim_end() {
        let result = STRING::trim_end("hello  ");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_contains() {
        let result = STRING::contains("Hello World", "World");
        assert!(result);
    }

    #[test]
    fn test_starts_with() {
        let result = STRING::starts_with("Hello World", "Hello");
        assert!(result);
    }

    #[test]
    fn test_ends_with() {
        let result = STRING::ends_with("Hello World", "World");
        assert!(result);
    }

    #[test]
    fn test_find() {
        let result = STRING::find("Hello World", "World");
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_rfind() {
        let result = STRING::rfind("Hello World World", "World");
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_find_all() {
        let result = STRING::find_all("Hello World World", "World");
        assert_eq!(result, vec![6, 12]);
    }

    #[test]
    fn test_replace() {
        let result = STRING::replace("Hello World", "World", "TJLang");
        assert_eq!(result, "Hello TJLang");
    }

    #[test]
    fn test_replace_first() {
        let result = STRING::replace_first("Hello World World", "World", "TJLang");
        assert_eq!(result, "Hello TJLang World");
    }

    #[test]
    fn test_replace_last() {
        let result = STRING::replace_last("Hello World World", "World", "TJLang");
        assert_eq!(result, "Hello World TJLang");
    }

    #[test]
    fn test_split() {
        let result = STRING::split("a,b,c", ",");
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_whitespace() {
        let result = STRING::split_whitespace("a b c");
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_lines() {
        let result = STRING::split_lines("line1\nline2\nline3");
        assert_eq!(result, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_split_chars() {
        let result = STRING::split_chars("hello");
        assert_eq!(result, vec!["h", "e", "l", "l", "o"]);
    }

    #[test]
    fn test_join() {
        let strings = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = STRING::join(&strings, ",");
        assert_eq!(result, "a,b,c");
    }

    #[test]
    fn test_slice() {
        let result = STRING::slice("Hello World", 0, 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_substring() {
        let result = STRING::substring("Hello World", 0, 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_pad_left() {
        let result = STRING::pad_left("hello", 10, ' ');
        assert_eq!(result, "     hello");
    }

    #[test]
    fn test_pad_right() {
        let result = STRING::pad_right("hello", 10, ' ');
        assert_eq!(result, "hello     ");
    }

    #[test]
    fn test_pad_center() {
        let result = STRING::pad_center("hello", 10, ' ');
        assert_eq!(result, "  hello   ");
    }

    #[test]
    fn test_format() {
        let args = vec!["World".to_string()];
        let result = STRING::format("Hello, {0}!", &args);
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_format_named() {
        let mut args = HashMap::new();
        args.insert("name".to_string(), "World".to_string());
        let result = STRING::format_named("Hello, {name}!", &args);
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_template() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "World".to_string());
        let result = STRING::template("Hello, ${name}!", &variables);
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_interpolate() {
        let values = vec!["World".to_string()];
        let result = STRING::interpolate("Hello, $0!", &values);
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_is_empty() {
        let result = STRING::is_empty("");
        assert!(result);
    }

    #[test]
    fn test_is_whitespace() {
        let result = STRING::is_whitespace("   ");
        assert!(result);
    }

    #[test]
    fn test_is_alpha() {
        let result = STRING::is_alpha("hello");
        assert!(result);
    }

    #[test]
    fn test_is_numeric() {
        let result = STRING::is_numeric("123");
        assert!(result);
    }

    #[test]
    fn test_is_alphanumeric() {
        let result = STRING::is_alphanumeric("hello123");
        assert!(result);
    }

    #[test]
    fn test_is_ascii() {
        let result = STRING::is_ascii("hello");
        assert!(result);
    }

    #[test]
    fn test_is_digit() {
        let result = STRING::is_digit("123");
        assert!(result);
    }

    #[test]
    fn test_is_hex() {
        let result = STRING::is_hex("1a2b3c");
        assert!(result);
    }

    #[test]
    fn test_encode_base64() {
        let result = STRING::encode_base64("Hello, World!");
        assert_eq!(result, "SGVsbG8sIFdvcmxkIQ==");
    }

    #[test]
    fn test_decode_base64() {
        let result = STRING::decode_base64("SGVsbG8sIFdvcmxkIQ==");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_encode_url() {
        let result = STRING::encode_url("Hello World");
        assert_eq!(result, "Hello%20World");
    }

    #[test]
    fn test_decode_url() {
        let result = STRING::decode_url("Hello%20World");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World");
    }

    #[test]
    fn test_to_unicode() {
        let result = STRING::to_unicode("Hello");
        assert_eq!(result, vec![72, 101, 108, 108, 111]);
    }

    #[test]
    fn test_from_unicode() {
        let codes = vec![72, 101, 108, 108, 111];
        let result = STRING::from_unicode(&codes);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_hash_sha1() {
        let result = STRING::hash_sha1("Hello, World!");
        assert_eq!(result.len(), 40); // SHA1 produces 40 character hex string
    }

    #[test]
    fn test_hash_sha256() {
        let result = STRING::hash_sha256("Hello, World!");
        assert_eq!(result.len(), 64); // SHA256 produces 64 character hex string
    }

    #[test]
    fn test_levenshtein_distance() {
        let result = STRING::levenshtein_distance("kitten", "sitting");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_jaro_similarity() {
        let result = STRING::jaro_similarity("MARTHA", "MARHTA");
        assert!((result - 0.944).abs() < 0.001);
    }

    #[test]
    fn test_to_snake_case() {
        let result = STRING::to_snake_case("HelloWorld");
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_to_camel_case() {
        let result = STRING::to_camel_case("hello world");
        assert_eq!(result, "helloWorld");
    }

    #[test]
    fn test_to_pascal_case() {
        let result = STRING::to_pascal_case("hello world");
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_to_kebab_case() {
        let result = STRING::to_kebab_case("HelloWorld");
        assert_eq!(result, "hello-world");
    }

    #[test]
    fn test_word_count() {
        let result = STRING::word_count("Hello world from TJLang");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_line_count() {
        let result = STRING::line_count("line1\nline2\nline3");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_char_frequency() {
        let result = STRING::char_frequency("hello");
        assert_eq!(result.get(&'l'), Some(&2));
    }

    #[test]
    fn test_word_frequency() {
        let result = STRING::word_frequency("hello world hello");
        assert_eq!(result.get("hello"), Some(&2));
    }

    #[test]
    fn test_remove_whitespace() {
        let result = STRING::remove_whitespace("h e l l o");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_remove_punctuation() {
        let result = STRING::remove_punctuation("Hello, World!");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_remove_digits() {
        let result = STRING::remove_digits("Hello123World");
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_remove_alpha() {
        let result = STRING::remove_alpha("Hello123World");
        assert_eq!(result, "123");
    }

    #[test]
    fn test_normalize_whitespace() {
        let result = STRING::normalize_whitespace("  hello   world  ");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_normalize_case() {
        let result = STRING::normalize_case("HeLLo WoRLd");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_normalize_unicode() {
        let result = STRING::normalize_unicode("Hello");
        assert_eq!(result, "Hello");
    }
}
