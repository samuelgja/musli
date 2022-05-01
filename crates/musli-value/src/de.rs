use core::fmt;
use core::marker;
use core::slice;

use musli::de::{
    AsDecoder, Decoder, NumberHint, PackDecoder, PairDecoder, PairsDecoder, SequenceDecoder,
    TypeHint, ValueVisitor, VariantDecoder,
};
use musli::error::Error;
use musli::mode::Mode;

use crate::error::ValueError;
use crate::value::{Number, Value};

/// Encoder for a single value.
pub struct ValueDecoder<'a, E = ValueError> {
    value: &'a Value,
    _marker: marker::PhantomData<E>,
}

impl<'a, E> ValueDecoder<'a, E> {
    #[inline]
    pub(crate) const fn new(value: &'a Value) -> Self {
        Self {
            value,
            _marker: marker::PhantomData,
        }
    }
}

macro_rules! ensure {
    ($self:expr, $hint:ident, $ident:ident $tt:tt, $pat:pat => $block:expr) => {
        match $self.value {
            $pat => $block,
            value => {
                let $hint = value.type_hint();
                return Err(E::from(ValueError::$ident $tt));
            }
        }
    };
}

impl<'de, E> Decoder<'de> for ValueDecoder<'de, E>
where
    E: Error + From<ValueError>,
{
    type Error = E;
    type Buffer = Self;
    type Some = Self;
    type Pack = IterValueDecoder<'de, E>;
    type Sequence = IterValueDecoder<'de, E>;
    type Tuple = IterValueDecoder<'de, E>;
    type Map = IterValuePairsDecoder<'de, E>;
    type Struct = IterValuePairsDecoder<'de, E>;
    type Variant = IterValueVariantDecoder<'de, E>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cannot be decoded from value")
    }

    #[inline]
    fn type_hint(&mut self) -> Result<TypeHint, Self::Error> {
        Ok(self.value.type_hint())
    }

    #[inline]
    fn decode_buffer<M>(self) -> Result<Self::Buffer, Self::Error>
    where
        M: Mode,
    {
        Ok(self)
    }

    #[inline]
    fn decode_unit(self) -> Result<(), Self::Error> {
        ensure!(self, hint, ExpectedUnit(hint), Value::Unit => Ok(()))
    }

    #[inline]
    fn decode_bool(self) -> Result<bool, Self::Error> {
        ensure!(self, hint, ExpectedBool(hint), Value::Bool(b) => Ok(*b))
    }

    #[inline]
    fn decode_char(self) -> Result<char, Self::Error> {
        ensure!(self, hint, ExpectedChar(hint), Value::Char(c) => Ok(*c))
    }

    #[inline]
    fn decode_u8(self) -> Result<u8, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::U8, hint), Value::Number(Number::U8(n)) => Ok(*n))
    }

    #[inline]
    fn decode_u16(self) -> Result<u16, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::U16, hint), Value::Number(Number::U16(n)) => Ok(*n))
    }

    #[inline]
    fn decode_u32(self) -> Result<u32, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::U32, hint), Value::Number(Number::U32(n)) => Ok(*n))
    }

    #[inline]
    fn decode_u64(self) -> Result<u64, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::U64, hint), Value::Number(Number::U64(n)) => Ok(*n))
    }

    #[inline]
    fn decode_u128(self) -> Result<u128, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::U128, hint), Value::Number(Number::U128(n)) => Ok(*n))
    }

    #[inline]
    fn decode_i8(self) -> Result<i8, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::I8, hint), Value::Number(Number::I8(n)) => Ok(*n))
    }

    #[inline]
    fn decode_i16(self) -> Result<i16, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::I16, hint), Value::Number(Number::I16(n)) => Ok(*n))
    }

    #[inline]
    fn decode_i32(self) -> Result<i32, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::I32, hint), Value::Number(Number::I32(n)) => Ok(*n))
    }

    #[inline]
    fn decode_i64(self) -> Result<i64, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::I64, hint), Value::Number(Number::I64(n)) => Ok(*n))
    }

    #[inline]
    fn decode_i128(self) -> Result<i128, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::I128, hint), Value::Number(Number::I128(n)) => Ok(*n))
    }

    #[inline]
    fn decode_usize(self) -> Result<usize, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::Usize, hint), Value::Number(Number::Usize(n)) => Ok(*n))
    }

    #[inline]
    fn decode_isize(self) -> Result<isize, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::Isize, hint), Value::Number(Number::Isize(n)) => Ok(*n))
    }

    #[inline]
    fn decode_f32(self) -> Result<f32, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::F32, hint), Value::Number(Number::F32(n)) => Ok(*n))
    }

    #[inline]
    fn decode_f64(self) -> Result<f64, Self::Error> {
        ensure!(self, hint, ExpectedNumber(NumberHint::F64, hint), Value::Number(Number::F64(n)) => Ok(*n))
    }

    #[inline]
    fn decode_array<const N: usize>(self) -> Result<[u8; N], Self::Error> {
        ensure!(self, hint, ExpectedBytes(hint), Value::Bytes(bytes) => {
            <[u8; N]>::try_from(bytes.as_slice()).map_err(|_| E::from(ValueError::ArrayOutOfBounds))
        })
    }

    #[inline]
    fn decode_bytes<V>(self, visitor: V) -> Result<V::Ok, V::Error>
    where
        V: ValueVisitor<'de, Target = [u8], Error = Self::Error>,
    {
        ensure!(self, hint, ExpectedBytes(hint), Value::Bytes(bytes) => {
            visitor.visit_borrowed(bytes)
        })
    }

    #[inline]
    fn decode_string<V>(self, visitor: V) -> Result<V::Ok, V::Error>
    where
        V: ValueVisitor<'de, Target = str, Error = Self::Error>,
    {
        ensure!(self, hint, ExpectedString(hint), Value::String(string) => {
            visitor.visit_borrowed(string)
        })
    }

    #[inline]
    fn decode_option(self) -> Result<Option<Self::Some>, Self::Error> {
        match self.value {
            Value::Unit => Ok(None),
            value => Ok(Some(ValueDecoder::new(value))),
        }
    }

    #[inline]
    fn decode_pack(self) -> Result<Self::Pack, Self::Error> {
        ensure!(self, hint, ExpectedSequence(hint), Value::Sequence(pack) => {
            Ok(IterValueDecoder::new(pack))
        })
    }

    #[inline]
    fn decode_sequence(self) -> Result<Self::Sequence, Self::Error> {
        ensure!(self, hint, ExpectedSequence(hint), Value::Sequence(sequence) => {
            Ok(IterValueDecoder::new(sequence))
        })
    }

    #[inline]
    fn decode_tuple(self, _: usize) -> Result<Self::Tuple, Self::Error> {
        ensure!(self, hint, ExpectedSequence(hint), Value::Sequence(sequence) => {
            Ok(IterValueDecoder::new(sequence))
        })
    }

    #[inline]
    fn decode_map(self) -> Result<Self::Map, Self::Error> {
        ensure!(self, hint, ExpectedMap(hint), Value::Map(map) => {
            Ok(IterValuePairsDecoder::new(map))
        })
    }

    #[inline]
    fn decode_struct(self, _: usize) -> Result<Self::Struct, Self::Error> {
        ensure!(self, hint, ExpectedMap(hint), Value::Map(st) => {
            Ok(IterValuePairsDecoder::new(st))
        })
    }

    #[inline]
    fn decode_variant(self) -> Result<Self::Variant, Self::Error> {
        ensure!(self, hint, ExpectedVariant(hint), Value::Variant(st) => {
            Ok(IterValueVariantDecoder::new(st))
        })
    }
}

