use std::char::DecodeUtf16Error;
use crate::types::{ParseError, Token, TokenValue};
use crate::types::ParseError::{Unknown};

struct Lexer<T: Iterator<Item=char>> {
    previous_char: Option<char>,
    chars: T
}

impl <T: Iterator<Item=char>> Lexer<T> {
    fn new(chars: T) -> Self {
        let lexer = Lexer { chars, previous_char: None };
        lexer
    }

    #[inline(always)]
    fn get_next_char(&mut self, error: ParseError) -> Result<char, ParseError> {
        self.get_next_char_option().ok_or(error)
    }

    #[inline(always)]
    fn get_next_char_option(&mut self) -> Option<char> {
        if let Some(c) = self.previous_char {
            self.previous_char = None;
            Some(c)
        }  else {
            self.chars.next()
        }
    }

    #[inline(always)]
    fn backtrack(&mut self, c: char) {
        self.previous_char = Some(c);
    }

    #[inline(always)]
    fn from_digit_int(c: char) -> i32 {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9, _ => panic!() // The panic case should never be reached
        }
    }

    #[inline(always)]
    fn from_digit_u64(c: char) -> u64 {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9, _ => panic!() // The panic case should never be reached
        }
    }

    fn parse_number(&mut self, initial: char) -> Result<f64, ParseError> {
        let (sign, initial) =
            if initial == '-' { (-1.0, self.get_next_char(Unknown)?) }
            else { (1.0, initial) };
        let mut mantissa: u64 = initial.to_digit(10).ok_or(Unknown)?.into();
        let mut offset = 0;
        let next = {
            let char = self.get_next_char_option();
            if let Some(mut char) = char {
                if mantissa == 0 {
                    char
                } else {
                    for _ in 0..16 {
                        match char {
                            '0'..='9' => {
                                mantissa *= 10;
                                mantissa += Self::from_digit_u64(char);
                            }
                            _ => {
                                break;
                            }
                        }
                        if let Some(c) = self.get_next_char_option() {
                            char = c;
                        } else {
                            return Ok(sign * (mantissa as f64));
                        }
                    }
                    loop {
                        match char {
                            '0'..='9' => {
                                offset -= 1;
                            }
                            _ => break
                        }
                        if let Some(c) = self.get_next_char_option() {
                            char = c;
                        } else {
                            return Ok(sign * (mantissa as f64) * 10.0f64.powi(-offset));
                        }
                    }
                    char
                }

            } else {
                return Ok(sign * (mantissa as f64));
            }
        };
        let next = if next == '.' {
            let mut char = self.get_next_char(Unknown)?;
            while mantissa < 2<<54 {
                match char {
                    '0'..='9' => {
                        mantissa *= 10;
                        mantissa += Self::from_digit_u64(char);
                        offset += 1;
                    }
                    _ => break
                }
                if let Some(c) = self.get_next_char_option() {
                    char = c;
                } else {
                    return Ok((sign * (mantissa as f64)) * 10.0f64.powi(-offset))
                }
            }
            loop {
                match char {
                    '0'..='9' => {}
                    _ => break,
                }
                if let Some(c) = self.get_next_char_option() {
                    char = c;
                } else {
                    return Ok((sign * (mantissa as f64)) * 10.0f64.powi(-offset))
                }
                
            }
            char
        } else {
            next
        };
        let mantissa = mantissa as f64;
        if next == 'e' || next == 'E' {
            let mut char = self.get_next_char(Unknown)?;
            let mut exponent = 0i32;
            let exponent_sign = if char == '-' {
                char = self.get_next_char(Unknown)?;
                -1
            } else if char == '+' {
                char = self.get_next_char(Unknown)?;
                1
            } else {
                1
            };
            let mut first_loop = true;
            loop {
                match char {
                    '0'..='9' => {
                        exponent += Self::from_digit_int(char);
                        exponent *= 10;
                    }
                    _ => {
                        self.backtrack(char);
                        if first_loop { return Err(Unknown) }
                        break;
                    }
                }
                // 400 will eventually generate an  anyway and this prevents integer overflow
                if exponent >= 4000 {
                    exponent = 4000;
                }
                if let Some(c) = self.get_next_char_option() {
                    char = c;
                } else {
                    break;
                }
                first_loop = false;
            }
            Ok(sign * mantissa * 10.0f64.powi(exponent_sign * exponent/10 - offset))
        } else {
            self.backtrack(next);
            Ok(sign * mantissa * 10.0f64.powi(-offset))
        }
    }

    fn parse_string(&mut self) -> Result<String, ParseError> {
        let mut string = String::new();
        while let Some(char) = self.get_next_char_option() {
            match char {
                '"' => {
                    return Ok(string)
                },
                '\\' => {
                    match self.get_next_char(Unknown)? {
                        '"' => string.push('"'),
                        '\\' => string.push('\\'),
                        '/' => string.push('/'),
                        'b' => string.push('\u{0008}'),
                        'f' => string.push('\u{000C}'),
                        'n' => string.push('\n'),
                        'r' => string.push('\r'),
                        't' => string.push('\t'),
                        'u' => {
                            let c1 = self.get_next_char(Unknown)?.to_digit(16).ok_or(Unknown)?;
                            let c2 = self.get_next_char(Unknown)?.to_digit(16).ok_or(Unknown)?;
                            let c3 = self.get_next_char(Unknown)?.to_digit(16).ok_or(Unknown)?;
                            let c4 = self.get_next_char(Unknown)?.to_digit(16).ok_or(Unknown)?;
                            let codepoint = c1 << 12 | c2 << 8 | c3 << 4 | c4;
                            if let Some(char) = char::from_u32(codepoint) {
                                string.push(char);
                            } else {
                                // We are in a utf-16 code_point
                                self.assert_next_char('\\', Unknown)?;
                                self.assert_next_char('u', Unknown)?;
                                let c1 = self.get_next_char(Unknown)?.to_digit(16).ok_or(Unknown)?;
                                let c2 = self.get_next_char(Unknown)?.to_digit(16).ok_or(Unknown)?;
                                let c3 = self.get_next_char(Unknown)?.to_digit(16).ok_or(Unknown)?;
                                let c4 = self.get_next_char(Unknown)?.to_digit(16).ok_or(Unknown)?;
                                let next_codepoint = c1 << 12 | c2 << 8 | c3 << 4 | c4;
                                let chars: Result<String, DecodeUtf16Error> = char::decode_utf16([codepoint as u16, next_codepoint as u16]).collect();
                                string.push_str(&chars.or(Err(Unknown))?)

                            }
                        }
                        _ => return Err(Unknown),
                    }
                }
                '\u{0000}'..='\u{001f}' => return Err(Unknown),
                _ => string.push(char)
            }
        };
        Err(Unknown)
    }

    fn consume_whitespace(&mut self) {
        while let Some(char) = self.get_next_char_option() {
            match char {
                ' ' | '\u{0009}' | '\u{000A}' | '\u{000D}' => continue,
                _ => { self.backtrack(char); return }
            }
        }
    }
    
    fn parse_false(&mut self) -> Result<Token, ParseError> {
        self.assert_next_char('a', Unknown)?;
        self.assert_next_char('l', Unknown)?;
        self.assert_next_char('s', Unknown)?;
        self.assert_next_char('e', Unknown)?;
        Ok(Token::new(TokenValue::False))
    }

    fn parse_true(&mut self) -> Result<Token, ParseError> {
        self.assert_next_char('r', Unknown)?;
        self.assert_next_char('u', Unknown)?;
        self.assert_next_char('e', Unknown)?;
        Ok(Token::new(TokenValue::True))
    }
    
    fn parse_null(&mut self) -> Result<Token, ParseError> {
        self.assert_next_char('u', Unknown)?;
        self.assert_next_char('l', Unknown)?;
        self.assert_next_char('l', Unknown)?;
        Ok(Token::new(TokenValue::Null))
    }

    fn assert_next_char<E>(&mut self, expected: char, on_fail: E) -> Result<(), E> {
        let actual = self.get_next_char_option();
        match actual {
            Some(actual) => if actual == expected { Ok(()) } else { Err(on_fail) },
            None => Err(on_fail)
        }
    }
}

