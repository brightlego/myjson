use rustc_hash::FxHashMap;
use myjson::stringify;
use myjson::types::JSONValue;
use myjson::types::JSONValue::{Array, False, Null, Number, Object, True};

fn assert_stringify(expected: &str, input: JSONValue) {
    assert_eq!(expected, stringify(&input));
}

#[test]
fn stringify_null() {
    assert_stringify("null", Null);
}

#[test]
fn stringify_true() {
    assert_stringify("true", True);
}

#[test]
fn stringify_false() {
    assert_stringify("false", False);
}

#[test]
fn stringify_string() {
    assert_stringify(r#""""#, JSONValue::String { string: "".to_string() });
    assert_stringify(r#""abc""#, JSONValue::String { string: "abc".to_string() });
    for i in (0x0..=0x7).into_iter().chain((0xe..=0x1f).into_iter()) {
        assert_stringify(&format!("\"\\u{i:0>4x}\""), JSONValue::String { string: <char>::from_u32(i).unwrap().to_string() });
    }
    assert_stringify("\"\\u000b\"", JSONValue::String { string: "\u{000b}".to_string() });
    assert_stringify("\"\\b\"", JSONValue::String { string: "\u{0008}".to_string() });
    assert_stringify("\"\\f\"", JSONValue::String { string: "\u{000C}".to_string() });
    assert_stringify("\"\\n\"", JSONValue::String { string: "\u{000A}".to_string() });
    assert_stringify("\"\\r\"", JSONValue::String { string: "\u{000D}".to_string() });
    assert_stringify("\"\\t\"", JSONValue::String { string: "\u{0009}".to_string() });
    assert_stringify(r#""\"""#, JSONValue::String { string: "\"".to_string() });
    assert_stringify(r#""\\""#, JSONValue::String { string: "\\".to_string() });
}

#[test]
fn stringify_array() {
    assert_stringify("[]", Array { data: vec![] });
    assert_stringify("[true]", Array { data: vec![True] });
    assert_stringify("[true,true]", Array { data: vec![True, True] });
    assert_stringify("[true,true,true]", Array { data: vec![True, True, True] });
}

#[test]
fn stringify_object() {
    assert_stringify(r#"{}"#, Object { data: FxHashMap::default() });
    assert_stringify(r#"{"a":true}"#, Object { data: FxHashMap::from_iter([("a".to_string(), True)]) });
    assert_stringify(r#"{"a":true,"b":false}"#, Object { data: FxHashMap::from_iter([("a".to_string(), True), ("b".to_string(), False)]) });
}

#[test]
fn stringify_number() {
    // The tests are not thorough as there is a large amount of room for implementation details
    // to change the result while it remaining correct.
    // Thorough tests are not needed as it almost directly calls a thoroughly tested 3rd party
    // library.
    assert_stringify("0.0", Number { number: 0. });
    assert_stringify("1e100", Number { number: 1e100 });
}