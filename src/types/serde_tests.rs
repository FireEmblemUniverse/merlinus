use serde_test::*;

use super::*;

fn id(s: String) -> Identifier {
    Identifier(s)
}

fn cust(s: Identifier) -> MType {
    MType::Custom(s)
}

fn mk_custom(s: String) -> MType {
    cust(id(s))
}

#[test]
fn test_custom() {
    let c = Custom {
        name: Identifier("Test".to_owned()),
        contents: Kind::Alias(mk_custom("Other".to_owned())),
    };

    assert_tokens(
        &c,
        &[
            Token::Struct {
                name: "Custom",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("Test"),
            Token::Str("metatype"),
            Token::Str("alias"),
            Token::Str("of"),
            Token::Str("Other"),
            Token::StructEnd,
        ],
    );
}

#[test]
fn test_pluralization() {
    let c = Custom {
        name: Identifier("Test".to_owned()),
        contents: Kind::Product(
            [(
                id("Field".to_owned()),
                Entry {
                    type_: mk_custom("Other".to_owned()),
                },
            )]
            .iter()
            .cloned()
            .collect(),
        ),
    };

    assert_de_tokens(
        &c,
        &[
            Token::Struct {
                name: "Custom",
                len: 3,
            },
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
        ],
    );

    assert_de_tokens(
        &c,
        &[
            Token::Struct {
                name: "Custom",
                len: 3,
            },
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
        ],
    );
}

#[test]
fn test_wrong_variant() {
    assert_de_tokens_error::<Custom>(
        &[
            Token::Struct {
                name: "Custom",
                len: 3,
            },
            Token::Str("name"),
            Token::Str("Test"),
            Token::Str("metatype"),
            Token::Str("one_of"),
            Token::Str("field"),
            Token::Map { len: Some(1) },
            Token::Str("Field"),
            Token::Map { len: Some(1) },
            Token::Str("type"),
            Token::Str("Other"),
            Token::MapEnd,
            Token::MapEnd,
            Token::StructEnd,
        ],
        "`one_of` type should have a member named `variants`",
    );
}
