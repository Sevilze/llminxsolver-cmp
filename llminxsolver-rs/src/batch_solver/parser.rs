//! Scramble syntax parser for batch solving
//!
//! Supports:
//! - Plain moves: `R U R'`
//! - Square brackets `[alg1, alg2]`: Series/multiple paths
//! - Angle brackets `<gen1, gen2>`: BFS generators
//! - Modifiers: `#1,3,5-10,15+` for selective case solving

use super::types::{BatchError, CaseModifiers, ParsedScramble, ScrambleSegment};

/// Parser for scramble syntax
pub struct ScrambleParser;

impl ScrambleParser {
    /// Parse a scramble string into structured segments
    ///
    /// # Examples
    /// - `"R U R'"` → `[Plain("R U R'")]`
    /// - `"[R U, R U R']"` → `[Series(["R U", "R U R'"])]`
    /// - `"<R U R' F'>"` → `[Generators(["R U R' F'"])]`
    /// - `"R U [R', R2] <F, F'> #1,3-5"` → Complex with modifiers
    ///
    /// # Errors
    /// Returns `BatchError::ParseError` for invalid syntax
    pub fn parse(scramble: &str) -> Result<ParsedScramble, BatchError> {
        let input = scramble.trim();

        if input.is_empty() {
            return Ok(ParsedScramble {
                segments: Vec::new(),
                modifiers: CaseModifiers::default(),
            });
        }

        // Extract modifiers (everything after #)
        let (body, modifiers) = Self::extract_modifiers(input);

        // Parse the main body into segments
        let segments = Self::parse_segments(body)?;

        Ok(ParsedScramble {
            segments,
            modifiers,
        })
    }

    /// Extract case modifiers from the input string
    /// Returns (body_without_modifiers, parsed_modifiers)
    fn extract_modifiers(input: &str) -> (&str, CaseModifiers) {
        match input.find('#') {
            Some(idx) => {
                let body = &input[..idx].trim();
                let modifier_str = &input[idx + 1..].trim();
                let modifiers = Self::parse_modifier_string(modifier_str);
                (body, modifiers)
            }
            None => (input, CaseModifiers::default()),
        }
    }

    /// Parse a modifier string like "1,3,5-10,15+"
    fn parse_modifier_string(input: &str) -> CaseModifiers {
        let mut modifiers = CaseModifiers::default();

        if input.is_empty() {
            return modifiers;
        }

        for part in input.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if part.contains('-') {
                // Range: "5-10"
                let range_parts: Vec<&str> = part.split('-').collect();
                if range_parts.len() == 2
                    && let (Ok(start), Ok(end)) = (
                        range_parts[0].trim().parse::<usize>(),
                        range_parts[1].trim().parse::<usize>(),
                    )
                    && start <= end
                    && start > 0
                {
                    modifiers.ranges.push((start, end));
                }
            } else if let Some(num_part) = part.strip_suffix('+') {
                // Start from: "15+"
                if let Ok(start) = num_part.parse::<usize>()
                    && start > 0
                {
                    modifiers.start_from = Some(start);
                }
            } else {
                // Specific case: "3"
                if let Ok(num) = part.parse::<usize>()
                    && num > 0
                {
                    modifiers.specific_cases.push(num);
                }
            }
        }

