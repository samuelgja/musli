use core::marker;

use musli::de::{
    Decode, Decoder, LengthHint, NumberHint, PairDecoder, PairsDecoder, SequenceDecoder, TypeHint,
    ValueVisitor, VariantDecoder,
};
use musli::en::{Encode, Encoder, PairsEncoder, SequenceEncoder, VariantEncoder};
use musli::error::Error;
use musli::mode::Mode;

use crate::de::ValueDecoder;

/// A dynamic value capable of representing any [Müsli] type whether it be
/// complex or simple.
///
/// [Müsli]: https://github.com/udoprog/musli
#[derive(Clone)]
#[non_exhaustive]
pub enum Value {
    /// The default unit value.
    Unit,
    /// A boolean value.
    Bool(bool),
    /// A character.
    Char(char),
    /// A number.
    Number(Number),
    /// An array.
    Bytes(Vec<u8>),
    /// A string in a value.
    String(String),
    /// A unit value.
    Sequence(Vec<Value>),
    /// A pair stored in the value.
    Map(Vec<(Value, Value)>),
    /// A variant pair. The first value identifies the variant, the second value
    /// contains the value of the variant.
    Variant(Box<(Value, Value)>),
}

impl Value {
    /// Get the type hint corresponding to the value.
    pub fn type_hint(&self) -> TypeHint {
        match self {
            Value::Unit => TypeHint::Unit,
            Value::Bool(..) => TypeHint::Bool,
            Value::Char(..) => TypeHint::Char,
            Value::Number(number) => TypeHint::Number(number.type_hint()),
            Value::Bytes(bytes) => TypeHint::Bytes(LengthHint::Exact(bytes.len())),
            Value::String(string) => TypeHint::String(LengthHint::Exact(string.len())),
            Value::Sequence(sequence) => TypeHint::Sequence(LengthHint::Exact(sequence.len())),
            Value::Map(map) => TypeHint::Map(LengthHint::Exact(map.len())),
            Value::Variant(..) => TypeHint::Variant,
        }
    }

    /// Get a decoder associated with a value.
    pub(crate) fn decoder(&self) -> ValueDecoder<'_> {
        ValueDecoder::new(self)
    }
}

#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum Number {
    /// `u8`
    U8(u8),
    /// `u16`
    U16(u16),
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `u128`
    U128(u128),
    /// `u8`
    I8(i8),
    /// `u16`
    I16(i16),
    /// `u32`
    I32(i32),
    /// `u64`
    I64(i64),
    /// `u128`
    I128(i128),
    /// `usize`
    Usize(usize),
    /// `isize`
    Isize(isize),
    /// `f32`
    F32(f32),
    /// `f64`
    F64(f64),
}

impl<M> Encode<M> for Number
where
    M: Mode,
{
    fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder,
    {
        match self {
            Number::U8(n) => encoder.encode_u8(*n),
            Number::U16(n) => encoder.encode_u16(*n),
            Number::U32(n) => encoder.encode_u32(*n),
            Number::U64(n) => encoder.encode_u64(*n),
            Number::U128(n) => encoder.encode_u128(*n),
            Number::I8(n) => encoder.encode_i8(*n),
            Number::I16(n) => encoder.encode_i16(*n),
            Number::I32(n) => encoder.encode_i32(*n),
            Number::I64(n) => encoder.encode_i64(*n),
            Number::I128(n) => encoder.encode_i128(*n),
            Number::Usize(n) => encoder.encode_usize(*n),
            Number::Isize(n) => encoder.encode_isize(*n),
            Number::F32(n) => encoder.encode_f32(*n),
            Number::F64(n) => encoder.encode_f64(*n),
        }
    }
}

impl Number {
    /// Get the type hint for the number.
    pub fn type_hint(&self) -> NumberHint {
        match self {
            Number::U8(_) => NumberHint::U8,
            Number::U16(_) => NumberHint::U16,
            Number::U32(_) => NumberHint::U32,
            Number::U64(_) => NumberHint::U64,
            Number::U128(_) => NumberHint::U128,
            Number::I8(_) => NumberHint::I8,
            Number::I16(_) => NumberHint::I16,
            Number::I32(_) => NumberHint::I32,
            Number::I64(_) => NumberHint::I64,
            Number::I128(_) => NumberHint::I128,
            Number::Usize(_) => NumberHint::Usize,
            Number::Isize(_) => NumberHint::Isize,
            Number::F32(_) => NumberHint::F32,
            Number::F64(_) => NumberHint::F64,
        }
    }
}

