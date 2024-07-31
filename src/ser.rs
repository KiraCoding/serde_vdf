//! Serialize a Rust data structure into VDF data.

use crate::error::{Error, Result};
use crate::io;
use alloc::string::{String, ToString};
use serde::ser::{self, Serialize};

pub struct Serializer {
    output: String,
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output += "\"";
        self.output += v;
        self.output += "\"";
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.output += "\"\"";
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + ser::Serialize,
    {
        self.output += "{";
        variant.serialize(&mut *self)?;
        variant.serialize(&mut *self)?;
        self.output += "}";
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output += "{";
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += "{";
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.output += "{";
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += "{";
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.output += "}";
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.output += "}";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.output += "}";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.output += "}}";
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.output += "}}";
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use serde::Serialize;

//     use crate::ser::to_string;

//     #[test]
//     fn test_struct() {
//         #[derive(Serialize)]
//         struct Test {
//             int: u32,
//             seq: Vec<&'static str>,
//         }

//         let test = Test {
//             int: 1,
//             seq: vec!["a", "b"],
//         };

//         let result = to_string(&test).unwrap();
//         println!("{}", result);
//     }
// }

/// A structure for serializing Rust values into VDF.
pub struct SerializerNew<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
}

impl<W> SerializerNew<W>
where
    W: io::Write,
{
    /// Creates a new VDF serializer.
    #[inline]
    pub fn new(writer: W) -> Self {
        Self::with_formatter(writer, CompactFormatter)
    }
}

impl<'a, W> SerializerNew<W, PrettyFormatter<'a>>
where
    W: io::Write,
{
    /// Creates a new VDF pretty print serializer.
    #[inline]
    pub fn pretty(writer: W) -> Self {
        Self::with_formatter(writer, PrettyFormatter::new())
    }
}

impl<W, F> SerializerNew<W, F>
where
    W: io::Write,
    F: Formatter,
{
    /// Creates a new VDF visitor whose output will be written to the writer specified.
    #[inline]
    pub fn with_formatter(writer: W, formatter: F) -> Self {
        Self { writer, formatter }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_ineer(self) -> W {
        self.writer
    }
}

impl<'a, W, F> ser::Serializer for &'a mut SerializerNew<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq;
    type SerializeTuple;
    type SerializeTupleStruct;
    type SerializeTupleVariant;
    type SerializeMap;
    type SerializeStruct;
    type SerializeStructVariant;

    #[inline]
    fn serialize_bool(self, value: bool) -> std::result::Result<Self::Ok, Self::Error> {
        self.formatter
            .write_bool(&mut self.writer, value)
            .map_err(Error::io)
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> std::result::Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i8(&mut self.writer, value)
            .map_err(Error::io)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<()> {
        self.formatter
            .write_i16(&mut self.writer, value)
            .map_err(Error::io)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<()> {
        self.formatter
            .write_i32(&mut self.writer, value)
            .map_err(Error::io)
    }

    #[inline]
    fn serialize_i64(self, value: i64) -> Result<()> {
        self.formatter
            .write_i64(&mut self.writer, value)
            .map_err(Error::io)
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok> {
        self.formatter
            .write_i128(&mut self.writer, value)
            .map_err(Error::io)
    }

    #[inline]
    fn serialize_u8(self, value: u8) -> Result<()> {
        self.formatter
            .write_u8(&mut self.writer, value)
            .map_err(Error::io)
    }

    #[inline]
    fn serialize_u16(self, value: u16) -> Result<()> {
        self.formatter
            .write_u16(&mut self.writer, value)
            .map_err(Error::io)
    }

    #[inline]
    fn serialize_u32(self, value: u32) -> Result<()> {
        self.formatter
            .write_u32(&mut self.writer, value)
            .map_err(Error::io)
    }

    #[inline]
    fn serialize_u64(self, value: u64) -> Result<()> {
        self.formatter
            .write_u64(&mut self.writer, value)
            .map_err(Error::io)
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok> {
        self.formatter
            .write_u128(&mut self.writer, value)
            .map_err(Error::io)
    }

    fn serialize_f32(self, v: f32) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        // A char encoded as UTF-8 takes 4 bytes at most.
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> std::result::Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> std::result::Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> std::result::Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::result::Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

pub trait Formatter {
    /// Writes a `"true"` or `"false"` value to the specified writer.
    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let s = if value {
            b"\"true\"" as &[u8]
        } else {
            b"\"false\"" as &[u8]
        };
        writer.write_all(s)
    }

    /// Writes an integer value like `"-123"` to the specified writer.
    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes an integer value like `"-123"` to the specified writer.
    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes an integer value like `"-123"` to the specified writer.
    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes an integer value like `"-123"` to the specified writer.
    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes an integer value like `"-123"` to the specified writer.
    #[inline]
    fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes an integer value like `"123"` to the specified writer.
    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes an integer value like `"123"` to the specified writer.
    #[inline]
    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes an integer value like `"123"` to the specified writer.
    #[inline]
    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes an integer value like `"123"` to the specified writer.
    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes an integer value like `"123"` to the specified writer.
    #[inline]
    fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes a floating point value like `"-31.26e+12"` to the specified writer.
    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Writes a floating point value like `"-31.26e+12"` to the specified writer.
    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")?;
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(value);
        writer.write_all(s.as_bytes())?;
        writer.write_all(b"\"")
    }

    /// Called before every object. Writes a `{` to the specified writer.
    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"{")
    }

    /// Called after every object. Writes a `}` to the specified writer.
    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"}")
    }
}

#[derive(Debug, Clone)]
pub struct CompactFormatter;

impl Formatter for CompactFormatter {}

pub struct PrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
}

impl<'a> PrettyFormatter<'a> {
    pub fn new() -> Self {
        PrettyFormatter::with_indent(b"  ")
    }

    pub fn with_indent(indent: &'a [u8]) -> Self {
        Self {
            current_indent: 0,
            has_value: false,
            indent,
        }
    }
}

impl<'a> Default for PrettyFormatter<'a> {
    fn default() -> Self {
        PrettyFormatter::new()
    }
}

impl<'a> Formatter for PrettyFormatter<'a> {
    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            tri!(writer.write_all(b"\n"));
            tri!(indent(writer, self.current_indent, self.indent));
        }

        writer.write_all(b"{")
    }
}

#[inline]
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

#[inline]
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        tri!(wr.write_all(s));
    }

    Ok(())
}
