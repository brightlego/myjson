use std::collections::HashMap;
use crate::lexer::lexer;
use crate::types::{JSONValue, ParseError, Token, TokenValue};
use crate::types::ParseError::{BadState, Unknown};

enum JSONCollections {
    Object { data: HashMap<String, JSONValue>, curr_label: Option<String> },
    Array { data: Vec<JSONValue> }
}

impl JSONCollections {
    fn add_value(&mut self, value: JSONValue) -> Result<(), ParseError> {
        match self {
            JSONCollections::Object { data, curr_label} => {
                if let Some(s) = curr_label.take() {
                    data.insert(s, value);
                    Ok(())
                } else {
                    Err(Unknown)
                }
            }
            JSONCollections::Array { data } => {
                data.push(value);
                Ok(())
            }
        }
    } 
}

fn parse_first(tokens: &mut impl Iterator<Item=Result<Token, ParseError>>) -> Result<JSONValue, ParseError> {
    let mut tokens = tokens.peekable();
    let mut values: Vec<JSONCollections> = Vec::new();
    while let Some(token) = tokens.next() {
        let token = token?;
        let mut expecting_separator = match token.value {
            TokenValue::True | TokenValue::False | TokenValue::Null | TokenValue::String(_) | TokenValue::Number(_) | TokenValue::EndArray | TokenValue::EndObject => true,
            _ => false
        };
        
        match (values.last_mut(), token.value) {
            (None, TokenValue::True) => return Ok(JSONValue::True),
            (None, TokenValue::False) => return Ok(JSONValue::False),
            (None, TokenValue::Null) => return Ok(JSONValue::Null),
            (None, TokenValue::String(s)) => return Ok(JSONValue::String(s)),
            (None, TokenValue::Number(n)) => return Ok(JSONValue::Number(n)),
            (None, TokenValue::BeginObject) => values.push(JSONCollections::Object{data: HashMap::new(), curr_label: None}),
            (None, TokenValue::BeginArray) => values.push(JSONCollections::Array {data: vec![]}),
            (Some(JSONCollections::Object {data, curr_label}), TokenValue::String(s)) => {
                if let Some(label) = curr_label.take() {
                    data.insert(label, JSONValue::String(s));
                } else {
                    *curr_label = Some(s);
                    if TokenValue::NameSeparator == tokens.next().ok_or(Unknown)??.value {
                        expecting_separator = false;
                    } else {
                        return Err(Unknown)
                    }
                }
            }
            (Some(collection), TokenValue::True) => collection.add_value(JSONValue::True)?,
            (Some(collection), TokenValue::False) => collection.add_value(JSONValue::False)?,
            (Some(collection), TokenValue::Null) => collection.add_value(JSONValue::Null)?,
            (Some(collection), TokenValue::String(s)) => collection.add_value(JSONValue::String(s))?,
            (Some(collection), TokenValue::Number(n)) => collection.add_value(JSONValue::Number(n))?,
            (Some(_), TokenValue::BeginArray) => values.push(JSONCollections::Array {data: vec![]}),
            (Some(_), TokenValue::BeginObject) => values.push(JSONCollections::Object {data: HashMap::new(), curr_label: None}),
            (Some(JSONCollections::Array {..}), TokenValue::EndArray) => {
                let JSONCollections::Array { data } = values.pop().unwrap() else { return Err(BadState) };
                if let Some(collection) = values.last_mut() {
                    collection.add_value(JSONValue::Array(data))?;
                } else {
                    return Ok(JSONValue::Array(data));
                };
            }
            (Some(JSONCollections::Object {..}), TokenValue::EndObject) => {
                let JSONCollections::Object { data, curr_label } = values.pop().unwrap() else { return Err(BadState) };
               
                if curr_label.is_some() {
                    return Err(Unknown);
                }
                
                if let Some(collection) = values.last_mut() {
                    collection.add_value(JSONValue::Object(data))?;
                } else {
                    return Ok(JSONValue::Object(data));
                };
            }
            (_, _) => return Err(Unknown)
        };
        
        if expecting_separator {
            if let Some(res) = tokens.peek() {
                if let Ok(res) = res {
                    match res.value {
                        TokenValue::ValueSeparator => {
                            tokens.next();
                            if let Some(res) = tokens.peek() {
                                if let Ok(res) = res {
                                    if res.value.can_be_value_start() {
                                        continue;
                                    }
                                }
                            }
                        }
                        TokenValue::EndObject | TokenValue::EndArray => {
                            continue;
                        }
                        _ => {}
                    }
                }
            }
            return Err(Unknown)
        }
    };
    Err(Unknown)
}

pub fn parse(chars: impl Iterator<Item=char>) -> Result<JSONValue, ParseError> {
    let mut tokens = lexer(chars);
    let res = parse_first(&mut tokens)?;
    if tokens.next() == None {
        Ok(res)
    } else {
        Err(Unknown)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::JSONValue::*;
    use super::*;
    
    fn assert_parse(expected: JSONValue, input: &str) {
        assert_eq!(Ok(expected), parse(input.chars()));
    }
    
    fn assert_parse_fail(input: &str) {
        let res = parse(input.chars());
        assert!(res.is_err(), "Expected an error, found {:?}", res.unwrap())
    }
    
    #[test]
    fn parse_literal() {
        assert_parse(True, "true");
        assert_parse(False, "false");
        assert_parse(Null, "null");
        assert_parse(Number(0.), "0");
        assert_parse(String("".to_string()), r#""""#);
    }
    
    #[test]
    fn parse_empty_array() {
        assert_parse(Array(vec![]), "[]");
    }

    #[test]
    fn parse_empty_object() {
        assert_parse(Object(HashMap::new()), "{}");
    }

    #[test]
    fn parse_varied_array() {
        assert_parse(Array(vec![Number(0.), String("".to_string()), Array(vec![]), Object(HashMap::new()), True]), r#"[0., "", [], {}, true]"#);
    }
    
    #[test]
    fn parse_object() {
        assert_parse(Object(HashMap::from([
                ("a".to_string(), Array(vec![])), 
                ("b".to_string(), Object(HashMap::new())), 
                ("c".to_string(), Number(0.)), 
                ("d".to_string(), Array(vec![Object(HashMap::new())]))
        ])), r#"{"a": [], "b": {}, "c": 0., "d": [{}]}"#)
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
    fn parse_no_trailing_data() {
        assert_parse_fail("0 []");
        assert_parse_fail("[] 0");
        assert_parse_fail("{} 0");
    }
}