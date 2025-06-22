use rustc_hash::FxHashMap;
use myjson::{parse, parse_bytes};
use myjson::types::JSONValue;
use myjson::types::JSONValue::{Array, False, Null, Number, Object, True};

fn assert_parse(expected: JSONValue, input: &str) {
    assert_eq!(Ok(expected.clone()), parse(input.chars()));
    assert_eq!(Ok(expected), parse_bytes(input.as_bytes()));
}

fn assert_parse_fail(input: &str) {
    let res1 = parse(input.chars());
    let res2 = parse_bytes(input.as_bytes());
    assert!(res1.is_err(), "Expected an error, found {:?}", res1.unwrap());
    assert!(res2.is_err(), "Expected an error, found {:?}", res2.unwrap());
}

#[test]
fn parse_literal() {
    assert_parse(True, "true");
    assert_parse(False, "false");
    assert_parse(Null, "null");
    assert_parse(Number { number: 0. }, "0");
    assert_parse( JSONValue::String { string: "".to_string() }, r#""""#);
}

#[test]
fn parse_empty_array() {
    assert_parse(Array { data: vec![] }, "[]");
}

#[test]
fn parse_empty_object() {
    assert_parse(Object { data: Default::default() }, "{}");
}

#[test]
fn parse_varied_array() {
    assert_parse(Array { data: vec![Number { number: 0. }, myjson::types::JSONValue::String { string: "".to_string() }, Array{ data: vec![] }, Object { data: Default::default() }, True, False, Null] }, r#"[0.0, "", [], {}, true, false, null]"#);
}

#[test]
fn parse_object() {
    assert_parse(Object {
        data: FxHashMap::from_iter([
            ("a".to_string(), Array { data: vec![] }),
            ("b".to_string(), Object{ data: Default::default() }),
            ("c".to_string(), Number { number: 0. }),
            ("d".to_string(), Array { data: vec![Object{ data: Default::default() }] }),
            ("e".to_string(), myjson::types::JSONValue::String { string: "f".to_string() })
        ])
    }, r#"{"a": [], "b": {}, "c": 0.0, "d": [{}], "e": "f"}"#)
}

#[test]
fn parse_no_trailing_commas() {
    assert_parse_fail("[,]");
    assert_parse_fail("[0,]");
    assert_parse_fail("[0,1,]");
    assert_parse_fail(r#"{,}"#);
    assert_parse_fail(r#"{"a":0,}"#);
    assert_parse_fail(r#"{"a":0,"b":0,}"#);
}

#[test]
fn parse_no_bad_separators() {
    assert_parse_fail("[1,");
    assert_parse_fail("[1");
    assert_parse_fail("[1$]");
    assert_parse_fail("[1[]]");
}

#[test]
fn parse_must_finish_object() {
    assert_parse_fail("[[1, 2], 3");
    assert_parse_fail("[[1, 2], [3]");
}

#[test]
fn parse_no_empty() {
    assert_parse_fail("");
    assert_parse_fail("_");
}

#[test]
fn parse_no_trailing_data() {
    assert_parse_fail("0 []");
    assert_parse_fail("[] 0");
    assert_parse_fail("{} 0");
}

#[test]
fn parse_no_bad_object() {
    assert_parse_fail(r#"{"a"}"#);
    assert_parse_fail(r#"{"a":}"#);
    assert_parse_fail(r#"{1: 2}"#);
    assert_parse_fail(r#"{"a": 1, b: 2}"#);
    assert_parse_fail(r#"{a: 1}"#);
    assert_parse_fail(r#"{"#);
    assert_parse_fail(r#"{"a""#);
    assert_parse_fail(r#"{"a":"#);
    assert_parse_fail(r#"{"a":true"#);
    assert_parse_fail(r#"{"a"_"#);
    assert_parse_fail(r#"{true}"#);
    assert_parse_fail(r#"{false}"#);
    assert_parse_fail(r#"{null}"#);
    assert_parse_fail(r#"{[]}"#);
    assert_parse_fail(r#"{{}}"#);
    assert_parse_fail(r#"{"a}"#);
    assert_parse_fail(r#"{"a":true)}"#);
}