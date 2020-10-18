use std::collections::HashMap;
use std::fmt;

use super::{Custom, Entry, Identifier, Kind as K, MObj, MType};
use serde::{
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};

const PRIMITIVE_TYPES: &'_ [(&'_ str, MType)] = &[
    ("Byte", MType::Byte),
    ("Short", MType::Short),
    ("Word", MType::Word),
    ("String", MType::MString),
];

const REPEATED_CONTENT_MSG: &str =
    "should have exactly one of `of`, `fields`, `variants`";

impl Serialize for Custom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Custom", 3)?;

        state.serialize_field("name", &self.name)?;

        match &self.contents {
            K::Product(fields) => {
                state.serialize_field("metatype", "record")?;
                state.serialize_field("fields", fields)?;
            }
            K::Coproduct(variants) => {
                state.serialize_field("metatype", "oneof")?;
                state.serialize_field("variants", variants)?;
            }
            K::Alias(ty) => {
                state.serialize_field("metatype", "alias")?;
                state.serialize_field("of", ty)?;
            }
            K::Many(ty) => {
                state.serialize_field("metatype", "many")?;
                state.serialize_field("of", ty)?;
            }
        }

        state.end()
    }
}

impl From<String> for MType {
    fn from(variant: String) -> Self {
        PRIMITIVE_TYPES
            .iter()
            .find(|(id, _)| *id == &*variant)
            .map(|(_, x)| x.clone())
            .unwrap_or(MType::Custom(Identifier(variant)))
    }
}

impl<'a> From<&'a str> for MType {
    fn from(variant: &'a str) -> Self {
        PRIMITIVE_TYPES
            .iter()
            .find(|(id, _)| *id == &*variant)
            .map(|(_, x)| x.clone())
            .unwrap_or_else(|| MType::Custom(Identifier(variant.to_string())))
    }
}

impl<'de> Deserialize<'de> for MType {
    fn deserialize<D>(de: D) -> Result<MType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TypeVisitor;

        impl<'de> Visitor<'de> for TypeVisitor {
            type Value = MType;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("identifier")
            }

            fn visit_str<E>(self, variant: &str) -> Result<Self::Value, E> {
                Ok(variant.into())
            }

            fn visit_string<E>(self, variant: String) -> Result<Self::Value, E> {
                Ok(variant.into())
            }
        }

        de.deserialize_string(TypeVisitor)
    }
}

impl<'de> Deserialize<'de> for Custom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'_ [&'_ str] = &[
            "name", "metatype", "fields", "field", "variants", "variant", "of",
        ];

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            Metatype,
            #[serde(alias = "field")]
            Fields,
            #[serde(alias = "variant")]
            Variants,
            Of,
        };

        #[derive(Deserialize, Debug)]
        #[serde(field_identifier)]
        enum Kind {
            #[serde(rename = "record")]
            Product,
            #[serde(rename = "one_of")]
            Coproduct,
            #[serde(rename = "many")]
            Many,
            #[serde(rename = "alias")]
            Alias,
        }

        enum _Meta {
            Of(MType),
            Variants(HashMap<Identifier, Entry>),
            Fields(HashMap<Identifier, Entry>),
        }

        struct TypeVisitor;

        impl<'de> Visitor<'de> for TypeVisitor {
            type Value = Custom;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("custom type definition")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Custom, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name: Option<Identifier> = None;
                let mut metatype: Option<Kind> = None;
                let mut contents: Option<_Meta> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(Identifier(map.next_value()?));
                        }
                        Field::Metatype => {
                            if metatype.is_some() {
                                return Err(de::Error::duplicate_field("metatype"));
                            }
                            metatype = Some(map.next_value()?);
                        }
                        Field::Of => {
                            if contents.is_some() {
                                return Err(de::Error::custom(REPEATED_CONTENT_MSG));
                            }
                            contents = Some(_Meta::Of(map.next_value()?));
                        }
                        Field::Variants => {
                            if contents.is_some() {
                                return Err(de::Error::custom(REPEATED_CONTENT_MSG));
                            }
                            contents = Some(_Meta::Variants(map.next_value()?));
                        }
                        Field::Fields => {
                            if contents.is_some() {
                                return Err(de::Error::custom(REPEATED_CONTENT_MSG));
                            }
                            contents = Some(_Meta::Fields(map.next_value()?));
                        }
                    }
                }

                let name: Identifier =
                    name.ok_or_else(|| de::Error::missing_field("name"))?;
                let metatype: Kind =
                    metatype.ok_or_else(|| de::Error::missing_field("metatype"))?;
                let contents: _Meta = contents.ok_or_else(|| {
                    de::Error::missing_field(match metatype {
                        Kind::Product => "fields",
                        Kind::Coproduct => "variants",
                        Kind::Alias | Kind::Many => "of",
                    })
                })?;

                match metatype {
                    Kind::Product => {
                        if let _Meta::Fields(fields) = contents {
                            Ok(Custom {
                                name,
                                contents: K::Product(fields),
                            })
                        } else {
                            Err(de::Error::custom(
                                "`record` type should have a member named `fields`",
                            ))
                        }
                    }
                    Kind::Coproduct => {
                        if let _Meta::Variants(fields) = contents {
                            Ok(Custom {
                                name,
                                contents: K::Coproduct(fields),
                            })
                        } else {
                            Err(de::Error::custom(
                                "`one_of` type should have a member named `variants`",
                            ))
                        }
                    }
                    Kind::Alias => {
                        if let _Meta::Of(ty) = contents {
                            Ok(Custom {
                                name,
                                contents: K::Alias(ty),
                            })
                        } else {
                            Err(de::Error::custom(
                                "`alias` type should have a member named `of`",
                            ))
                        }
                    }
                    Kind::Many => {
                        if let _Meta::Of(ty) = contents {
                            Ok(Custom {
                                name,
                                contents: K::Many(ty),
                            })
                        } else {
                            Err(de::Error::custom(
                                "`many` type should have a member named `of`",
                            ))
                        }
                    }
                }
            }
        }

        deserializer.deserialize_struct("Custom", FIELDS, TypeVisitor)
    }
}

