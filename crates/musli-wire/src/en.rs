use core::{fmt, marker};

use crate::integer_encoding::{TypedIntegerEncoding, TypedUsizeEncoding};
use crate::tag::{Kind, Tag};
use musli::en::{Encoder, PackEncoder, PairEncoder, SequenceEncoder};
use musli::error::Error;
use musli_binary_common::fixed_bytes::FixedBytes;
use musli_binary_common::writer::Writer;
use musli_storage::en::StorageEncoder;

/// A very simple encoder.
pub struct WireEncoder<W, I, L, const P: usize>
where
    I: TypedIntegerEncoding,
    L: TypedUsizeEncoding,
{
    writer: W,
    _marker: marker::PhantomData<(I, L)>,
}

impl<W, I, L, const P: usize> WireEncoder<W, I, L, P>
where
    I: TypedIntegerEncoding,
    L: TypedUsizeEncoding,
{
    /// Construct a new fixed width message encoder.
    #[inline]
    pub(crate) fn new(writer: W) -> Self {
        Self {
            writer,
            _marker: marker::PhantomData,
        }
    }
}

pub struct WirePackEncoder<W, I, L, const P: usize>
where
    W: Writer,
    I: TypedIntegerEncoding,
    L: TypedUsizeEncoding,
{
    pack_buf: FixedBytes<P, W::Error>,
    writer: W,
    _marker: marker::PhantomData<(I, L)>,
}

impl<W, I, L, const P: usize> WirePackEncoder<W, I, L, P>
where
    W: Writer,
    I: TypedIntegerEncoding,
    L: TypedUsizeEncoding,
{
    /// Construct a new fixed width message encoder.
    #[inline]
    pub(crate) fn new(writer: W) -> Self {
        Self {
            pack_buf: FixedBytes::new(),
            writer,
            _marker: marker::PhantomData,
        }
    }
}

