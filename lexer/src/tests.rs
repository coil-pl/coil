use coil_error::Error;

use crate::{
    token::{Keyword, Literal, Operator, Parenthesis, TokenKind},
    Lexer, Token, INVALID_STRING_ESCAPE, UNEXPECTED, UNFINISHED_STRING, UNFINISHED_STRING_ESCAPE,
};

fn quick_lex(source: &str) -> Result<Vec<Token>, Error> {
    let lx = Lexer::new("<inline>", source);
    let v: Vec<_> = lx.collect();
    let mut result = Vec::with_capacity(v.len());
    for i in v {
        result.push(i?);
    }
    Ok(result)
}

#[test]
fn test_num_integer() {
    let source = "0 1 24 0x12fA 0o07 0b1001";
    let expected = [
        TokenKind::Literal(Literal::Integer { radix: 10 }, "0".into()),
        TokenKind::Literal(Literal::Integer { radix: 10 }, "1".into()),
        TokenKind::Literal(Literal::Integer { radix: 10 }, "24".into()),
        TokenKind::Literal(Literal::Integer { radix: 16 }, "12fA".into()),
        TokenKind::Literal(Literal::Integer { radix: 8 }, "07".into()),
        TokenKind::Literal(Literal::Integer { radix: 2 }, "1001".into()),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);

    let source = "0. ";
    let expected = [
        TokenKind::Literal(Literal::Integer { radix: 10 }, "0".into()),
        TokenKind::Operator(Operator::Dot),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_num_float() {
    let source = "1.0 3.14159 0x2.b7e151628aed2a6abf7158809cf4f 0o7.0 0b1.1";
    let expected = [
        TokenKind::Literal(Literal::Float { radix: 10 }, "1.0".into()),
        TokenKind::Literal(Literal::Float { radix: 10 }, "3.14159".into()),
        TokenKind::Literal(
            Literal::Float { radix: 16 },
            "2.b7e151628aed2a6abf7158809cf4f".into(),
        ),
        TokenKind::Literal(Literal::Float { radix: 8 }, "7.0".into()),
        TokenKind::Literal(Literal::Float { radix: 2 }, "1.1".into()),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_num_exp() {
    let source = "1e6 0.314159e1 2.1e-9 6.3e+2 0o3.7e4 0b0101.1001e101";
    let expected = [
        TokenKind::Literal(Literal::Float { radix: 10 }, "1e6".into()),
        TokenKind::Literal(Literal::Float { radix: 10 }, "0.314159e1".into()),
        TokenKind::Literal(Literal::Float { radix: 10 }, "2.1e-9".into()),
        TokenKind::Literal(Literal::Float { radix: 10 }, "6.3e+2".into()),
        TokenKind::Literal(Literal::Float { radix: 8 }, "3.7e4".into()),
        TokenKind::Literal(Literal::Float { radix: 2 }, "0101.1001e101".into()),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_num_err() {
    let source = "1.";
    let err = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(err.code, crate::UNEXPECTED_EOF);
    assert_eq!(err.file.as_ref(), "<inline>");
    assert_eq!(err.line, 1);
    assert_eq!(
        err.message.as_ref(),
        "expected a digit but found end of file"
    );

    let source = "1e";
    let err = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(err.code, crate::UNEXPECTED_EOF);
    assert_eq!(err.file.as_ref(), "<inline>");
    assert_eq!(err.line, 1);
    assert_eq!(
        err.message.as_ref(),
        "expected a digit, '+', '-' but found end of file"
    );

    let source = "1e+";
    let err = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(err.code, crate::UNEXPECTED_EOF);
    assert_eq!(err.file.as_ref(), "<inline>");
    assert_eq!(err.line, 1);
    assert_eq!(
        err.message.as_ref(),
        "expected a digit but found end of file"
    );

    let source = "1e-";
    let err = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(err.code, crate::UNEXPECTED_EOF);
    assert_eq!(err.file.as_ref(), "<inline>");
    assert_eq!(err.line, 1);
    assert_eq!(
        err.message.as_ref(),
        "expected a digit but found end of file"
    );

    let source = "1e.";
    let err = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(err.code, crate::UNEXPECTED);
    assert_eq!(err.file.as_ref(), "<inline>");
    assert_eq!(err.line, 1);
    assert_eq!(
        err.message.as_ref(),
        "expected a digit, '+' or '-' but found '.'"
    );

    let source = "1e+.";
    let err = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(err.code, crate::UNEXPECTED);
    assert_eq!(err.file.as_ref(), "<inline>");
    assert_eq!(err.line, 1);
    assert_eq!(err.message.as_ref(), "expected a digit but found '.'");

    let source = "1e-.";
    let err = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(err.code, crate::UNEXPECTED);
    assert_eq!(err.file.as_ref(), "<inline>");
    assert_eq!(err.line, 1);
    assert_eq!(err.message.as_ref(), "expected a digit but found '.'");

    let source = "0x";
    let err = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(err.code, crate::UNEXPECTED_EOF);
    assert_eq!(err.file.as_ref(), "<inline>");
    assert_eq!(err.line, 1);
    assert_eq!(
        err.message.as_ref(),
        "expected a digit but found end of file"
    );

    let source = "0x.";
    let err = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(err.code, crate::UNEXPECTED);
    assert_eq!(err.file.as_ref(), "<inline>");
    assert_eq!(err.line, 1);
    assert_eq!(err.message.as_ref(), "expected a digit but found '.'");
}

#[test]
fn test_op_arithmetic() {
    let source = "+ += - -= * *= / /= % %=";
    let expected = [
        TokenKind::Operator(Operator::Plus),
        TokenKind::Operator(Operator::PlusAssign),
        TokenKind::Operator(Operator::Minus),
        TokenKind::Operator(Operator::MinusAssign),
        TokenKind::Operator(Operator::Star),
        TokenKind::Operator(Operator::StarAssign),
        TokenKind::Operator(Operator::Slash),
        TokenKind::Operator(Operator::SlashAssign),
        TokenKind::Operator(Operator::Percent),
        TokenKind::Operator(Operator::PercentAssign),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_op_comparison() {
    let source = "== != > >= < <=";
    let expected = [
        TokenKind::Operator(Operator::Eq),
        TokenKind::Operator(Operator::NotEq),
        TokenKind::Operator(Operator::Greater),
        TokenKind::Operator(Operator::GreaterEq),
        TokenKind::Operator(Operator::Lesser),
        TokenKind::Operator(Operator::LesserEq),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_op_logic() {
    let source = "! && &&= || ||=";
    let expected = [
        TokenKind::Operator(Operator::Not),
        TokenKind::Operator(Operator::And),
        TokenKind::Operator(Operator::AndAssign),
        TokenKind::Operator(Operator::Or),
        TokenKind::Operator(Operator::OrAssign),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_op_bitwise_except_shift() {
    let source = "~ & &= | |= ^ ^=";
    let expected = [
        TokenKind::Operator(Operator::BitNot),
        TokenKind::Operator(Operator::BitAnd),
        TokenKind::Operator(Operator::BitAndAssign),
        TokenKind::Operator(Operator::BitOr),
        TokenKind::Operator(Operator::BitOrAssign),
        TokenKind::Operator(Operator::BitXor),
        TokenKind::Operator(Operator::BitXorAssign),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_op_bitwise_shift() {
    let source = "<< <<= >> >>=";
    let expected = [
        TokenKind::Operator(Operator::BitShiftLeft),
        TokenKind::Operator(Operator::BitShiftLeftAssign),
        TokenKind::Operator(Operator::BitShiftRight),
        TokenKind::Operator(Operator::BitShiftRightAssign),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_op_misc() {
    let source = "= ; : . .. , ? -> =>";
    let expected = [
        TokenKind::Operator(Operator::Assign),
        TokenKind::Operator(Operator::Semicolon),
        TokenKind::Operator(Operator::Colon),
        TokenKind::Operator(Operator::Dot),
        TokenKind::Operator(Operator::DoubleDot),
        TokenKind::Operator(Operator::Comma),
        TokenKind::Operator(Operator::QuestionMark),
        TokenKind::Operator(Operator::Arrow),
        TokenKind::Operator(Operator::Bolt),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_op_parenthesis() {
    let source = "()[]{}";
    let expected = [
        TokenKind::Parenthesis {
            closing: false,
            kind: Parenthesis::Normal,
        },
        TokenKind::Parenthesis {
            closing: true,
            kind: Parenthesis::Normal,
        },
        TokenKind::Parenthesis {
            closing: false,
            kind: Parenthesis::Square,
        },
        TokenKind::Parenthesis {
            closing: true,
            kind: Parenthesis::Square,
        },
        TokenKind::Parenthesis {
            closing: false,
            kind: Parenthesis::Curly,
        },
        TokenKind::Parenthesis {
            closing: true,
            kind: Parenthesis::Curly,
        },
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_keywords_b2e() {
    let source = "break consttime continue do else enum extern";
    let expected = [
        Keyword::Break,
        Keyword::Consttime,
        Keyword::Continue,
        Keyword::Do,
        Keyword::Else,
        Keyword::Enum,
        Keyword::Extern,
    ]
    .map(|kw| Token::new(TokenKind::Keyword(kw), 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_keywords_f2i() {
    let source = "false fn for if impl import in is";
    let expected = [
        Keyword::False,
        Keyword::Fn,
        Keyword::For,
        Keyword::If,
        Keyword::Impl,
        Keyword::Import,
        Keyword::In,
        Keyword::Is,
    ]
    .map(|kw| Token::new(TokenKind::Keyword(kw), 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_keywords_l2p() {
    let source = "launch let match module mut pub";
    let expected = [
        Keyword::Launch,
        Keyword::Let,
        Keyword::Match,
        Keyword::Module,
        Keyword::Mut,
        Keyword::Pub,
    ]
    .map(|kw| Token::new(TokenKind::Keyword(kw), 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_keywords_r2t() {
    let source = "return Self static struct trait true type";
    let expected = [
        Keyword::Return,
        Keyword::SelfType,
        Keyword::Static,
        Keyword::Struct,
        Keyword::Trait,
        Keyword::True,
        Keyword::Type,
    ]
    .map(|kw| Token::new(TokenKind::Keyword(kw), 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_keywords_u2w() {
    let source = "unsafe union where while";
    let expected = [
        Keyword::Unsafe,
        Keyword::Union,
        Keyword::Where,
        Keyword::While,
    ]
    .map(|kw| Token::new(TokenKind::Keyword(kw), 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_identifiers() {
    let source = "self abcdef123 æʁ̥õʰɹıüş ЇЈЉЊЋЌЎЏАБ զէըթժի اللهواكبر 私の名前は";
    let expected = [
        TokenKind::Identifier("self".into()),
        TokenKind::Identifier("abcdef123".into()),
        TokenKind::Identifier("æʁ̥õʰɹıüş".into()),
        TokenKind::Identifier("ЇЈЉЊЋЌЎЏАБ".into()),
        TokenKind::Identifier("զէըթժի".into()),
        TokenKind::Identifier("اللهواكبر".into()),
        TokenKind::Identifier("私の名前は".into()),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_string() {
    let source = r#""I am a string" "Me two" """#;
    let expected = [
        TokenKind::Literal(Literal::String, "I am a string".into()),
        TokenKind::Literal(Literal::String, "Me two".into()),
        TokenKind::Literal(Literal::String, "".into()),
    ]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_string_raw() {
    let source = r#"r"\to \be\""#;
    let expected =
        [TokenKind::Literal(Literal::String, r"\to \be\".into())].map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_string_escape() {
    let source = r#""\\\"\'\a\b\f\n\r\t\v\x70\u0120\U00102130""#;
    let expected = [TokenKind::Literal(
        Literal::String,
        "\\\"\'\x07\x08\x0c\n\r\t\x0b\x70\u{0120}\u{102130}".into(),
    )]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_string_multiline() {
    let source = r####"#"I am
    a multiline string"# ###""#"###"####;
    let expected = [
        Token::new(
            TokenKind::Literal(Literal::String, "I am\n    a multiline string".into()),
            1,
        ),
        Token::new(TokenKind::Literal(Literal::String, r##""#"##.into()), 2),
    ];
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_string_multiline_escape() {
    let source = r##"#"\\\"\'\a\b\f\n\r\t\v\x70\u0120\U00102130"#"##;
    let expected = [TokenKind::Literal(
        Literal::String,
        "\\\"\'\x07\x08\x0c\n\r\t\x0b\x70\u{0120}\u{102130}".into(),
    )]
    .map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_string_multiline_raw() {
    let source = r##"r#"\to \be\"#"##;
    let expected =
        [TokenKind::Literal(Literal::String, r"\to \be\".into())].map(|x| Token::new(x, 1));
    let tokens = quick_lex(source).expect("expected source to be fully lexed");
    assert_eq!(&tokens, &expected,);
}

#[test]
fn test_string_err() {
    let source = r#""
"#;
    let tokens = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(tokens.code, UNFINISHED_STRING);
    assert_eq!(tokens.message.as_ref(), "unfinished string");
    assert_eq!(tokens.line, 1);
    assert_eq!(tokens.file.as_ref(), "<inline>");

    let source = r#""\z""#;
    let tokens = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(tokens.code, INVALID_STRING_ESCAPE);
    assert_eq!(tokens.message.as_ref(), "invalid string escape: '\\z'");
    assert_eq!(tokens.line, 1);
    assert_eq!(tokens.file.as_ref(), "<inline>");

    let source = r#""\x1g""#;
    let tokens = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(tokens.code, INVALID_STRING_ESCAPE);
    assert_eq!(tokens.message.as_ref(), "invalid string escape: '\\x1g'");
    assert_eq!(tokens.line, 1);
    assert_eq!(tokens.file.as_ref(), "<inline>");

    let source = r#""\u1g00""#;
    let tokens = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(tokens.code, INVALID_STRING_ESCAPE);
    assert_eq!(tokens.message.as_ref(), "invalid string escape: '\\u1g00'");
    assert_eq!(tokens.line, 1);
    assert_eq!(tokens.file.as_ref(), "<inline>");

    let source = r#""\U1g000000""#;
    let tokens = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(tokens.code, INVALID_STRING_ESCAPE);
    assert_eq!(
        tokens.message.as_ref(),
        "invalid string escape: '\\U1g000000'"
    );
    assert_eq!(tokens.line, 1);
    assert_eq!(tokens.file.as_ref(), "<inline>");

    for source in [r#""\"#, r#""\x1"#, r#""\u1"#, r#""\U1"#] {
        let tokens = quick_lex(source).expect_err("expected to get an error");
        assert_eq!(tokens.code, UNFINISHED_STRING_ESCAPE);
        assert_eq!(tokens.message.as_ref(), "unfinished string escape");
        assert_eq!(tokens.line, 1);
        assert_eq!(tokens.file.as_ref(), "<inline>");
        assert_eq!(
            &tokens.notes,
            &[
                "maybe finish the string with a '\"'".into(),
                "if you wanted to make a raw string, add r before the string".into(),
            ]
        );
    }

    let source = r##"#a""#"##;
    let tokens = quick_lex(source).expect_err("expected to get an error");
    assert_eq!(tokens.code, UNEXPECTED);
    assert_eq!(
        tokens.message.as_ref(),
        "expected one of '#' or '\"' but found 'a'"
    );
    assert_eq!(tokens.line, 1);
    assert_eq!(tokens.file.as_ref(), "<inline>");

    for source in [r##"#""##, r##"#"""##] {
        let tokens = quick_lex(source).expect_err("expected to get an error");
        assert_eq!(tokens.code, UNFINISHED_STRING);
        assert_eq!(tokens.message.as_ref(), "unfinished string");
        assert_eq!(tokens.line, 1);
        assert_eq!(tokens.file.as_ref(), "<inline>");
        assert_eq!(
            &tokens.notes,
            &["maybe finish the string with a '\"#'".into()]
        );
    }
}
