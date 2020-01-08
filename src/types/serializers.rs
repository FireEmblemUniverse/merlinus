
use std::fmt;
use std::collections::HashMap;

use serde::{
    Serialize, Deserialize,
    ser::{Serializer, SerializeStruct},
    de::{self, Deserializer, Visitor, MapAccess},
};
use super::{Custom, Identifier, Meta, MType, Entry};

const PRIMITIVE_TYPES: &'_ [(&'_ str, MType)] = &[
    ("Byte", MType::Byte),
    ("Short", MType::Short),
    ("Word", MType::Word),
    ("String", MType::MString),
];

const REPEATED_CONTENT_MSG: &str = "should have exactly one of `of`, `fields`, `variants`";

impl Serialize for Custom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S:Serializer
    {
        let mut state = serializer.serialize_struct("Custom", 3)?;

        state.serialize_field("name", &self.name)?;

        match &self.contents {
            Meta::Product(fields) => {
                state.serialize_field("metatype", "record")?;
                state.serialize_field("fields", fields)?;
            },
            Meta::Coproduct(variants) => {
                state.serialize_field("metatype", "oneof")?;
                state.serialize_field("variants", variants)?;
            },
            Meta::Alias(ty) => {
                state.serialize_field("metatype", "alias")?;
                state.serialize_field("of", ty)?;
            },
            Meta::Many(ty) => {
                state.serialize_field("metatype", "many")?;
                state.serialize_field("of", ty)?;
            },
        }

        state.end()
    }
}

impl From<String> for MType {
    fn from(variant: String) -> Self {
        PRIMITIVE_TYPES.iter()
            .find(|(id,_)| *id == &*variant)
            .map(|(_,x)| x.clone())
            .unwrap_or(MType::Custom(Identifier(variant)))
    }
}

impl<'a> From<&'a str> for MType {
    fn from(variant: &'a str) -> Self {
        PRIMITIVE_TYPES.iter()
            .find(|(id,_)| *id == &*variant)
            .map(|(_,x)| x.clone())
            .unwrap_or_else(|| MType::Custom(Identifier(variant.to_string())))
    }
}

impl<'de> Deserialize<'de> for MType {
    fn deserialize<D>(de: D) -> Result<MType, D::Error>
        where D: Deserializer<'de>
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

            fn visit_string<E>(self, variant:String) -> Result<Self::Value,E> {
                Ok(variant.into())
            }
        }

        de.deserialize_string(TypeVisitor)
    }
}

impl<'de> Deserialize<'de> for Custom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        const FIELDS: &'_ [&'_ str] =
            &[ "name"
             , "metatype"
             , "fields"
             , "field"
             , "variants"
             , "variant"
             , "of"
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

            fn expecting(&self, formatter:&mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("custom type definition")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Custom, V::Error>
                where V: MapAccess<'de>
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
                        },
                        Field::Metatype => {
                            if metatype.is_some() {
                                return Err(de::Error::duplicate_field("metatype"));
                            }
                            metatype = Some(map.next_value()?);
                        },
                        Field::Of => {
                            if contents.is_some() {
                                return Err(de::Error::custom(REPEATED_CONTENT_MSG));
                            }
                            contents = Some(_Meta::Of(map.next_value()?));
                        },
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
                let contents: _Meta =
                    contents.ok_or_else(|| de::Error::missing_field(
                        match metatype {
                            Kind::Product => "fields",
                            Kind::Coproduct => "variants",
                            Kind::Alias | Kind::Many => "of",
                        }
                    ))?;

                match metatype {
                    Kind::Product => {
                        if let _Meta::Fields(fields) = contents {
                            Ok(Custom { name, contents : Meta::Product(fields) })
                        }
                        else {
                            Err(de::Error::custom("`record` type should have a member named `fields`"))
                        }
                    },
                    Kind::Coproduct => {
                        if let _Meta::Variants(fields) = contents {
                            Ok(Custom { name, contents : Meta::Coproduct(fields) })
                        }
                        else {
                            Err(de::Error::custom("`one_of` type should have a member named `variants`"))
                        }
                    },
                    Kind::Alias => {
                        if let _Meta::Of(ty) = contents {
                            Ok(Custom { name, contents : Meta::Alias(ty) })
                        }
                        else {
                            Err(de::Error::custom("`alias` type should have a member named `of`"))
                        }
                    },
                    Kind::Many => {
                        if let _Meta::Of(ty) = contents {
                            Ok(Custom { name, contents : Meta::Many(ty) })
                        }
                        else {
                            Err(de::Error::custom("`many` type should have a member named `of`"))
                        }
                    }
                }
            }
        }

        deserializer.deserialize_struct("Custom", FIELDS, TypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::*;

    fn id(s : String) -> Identifier {
        Identifier(s)
    }

    fn cust(s : Identifier) -> MType {
        MType::Custom(s)
    }

    fn mk_custom(s : String) -> MType {
        cust(id(s))
    }

    #[test]
    fn test_custom() {
        let c = Custom {
            name: Identifier("Test".to_owned()),
            contents: Meta::Alias(mk_custom("Other".to_owned())),
        };

        assert_tokens(&c, &[
            Token::Struct { name: "Custom", len: 3 },
                Token::Str("name"),
                Token::Str("Test"),
                Token::Str("metatype"),
                Token::Str("alias"),
                Token::Str("of"),
                Token::Str("Other"),
            Token::StructEnd,
        ]);
    }

    #[test]
    fn test_pluralization() {
        let c = Custom {
            name: Identifier("Test".to_owned()),
            contents: Meta::Product(
                [ ( id("Field".to_owned())
                  , Entry { type_: mk_custom("Other".to_owned()) }
                  )
                ].iter().cloned().collect()
            ),
        };

        assert_de_tokens(&c, &[
            Token::Struct { name: "Custom", len: 3 },
                Token::Str("name"),
                Token::Str("Test"),
                Token::Str("metatype"),
                Token::Str("record"),
                Token::Str("field"),
                Token::Map { len: Some(1) },
                    Token::Str("Field"),
                    Token::Map { len: Some(1) },
                        Token::Str("type"),
                        Token::Str("Other"),
                    Token::MapEnd,
                Token::MapEnd,
            Token::StructEnd,
        ]);

        assert_de_tokens(&c, &[
            Token::Struct { name: "Custom", len: 3 },
                Token::Str("name"),
                Token::Str("Test"),
                Token::Str("metatype"),
                Token::Str("record"),
                Token::Str("fields"),
                Token::Map { len: Some(1) },
                    Token::Str("Field"),
                    Token::Map { len: Some(1) },
                        Token::Str("type"),
                        Token::Str("Other"),
                    Token::MapEnd,
                Token::MapEnd,
            Token::StructEnd,
        ]);
    }

    #[test]
    fn test_wrong_variant() {
        assert_de_tokens_error::<Custom>(
            &[ Token::Struct { name: "Custom", len: 3 }
             ,   Token::Str("name")
             ,   Token::Str("Test")
             ,   Token::Str("metatype")
             ,   Token::Str("one_of")
             ,   Token::Str("field")
             ,   Token::Map { len: Some(1) }
             ,     Token::Str("Field")
             ,     Token::Map { len: Some(1) }
             ,       Token::Str("type")
             ,       Token::Str("Other")
             ,     Token::MapEnd
             ,   Token::MapEnd
             , Token::StructEnd
             ],
            "`one_of` type should have a member named `variants`"
        );
    }
}

