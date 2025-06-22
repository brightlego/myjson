use myjson::{parse, parse_bytes};
use myjson::types::{JSONValue, ParseError};

use std::str::Chars;
use myjson::types::JSONValue::{False, Null, Number, String, True};

fn assert_parse(expected: JSONValue, input: &str) {
    assert_eq!(Ok(expected), parse_bytes(input.as_bytes()));
}

fn assert_parse_fail(input: &str) {
    let res = parse_bytes(input.as_bytes());
    assert!(res.is_err(), "Expected an error, found {:?}", res.unwrap());
}
#[test]
fn num_zero() {
    assert_parse(Number{ number: 0.0 }, "0");
}

#[test]
fn num_small() {
    assert_parse(Number{ number: 1.0 }, "1");
    assert_parse(Number { number: 10.0 }, "10");
    assert_parse(Number { number: 512.0 }, "512");
}

#[test]
fn num_decimal() {
    assert_parse(Number { number: 531.321 }, "531.321");
}

#[test]
fn num_zero_decimal() {
    assert_parse(Number { number: 0.01 }, "0.01");
}

#[test]
fn num_negative() {
    assert_parse(Number { number: -3.14 }, "-3.14");
}

#[test]
fn num_pos_exponent() {
    assert_parse(Number { number: 1.14e10 }, "1.14e10");
    assert_parse(Number { number: 1.14e10 }, "1.14E10");
}

#[test]
fn num_neg_exponent() {
    assert_parse(Number { number: 1.14e-10 }, "1.14e-10");
    assert_parse(Number { number: 1.14e-10 }, "1.14E-10");
}

#[test]
fn num_plus_exponent() {
    assert_parse(Number { number: 1.14e10 }, "1.14e+10");
    assert_parse(Number { number: 1.14e10 }, "1.14E+10");
}

#[test]
fn num_exponent_too_large() {
    assert_parse(Number { number: f64::INFINITY }, "1.14e+1000");
    assert_parse(Number { number: f64::NEG_INFINITY }, "-1.14e+1000");

}

#[test]
fn num_mantissa_too_large() {
    assert_parse(Number { number:18446744073709551616.0 }, "18446744073709551616");
    assert_parse(Number { number: -18446744073709551616.0 }, "-18446744073709551616");
    assert_parse(Number { number:184467440737095516160.0 }, "184467440737095516160 ");
    assert_parse(Number { number: 1.8446744073709553 }, "1.8446744073709551616000");
    assert_parse(Number { number: 1.8446744073709553 }, "1.8446744073709551616000 ");
    assert_parse(Number { number: -1.8446744073709553 }, "-1.8446744073709551616000");

}

#[test]
fn num_exponent_too_small() {
    assert_parse(Number { number: 0.0 }, "1.14e-1000 ");
    assert_parse(Number { number: 0.0 }, "-1.14e-1000 ");
}

#[test]
fn num_exponent_far_too_large() {
    assert_parse(Number { number: f64::INFINITY }, "1.14e18446744073709551616");
    assert_parse(Number { number: f64::NEG_INFINITY }, "-1.14e18446744073709551616");
    assert_parse(Number { number: f64::INFINITY }, "1.14e184467440737095516160000");
    assert_parse(Number { number: f64::NEG_INFINITY }, "-1.14e184467440737095516160000");
}

#[test]
fn num_exponent_far_too_small() {
    assert_parse(Number { number: 0.0 }, "1.14e-18446744073709551616");
    assert_parse(Number { number: 0.0 }, "-1.14e-18446744073709551616");
    assert_parse(Number { number: 0.0 }, "1.14e-184467440737095516160000");
    assert_parse(Number { number: 0.0 }, "-1.14e-184467440737095516160000");

}

#[test]
fn num_fail_only_minus() {
    assert_parse_fail("-");
}

#[test]
fn num_fail_minus_plus() {
    assert_parse_fail("-+");
}

#[test]
fn num_fail_only_plus() {
    assert_parse_fail("+");
}

#[test]
fn num_fail_leading_plus() {
    assert_parse_fail("+1");
}

#[test]
fn num_fail_trailing_decimal_point() {
    assert_parse_fail("1.");
    assert_parse_fail("1. ");
}

#[test]
fn num_fail_leading_decimal_point() {
    assert_parse_fail(".1");
}

#[test]
fn num_fail_leading_e() {
    assert_parse_fail("e10");
}

#[test]
fn num_fail_trailing_e() {
    assert_parse_fail("10e");
    assert_parse_fail("10e ");
    assert_parse_fail("10e+");
    assert_parse_fail("10e-");
}