impl<'de> Deserialize<'de> for MObj {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObjVisitor;

        impl<'de> Visitor<'de> for ObjVisitor {
            type Value = MObj;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("A Merlinus-compatible object")
            }

            fn visit_i8<E>(self, value: i8) -> Result<MObj, E>
            where
                E: de::Error,
            {
                Ok(MObj::Byte(value))
            }

            fn visit_i16<E>(self, value: i16) -> Result<MObj, E>
            where
                E: de::Error,
            {
                Ok(MObj::Short(value))
            }

            fn visit_i32<E>(self, value: i32) -> Result<MObj, E>
            where
                E: de::Error,
            {
                Ok(MObj::Word(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<MObj, E>
            where
                E: de::Error,
            {
                use std::i32;
                if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
                    Ok(MObj::Word(value as i32))
                } else {
                    Err(E::custom(format!("i32 out of range: {}", value)))
                }
            }

            fn visit_u8<E>(self, value: u8) -> Result<MObj, E>
            where
                E: de::Error,
            {
                Ok(MObj::Byte(value as i8))
            }

            fn visit_u16<E>(self, value: u16) -> Result<MObj, E>
            where
                E: de::Error,
            {
                Ok(MObj::Short(value as i16))
            }

            fn visit_u32<E>(self, value: u32) -> Result<MObj, E>
            where
                E: de::Error,
            {
                Ok(MObj::Word(value as i32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<MObj, E>
            where
                E: de::Error,
            {
                use std::u32;
                if value <= u64::from(u32::MAX) {
                    Ok(MObj::Word(value as i32))
                } else {
                    Err(E::custom(format!("u32 out of range: {}", value)))
                }
            }

            fn visit_string<E>(self, s: String) -> Result<MObj, E>
            where
                E: de::Error,
            {
                Ok(MObj::String(s))
            }

            fn visit_unit<E>(self) -> Result<MObj, E>
            where
                E: de::Error,
            {
                Ok(MObj::Unit)
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<MObj, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let mut result = Vec::with_capacity(seq.size_hint().unwrap_or(0));

                while let Some(v) = seq.next_element()? {
                    result.push(v);
                }

                Ok(MObj::List(result))
            }

            fn visit_map<M>(self, mut m: M) -> Result<MObj, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut result = HashMap::with_capacity(m.size_hint().unwrap_or(0));

                while let Some((k, v)) = m.next_entry()? {
                    result.insert(k, v);
                }

                Ok(MObj::Object(result))
            }
        }

        deserializer.deserialize_any(ObjVisitor)
    }
}
