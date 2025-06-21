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
            '\u{0008}' => out_string.push_str("\\u0008"),
            '\u{0009}' => out_string.push_str("\\u0009"),
            '\u{000a}' => out_string.push_str("\\u000a"),
            '\u{000b}' => out_string.push_str("\\u000b"),
            '\u{000c}' => out_string.push_str("\\u000c"),
            '\u{000d}' => out_string.push_str("\\u000d"),
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
#[cfg(test)]
mod tests {
    use crate::types::JSONValue::*;
    use super::*;

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
        assert_stringify(r#""""#, String { string: "".to_string() });
        assert_stringify(r#""abc""#, String { string: "abc".to_string() });
        for i in 0u32..=0x1f {
            assert_stringify(&format!("\"\\u{i:0>4x}\""), String { string: <char>::from_u32(i).unwrap().to_string() });

        }
        assert_stringify(r#""\"""#, String { string: "\"".to_string() });
        assert_stringify(r#""\\""#, String { string: "\\".to_string() });

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
}