use rustc_hash::FxHashMap;
use crate::types::{JSONValue, ParseError};
use crate::types::ParseError::Unknown;

struct ByteParser<'a> {
    data: &'a [u8],
    head: usize
}

impl <'a> ByteParser<'a> {
    fn new(data: &'a [u8]) -> Self {
        ByteParser {
            data,
            head: 0
        }
    }
    
    fn parse_hex(&mut self) -> Result<u8, ParseError> {
        if self.head >= self.data.len() {
            return Err(Unknown)
        };
        let byte = self.data[self.head];
        self.head += 1;
        match byte {
            0x30..=0x39 => Ok(byte - 0x30),
            0x41..=0x46 => Ok(byte - 0x41 + 10),
            0x61..=0x66 => Ok(byte - 0x61 + 10),
            _ => Err(Unknown)
        }
    }
    
    fn parse_u16(&mut self) -> Result<u16, ParseError> {
        let h1 = self.parse_hex()? as u16;
        let h2 = self.parse_hex()? as u16;
        let h3 = self.parse_hex()? as u16;
        let h4 = self.parse_hex()? as u16;
        Ok(h1 << 12 | h2 << 8 | h3 << 4 | h4)
    }
    
    fn assert_next_byte(&mut self, expected: u8) -> Result<(), ParseError> {
        if Some(expected) == self.data.get(self.head).copied() {
            self.head += 1;
            Ok(())
        } else {
            Err(Unknown)
        }
    }
    
    fn parse_string(&mut self) -> Result<String, ParseError> {
        let mut bytes = Vec::new();
        while self.head < self.data.len() {
            match self.data[self.head] {
                0x00..=0x1f => return Err(Unknown),
                0x22 => {
                    self.head += 1;
                    self.consume_whitespace();
                    return String::from_utf8(bytes).or(Err(Unknown))
                },
                0x5c => {
                    self.head += 1;
                    if self.head >= self.data.len() {
                        return Err(Unknown)
                    }
                    self.head += 1;
                    match self.data[self.head-1] {
                        0x22 => bytes.push(0x22), // \"
                        0x5c => bytes.push(0x5c), // \\
                        0x2f => bytes.push(0x2f), // \/
                        0x62 => bytes.push(0x08), // \b
                        0x66 => bytes.push(0x0c), // \f
                        0x6e => bytes.push(0x0a), // \n
                        0x72 => bytes.push(0x0d), // \r
                        0x74 => bytes.push(0x09), // \t
                        0x75 => { // \uXXXX
                            let c1 = self.parse_u16()?;
                            let encoded = match c1 {
                                0xd800..=0xdfff => {
                                    self.assert_next_byte(0x5c)?; // \
                                    self.assert_next_byte(0x75)?; // u
                                    let c2 = self.parse_u16()?;
                                    String::from_utf16(&[c1, c2]).or(Err(Unknown))?
                                }
                                _ => {
                                    String::from_utf16(&[c1]).unwrap()
                                }
                            };
                            for byte in encoded.bytes() {
                                bytes.push(byte)
                            }
                        }
                        _ => return Err(Unknown)
                    }
                }
                _ => {
                    bytes.push(self.data[self.head]);
                    self.head += 1;
                },
            }
        };
        Err(Unknown)
    }
    
    fn parse_int(&mut self) -> Result<(u64, i32), ParseError> {
        if self.head >= self.data.len() {
            return Err(Unknown)
        }
        let mut int = match self.data[self.head] {
            0x30 => {
                self.head += 1;
                return Ok((0, 0))
            },
            0x31..=0x39 => {
                self.head += 1;
                (self.data[self.head-1] - 0x30) as u64
            }
            _ => return Err(Unknown)
        };
        while self.head < self.data.len() && int < 2<<56 {
            match self.data[self.head] {
                0x30..=0x39 => {
                    int *= 10;
                    int += (self.data[self.head] - 0x30) as u64
                }
                _ => break
            }
            self.head += 1;
        };
        let mut offset = 0;
        if int >= 2<< 56 {
            while self.head < self.data.len() {
                match self.data[self.head] {
                    0x30..=0x39 => offset += 1,
                    _ => break,
                }
                self.head += 1;
            };
        }
        Ok((int, offset))
    }
    
    fn parse_frac(&mut self, mut mantissa: u64, mut offset: i32) -> Result<(u64, i32), ParseError> {
        match self.data.get(self.head).ok_or(Unknown)? {
            0x30..=0x39 => {}
            _ => return Err(Unknown)
        };
        
        while self.head < self.data.len() && mantissa < 2<<56 {
            match self.data[self.head] {
                0x30..=0x39 => {
                    mantissa *= 10;
                    mantissa += (self.data[self.head] - 0x30) as u64;
                    offset -= 1;
                }
                _ => break
            }
            self.head += 1;
        };
        
        if mantissa >= 2 << 56 {
            while self.head < self.data.len() {
                match self.data[self.head] {
                    0x30..=0x39 => {}
                    _ => break,
                }
                self.head += 1;
            };
        };
        
        Ok((mantissa, offset))
        
    }
    
    fn parse_exp(&mut self) -> Result<i32, ParseError> {
        if self.head >= self.data.len() {
            return Err(Unknown);
        }
        let sign = match self.data[self.head] {
            0x2d => {
                self.head += 1;
                -1
            }
            0x2b => {
                self.head += 1;
                1
            }
            _ => 1,
        };
        match self.data.get(self.head).ok_or(Unknown)? {
            0x30..=0x39 => {}
            _ => return Err(Unknown)
        };
        let mut exp = 0;
        
        while self.head < self.data.len() && exp < 400 {
            match self.data[self.head] {
                0x30..=0x39 => {
                    exp *= 10;
                    exp += (self.data[self.head] - 0x30) as i32;
                }
                _ => break,
            }
            self.head += 1;
        };

        if exp >= 400 {
            while self.head < self.data.len() {
                match self.data[self.head] {
                    0x30..=0x39 => {}
                    _ => break,
                }
                self.head += 1;
            };
        };
        Ok(sign * exp)
    }
    