impl<'a, E> AsDecoder for ValueDecoder<'a, E>
where
    E: Error + From<ValueError>,
{
    type Error = E;
    type Decoder<'this> = ValueDecoder<'this, E> where Self: 'this;

    #[inline]
    fn as_decoder(&self) -> Result<Self::Decoder<'_>, Self::Error> {
        Ok(ValueDecoder::new(self.value))
    }
}

/// A decoder over a simple value iterator.
pub struct IterValueDecoder<'de, E> {
    iter: slice::Iter<'de, Value>,
    _marker: marker::PhantomData<E>,
}

impl<'de, E> IterValueDecoder<'de, E> {
    #[inline]
    fn new(values: &'de [Value]) -> Self {
        Self {
            iter: values.iter(),
            _marker: marker::PhantomData,
        }
    }
}

impl<'de, E> PackDecoder<'de> for IterValueDecoder<'de, E>
where
    E: Error + From<ValueError>,
{
    type Error = E;

    type Decoder<'this> = ValueDecoder<'de, E>
    where
        Self: 'this;

    #[inline]
    fn next(&mut self) -> Result<Self::Decoder<'_>, Self::Error> {
        match self.iter.next() {
            Some(value) => Ok(ValueDecoder::new(value)),
            None => Err(E::from(ValueError::ExpectedPackValue)),
        }
    }
}

