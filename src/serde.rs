macro_rules! serde_impls {
    ($($type:ident),* $(,)?) => {
        $(
            #[cfg(feature = "serde")]
            impl ::serde::Serialize for $type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    serializer.serialize_newtype_struct(stringify!($type), &self.to_bits())
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> ::serde::Deserialize<'de> for $type {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    struct Visitor;

                    impl<'de> ::serde::de::Visitor<'de> for Visitor {
                        type Value = $type;

                        fn expecting(
                            &self,
                            formatter: &mut core::fmt::Formatter<'_>,
                        ) -> core::fmt::Result {
                            formatter.write_str(concat!("tuple struct ", stringify!($type)))
                        }

                        fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                        where
                            D: ::serde::Deserializer<'de>,
                        {
                            Ok(<$type>::from_bits(<u8 as ::serde::Deserialize>::deserialize(
                                deserializer,
                            )?))
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            value.parse::<$type>().map_err(|_| {
                                ::serde::de::Error::invalid_value(
                                    ::serde::de::Unexpected::Str(value),
                                    &"a float string",
                                )
                            })
                        }

                        fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            Ok(<$type>::from_f32(value))
                        }

                        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            Ok(<$type>::from_f64(value))
                        }
                    }

                    deserializer.deserialize_newtype_struct(stringify!($type), Visitor)
                }
            }
        )*
    };
}
