use rustc_hash::FxHashMap;
use crate::types::{JSONValue};

fn stringify_number(number: &f64, out_string: &mut String) {
    // JSON numbers should never have NaNs or Infinities in them
    out_string.push_str(&ryu::Buffer::new().format_finite(*number));
}

fn stringify_string(string: &String, out_string: &mut String) {
    out_string.push('"');
    for char in string.chars() {
        match char {
            '\u{0000}' => out_string.push_str("\\u0000"),
            '\u{0001}' => out_string.push_str("\\u0001"),
            '\u{0002}' => out_string.push_str("\\u0002"),
            '\u{0003}' => out_string.push_str("\\u0003"),
            '\u{0004}' => out_string.push_str("\\u0004"),
            '\u{0005}' => out_string.push_str("\\u0005"),
            '\u{0006}' => out_string.push_str("\\u0006"),
            '\u{0007}' => out_string.push_str("\\u0007"),
            '\u{0008}' => out_string.push_str("\\b"),
            '\u{0009}' => out_string.push_str("\\t"),
            '\u{000a}' => out_string.push_str("\\n"),
            '\u{000b}' => out_string.push_str("\\u000b"),
            '\u{000c}' => out_string.push_str("\\f"),
            '\u{000d}' => out_string.push_str("\\r"),
            '\u{000e}' => out_string.push_str("\\u000e"),
            '\u{000f}' => out_string.push_str("\\u000f"),
            '\u{0010}' => out_string.push_str("\\u0010"),
            '\u{0011}' => out_string.push_str("\\u0011"),
            '\u{0012}' => out_string.push_str("\\u0012"),
            '\u{0013}' => out_string.push_str("\\u0013"),
            '\u{0014}' => out_string.push_str("\\u0014"),
            '\u{0015}' => out_string.push_str("\\u0015"),
            '\u{0016}' => out_string.push_str("\\u0016"),
            '\u{0017}' => out_string.push_str("\\u0017"),
            '\u{0018}' => out_string.push_str("\\u0018"),
            '\u{0019}' => out_string.push_str("\\u0019"),
            '\u{001a}' => out_string.push_str("\\u001a"),
            '\u{001b}' => out_string.push_str("\\u001b"),
            '\u{001c}' => out_string.push_str("\\u001c"),
            '\u{001d}' => out_string.push_str("\\u001d"),
            '\u{001e}' => out_string.push_str("\\u001e"),
            '\u{001f}' => out_string.push_str("\\u001f"),
            '"' => out_string.push_str("\\\""),
            '\\' => out_string.push_str("\\\\"),
            _ => out_string.push(char)
        }
    }
    out_string.push('"');
}

fn stringify_object(object: &FxHashMap<String, JSONValue>, out_string: &mut String) {
    out_string.push('{');
    let mut is_first = true;
    for (key, value) in object {
        if is_first {
            is_first = false;
        } else {
            out_string.push(',');
        }
        stringify_string(key, out_string);
        out_string.push(':');
        stringify_internal(value, out_string);
    };
    out_string.push('}');
}

fn stringify_array(array: &Vec<JSONValue>, out_string: &mut String) {
    out_string.push('[');
    let mut is_first = true;
    for value in array {
        if is_first {
            is_first = false;
        } else {
            out_string.push(',');
        }
        stringify_internal(value, out_string);
    }
    out_string.push(']');
}

fn stringify_internal(value: &JSONValue, out_string: &mut String) {
    match value {
        JSONValue::False => out_string.push_str("false"),
        JSONValue::True => out_string.push_str("true"),
        JSONValue::Null => out_string.push_str("null"),
        JSONValue::Object { data: object } => stringify_object(object, out_string),
        JSONValue::Array { data: array } => stringify_array(array, out_string),
        JSONValue::Number { number } => stringify_number(number, out_string),
        JSONValue::String { string } => stringify_string(string, out_string),
    }
}

pub fn stringify(value: &JSONValue) -> String {
    let mut string = String::new();
    stringify_internal(value, &mut string);
    string
}