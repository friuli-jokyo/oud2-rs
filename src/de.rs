//! Deserialize .oud2 data to a Rust data structure.

use std::io::Read;

use serde::de::{self, DeserializeSeed, IntoDeserializer, Visitor};

use crate::error::Error;

/// A structure that deserializes .oud2 data into Rust values.
pub struct Deserializer<'de> {
    input: &'de str,
    position: usize,
}

impl<'de> Deserializer<'de> {
    /// Create a new deserializer from a string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input, position: 0 }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.position += ch.len_utf8();
        Some(ch)
    }

    fn consume_char(&mut self, expected: char) -> Result<(), Error> {
        match self.next_char() {
            Some(ch) if ch == expected => Ok(()),
            Some(_) => Err(Error::Message(format!("Expected '{}'", expected))),
            None => Err(Error::UnexpectedEof),
        }
    }

    fn parse_key(&mut self) -> Result<&'de str, Error> {
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if ch == '=' || ch == '.' || ch == '\n' {
                break;
            }
            self.next_char();
        }
        if start == self.position {
            return Err(Error::UnexpectedEof);
        }
        Ok(&self.input[start..self.position])
    }

    fn parse_value(&mut self) -> Result<&'de str, Error> {
        self.consume_char('=')?;
        let start = self.position;
        let mut end = start;

        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            } else if ch == '\\' {
                self.next_char(); // consume backslash
                if let Some(escaped) = self.next_char() {
                    match escaped {
                        'n' | '\\' => {
                            end = self.position;
                        }
                        _ => {
                            end = self.position;
                        }
                    }
                }
            } else {
                self.next_char();
                end = self.position;
            }
        }

        Ok(&self.input[start..end])
    }

    fn unescape_value(&self, value: &str) -> String {
        let mut result = String::new();
        let mut chars = value.chars();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('\\') => result.push('\\'),
                    Some(c) => {
                        result.push('\\');
                        result.push(c);
                    }
                    None => result.push('\\'),
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("any"))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        match value_str {
            "1" | "true" => visitor.visit_bool(true),
            "0" | "false" => visitor.visit_bool(false),
            _ => Err(Error::InvalidBool),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<i8>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<i16>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<i32>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<i64>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<u8>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<u16>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<u32>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<u64>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<f32>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let value = value_str.parse::<f64>().map_err(|_| Error::InvalidNumber)?;
        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let unescaped = self.unescape_value(value_str);
        let mut chars = unescaped.chars();
        match (chars.next(), chars.next()) {
            (Some(ch), None) => visitor.visit_char(ch),
            _ => Err(Error::Message("Expected single character".to_string())),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let unescaped = self.unescape_value(value_str);
        visitor.visit_str(&unescaped)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("bytes"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("byte_buf"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // Check if we have a value after '='
        if self.peek_char() == Some('=') {
            self.next_char(); // consume '='
            if self.peek_char() == Some('\n') {
                // Empty value means None
                visitor.visit_none()
            } else {
                // Rewind to before '='
                self.position -= 1;
                visitor.visit_some(self)
            }
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.consume_char('=')?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("tuple"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("tuple_struct"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Unsupported("map"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(MapAccess::new(self))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let value_str = self.parse_value()?;
        let unescaped = self.unescape_value(value_str);
        visitor.visit_enum(unescaped.into_deserializer())
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let key = self.parse_key()?;
        visitor.visit_str(key)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct MapAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        MapAccess { de }
    }
}

impl<'de, 'a> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        // Skip newlines and whitespace
        while let Some(ch) = self.de.peek_char() {
            if ch == '\n' || ch == ' ' || ch == '\t' || ch == '\r' {
                self.de.next_char();
            } else {
                break;
            }
        }

        // Check for end of struct marker '.'
        if self.de.peek_char() == Some('.') {
            self.de.next_char();
            // Consume optional newline after '.'
            if self.de.peek_char() == Some('\n') {
                self.de.next_char();
            }
            return Ok(None);
        }

        // Check for EOF
        if self.de.is_eof() {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        // Check if this is a nested struct (key followed by '.')
        if self.de.peek_char() == Some('.') {
            self.de.next_char(); // consume '.'
            if self.de.peek_char() == Some('\n') {
                self.de.next_char(); // consume '\n'
            }
            let value = seed.deserialize(&mut *self.de)?;
            return Ok(value);
        }

        // Regular value
        let value = seed.deserialize(&mut *self.de)?;

        // Consume newline after value
        if self.de.peek_char() == Some('\n') {
            self.de.next_char();
        }

        Ok(value)
    }
}

struct SeqAccess<'a, 'de> {
    de: &'a mut Deserializer<'de>,
    key: String,
    first: bool,
}

impl<'a, 'de> SeqAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        SeqAccess {
            de,
            key: String::new(),
            first: true,
        }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.first {
            self.first = false;
            // For the first element, the key should already be parsed by the parent
            // Check if it's a nested struct
            return if self.de.peek_char() == Some('.') {
                self.de.next_char(); // consume '.'
                if self.de.peek_char() == Some('\n') {
                    self.de.next_char(); // consume '\n'
                }
                let value = seed.deserialize(&mut *self.de)?;
                Ok(Some(value))
            } else {
                // Simple value
                let value = seed.deserialize(&mut *self.de)?;
                // Consume newline
                if self.de.peek_char() == Some('\n') {
                    self.de.next_char();
                }
                Ok(Some(value))
            };
        }

        // Skip newlines and whitespace
        while let Some(ch) = self.de.peek_char() {
            if ch == '\n' || ch == ' ' || ch == '\t' || ch == '\r' {
                self.de.next_char();
            } else {
                break;
            }
        }

        // Check for end
        if self.de.is_eof() || self.de.peek_char() == Some('.') {
            return Ok(None);
        }

        // Save current position to potentially rewind
        let saved_pos = self.de.position;

        // Try to read a key
        let key = match self.de.parse_key() {
            Ok(k) => k,
            Err(_) => return Ok(None),
        };

        // If this is the first iteration, save the key
        if self.key.is_empty() {
            self.key = key.to_string();
        }

        // Check if the key matches
        if key != self.key {
            // Different key, rewind and end sequence
            self.de.position = saved_pos;
            return Ok(None);
        }

        // Check if it's a nested struct
        if self.de.peek_char() == Some('.') {
            self.de.next_char(); // consume '.'
            if self.de.peek_char() == Some('\n') {
                self.de.next_char(); // consume '\n'
            }
            let value = seed.deserialize(&mut *self.de)?;
            Ok(Some(value))
        } else {
            // Simple value
            let value = seed.deserialize(&mut *self.de)?;
            // Consume newline
            if self.de.peek_char() == Some('\n') {
                self.de.next_char();
            }
            Ok(Some(value))
        }
    }
}

/// Deserialize an instance of type `T` from a string of .oud2 data.
///
/// # Errors
///
/// Deserialization can fail if the data is not valid .oud2 format or does not match
/// the structure of `T`.
pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let result = T::deserialize(&mut deserializer)?;

    // Check for trailing characters
    while let Some(ch) = deserializer.peek_char() {
        if ch == '\n' || ch == ' ' || ch == '\t' || ch == '\r' {
            deserializer.next_char();
        } else {
            return Err(Error::TrailingCharacters);
        }
    }

    Ok(result)
}

/// Deserialize an instance of type `T` from bytes of .oud2 data.
///
/// # Errors
///
/// Deserialization can fail if the data is not valid UTF-8, not valid .oud2 format,
/// or does not match the structure of `T`.
pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let s = std::str::from_utf8(v).map_err(|e| Error::Message(format!("Invalid UTF-8: {}", e)))?;
    from_str(s)
}

/// Deserialize an instance of type `T` from an I/O stream of .oud2 data.
///
/// # Errors
///
/// Deserialization can fail if the data cannot be read, is not valid UTF-8,
/// not valid .oud2 format, or does not match the structure of `T`.
pub fn from_reader<R, T>(mut reader: R) -> Result<T, Error>
where
    R: Read,
    T: de::DeserializeOwned,
{
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    let s =
        std::str::from_utf8(&bytes).map_err(|e| Error::Message(format!("Invalid UTF-8: {}", e)))?;
    from_str(s)
}
