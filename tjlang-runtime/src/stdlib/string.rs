//! STRING Module - String operations
//!
//! Provides comprehensive string functionality including:
//! - String manipulation and formatting
//! - Regular expressions
//! - Encoding/decoding operations
//! - String searching and replacement
//! - Text processing utilities
//! - Unicode operations
//! - String validation
//! - Template processing

use crate::values::Value;
use std::collections::HashMap;

/// STRING module for string operations
pub struct STRING;

impl STRING {
    // Basic string operations
    pub fn length(s: &str) -> usize {
        s.len()
    }
    pub fn char_count(s: &str) -> usize {
        s.chars().count()
    }
    pub fn byte_count(s: &str) -> usize {
        s.len()
    }

    pub fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }
    pub fn to_lowercase(s: &str) -> String {
        s.to_lowercase()
    }
    pub fn to_titlecase(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => {
                first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
            }
        }
    }

    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }
    pub fn trim_start(s: &str) -> String {
        s.trim_start().to_string()
    }
    pub fn trim_end(s: &str) -> String {
        s.trim_end().to_string()
    }

    // String searching
    pub fn contains(s: &str, pattern: &str) -> bool {
        s.contains(pattern)
    }
    pub fn starts_with(s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }
    pub fn ends_with(s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }

    pub fn find(s: &str, pattern: &str) -> Option<usize> {
        s.find(pattern)
    }
    pub fn rfind(s: &str, pattern: &str) -> Option<usize> {
        s.rfind(pattern)
    }
    pub fn find_all(s: &str, pattern: &str) -> Vec<usize> {
        s.match_indices(pattern).map(|(i, _)| i).collect()
    }

    // String replacement
    pub fn replace(s: &str, from: &str, to: &str) -> String {
        s.replace(from, to)
    }
    pub fn replace_first(s: &str, from: &str, to: &str) -> String {
        if let Some(pos) = s.find(from) {
            format!("{}{}{}", &s[..pos], to, &s[pos + from.len()..])
        } else {
            s.to_string()
        }
    }
    pub fn replace_last(s: &str, from: &str, to: &str) -> String {
        if let Some(pos) = s.rfind(from) {
            format!("{}{}{}", &s[..pos], to, &s[pos + from.len()..])
        } else {
            s.to_string()
        }
    }

    // String splitting and joining
    pub fn split(s: &str, delimiter: &str) -> Vec<String> {
        s.split(delimiter).map(|s| s.to_string()).collect()
    }
    pub fn split_whitespace(s: &str) -> Vec<String> {
        s.split_whitespace().map(|s| s.to_string()).collect()
    }
    pub fn split_lines(s: &str) -> Vec<String> {
        s.lines().map(|s| s.to_string()).collect()
    }
    pub fn split_chars(s: &str) -> Vec<String> {
        s.chars().map(|c| c.to_string()).collect()
    }

    pub fn join(strings: &[String], separator: &str) -> String {
        strings.join(separator)
    }

    // String slicing
    pub fn slice(s: &str, start: usize, end: usize) -> String {
        let chars: Vec<char> = s.chars().collect();
        if start >= chars.len() || end > chars.len() || start >= end {
            return String::new();
        }
        chars[start..end].iter().collect()
    }
    pub fn substring(s: &str, start: usize, length: usize) -> String {
        let chars: Vec<char> = s.chars().collect();
        if start >= chars.len() {
            return String::new();
        }
        let end = (start + length).min(chars.len());
        chars[start..end].iter().collect()
    }

    // String padding
    pub fn pad_left(s: &str, width: usize, fill_char: char) -> String {
        if s.len() >= width {
            s.to_string()
        } else {
            let padding = width - s.len();
            format!("{}{}", fill_char.to_string().repeat(padding), s)
        }
    }
    pub fn pad_right(s: &str, width: usize, fill_char: char) -> String {
        if s.len() >= width {
            s.to_string()
        } else {
            let padding = width - s.len();
            format!("{}{}", s, fill_char.to_string().repeat(padding))
        }
    }
    pub fn pad_center(s: &str, width: usize, fill_char: char) -> String {
        if s.len() >= width {
            s.to_string()
        } else {
            let padding = width - s.len();
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;
            format!(
                "{}{}{}",
                fill_char.to_string().repeat(left_padding),
                s,
                fill_char.to_string().repeat(right_padding)
            )
        }
    }

    // String formatting
    pub fn format(template: &str, args: &[String]) -> String {
        let mut result = template.to_string();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        result
    }

    pub fn format_named(template: &str, args: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (key, value) in args {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    // String validation
    pub fn is_empty(s: &str) -> bool {
        s.is_empty()
    }
    pub fn is_whitespace(s: &str) -> bool {
        s.chars().all(|c| c.is_whitespace())
    }
    pub fn is_alpha(s: &str) -> bool {
        s.chars().all(|c| c.is_alphabetic())
    }
    pub fn is_numeric(s: &str) -> bool {
        s.chars().all(|c| c.is_numeric())
    }
    pub fn is_alphanumeric(s: &str) -> bool {
        s.chars().all(|c| c.is_alphanumeric())
    }
    pub fn is_ascii(s: &str) -> bool {
        s.is_ascii()
    }
    pub fn is_digit(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_digit())
    }
    pub fn is_hex(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_hexdigit())
    }

    // String conversion
    pub fn to_ascii(s: &str) -> String {
        s.to_ascii_lowercase()
    }
    pub fn to_unicode(s: &str) -> Vec<u32> {
        s.chars().map(|c| c as u32).collect()
    }
    pub fn from_unicode(codes: &[u32]) -> String {
        codes
            .iter()
            .filter_map(|&code| char::from_u32(code))
            .collect()
    }

    // String encoding/decoding
    pub fn encode_base64(s: &str) -> String {
        use base64::{engine::general_purpose, Engine as _};
        general_purpose::STANDARD.encode(s.as_bytes())
    }

    pub fn decode_base64(s: &str) -> Result<String, String> {
        use base64::{engine::general_purpose, Engine as _};
        general_purpose::STANDARD
            .decode(s)
            .map_err(|e| e.to_string())
            .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
    }

    pub fn encode_url(s: &str) -> String {
        use url::form_urlencoded::byte_serialize;
        // For proper URL encoding, we need to encode spaces as %20, not +
        s.chars()
            .map(|c| match c {
                ' ' => "%20".to_string(),
                c if c.is_ascii_alphanumeric() => c.to_string(),
                c => format!("%{:02X}", c as u8),
            })
            .collect()
    }

    pub fn decode_url(s: &str) -> Result<String, String> {
        use url::form_urlencoded::parse;
        Ok(parse(s.as_bytes()).map(|(key, _)| key).collect::<String>())
    }

    // String hashing
    pub fn hash_sha1(s: &str) -> String {
        use sha1::{Digest, Sha1};
        let mut hasher = Sha1::new();
        hasher.update(s.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn hash_sha256(s: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // String similarity
    pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let s1_len = s1_chars.len();
        let s2_len = s2_chars.len();

        let mut matrix = vec![vec![0; s2_len + 1]; s1_len + 1];

        for i in 0..=s1_len {
            matrix[i][0] = i;
        }
        for j in 0..=s2_len {
            matrix[0][j] = j;
        }

        for i in 1..=s1_len {
            for j in 1..=s2_len {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                    0
                } else {
                    1
                };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[s1_len][s2_len]
    }

    pub fn jaro_similarity(s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 1.0;
        }
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let s1_len = s1_chars.len();
        let s2_len = s2_chars.len();

        let match_window = (s1_len.max(s2_len) / 2) - 1;
        let mut s1_matches = vec![false; s1_len];
        let mut s2_matches = vec![false; s2_len];

        let mut matches = 0;
        let mut transpositions = 0;

        for i in 0..s1_len {
            let start = if i >= match_window {
                i - match_window
            } else {
                0
            };
            let end = (i + match_window + 1).min(s2_len);

            for j in start..end {
                if s2_matches[j] || s1_chars[i] != s2_chars[j] {
                    continue;
                }
                s1_matches[i] = true;
                s2_matches[j] = true;
                matches += 1;
                break;
            }
        }

        if matches == 0 {
            return 0.0;
        }

        let mut k = 0;
        for i in 0..s1_len {
            if !s1_matches[i] {
                continue;
            }
            while !s2_matches[k] {
                k += 1;
            }
            if s1_chars[i] != s2_chars[k] {
                transpositions += 1;
            }
            k += 1;
        }

        let jaro = (matches as f64 / s1_len as f64
            + matches as f64 / s2_len as f64
            + (matches as f64 - transpositions as f64 / 2.0) / matches as f64)
            / 3.0;
        jaro
    }

    // String templates
    pub fn template(template: &str, variables: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (key, value) in variables {
            result = result.replace(&format!("${{{}}}", key), value);
        }
        result
    }

    // String interpolation
    pub fn interpolate(template: &str, values: &[String]) -> String {
        let mut result = template.to_string();
        for (i, value) in values.iter().enumerate() {
            result = result.replace(&format!("${}", i), value);
        }
        result
    }

    // String case conversion
    pub fn to_snake_case(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
            .trim_start_matches('_')
            .to_string()
    }

    pub fn to_camel_case(s: &str) -> String {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.is_empty() {
            return String::new();
        }

        let first_word = words[0].to_lowercase();
        let other_words: String = words[1..]
            .iter()
            .map(|word| Self::capitalize(word))
            .collect();

        format!("{}{}", first_word, other_words)
    }

    pub fn to_pascal_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| Self::capitalize(word))
            .collect::<String>()
    }

    pub fn to_kebab_case(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("-{}", c.to_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
            .trim_start_matches('-')
            .to_string()
    }

    // String statistics
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }
    pub fn line_count(s: &str) -> usize {
        s.lines().count()
    }
    pub fn char_frequency(s: &str) -> HashMap<char, usize> {
        let mut freq = HashMap::new();
        for c in s.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        freq
    }

    pub fn word_frequency(s: &str) -> HashMap<String, usize> {
        let mut freq = HashMap::new();
        for word in s.split_whitespace() {
            let word = word.to_lowercase();
            *freq.entry(word).or_insert(0) += 1;
        }
        freq
    }

    // String cleaning
    pub fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
    pub fn remove_punctuation(s: &str) -> String {
        s.chars().filter(|c| !c.is_ascii_punctuation()).collect()
    }
    pub fn remove_digits(s: &str) -> String {
        s.chars().filter(|c| !c.is_ascii_digit()).collect()
    }
    pub fn remove_alpha(s: &str) -> String {
        s.chars().filter(|c| !c.is_alphabetic()).collect()
    }

    // String normalization
    pub fn normalize_whitespace(s: &str) -> String {
        s.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    pub fn normalize_case(s: &str) -> String {
        s.to_lowercase()
    }
    pub fn normalize_unicode(s: &str) -> String {
        s.to_string()
    } // TODO: Implement Unicode normalization

    // String comparison
    pub fn equals_ignore_case(s1: &str, s2: &str) -> bool {
        s1.eq_ignore_ascii_case(s2)
    }
    pub fn compare(s1: &str, s2: &str) -> i32 {
        s1.cmp(s2) as i32
    }
    pub fn compare_ignore_case(s1: &str, s2: &str) -> i32 {
        s1.to_lowercase().cmp(&s2.to_lowercase()) as i32
    }
}
