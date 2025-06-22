use std::hint::unreachable_unchecked;
use rustc_hash::FxHashMap;
use crate::lexer::lexer;
use crate::types::{JSONValue, ParseError, Token, TokenValue};
use crate::types::ParseError::{BadState, Unknown};

enum JSONCollections {
    Object { data: FxHashMap<String, JSONValue>, curr_label: Option<String> },
    Array { data: Vec<JSONValue> }
}

impl JSONCollections {

    fn to_object(self) -> Option<(FxHashMap<String, JSONValue>, Option<String>)> {
        match self {
            JSONCollections::Object { data, curr_label} => Some((data, curr_label)),
            _ => None
        }
    }
    
    fn to_array(self) -> Option<Vec<JSONValue>>  {
        match self {
            JSONCollections::Array { data } => Some(data),
            _ => None
        }
    }
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

fn ensure_separator(tokens: &mut impl Iterator<Item=Result<Token, ParseError>>) -> Result<Token, ParseError> {
    let token = tokens.next().ok_or(Unknown)??;
    match token.value {
        TokenValue::EndArray | TokenValue::EndObject => Ok(token),
        TokenValue::ValueSeparator => {
            let token = tokens.next().ok_or(Unknown)??;
            if token.value.can_be_value_start() {
                Ok(token)
            } else {
                Err(Unknown)
            }
        }
        _ => Err(Unknown)
    }
}

fn parse_first(tokens: &mut impl Iterator<Item=Result<Token, ParseError>>) -> Result<JSONValue, ParseError> {
    let mut values: Vec<JSONCollections> = Vec::new();
    let mut token = tokens.next().ok_or(Unknown)??;
    loop {
        let mut expecting_separator = match token.value {
            TokenValue::True | TokenValue::False | TokenValue::Null | TokenValue::String(_) | TokenValue::Number(_) | TokenValue::EndArray | TokenValue::EndObject => true,
            _ => false
        };
        
        match (values.last_mut(), token.value) {
            (None, TokenValue::True) => return Ok(JSONValue::True),
            (None, TokenValue::False) => return Ok(JSONValue::False),
            (None, TokenValue::Null) => return Ok(JSONValue::Null),
            (None, TokenValue::String(s)) => return Ok(JSONValue::String { string: s }),
            (None, TokenValue::Number(n)) => return Ok(JSONValue::Number { number: n }),
            (None, TokenValue::BeginObject) => values.push(JSONCollections::Object{data: Default::default(), curr_label: None}),
            (None, TokenValue::BeginArray) => values.push(JSONCollections::Array {data: vec![]}),
            (Some(JSONCollections::Object {data, curr_label}), TokenValue::String(s)) => {
                if let Some(label) = curr_label.take() {
                    data.insert(label, JSONValue::String { string: s });
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
            (Some(JSONCollections::Array { data }), TokenValue::String(s)) => data.push(JSONValue::String { string: s }),
            (Some(collection), TokenValue::Number(n)) => collection.add_value(JSONValue::Number { number: n })?,
            (Some(_), TokenValue::BeginArray) => values.push(JSONCollections::Array {data: vec![]}),
            (Some(_), TokenValue::BeginObject) => values.push(JSONCollections::Object {data: Default::default(), curr_label: None}),
            (Some(JSONCollections::Array {..}), TokenValue::EndArray) => {
                let data = values.pop().unwrap().to_array().unwrap();
                if let Some(collection) = values.last_mut() {
                    collection.add_value(JSONValue::Array { data })?;
                } else {
                    return Ok(JSONValue::Array { data });
                };
            }
            (Some(JSONCollections::Object {..}), TokenValue::EndObject) => {
                let (data, curr_label) = values.pop().unwrap().to_object().unwrap();
               
                if curr_label.is_some() {
                    return Err(Unknown);
                }
                
                if let Some(collection) = values.last_mut() {
                    collection.add_value(JSONValue::Object { data })?;
                } else {
                    return Ok(JSONValue::Object { data });
                };
            }
            (_, _) => return Err(Unknown)
        };
        
        if expecting_separator {
            token = ensure_separator(tokens)?;
        } else {
            token = tokens.next().ok_or(Unknown)??;
        }
    };
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
    use super::*;

    #[test]
    fn test_to_object_array() {
        assert!((JSONCollections::Object { data: FxHashMap::default(), curr_label: None}).to_object().is_some());
        assert!((JSONCollections::Object { data: FxHashMap::default(), curr_label: None}).to_array().is_none());
        assert!((JSONCollections::Array { data: vec![]}).to_object().is_none());
        assert!((JSONCollections::Array { data: vec![]}).to_array().is_some());
    }
}