impl<W, I, L, const P: usize> Encoder for WireEncoder<W, I, L, P>
where
    W: Writer,
    I: TypedIntegerEncoding,
    L: TypedUsizeEncoding,
{
    type Error = W::Error;

    type Pack = WirePackEncoder<W, I, L, P>;
    type Some = Self;
    type Sequence = Self;
    type Map = Self;
    type Struct = Self;
    type Tuple = Self;
    type Variant = Self;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type supported by the wire encoder")
    }

    #[inline]
    fn encode_unit(mut self) -> Result<(), Self::Error> {
        self.writer.write_byte(Tag::new(Kind::Sequence, 0).byte())?;
        Ok(())
    }

    #[inline]
    fn encode_pack(self) -> Result<Self::Pack, Self::Error> {
        Ok(WirePackEncoder::new(self.writer))
    }

    #[inline]
    fn encode_array<const N: usize>(self, array: [u8; N]) -> Result<(), Self::Error> {
        self.encode_bytes(array.as_slice())
    }

    #[inline]
    fn encode_bytes(mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        encode_prefix::<W, L>(&mut self.writer, bytes.len())?;
        self.writer.write_bytes(bytes)?;
        Ok(())
    }

    #[inline]
    fn encode_bytes_vectored(mut self, vectors: &[&[u8]]) -> Result<(), Self::Error> {
        let len = vectors.into_iter().map(|v| v.len()).sum();

        let (tag, embedded) = Tag::with_len(Kind::Prefix, len);
        self.writer.write_byte(tag.byte())?;

        if !embedded {
            L::encode_usize(&mut self.writer, len)?;
        }

        for bytes in vectors {
            self.writer.write_bytes(bytes)?;
        }

        Ok(())
    }

    #[inline]
    fn encode_string(self, string: &str) -> Result<(), Self::Error> {
        self.encode_bytes(string.as_bytes())
    }

    #[inline]
    fn encode_usize(mut self, value: usize) -> Result<(), Self::Error> {
        L::encode_typed_usize(&mut self.writer, value)
    }

    #[inline]
    fn encode_isize(mut self, value: isize) -> Result<(), Self::Error> {
        L::encode_typed_usize(&mut self.writer, value as usize)
    }

    #[inline]
    fn encode_bool(mut self, value: bool) -> Result<(), Self::Error> {
        self.writer
            .write_byte(Tag::new(Kind::Byte, if value { 1 } else { 0 }).byte())
    }

    #[inline]
    fn encode_char(self, value: char) -> Result<(), Self::Error> {
        self.encode_u32(value as u32)
    }

    #[inline]
    fn encode_u8(mut self, value: u8) -> Result<(), Self::Error> {
        let (tag, embedded) = Tag::with_byte(Kind::Byte, value);
        self.writer.write_byte(tag.byte())?;

        if !embedded {
            self.writer.write_byte(value)?;
        }

        Ok(())
    }

    #[inline]
    fn encode_u16(mut self, value: u16) -> Result<(), Self::Error> {
        I::encode_typed_unsigned(&mut self.writer, value)
    }

    #[inline]
    fn encode_u32(mut self, value: u32) -> Result<(), Self::Error> {
        I::encode_typed_unsigned(&mut self.writer, value)
    }

    #[inline]
    fn encode_u64(mut self, value: u64) -> Result<(), Self::Error> {
        I::encode_typed_unsigned(&mut self.writer, value)
    }

    #[inline]
    fn encode_u128(mut self, value: u128) -> Result<(), Self::Error> {
        I::encode_typed_unsigned(&mut self.writer, value)
    }

    #[inline]
    fn encode_i8(self, value: i8) -> Result<(), Self::Error> {
        self.encode_u8(value as u8)
    }

    #[inline]
    fn encode_i16(mut self, value: i16) -> Result<(), Self::Error> {
        I::encode_typed_signed(&mut self.writer, value)
    }

    #[inline]
    fn encode_i32(mut self, value: i32) -> Result<(), Self::Error> {
        I::encode_typed_signed(&mut self.writer, value)
    }

    #[inline]
    fn encode_i64(mut self, value: i64) -> Result<(), Self::Error> {
        I::encode_typed_signed(&mut self.writer, value)
    }

    #[inline]
    fn encode_i128(mut self, value: i128) -> Result<(), Self::Error> {
        I::encode_typed_signed(&mut self.writer, value)
    }

    #[inline]
    fn encode_f32(self, value: f32) -> Result<(), Self::Error> {
        self.encode_u32(value.to_bits())
    }

    #[inline]
    fn encode_f64(self, value: f64) -> Result<(), Self::Error> {
        self.encode_u64(value.to_bits())
    }

    #[inline]
    fn encode_some(mut self) -> Result<Self::Some, Self::Error> {
        self.writer.write_byte(Tag::new(Kind::Sequence, 1).byte())?;
        Ok(self)
    }

    #[inline]
    fn encode_none(mut self) -> Result<(), Self::Error> {
        self.writer.write_byte(Tag::new(Kind::Sequence, 0).byte())?;
        Ok(())
    }

    #[inline]
    fn encode_sequence(mut self, len: usize) -> Result<Self::Sequence, Self::Error> {
        let (tag, embedded) = Tag::with_len(Kind::Sequence, len);
        self.writer.write_byte(tag.byte())?;

        if !embedded {
            L::encode_usize(&mut self.writer, len)?;
        }

        Ok(self)
    }

    #[inline]
    fn encode_map(mut self, len: usize) -> Result<Self::Map, Self::Error> {
        let len = len
            .checked_mul(2)
            .ok_or_else(|| Self::Error::message(Overflow))?;
        let (tag, embedded) = Tag::with_len(Kind::Sequence, len);
        self.writer.write_byte(tag.byte())?;

        if !embedded {
            L::encode_usize(&mut self.writer, len)?;
        }

        Ok(self)
    }

    #[inline]
    fn encode_struct(mut self, len: usize) -> Result<Self::Struct, Self::Error> {
        let len = len
            .checked_mul(2)
            .ok_or_else(|| Self::Error::message(Overflow))?;
        let (tag, embedded) = Tag::with_len(Kind::Sequence, len);
        self.writer.write_byte(tag.byte())?;

        if !embedded {
            L::encode_usize(&mut self.writer, len)?;
        }

        Ok(self)
    }

    #[inline]
    fn encode_tuple(mut self, len: usize) -> Result<Self::Tuple, Self::Error> {
        let len = len
            .checked_mul(2)
            .ok_or_else(|| Self::Error::message(Overflow))?;
        let (tag, embedded) = Tag::with_len(Kind::Sequence, len);
        self.writer.write_byte(tag.byte())?;

        if !embedded {
            L::encode_usize(&mut self.writer, len)?;
        }

        Ok(self)
    }

    #[inline]
    fn encode_unit_struct(mut self) -> Result<(), Self::Error> {
        self.writer.write_byte(Tag::new(Kind::Sequence, 0).byte())?;
        Ok(())
    }

    #[inline]
    fn encode_variant(mut self) -> Result<Self::Variant, Self::Error> {
        self.writer.write_byte(Tag::new(Kind::Sequence, 2).byte())?;
        Ok(self)
    }
}