        modifiers
    }

    /// Parse the scramble body into segments
    fn parse_segments(input: &str) -> Result<Vec<ScrambleSegment>, BatchError> {
        let mut segments = Vec::new();
        let mut current = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '[' => {
                    // Save current plain segment if not empty
                    if !current.trim().is_empty() {
                        segments.push(ScrambleSegment::Plain(current.trim().to_string()));
                        current.clear();
                    }

                    // Parse series content
                    let content = Self::extract_bracketed_content(&mut chars, ']')?;
                    let series = Self::split_series_content(&content);
                    segments.push(ScrambleSegment::Series(series));
                }
                '<' => {
                    // Save current plain segment if not empty
                    if !current.trim().is_empty() {
                        segments.push(ScrambleSegment::Plain(current.trim().to_string()));
                        current.clear();
                    }

                    // Parse generators content
                    let content = Self::extract_bracketed_content(&mut chars, '>')?;
                    let generators = Self::split_series_content(&content);
                    segments.push(ScrambleSegment::Generators(generators));
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        // Don't forget the last plain segment
        if !current.trim().is_empty() {
            segments.push(ScrambleSegment::Plain(current.trim().to_string()));
        }

        Ok(segments)
    }

    /// Extract content until the closing bracket is found
    /// Handles nested brackets of the same type
    fn extract_bracketed_content<I>(
        chars: &mut std::iter::Peekable<I>,
        closing: char,
    ) -> Result<String, BatchError>
    where
        I: Iterator<Item = char>,
    {
        let mut content = String::new();
        let mut depth = 1;

        for ch in chars.by_ref() {
            if ch == closing {
                depth -= 1;
                if depth == 0 {
                    return Ok(content);
                }
                content.push(ch);
            } else if ch == '[' || ch == '<' {
                depth += 1;
                content.push(ch);
            } else {
                content.push(ch);
            }
        }

        Err(BatchError::ParseError(format!(
            "Unclosed bracket, expected '{}'",
            closing
        )))
    }

    /// Split series content by commas (respecting nested brackets)
    fn split_series_content(content: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut bracket_depth = 0;

        for ch in content.chars() {
            match ch {
                '[' | '<' => {
                    bracket_depth += 1;
                    current.push(ch);
                }
                ']' | '>' => {
                    bracket_depth -= 1;
                    current.push(ch);
                }
                ',' if bracket_depth == 0 => {
                    if !current.trim().is_empty() {
                        result.push(current.trim().to_string());
                    }
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        // Don't forget the last item
        if !current.trim().is_empty() {
            result.push(current.trim().to_string());
        }

        result
    }

    /// Parse equivalence definition string
    /// Format: `{piece1 piece2 piece3}` for equivalences
    ///         `n: piece1 piece2` for orientation groups
    #[allow(clippy::type_complexity)]
    pub fn parse_equivalences(input: &str) -> (Vec<Vec<String>>, Vec<(u8, Vec<String>)>) {
        let mut equivalences: Vec<Vec<String>> = Vec::new();
        let mut orientation_groups: Vec<(u8, Vec<String>)> = Vec::new();

        let mut current = String::new();
        let mut in_braces = false;

        for ch in input.chars() {
            match ch {
                '{' => {
                    in_braces = true;
                    current.clear();
                }
                '}' => {
                    in_braces = false;
                    let pieces: Vec<String> = current
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if !pieces.is_empty() {
                        equivalences.push(pieces);
                    }
                    current.clear();
                }
                _ if in_braces => {
                    current.push(ch);
                }
                _ => {}
            }
        }

        // Parse orientation groups
        for line in input.lines() {
            let line = line.trim();
            if let Some(colon_idx) = line.find(':') {
                let num_str = line[..colon_idx].trim();
                if let Ok(num) = num_str.parse::<u8>() {
                    let pieces: Vec<String> = line[colon_idx + 1..]
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if !pieces.is_empty() {
                        orientation_groups.push((num, pieces));
                    }
                }
            }
        }

        (equivalences, orientation_groups)
    }

    /// Parse a move string into individual moves
    pub fn parse_moves(input: &str) -> Vec<String> {
        input
            .split_whitespace()
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plain_moves() {
        let result = ScrambleParser::parse("R U R'").unwrap();
        assert_eq!(result.segments.len(), 1);
        assert!(matches!(&result.segments[0], ScrambleSegment::Plain(s) if s == "R U R'"));
    }

    #[test]
    fn test_parse_series() {
        let result = ScrambleParser::parse("[R U, R U R']").unwrap();
        assert_eq!(result.segments.len(), 1);
        assert!(matches!(&result.segments[0], ScrambleSegment::Series(v) if v.len() == 2));
    }

    #[test]
    fn test_parse_generators() {
        let result = ScrambleParser::parse("<R U R', F>").unwrap();
        assert_eq!(result.segments.len(), 1);
        assert!(matches!(&result.segments[0], ScrambleSegment::Generators(v) if v.len() == 2));
    }

    #[test]
    fn test_parse_complex() {
        let result = ScrambleParser::parse("R U [R', R2] <F, F'>").unwrap();
        assert_eq!(result.segments.len(), 3);
        assert!(matches!(&result.segments[0], ScrambleSegment::Plain(s) if s == "R U"));
        assert!(matches!(&result.segments[1], ScrambleSegment::Series(v) if v.len() == 2));
        assert!(matches!(&result.segments[2], ScrambleSegment::Generators(v) if v.len() == 2));
    }

    #[test]
    fn test_parse_modifiers_specific() {
        let result = ScrambleParser::parse("R U #1,3,5").unwrap();
        assert_eq!(result.modifiers.specific_cases, vec![1, 3, 5]);
    }

    #[test]
    fn test_parse_modifiers_range() {
        let result = ScrambleParser::parse("R U #5-10").unwrap();
        assert_eq!(result.modifiers.ranges, vec![(5, 10)]);
    }

    #[test]
    fn test_parse_modifiers_start_from() {
        let result = ScrambleParser::parse("R U #15+").unwrap();
        assert_eq!(result.modifiers.start_from, Some(15));
    }

    #[test]
    fn test_parse_modifiers_combined() {
        let result = ScrambleParser::parse("R U #1,3,5-10,15+").unwrap();
        assert_eq!(result.modifiers.specific_cases, vec![1, 3]);
        assert_eq!(result.modifiers.ranges, vec![(5, 10)]);
        assert_eq!(result.modifiers.start_from, Some(15));
    }

    #[test]
    fn test_parse_equivalences() {
        let (equivs, _) = ScrambleParser::parse_equivalences("{UC1 UC2 UC3} {UE1 UE2}");
        assert_eq!(equivs.len(), 2);
        assert_eq!(equivs[0], vec!["UC1", "UC2", "UC3"]);
        assert_eq!(equivs[1], vec!["UE1", "UE2"]);
    }

    #[test]
    fn test_parse_orientation_groups() {
        let (_, groups) = ScrambleParser::parse_equivalences("1: UC1 UC2 UC3\n2: RC1 RC5");
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].0, 1);
        assert_eq!(groups[0].1, vec!["UC1", "UC2", "UC3"]);
        assert_eq!(groups[1].0, 2);
        assert_eq!(groups[1].1, vec!["RC1", "RC5"]);
    }

    #[test]
    fn test_empty_input() {
        let result = ScrambleParser::parse("").unwrap();
        assert!(result.segments.is_empty());
    }

    #[test]
    fn test_unclosed_bracket_error() {
        let result = ScrambleParser::parse("[R U R'");
        assert!(result.is_err());
    }
}