    fn parse_number(&mut self) -> Result<f64, ParseError> {
        let sign = if self.data[self.head - 1] == 0x2d {
            -1.0
        } else {
            self.head -= 1;
            1.0
        };
        let (mut int, mut offset) = self.parse_int()?;
        
        if self.head >= self.data.len() {
            return Ok(sign * (int as f64) * 10.0f64.powi(offset));
        }
        
        if 0x2e == self.data[self.head] {
            self.head += 1;
            (int, offset) = self.parse_frac(int, offset)?;
        }

        if self.head >= self.data.len() {
            return Ok(sign * (int as f64) * 10.0f64.powi(offset));
        }
        
        match self.data[self.head] {
            0x65 | 0x45 => { // e | E
                self.head += 1;
                let exp = self.parse_exp()?;
                Ok(sign * (int as f64)* 10.0f64.powi(exp + offset))
            }
            _ => {
                Ok(sign * (int as f64) * 10.0f64.powi(offset))
            }
        }
    }
    
    fn parse_true(&mut self) -> Result<(), ParseError> {
        self.assert_next_byte(0x72)?;
        self.assert_next_byte(0x75)?;
        self.assert_next_byte(0x65)?;
        self.consume_whitespace();
        Ok(())
    }
    
    fn parse_false(&mut self) -> Result<(), ParseError> {
        self.assert_next_byte(0x61)?;
        self.assert_next_byte(0x6c)?;
        self.assert_next_byte(0x73)?;
        self.assert_next_byte(0x65)?;
        self.consume_whitespace();
        Ok(())
    }
    
    fn parse_null(&mut self) -> Result<(), ParseError> {
        self.assert_next_byte(0x75)?;
        self.assert_next_byte(0x6c)?;
        self.assert_next_byte(0x6c)?;
        self.consume_whitespace();
        Ok(())
    }
    
    fn parse_array(&mut self) -> Result<Vec<JSONValue>, ParseError> {
        let mut data = Vec::new();
        self.consume_whitespace();
        if self.data.get(self.head).copied() == Some(0x5d) {
            self.head += 1;
            self.consume_whitespace();
            return Ok(data);
        }
        while self.head < self.data.len() {
            let val = self.parse()?;
            data.push(val);
            match self.data.get(self.head).ok_or(Unknown)? {
                0x5d => {
                    self.head += 1;
                    self.consume_whitespace();
                    return Ok(data)
                }
                0x2c => {
                    self.head += 1;
                    self.consume_whitespace();
                }
                _ => {
                    return Err(Unknown)
                }
            }
        };
        Err(Unknown)
    }
    
    fn parse_object(&mut self) -> Result<FxHashMap<String, JSONValue>, ParseError> {
        let mut data = FxHashMap::default();
        self.consume_whitespace();
        if self.data.get(self.head).copied() == Some(0x7d) {
            self.head += 1;
            self.consume_whitespace();
            return Ok(data);
        }
        while self.head < self.data.len() {
            self.assert_next_byte(0x22)?; // "
            let key = self.parse_string()?;
            self.assert_next_byte(0x3a)?; // :
            self.consume_whitespace();
            let val = self.parse()?;
            data.insert(key, val);
            match self.data.get(self.head).ok_or(Unknown)? {
                0x7d => {
                    self.head += 1;
                    self.consume_whitespace();
                    return Ok(data)
                }
                0x2c => {
                    self.head += 1;
                    self.consume_whitespace();
                }
                _ => {
                    return Err(Unknown)
                }
            }
        };
        Err(Unknown)
    }
    
    fn consume_whitespace(&mut self) {
        while self.head < self.data.len() {
            match self.data[self.head] {
                0x20 | 0x09 | 0x0a | 0x0d => self.head += 1,
                _ => return
            }
        }
    }
    
    fn parse(&mut self) -> Result<JSONValue, ParseError> {
        self.consume_whitespace();
        if self.head >= self.data.len() {
            return Err(Unknown)
        }
        self.head += 1;
        let res = match self.data[self.head - 1] {
            0x5b => {
                let data = self.parse_array()?;
                Ok(JSONValue::Array { data })
            }
            0x7b => {
                let data = self.parse_object()?;
                Ok(JSONValue::Object { data })
            }
            0x66 => {
                self.parse_false()?;
                Ok(JSONValue::False)
            }
            0x6e => {
                self.parse_null()?;
                Ok(JSONValue::Null)
            }
            0x74 => {
                self.parse_true()?;
                Ok(JSONValue::True)
            }
            0x22 => {
                let string = self.parse_string()?;
                Ok(JSONValue::String { string })
            }
            0x2d | 0x30..=0x39 => {
                let number = self.parse_number()?;
                Ok(JSONValue::Number { number })
            }
            _ => Err(Unknown)
        };
        self.consume_whitespace();
        res
    }
}

pub fn parse_bytes(bytes: &[u8]) -> Result<JSONValue, ParseError> {
    let mut parser = ByteParser::new(bytes);
    parser.consume_whitespace();
    let res = parser.parse()?;
    if parser.head < parser.data.len() {
        Err(Unknown)
    } else {
        Ok(res)
    }
}