impl<'de, E> SequenceDecoder<'de> for IterValueDecoder<'de, E>
where
    E: Error + From<ValueError>,
{
    type Error = E;

    type Decoder<'this> = ValueDecoder<'de, E>
    where
        Self: 'this;

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.iter.size_hint().1
    }

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Decoder<'_>>, Self::Error> {
        match self.iter.next() {
            Some(value) => Ok(Some(ValueDecoder::new(value))),
            None => Ok(None),
        }
    }
}

/// A decoder over a simple value pair iterator.
pub struct IterValuePairsDecoder<'de, E> {
    iter: slice::Iter<'de, (Value, Value)>,
    _marker: marker::PhantomData<E>,
}

impl<'de, E> IterValuePairsDecoder<'de, E> {
    #[inline]
    fn new(values: &'de [(Value, Value)]) -> Self {
        Self {
            iter: values.iter(),
            _marker: marker::PhantomData,
        }
    }
}

impl<'de, E> PairsDecoder<'de> for IterValuePairsDecoder<'de, E>
where
    E: Error + From<ValueError>,
{
    type Error = E;

    type Decoder<'this> = IterValuePairDecoder<'de, E>
    where
        Self: 'this;

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.iter.size_hint().1
    }

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Decoder<'_>>, Self::Error> {
        Ok(self.iter.next().map(IterValuePairDecoder::new))
    }
}

/// A decoder over a simple value pair iterator.
pub struct IterValuePairDecoder<'de, E> {
    pair: &'de (Value, Value),
    _marker: marker::PhantomData<E>,
}

impl<'de, E> IterValuePairDecoder<'de, E> {
    #[inline]
    const fn new(pair: &'de (Value, Value)) -> Self {
        Self {
            pair,
            _marker: marker::PhantomData,
        }
    }
}

impl<'de, E> PairDecoder<'de> for IterValuePairDecoder<'de, E>
where
    E: Error + From<ValueError>,
{
    type Error = E;

    type First<'this> = ValueDecoder<'de, E>
    where
        Self: 'this;

    type Second = ValueDecoder<'de, E>;

    #[inline]
    fn first(&mut self) -> Result<Self::First<'_>, Self::Error> {
        Ok(ValueDecoder::new(&self.pair.0))
    }

    #[inline]
    fn second(self) -> Result<Self::Second, Self::Error> {
        Ok(ValueDecoder::new(&self.pair.1))
    }

    #[inline]
    fn skip_second(self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

/// A decoder over a simple value pair as a variant.
pub struct IterValueVariantDecoder<'de, E> {
    pair: &'de (Value, Value),
    _marker: marker::PhantomData<E>,
}

impl<'de, E> IterValueVariantDecoder<'de, E> {
    #[inline]
    const fn new(pair: &'de (Value, Value)) -> Self {
        Self {
            pair,
            _marker: marker::PhantomData,
        }
    }
}

impl<'de, E> VariantDecoder<'de> for IterValueVariantDecoder<'de, E>
where
    E: Error + From<ValueError>,
{
    type Error = E;

    type Tag<'this> = ValueDecoder<'de, E>
    where
        Self: 'this;

    type Variant<'this> = ValueDecoder<'de, E>
    where
        Self: 'this;

    #[inline]
    fn tag(&mut self) -> Result<Self::Tag<'_>, Self::Error> {
        Ok(ValueDecoder::new(&self.pair.0))
    }

    #[inline]
    fn variant(&mut self) -> Result<Self::Variant<'_>, Self::Error> {
        Ok(ValueDecoder::new(&self.pair.1))
    }

    #[inline]
    fn skip_variant(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}