impl<W, I, L, const P: usize> PackEncoder for WirePackEncoder<W, I, L, P>
where
    W: Writer,
    I: TypedIntegerEncoding,
    L: TypedUsizeEncoding,
{
    type Error = W::Error;
    type Encoder<'this> = StorageEncoder<&'this mut FixedBytes<P, W::Error>, I, L> where Self: 'this;

    #[inline]
    fn next(&mut self) -> Result<Self::Encoder<'_>, Self::Error> {
        Ok(StorageEncoder::new(&mut self.pack_buf))
    }

    #[inline]
    fn finish(mut self) -> Result<(), Self::Error> {
        encode_prefix::<W, L>(&mut self.writer, self.pack_buf.len())?;
        self.writer.write_bytes(self.pack_buf.as_bytes())?;
        Ok(())
    }
}

impl<W, I, L, const P: usize> SequenceEncoder for WireEncoder<W, I, L, P>
where
    W: Writer,
    I: TypedIntegerEncoding,
    L: TypedUsizeEncoding,
{
    type Error = W::Error;
    type Next<'this> = WireEncoder<&'this mut W, I, L, P> where Self: 'this;

    #[inline]
    fn encode_next(&mut self) -> Result<Self::Next<'_>, Self::Error> {
        Ok(WireEncoder::new(&mut self.writer))
    }

    #[inline]
    fn finish(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<W, I, L, const P: usize> PairEncoder for WireEncoder<W, I, L, P>
where
    W: Writer,
    I: TypedIntegerEncoding,
    L: TypedUsizeEncoding,
{
    type Error = W::Error;
    type First<'this> = WireEncoder<&'this mut W, I, L, P> where Self: 'this;
    type Second<'this> = WireEncoder<&'this mut W, I, L, P> where Self: 'this;

    #[inline]
    fn encode_first(&mut self) -> Result<Self::First<'_>, Self::Error> {
        Ok(WireEncoder::new(&mut self.writer))
    }

    #[inline]
    fn encode_second(&mut self) -> Result<Self::Second<'_>, Self::Error> {
        Ok(WireEncoder::new(&mut self.writer))
    }

    #[inline]
    fn finish(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct Overflow;

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "integer overflow")
    }
}

/// Encode a length prefix.
#[inline]
fn encode_prefix<W, L>(writer: &mut W, len: usize) -> Result<(), W::Error>
where
    W: Writer,
    L: TypedUsizeEncoding,
{
    let (tag, embedded) = Tag::with_len(Kind::Prefix, len);
    writer.write_byte(tag.byte())?;

    if !embedded {
        L::encode_usize(writer, len)?;
    }

    Ok(())
}