impl <T: Iterator<Item=char>> Iterator for Lexer<T> {
    type Item = Result<Token, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_whitespace();
        let char = self.get_next_char_option()?;
        match char {
            '"' => Some(self.parse_string().map(|s| Token::new(TokenValue::String(s)))),
            '-' | '0'..='9' => Some(self.parse_number(char).map(|n| Token::new(TokenValue::Number(n)))),
            'f' => Some(self.parse_false()),
            't' => Some(self.parse_true()),
            'n' => Some(self.parse_null()),
            '{' => Some(Ok(Token::new(TokenValue::BeginObject))),
            '}' => Some(Ok(Token::new(TokenValue::EndObject))),
            '[' => Some(Ok(Token::new(TokenValue::BeginArray))),
            ']' => Some(Ok(Token::new(TokenValue::EndArray))),
            ':' => Some(Ok(Token::new(TokenValue::NameSeparator))),
            ',' => Some(Ok(Token::new(TokenValue::ValueSeparator))),
            _ => Some(Err(Unknown))
        }

    }
}

pub fn lexer(chars: impl Iterator<Item=char>) -> impl Iterator<Item=Result<Token, ParseError>> {
    Lexer::new(chars)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TokenValue::*;

    fn get_tokens(string: &str) -> Result<Vec<TokenValue>, ParseError> {
        lexer(string.chars())
            .map(|val| val.map(|val| val.value))
            .collect()
    }

    fn test_lex_tokens(expected: impl IntoIterator<Item=TokenValue>, input_str: &str) {
        assert_eq!(Ok(expected.into_iter().collect()), get_tokens(input_str))
    }

    fn test_lex_fail(input_str: &str) {
        let res = get_tokens(input_str);
        assert!(res.is_err(), "expected an error, found {:?}", res.unwrap())
    }

    #[test]
    fn num_zero() {
        test_lex_tokens([Number(0.0)], "0");
    }

    #[test]
    fn num_small() {
        test_lex_tokens([Number(1.0)], "1");
        test_lex_tokens([Number(10.0)], "10");
        test_lex_tokens([Number(512.0)], "512");
    }

    #[test]
    fn num_decimal() {
        test_lex_tokens([Number(531.321)], "531.321");
    }

    #[test]
    fn num_zero_decimal() {
        test_lex_tokens([Number(0.01)], "0.01");
    }

    #[test]
    fn num_negative() {
        test_lex_tokens([Number(-3.14)], "-3.14");
    }

    #[test]
    fn num_pos_exponent() {
        test_lex_tokens([Number(1.14e10)], "1.14e10");
        test_lex_tokens([Number(1.14e10)], "1.14E10");
    }

    #[test]
    fn num_neg_exponent() {
        test_lex_tokens([Number(1.14e-10)], "1.14e-10");
        test_lex_tokens([Number(1.14e-10)], "1.14E-10");
    }

    #[test]
    fn num_plus_exponent() {
        test_lex_tokens([Number(1.14e10)], "1.14e+10");
        test_lex_tokens([Number(1.14e10)], "1.14E+10");
    }

    #[test]
    fn num_exponent_too_large() {
        test_lex_tokens([Number(f64::INFINITY)], "1.14e+1000");
        test_lex_tokens([Number(f64::NEG_INFINITY)], "-1.14e+1000");

    }

    #[test]
    fn num_mantissa_too_large() {
        test_lex_tokens([Number(18446744073709551616.0)], "18446744073709551616");
        test_lex_tokens([Number(-18446744073709551616.0)], "-18446744073709551616");
        test_lex_tokens([Number(1.8446744073709553)], "1.8446744073709551616000");
        test_lex_tokens([Number(-1.8446744073709553)], "-1.8446744073709551616000");

    }

    #[test]
    fn num_exponent_too_small() {
        test_lex_tokens([Number(0.0)], "1.14e-1000");
        test_lex_tokens([Number(0.0)], "-1.14e-1000");
    }

    #[test]
    fn num_exponent_far_too_large() {
        test_lex_tokens([Number(f64::INFINITY)], "1.14e18446744073709551616");
        test_lex_tokens([Number(f64::NEG_INFINITY)], "-1.14e18446744073709551616");
        test_lex_tokens([Number(f64::INFINITY)], "1.14e184467440737095516160000");
        test_lex_tokens([Number(f64::NEG_INFINITY)], "-1.14e184467440737095516160000");
    }

    #[test]
    fn num_exponent_far_too_small() {
        test_lex_tokens([Number(0.0)], "1.14e-18446744073709551616");
        test_lex_tokens([Number(0.0)], "-1.14e-18446744073709551616");
        test_lex_tokens([Number(0.0)], "1.14e-184467440737095516160000");
        test_lex_tokens([Number(0.0)], "-1.14e-184467440737095516160000");

    }

    #[test]
    fn num_fail_only_minus() {
        test_lex_fail("-");
    }

    #[test]
    fn num_fail_only_plus() {
        test_lex_fail("+");
    }

    #[test]
    fn num_fail_leading_plus() {
        test_lex_fail("+1");
    }

    #[test]
    fn num_fail_trailing_decimal_point() {
        test_lex_fail("1.");
    }

    #[test]
    fn num_fail_leading_decimal_point() {
        test_lex_fail(".1");
    }

    #[test]
    fn num_fail_leading_e() {
        test_lex_fail("e10");
    }

    #[test]
    fn num_fail_trailing_e() {
        test_lex_fail("10e");
        test_lex_fail("10e+");
        test_lex_fail("10e-");
    }

    #[test]
    fn num_fail_decimal_e() {
        test_lex_fail("10e10.1");
    }

    #[test]
    fn string_empty() {
        test_lex_tokens([String("".to_string())], r#""""#);
    }
    #[test]
    fn string_nonempty() {
        test_lex_tokens([String("1".to_string())], r#""1""#);
        test_lex_tokens([String("123".to_string())], r#""123""#);
        test_lex_tokens([String(" abc ".to_string())], r#"" abc ""#);
    }

    #[test]
    fn string_unicode() {
        test_lex_tokens([String("軅".to_string())], r#""軅""#);
        test_lex_tokens([String("쎨".to_string())], r#""쎨""#);
        test_lex_tokens([String("🫸🏿".to_string())], r#""🫸🏿""#);
    }

    #[test]
    fn string_escape() {
        test_lex_tokens([String("\r".to_string())], r#""\r""#);
        test_lex_tokens([String("\n".to_string())], r#""\n""#);
        test_lex_tokens([String("\t".to_string())], r#""\t""#);
        test_lex_tokens([String("/".to_string())], r#""\/""#);
        test_lex_tokens([String("\\".to_string())], r#""\\""#);
        test_lex_tokens([String("\u{0008}".to_string())], r#""\b""#);
        test_lex_tokens([String("\u{000C}".to_string())], r#""\f""#);
        test_lex_tokens([String("\"".to_string())], r#""\"""#);
        test_lex_tokens([String("\u{0000}".to_string())], r#""\u0000""#);
        test_lex_tokens([String("\u{ABCD}".to_string())], r#""\uABCD""#);
        test_lex_tokens([String("\u{1523}".to_string())], r#""\u1523""#);
        test_lex_tokens([String("\u{6561}".to_string())], r#""\u6561""#);
        test_lex_tokens([String("\u{FFFF}".to_string())], r#""\uFFFF""#);
        test_lex_tokens([String("𝄞".to_string())], r#""\uD834\uDD1E""#);
    }

    #[test]
    fn string_bad_escape() {
        test_lex_fail(r#""\a""#);
        test_lex_fail(r#""\""#);
        test_lex_fail(r#""\u01""#);
        test_lex_fail(r#""\u012z""#);
    }
    #[test]
    fn string_bad_characters() {
        test_lex_fail("\"\u{0000}\"");
        test_lex_fail("\"\u{001f}\"");
        test_lex_fail(r#""\uD834""#);
    }
    #[test]
    fn string_bad_quotes() {
        test_lex_fail(r#"""""#);
        test_lex_fail(r#"""#);
        test_lex_fail(r#""a"#);
    }

    #[test]
    fn brackets() {
        test_lex_tokens([BeginObject], "{");
        test_lex_tokens([EndObject], "}");
        test_lex_tokens([BeginArray], "[");
        test_lex_tokens([EndArray], "]");
    }

    #[test]
    fn separators() {
        test_lex_tokens([ValueSeparator], ",");
        test_lex_tokens([NameSeparator], ":");
    }
    
    #[test]
    fn true_false_null() {
        test_lex_tokens([True], "true");
        test_lex_tokens([False], "false");
        test_lex_tokens([Null], "null");
    }

    #[test]
    fn bad_true_false_null() {
        test_lex_fail("True");
        test_lex_fail("False");
        test_lex_fail("Null");
        test_lex_fail("talse");
        test_lex_fail("frue");
        test_lex_fail("nul");
        test_lex_fail("nulll");
        test_lex_fail("Null");
    }
    
    #[test]
    fn preceding_whitespace() {
        test_lex_tokens([BeginObject], " {");
        test_lex_tokens([BeginObject], "\n{");
        test_lex_tokens([BeginObject], "\t{");
        test_lex_tokens([BeginObject], "\r{");
        test_lex_tokens([BeginObject], "  {");
        test_lex_tokens([BeginObject], "\n {");
    }

    #[test]
    fn succeeding_whitespace() {
        test_lex_tokens([BeginObject], "{ ");
        test_lex_tokens([BeginObject], "{\n");
        test_lex_tokens([BeginObject], "{\t");
        test_lex_tokens([BeginObject], "{\r");
        test_lex_tokens([BeginObject], "{  ");
        test_lex_tokens([BeginObject], "{\n");
    }
    #[test]
    fn adjacent_num() {
        test_lex_tokens([BeginObject, Number(0.)], "{0.0e0");
        test_lex_tokens([BeginObject, Number(0.)], "{-0.0");
        test_lex_tokens([BeginObject, Number(0.)], "{0");
        test_lex_tokens([Number(40.), BeginObject], "4.0e1{");
        test_lex_tokens([Number(-3.), BeginObject], "-3.0{");
        test_lex_tokens([Number(100.), BeginObject], "1e2{");
        test_lex_tokens([Number(1.), BeginObject], "1{");
    }
    #[test]
    fn adjacent_string() {
        test_lex_tokens([BeginObject, String("".to_string())], r#"{"""#);
        test_lex_tokens([String("".to_string()), BeginObject], r#"""{"#);
    }
    #[test]
    fn internal_whitespace() {
        test_lex_tokens([BeginObject, EndObject], r#"{ }"#);
        test_lex_tokens([BeginObject, EndObject], r#"{    }"#);
        test_lex_tokens([BeginObject, EndObject], r#"{}"#);
    }
}