impl<'de, M> Decode<'de, M> for Value
where
    M: Mode,
{
    fn decode<D>(mut decoder: D) -> Result<Self, D::Error>
    where
        D: Decoder<'de>,
    {
        match decoder.type_hint()? {
            TypeHint::Unit => {
                decoder.decode_unit()?;
                Ok(Value::Unit)
            }
            TypeHint::Bool => {
                let b = decoder.decode_bool()?;
                Ok(Value::Bool(b))
            }
            TypeHint::Char => {
                let c = decoder.decode_char()?;
                Ok(Value::Char(c))
            }
            TypeHint::Number(number) => Ok(match number {
                NumberHint::U8 => Value::Number(Number::U8(decoder.decode_u8()?)),
                NumberHint::U16 => Value::Number(Number::U16(decoder.decode_u16()?)),
                NumberHint::U32 => Value::Number(Number::U32(decoder.decode_u32()?)),
                NumberHint::U64 => Value::Number(Number::U64(decoder.decode_u64()?)),
                NumberHint::U128 => Value::Number(Number::U128(decoder.decode_u128()?)),
                NumberHint::I8 => Value::Number(Number::I8(decoder.decode_i8()?)),
                NumberHint::I16 => Value::Number(Number::I16(decoder.decode_i16()?)),
                NumberHint::I32 => Value::Number(Number::I32(decoder.decode_i32()?)),
                NumberHint::I64 => Value::Number(Number::I64(decoder.decode_i64()?)),
                NumberHint::I128 => Value::Number(Number::I128(decoder.decode_i128()?)),
                NumberHint::Usize => Value::Number(Number::Usize(decoder.decode_usize()?)),
                NumberHint::Isize => Value::Number(Number::Isize(decoder.decode_isize()?)),
                NumberHint::F32 => Value::Number(Number::F32(decoder.decode_f32()?)),
                NumberHint::F64 => Value::Number(Number::F64(decoder.decode_f64()?)),
                hint => {
                    return Err(D::Error::message(format_args!(
                        "Value: unsupported type {hint}"
                    )))
                }
            }),
            TypeHint::Bytes(..) => decoder.decode_bytes(BytesVisitor(marker::PhantomData)),
            TypeHint::String(_) => decoder.decode_string(StringVisitor(marker::PhantomData)),
            TypeHint::Sequence(len) => {
                let mut out = Vec::with_capacity(len.size_hint());

                let mut seq = decoder.decode_sequence()?;

                while let Some(item) = seq.next()? {
                    out.push(Decode::<M>::decode(item)?);
                }

                Ok(Value::Sequence(out))
            }
            TypeHint::Map(len) => {
                let mut out = Vec::with_capacity(len.size_hint());

                let mut seq = decoder.decode_map()?;

                while let Some(mut item) = seq.next()? {
                    let first = Decode::<M>::decode(item.first()?)?;
                    let second = Decode::<M>::decode(item.second()?)?;
                    out.push((first, second));
                }

                Ok(Value::Map(out))
            }
            TypeHint::Variant => {
                let mut variant = decoder.decode_variant()?;
                let first = Decode::<M>::decode(variant.tag()?)?;
                let second = Decode::<M>::decode(variant.variant()?)?;
                Ok(Value::Variant(Box::new((first, second))))
            }
            hint => Err(D::Error::message(format_args!(
                "Value: unsupported type {hint}"
            ))),
        }
    }
}

struct BytesVisitor<E>(marker::PhantomData<E>);

impl<'de, E> ValueVisitor<'de> for BytesVisitor<E>
where
    E: Error,
{
    type Target = [u8];
    type Ok = Value;
    type Error = E;

    #[inline]
    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "expecting bytes")
    }

    #[inline]
    fn visit_owned(self, bytes: Vec<u8>) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bytes(bytes))
    }

    #[inline]
    fn visit_any(self, bytes: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Bytes(bytes.to_vec()))
    }
}

struct StringVisitor<E>(marker::PhantomData<E>);

impl<'de, E> ValueVisitor<'de> for StringVisitor<E>
where
    E: Error,
{
    type Target = str;
    type Ok = Value;
    type Error = E;

    #[inline]
    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "expecting bytes")
    }

    #[inline]
    fn visit_owned(self, string: String) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(string))
    }

    #[inline]
    fn visit_any(self, string: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::String(string.to_owned()))
    }
}

impl<M> Encode<M> for Value
where
    M: Mode,
{
    fn encode<E>(&self, encoder: E) -> Result<E::Ok, E::Error>
    where
        E: Encoder,
    {
        match self {
            Value::Unit => encoder.encode_unit(),
            Value::Bool(b) => encoder.encode_bool(*b),
            Value::Char(c) => encoder.encode_char(*c),
            Value::Number(n) => Encode::<M>::encode(n, encoder),
            Value::Bytes(bytes) => encoder.encode_bytes(bytes),
            Value::String(string) => encoder.encode_string(string),
            Value::Sequence(values) => {
                let mut sequence = encoder.encode_sequence(values.len())?;

                for value in values {
                    Encode::<M>::encode(value, sequence.next()?)?;
                }

                sequence.end()
            }
            Value::Map(values) => {
                let mut map = encoder.encode_map(values.len())?;

                for (first, second) in values {
                    map.insert::<M, _, _>(first, second)?;
                }

                map.end()
            }
            Value::Variant(variant) => {
                let (tag, variant) = &**variant;
                let encoder = encoder.encode_variant()?;
                encoder.insert::<M, _, _>(tag, variant)
            }
        }
    }
}