#[test]
fn num_fail_decimal_e() {
    assert_parse_fail("10e10.1");
}

#[test]
fn string_empty() {
    assert_parse(String { string: "".to_string()}, r#""""#);
}
#[test]
fn string_nonempty() {
    assert_parse(String { string: "1".to_string()}, r#""1""#);
    assert_parse(String { string: "123".to_string()}, r#""123""#);
    assert_parse(String { string: " abc ".to_string()}, r#"" abc ""#);
}

#[test]
fn string_unicode() {
    assert_parse(String { string: "軅".to_string()}, r#""軅""#);
    assert_parse(String { string: "쎨".to_string()}, r#""쎨""#);
    assert_parse(String { string: "🫸🏿".to_string()}, r#""🫸🏿""#);
}

#[test]
fn string_escape() {
    assert_parse(String { string: "\r".to_string()}, r#""\r""#);
    assert_parse(String { string: "\n".to_string()}, r#""\n""#);
    assert_parse(String { string: "\t".to_string()}, r#""\t""#);
    assert_parse(String { string: "/".to_string()}, r#""\/""#);
    assert_parse(String { string: "\\".to_string()}, r#""\\""#);
    assert_parse(String { string: "\u{0008}".to_string()}, r#""\b""#);
    assert_parse(String { string: "\u{000C}".to_string()}, r#""\f""#);
    assert_parse(String { string: "\"".to_string()}, r#""\"""#);
    assert_parse(String { string: "\u{0000}".to_string()}, r#""\u0000""#);
    assert_parse(String { string: "\u{ABCD}".to_string()}, r#""\uABCD""#);
    assert_parse(String { string: "\u{ABCD}".to_string()}, r#""\uabcd""#);
    assert_parse(String { string: "\u{1523}".to_string()}, r#""\u1523""#);
    assert_parse(String { string: "\u{6561}".to_string()}, r#""\u6561""#);
    assert_parse(String { string: "\u{FFFF}".to_string()}, r#""\uFFFF""#);
    assert_parse(String { string: "𝄞".to_string()}, r#""\uD834\uDD1E""#);
}

#[test]
fn string_bad_escape() {
    assert_parse_fail(r#""\a""#);
    assert_parse_fail(r#""\""#);
    assert_parse_fail(r#""\u01""#);
    assert_parse_fail(r#""\u012z""#);
    assert_parse_fail(r#""\"#);
    assert_parse_fail(r#""\u"#);
    assert_parse_fail(r#""\u_"#);
    assert_parse_fail(r#""\u0"#);
    assert_parse_fail(r#""\u0_"#);
    assert_parse_fail(r#""\u00"#);
    assert_parse_fail(r#""\u00_"#);
    assert_parse_fail(r#""\u000"#);
    assert_parse_fail(r#""\u000_"#);
    assert_parse_fail(r#""\uD834\"#);
    assert_parse_fail(r#""\uD834\u"#);
    assert_parse_fail(r#""\uD834\u_"#);
    assert_parse_fail(r#""\uD834\u0"#);
    assert_parse_fail(r#""\uD834\u0_"#);
    assert_parse_fail(r#""\uD834\u00"#);
    assert_parse_fail(r#""\uD834\u00_"#);
    assert_parse_fail(r#""\uD834\u000"#);
    assert_parse_fail(r#""\uD834\u000_"#);
    assert_parse_fail(r#""\uD834\u0000"#);
}
#[test]
fn string_bad_characters() {
    assert_parse_fail("\"\u{0000}\"");
    assert_parse_fail("\"\u{001f}\"");
    assert_parse_fail(r#""\uD834""#);
}
#[test]
fn string_bad_quotes() {
    assert_parse_fail(r#"""""#);
    assert_parse_fail(r#"""#);
    assert_parse_fail(r#""a"#);
}
#[test]
fn true_false_null() {
    assert_parse(True, "true");
    assert_parse(False, "false");
    assert_parse(Null, "null");
}

#[test]
fn bad_true_false_null() {
    assert_parse_fail("True");
    assert_parse_fail("False");
    assert_parse_fail("Null");
    assert_parse_fail("tals");
    assert_parse_fail("trls");
    assert_parse_fail("trul");
    assert_parse_fail("frue");
    assert_parse_fail("farue");
    assert_parse_fail("falrue");
    assert_parse_fail("falsrue");
    assert_parse_fail("na");
    assert_parse_fail("nua");
    assert_parse_fail("nula");
    assert_parse_fail("nul");
    assert_parse_fail("nulll");
    assert_parse_fail("Null");
}