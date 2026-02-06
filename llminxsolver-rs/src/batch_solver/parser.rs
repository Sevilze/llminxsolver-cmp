//! Scramble syntax parser for batch solving
//!
//! Supports:
//! - Plain moves: `R U R'`
//! - Square brackets `[alg1, alg2]`: Series/multiple paths
//! - Angle brackets `<gen1, gen2>`: BFS generators
//! - Modifiers: `#1,3,5-10,15+` for selective case solving

use super::types::{BatchError, CaseModifiers, ParsedScramble, ScrambleSegment};
use crate::minx::Move;

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
                    let series = Self::split_moves(&content);
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
                    let generators = Self::split_moves(&content);
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

    fn split_moves(content: &str) -> Vec<String> {
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

        if result.len() == 1 && !content.contains(',') {
            return content
                .split_whitespace()
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        result
    }

    /// Parse a move string into Move enum values
    pub fn parse_moves(input: &str) -> Result<Vec<Move>, BatchError> {
        let move_strs: Vec<&str> = input.split_whitespace().collect();
        let mut moves = Vec::with_capacity(move_strs.len());

        for s in move_strs {
            let mv = Self::parse_single_move(s.trim())?;
            moves.push(mv);
        }

        Ok(moves)
    }

    /// Parse a single move string
    pub fn parse_single_move(input: &str) -> Result<Move, BatchError> {
        // Handle common move formats
        let input = input.trim();

        // Standard face turns
        match input {
            "R" => return Ok(Move::R),
            "R'" | "Ri" => return Ok(Move::Ri),
            "R2" => return Ok(Move::R2),
            "R2'" | "R2i" => return Ok(Move::R2i),

            "L" => return Ok(Move::L),
            "L'" | "Li" => return Ok(Move::Li),
            "L2" => return Ok(Move::L2),
            "L2'" | "L2i" => return Ok(Move::L2i),

            "U" => return Ok(Move::U),
            "U'" | "Ui" => return Ok(Move::Ui),
            "U2" => return Ok(Move::U2),
            "U2'" | "U2i" => return Ok(Move::U2i),

            "F" => return Ok(Move::F),
            "F'" | "Fi" => return Ok(Move::Fi),
            "F2" => return Ok(Move::F2),
            "F2'" | "F2i" => return Ok(Move::F2i),

            "bL" => return Ok(Move::bL),
            "bL'" | "bLi" => return Ok(Move::bLi),
            "bL2" => return Ok(Move::bL2),
            "bL2'" | "bL2i" => return Ok(Move::bL2i),

            "bR" => return Ok(Move::bR),
            "bR'" | "bRi" => return Ok(Move::bRi),
            "bR2" => return Ok(Move::bR2),
            "bR2'" | "bR2i" => return Ok(Move::bR2i),

            "D" => return Ok(Move::D),
            "D'" | "Di" => return Ok(Move::Di),
            "D2" => return Ok(Move::D2),
            "D2'" | "D2i" => return Ok(Move::D2i),

            _ => {}
        }

        Err(BatchError::InvalidMove(format!(
            "Unrecognized move: '{}'",
            input
        )))
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

    #[test]
    fn test_parse_only_whitespace() {
        let result = ScrambleParser::parse("   ").unwrap();
        assert!(result.segments.is_empty());
    }

    #[test]
    fn test_parse_nested_brackets() {
        let result = ScrambleParser::parse("[[R U]]").unwrap();
        assert_eq!(result.segments.len(), 1);
    }

    #[test]
    fn test_parse_modifier_invalid_range() {
        let result = ScrambleParser::parse("R U #10-5").unwrap();
        assert!(result.modifiers.ranges.is_empty());
    }

    #[test]
    fn test_parse_modifier_zero_case() {
        let result = ScrambleParser::parse("R U #0").unwrap();
        assert!(result.modifiers.specific_cases.is_empty());
    }

    #[test]
    fn test_parse_empty_series() {
        let result = ScrambleParser::parse("[,]").unwrap();
        assert_eq!(result.segments.len(), 1);
    }

    #[test]
    fn test_split_moves() {
        let moves = ScrambleParser::split_moves("R U R' U'");
        assert_eq!(moves, vec!["R", "U", "R'", "U'"]);
    }

    #[test]
    fn test_split_moves_empty() {
        let moves = ScrambleParser::split_moves("");
        assert!(moves.is_empty());
    }

    #[test]
    fn test_split_moves_with_extra_spaces() {
        let moves = ScrambleParser::split_moves("  R   U  ");
        assert_eq!(moves, vec!["R", "U"]);
    }

    #[test]
    fn test_unclosed_angle_bracket() {
        let result = ScrambleParser::parse("<R U");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_equivalences_empty() {
        let (equivs, groups) = ScrambleParser::parse_equivalences("");
        assert!(equivs.is_empty());
        assert!(groups.is_empty());
    }

    #[test]
    fn test_parse_equivalences_only_braces() {
        let (equivs, _) = ScrambleParser::parse_equivalences("{}");
        assert!(equivs.is_empty());
    }

    #[test]
    fn test_parse_modifiers_empty_parts() {
        let result = ScrambleParser::parse("R #1,,2").unwrap();
        assert_eq!(result.modifiers.specific_cases, vec![1, 2]);
    }

    #[test]
    fn test_multiple_series_and_generators() {
        let result = ScrambleParser::parse("[R, U] [U, D] <D, F>").unwrap();
        assert_eq!(result.segments.len(), 3);
    }

    #[test]
    fn test_series_with_nested_content() {
        let result = ScrambleParser::parse("[R [U], F]").unwrap();
        assert_eq!(result.segments.len(), 1);
        if let ScrambleSegment::Series(v) = &result.segments[0] {
            assert_eq!(v.len(), 2);
        }
    }
}
