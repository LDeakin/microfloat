#![cfg(feature = "serde")]

use core::fmt;

use microfloat::{f4e2m1fn, f8e4m3};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[test]
fn serde_traits_compile() {
    fn assert_serde<T: Serialize + for<'de> Deserialize<'de>>() {}

    assert_serde::<f8e4m3>();
    assert_serde::<f4e2m1fn>();
}

#[derive(Debug)]
struct TestError(String);

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for TestError {}

impl serde::ser::Error for TestError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

impl serde::de::Error for TestError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self(msg.to_string())
    }
}

#[derive(Debug, PartialEq)]
enum Captured {
    U8(u8),
    F32(u32),
    String(String),
}

struct CaptureSerializer;

macro_rules! unsupported_serialize {
    ($($method:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {
        $(
            fn $method(self, $($arg: $ty),*) -> Result<$ret, Self::Error> {
                $(let _ = $arg;)*
                Err(serde::ser::Error::custom("unsupported serialization shape"))
            }
        )*
    };
}

impl Serializer for CaptureSerializer {
    type Ok = Captured;
    type Error = TestError;
    type SerializeSeq = serde::ser::Impossible<Captured, TestError>;
    type SerializeTuple = serde::ser::Impossible<Captured, TestError>;
    type SerializeTupleStruct = serde::ser::Impossible<Captured, TestError>;
    type SerializeTupleVariant = serde::ser::Impossible<Captured, TestError>;
    type SerializeMap = serde::ser::Impossible<Captured, TestError>;
    type SerializeStruct = serde::ser::Impossible<Captured, TestError>;
    type SerializeStructVariant = serde::ser::Impossible<Captured, TestError>;

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Captured::U8(value))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Captured::F32(value.to_bits()))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Captured::String(value.to_owned()))
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        Ok(Captured::String(value.to_string()))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        assert_eq!(name, "f8e4m3");
        value.serialize(self)
    }

    unsupported_serialize! {
        serialize_bool(value: bool) -> Self::Ok;
        serialize_i8(value: i8) -> Self::Ok;
        serialize_i16(value: i16) -> Self::Ok;
        serialize_i32(value: i32) -> Self::Ok;
        serialize_i64(value: i64) -> Self::Ok;
        serialize_u16(value: u16) -> Self::Ok;
        serialize_u32(value: u32) -> Self::Ok;
        serialize_u64(value: u64) -> Self::Ok;
        serialize_f64(value: f64) -> Self::Ok;
        serialize_char(value: char) -> Self::Ok;
        serialize_bytes(value: &[u8]) -> Self::Ok;
        serialize_none() -> Self::Ok;
        serialize_unit() -> Self::Ok;
        serialize_unit_struct(name: &'static str) -> Self::Ok;
        serialize_seq(len: Option<usize>) -> Self::SerializeSeq;
        serialize_tuple(len: usize) -> Self::SerializeTuple;
        serialize_tuple_struct(name: &'static str, len: usize) -> Self::SerializeTupleStruct;
        serialize_map(len: Option<usize>) -> Self::SerializeMap;
        serialize_struct(name: &'static str, len: usize) -> Self::SerializeStruct;
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        let _ = value;
        Err(serde::ser::Error::custom("unsupported serialization shape"))
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        let _ = value;
        Err(serde::ser::Error::custom("unsupported serialization shape"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        Err(serde::ser::Error::custom("unsupported serialization shape"))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let _ = (name, variant_index, variant);
        Err(serde::ser::Error::custom("unsupported serialization shape"))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = (name, variant_index, variant, value);
        Err(serde::ser::Error::custom("unsupported serialization shape"))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let _ = (name, variant_index, variant, len);
        Err(serde::ser::Error::custom("unsupported serialization shape"))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let _ = (name, variant_index, variant, len);
        Err(serde::ser::Error::custom("unsupported serialization shape"))
    }
}

struct BitsDeserializer(u8);

impl<'de> Deserializer<'de> for BitsDeserializer {
    type Error = TestError;

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        assert_eq!(name, "f8e4m3");
        visitor.visit_newtype_struct(serde::de::value::U8Deserializer::<TestError>::new(self.0))
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_newtype_struct("f8e4m3", visitor)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes
        byte_buf option unit unit_struct seq tuple tuple_struct map struct enum identifier
        ignored_any
    }
}

struct UnitDeserializer;

impl<'de> Deserializer<'de> for UnitDeserializer {
    type Error = TestError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes
        byte_buf option unit_struct seq tuple tuple_struct map struct enum identifier
        ignored_any newtype_struct
    }
}

#[test]
fn serializes_and_deserializes_representations() {
    let value = f8e4m3::from_f32(1.5);

    assert_eq!(
        value.serialize(CaptureSerializer).unwrap(),
        Captured::U8(value.to_bits())
    );
    assert_eq!(
        value.serialize_as_f32(CaptureSerializer).unwrap(),
        Captured::F32(1.5f32.to_bits())
    );
    assert_eq!(
        value.serialize_as_string(CaptureSerializer).unwrap(),
        Captured::String("1.5".to_owned())
    );

    assert_eq!(
        f8e4m3::deserialize(BitsDeserializer(value.to_bits())).unwrap(),
        value
    );
    assert_eq!(
        f8e4m3::deserialize(serde::de::value::StrDeserializer::<TestError>::new("1.5")).unwrap(),
        value
    );
    assert!(
        f8e4m3::deserialize(serde::de::value::StrDeserializer::<TestError>::new(
            "not a float"
        ))
        .is_err()
    );
    assert_eq!(
        f8e4m3::deserialize(serde::de::value::F32Deserializer::<TestError>::new(1.5)).unwrap(),
        value
    );
    assert_eq!(
        f8e4m3::deserialize(serde::de::value::F64Deserializer::<TestError>::new(1.5)).unwrap(),
        value
    );
}

#[test]
fn deserialization_expectation() {
    let err = f8e4m3::deserialize(UnitDeserializer)
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("tuple struct"),
        "error message should mention tuple struct f8e4m3 but was: {err}"
    );
}
