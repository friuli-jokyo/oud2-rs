//! Serialize a Rust data structure into .oud2 data.

use std::io;

use serde::ser::{self};

use crate::error::Error;

/// A structure for serializing Rust values into .oud2 data.
pub struct Serializer<W> {
    writer: W,
    formatter: Formatter,
}

impl<W> Serializer<W>
where
    W: std::io::Write,
{
    /// Creates a new serializer.
    pub fn new(writer: W) -> Self {
        Serializer {
            writer,
            formatter: Formatter {
                directory_depth: 0,
                directory_names: Vec::new(),
            },
        }
    }
}

impl<'a, W> serde::ser::Serializer for &'a mut Serializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_bool(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_i8(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_i16(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_i32(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_i64(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_u8(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_u16(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_u32(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_u64(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_f32(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_f64(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        self.formatter
            .write_escaped_str(&mut self.writer, v)
            .map_err(Error::from)
    }

    #[inline]
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported("bytes"))
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.formatter.key_value_separator(&mut self.writer)?;
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        Err(Error::Unsupported("newtype variant"))
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Compound {
            ser: self,
            state: State::SeqStart,
        })
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::Unsupported("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::Unsupported("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::Unsupported("tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::Unsupported("map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.formatter
            .directory_begin(&mut self.writer)
            .map_err(Error::from)?;
        Ok(Compound {
            ser: self,
            state: State::Other,
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::Unsupported("struct variant"))
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + std::fmt::Display,
    {
        self.serialize_str(&value.to_string())
    }
}

/// A structure for formatting .oud2 data.
#[derive(Clone, Debug)]
pub struct Formatter {
    directory_depth: usize,
    directory_names: Vec<String>,
}

impl Formatter {
    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(if value { b"1" } else { b"0" })
    }

    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())
    }

    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())
    }

    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())
    }

    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())
    }

    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())
    }

    #[inline]
    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())
    }

    #[inline]
    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())
    }

    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = itoa::Buffer::new();
        writer.write_all(buf.format(value).as_bytes())
    }

    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = ryu::Buffer::new();
        writer.write_all(buf.format_finite(value).as_bytes())
    }

    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buf = ryu::Buffer::new();
        writer.write_all(buf.format_finite(value).as_bytes())
    }

    #[inline]
    fn write_escaped_str<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let bytes = value.as_bytes();

        for &byte in bytes.iter() {
            match byte {
                b'\\' => writer.write_all(b"\\\\")?,
                b'\n' => writer.write_all(b"\\n")?,
                _ => writer.write_all(&[byte])?,
            }
        }

        Ok(())
    }

    /// Write `.\n` at the beginning of a directory.
    #[inline]
    fn directory_begin<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.directory_depth != 0 {
            writer.write_all(b".\n")?
        }
        self.directory_depth += 1;
        Ok(())
    }

    /// Write `.` at the end of a directory.
    #[inline]
    fn directory_end<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.directory_depth -= 1;
        if self.directory_depth != 0 {
            writer.write_all(b".")?
        }
        Ok(())
    }

    /// Write `=` between a key and a value.
    #[inline]
    fn key_value_separator<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"=")
    }
}

#[derive(Clone, Debug, PartialEq)]
enum State {
    SeqStart,
    Other,
}

#[doc(hidden)]
pub struct Compound<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    state: State,
}

impl<W> ser::SerializeSeq for Compound<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if self.state == State::SeqStart {
            self.state = State::Other;
        } else {
            let last_directory_name = match self.ser.formatter.directory_names.last() {
                Some(name) => name,
                None => return Err(Error::RootTypeNotStruct),
            };
            self.ser.writer.write_all(b"\n")?;
            self.ser.writer.write_all(last_directory_name.as_bytes())?;
        }
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W> ser::SerializeStruct for Compound<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.ser.formatter.directory_names.push(key.to_string());
        self.ser.writer.write_all(key.as_bytes())?;
        value.serialize(&mut *self.ser)?;
        self.ser.formatter.directory_names.pop();
        self.ser.writer.write_all(b"\n").map_err(Error::from)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser
            .formatter
            .directory_end(&mut self.ser.writer)
            .map_err(Error::from)
    }
}

impl<W> ser::SerializeStructVariant for Compound<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeStruct::end(self)
    }
}

/// Serialize the given data structure as .oud2 into the I/O stream.
///
///  # Errors
///
/// Serialization can fail if
/// - `T`'s implementation of `Serialize` decides to fail.
/// - Root type is not a struct.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<(), Error>
where
    W: io::Write,
    T: serde::Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

/// Serialize the given data structure as .oud2 into a byte vector.
///
/// # Errors
/// Serialization can fail if
/// - `T`'s implementation of `Serialize` decides to fail.
/// - Root type is not a struct.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: serde::Serialize,
{
    let mut buf = Vec::new();
    to_writer(&mut buf, value)?;
    Ok(buf)
}

/// Serialize the given data structure as .oud2 into String.
///
/// # Errors
/// Serialization can fail if
/// - `T`'s implementation of `Serialize` decides to fail.
/// - Root type is not a struct.
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: serde::Serialize,
{
    let vec = to_vec(value)?;
    let string = unsafe { String::from_utf8_unchecked(vec) };
    Ok(string)
}
