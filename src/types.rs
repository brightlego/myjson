use std::collections::HashMap;


// JSON specification: https://datatracker.ietf.org/doc/html/rfc7159
#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum JSONValue {
    False,
    True,
    Null,
    Object(HashMap<String, JSONValue>),
    Array(Vec<JSONValue>),
    Number(f64),
    String(String),
}

// NaN can never be a valid JSON number when parsed
impl Eq for JSONValue {}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub(crate) enum TokenValue {
    False, // false
    True, // true
    Null, // null
    BeginArray, // [
    EndArray, // ]
    BeginObject, // {
    EndObject, // }
    NameSeparator, // :
    ValueSeparator, // ,
    Number(f64), // a number as specified in Section 6 
    String(String), // a string as specified in Section 7
}

impl TokenValue {
    pub(crate) fn can_be_value_start(&self) -> bool {
        match self {
            TokenValue::False | TokenValue::True | TokenValue::Number(_) | TokenValue::String(_) | TokenValue::Null | TokenValue::BeginObject | TokenValue::BeginArray => true,
            _ => false
        }
    }
}

// NaN can never be a valid JSON number when parsed
impl Eq for TokenValue {}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Token {
    pub(crate) value: TokenValue,
}

impl Token {
    pub(crate) fn new(value: TokenValue) -> Self {
        Token { value }
    }
}
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    Unknown,
    BadState, // The program has entered an invalid state. This should never happen
}