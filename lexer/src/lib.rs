#![feature(iter_advance_by)]
use std::hint::unreachable_unchecked;

use coil_error::Error;
use coil_error::ErrorCode;

mod cursor;
use cursor::LexerCursor;

mod char_trait_ext;
mod token;
use char_trait_ext::CharTraitExt;

use phf::phf_map;
pub use token::Keyword;
pub use token::Literal;
pub use token::Operator;
pub use token::Parenthesis;
pub use token::Token;
pub use token::TokenKind;

struct ParseStringOptions {
    pub raw: bool,
}

pub struct Lexer {
    cursor: LexerCursor,
    file: Box<str>,
    line: usize,
}

fn next_after_while<I: Iterator>(
    it: &mut I,
    mut predicate: impl FnMut(&I::Item) -> bool,
) -> Option<I::Item> {
    let mut a = it.next();
    while predicate(a.as_ref()?) {
        a = it.next();
    }
    a
}

const UNEXPECTED_EOF: ErrorCode = ErrorCode::lexer(1);
const UNEXPECTED: ErrorCode = ErrorCode::lexer(2);
const UNFINISHED_STRING: ErrorCode = ErrorCode::lexer(3);
const UNFINISHED_STRING_ESCAPE: ErrorCode = ErrorCode::lexer(4);
const INVALID_STRING_ESCAPE: ErrorCode = ErrorCode::lexer(5);

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "break" => Keyword::Break,
    "consttime" => Keyword::Consttime,
    "continue" => Keyword::Continue,
    "do" => Keyword::Do,
    "else" => Keyword::Else,
    "enum" => Keyword::Enum,
    "extern" => Keyword::Extern,
    "false" => Keyword::False,
    "fn" => Keyword::Fn,
    "for" => Keyword::For,
    "if" => Keyword::If,
    "impl" => Keyword::Impl,
    "import" => Keyword::Import,
    "in" => Keyword::In,
    "is" => Keyword::Is,
    "launch" => Keyword::Launch,
    "let" => Keyword::Let,
    "match" => Keyword::Match,
    "module" => Keyword::Module,
    "mut" => Keyword::Mut,
    "pub" => Keyword::Pub,
    "return" => Keyword::Return,
    "Self" => Keyword::SelfType,
    "static" => Keyword::Static,
    "struct" => Keyword::Struct,
    "trait" => Keyword::Trait,
    "true" => Keyword::True,
    "type" => Keyword::Type,
    "unsafe" => Keyword::Unsafe,
    "union" => Keyword::Union,
    "where" => Keyword::Where,
    "while" => Keyword::While,
};

impl Lexer {
    #[inline]
    pub fn new(file: &str, source: &str) -> Self {
        Self {
            cursor: LexerCursor::new(source),
            file: file.into(),
            line: 1,
        }
    }

    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline]
    pub fn cursor(&self) -> &LexerCursor {
        &self.cursor
    }

    pub fn parse_num(&mut self, radix: usize) -> Result<Token, Error> {
        let mut buf = String::new();

        let num_n = radix.min(10);
        let max_num = ('0' as usize + num_n - 1) as u8 as char;
        let num_range = '0'..=max_num;

        let alph_n = radix.saturating_sub(10);
        let max_alph = ('A' as usize + alph_n - 1) as u8 as char;
        let alph_range = 'A'..=max_alph;

        let can_exp = !('A'..=max_alph).contains(&'E');
        let mut float = false;
        let mut exp = false;

        while let Some(x) = self.cursor.next() {
            if x == '.' && !float && !exp {
                let Some(nx) = self.cursor.next() else {
                    return Err(Error::new(
                        UNEXPECTED_EOF,
                        "expected a digit but found end of file",
                        &self.file,
                        self.line,
                    ));
                };
                self.cursor.rewind(1);
                if !num_range.contains(&nx) && !alph_range.contains(&nx.to_ascii_uppercase()) {
                    self.cursor.rewind(1);
                    return Ok(Token::new(
                        TokenKind::Literal(Literal::Integer { radix }, buf.into()),
                        self.line,
                    ));
                }
                buf.push(x);
                float = true;
                continue;
            }
            if can_exp && (x == 'e' || x == 'E') && !exp {
                buf.push(x);
                let Some(nx) = self.cursor.next() else {
                    return Err(Error::new(
                        UNEXPECTED_EOF,
                        "expected a digit, '+', '-' but found end of file",
                        &self.file,
                        self.line,
                    ));
                };
                if !num_range.contains(&nx)
                    && !alph_range.contains(&nx.to_ascii_uppercase())
                    && nx != '+'
                    && nx != '-'
                {
                    return Err(Error::new(
                        UNEXPECTED,
                        &format!("expected a digit, '+' or '-' but found {nx:?}"),
                        &self.file,
                        self.line,
                    ));
                }
                if nx == '+' || nx == '-' {
                    buf.push(nx);
                    let Some(nx) = self.cursor.next() else {
                        return Err(Error::new(
                            UNEXPECTED_EOF,
                            "expected a digit but found end of file",
                            &self.file,
                            self.line,
                        ));
                    };
                    if !num_range.contains(&nx) && !alph_range.contains(&nx.to_ascii_uppercase()) {
                        return Err(Error::new(
                            UNEXPECTED,
                            &format!("expected a digit but found {nx:?}"),
                            &self.file,
                            self.line,
                        ));
                    }
                }
                self.cursor.rewind(1);
                float = true;
                exp = true;
                continue;
            }
            if !num_range.contains(&x) && !alph_range.contains(&x.to_ascii_uppercase()) {
                self.cursor.rewind(1);
                return Ok(Token::new(
                    TokenKind::Literal(
                        if float {
                            Literal::Float { radix }
                        } else {
                            Literal::Integer { radix }
                        },
                        buf.into(),
                    ),
                    self.line,
                ));
            }
            buf.push(x);
        }
        return Ok(Token::new(
            TokenKind::Literal(
                if float {
                    Literal::Float { radix }
                } else {
                    Literal::Integer { radix }
                },
                buf.into(),
            ),
            self.line,
        ));
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let Some(current) = next_after_while(&mut self.cursor, |c| {
            if *c == '\n' {
                self.line += 1;
            }
            c.is_ascii_whitespace()
        }) else {
            return Ok(None);
        };
        self.cursor.rewind(1);
        match current {
            '0' => {
                let _ = self.cursor.advance(1);
                match self.cursor.next() {
                    Some(base @ ('x' | 'o' | 'b')) => {
                        let radix = match base {
                            'x' => 16,
                            'o' => 8,
                            'b' => 2,
                            _ => unsafe { unreachable_unchecked() },
                        };
                        let num_n = radix.min(10);
                        let max_num = ('0' as usize + num_n - 1) as u8 as char;
                        let num_range = '0'..=max_num;

                        let alph_n = radix.saturating_sub(10);
                        let max_alph = ('A' as usize + alph_n - 1) as u8 as char;
                        let alph_range = 'A'..=max_alph;

                        let Some(x) = self.cursor.next() else {
                            return Err(Error::new(
                                UNEXPECTED_EOF,
                                "expected a digit but found end of file",
                                &self.file,
                                self.line,
                            ));
                        };
                        if !num_range.contains(&x) && !alph_range.contains(&x) {
                            return Err(Error::new(
                                UNEXPECTED,
                                &format!("expected a digit but found {x:?}"),
                                &self.file,
                                self.line,
                            ));
                        };
                        self.cursor.rewind(1);
                        return self.parse_num(radix).map(Some);
                    }
                    Some('.') => {
                        self.cursor.rewind(2);
                        return self.parse_num(10).map(Some);
                    }
                    _ => {
                        self.cursor.rewind(1);
                        return Ok(Token {
                            kind: TokenKind::Literal(Literal::Integer { radix: 10 }, "0".into()),
                            line: self.line,
                        })
                        .map(Some);
                    }
                }
            }
            '1'..='9' => {
                return self.parse_num(10).map(Some);
            }
            ',' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Operator(Operator::Comma),
                    self.line,
                )));
            }
            ';' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Operator(Operator::Semicolon),
                    self.line,
                )));
            }
            ':' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Operator(Operator::Colon),
                    self.line,
                )));
            }
            '?' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Operator(Operator::QuestionMark),
                    self.line,
                )));
            }
            '.' => {
                let _ = self.cursor.advance(1);
                if let Some('.') = self.cursor.current() {
                    let _ = self.cursor.advance(1);
                    return Ok(Some(Token::new(
                        TokenKind::Operator(Operator::DoubleDot),
                        self.line,
                    )));
                }
                return Ok(Some(Token::new(
                    TokenKind::Operator(Operator::Dot),
                    self.line,
                )));
            }
            '=' => {
                let _ = self.cursor.advance(1);
                if let Some('>') = self.cursor.current() {
                    let _ = self.cursor.advance(1);
                    return Ok(Some(Token::new(
                        TokenKind::Operator(Operator::Bolt),
                        self.line,
                    )));
                }
                if let Some('=') = self.cursor.current() {
                    let _ = self.cursor.advance(1);
                    return Ok(Some(Token::new(
                        TokenKind::Operator(Operator::Eq),
                        self.line,
                    )));
                }
                return Ok(Some(Token::new(
                    TokenKind::Operator(Operator::Assign),
                    self.line,
                )));
            }
            '-' => {
                let _ = self.cursor.advance(1);
                if let Some('>') = self.cursor.current() {
                    let _ = self.cursor.advance(1);
                    return Ok(Some(Token::new(
                        TokenKind::Operator(Operator::Arrow),
                        self.line,
                    )));
                }
                if let Some('=') = self.cursor.current() {
                    let _ = self.cursor.advance(1);
                    return Ok(Some(Token::new(
                        TokenKind::Operator(Operator::MinusAssign),
                        self.line,
                    )));
                }
                return Ok(Some(Token::new(
                    TokenKind::Operator(Operator::Minus),
                    self.line,
                )));
            }
            '+' | '/' | '*' | '%' | '!' | '^' => {
                let _ = self.cursor.advance(1);
                if let Some('=') = self.cursor.current() {
                    let _ = self.cursor.advance(1);
                    return Ok(Some(Token::new(
                        TokenKind::Operator(match current {
                            '+' => Operator::PlusAssign,
                            '/' => Operator::SlashAssign,
                            '*' => Operator::StarAssign,
                            '%' => Operator::PercentAssign,
                            '!' => Operator::NotEq,
                            '^' => Operator::BitXorAssign,
                            _ => unreachable!(),
                        }),
                        self.line,
                    )));
                }
                return Ok(Some(Token::new(
                    TokenKind::Operator(match current {
                        '+' => Operator::Plus,
                        '/' => Operator::Slash,
                        '*' => Operator::Star,
                        '%' => Operator::Percent,
                        '!' => Operator::Not,
                        '^' => Operator::BitXor,
                        _ => unreachable!(),
                    }),
                    self.line,
                )));
            }
            '>' | '<' | '&' | '|' => {
                let _ = self.cursor.advance(1);
                if let Some('=') = self.cursor.current() {
                    let _ = self.cursor.advance(1);
                    return Ok(Some(Token::new(
                        TokenKind::Operator(match current {
                            '>' => Operator::GreaterEq,
                            '<' => Operator::LesserEq,
                            '&' => Operator::BitAndAssign,
                            '|' => Operator::BitOrAssign,
                            _ => unreachable!(),
                        }),
                        self.line,
                    )));
                }
                if Some(current) == self.cursor.current() {
                    let _ = self.cursor.advance(1);
                    if let Some('=') = self.cursor.current() {
                        let _ = self.cursor.advance(1);
                        return Ok(Some(Token::new(
                            TokenKind::Operator(match current {
                                '>' => Operator::BitShiftRightAssign,
                                '<' => Operator::BitShiftLeftAssign,
                                '&' => Operator::AndAssign,
                                '|' => Operator::OrAssign,
                                _ => unreachable!(),
                            }),
                            self.line,
                        )));
                    }
                    return Ok(Some(Token::new(
                        TokenKind::Operator(match current {
                            '>' => Operator::BitShiftRight,
                            '<' => Operator::BitShiftLeft,
                            '&' => Operator::And,
                            '|' => Operator::Or,
                            _ => unreachable!(),
                        }),
                        self.line,
                    )));
                }
                return Ok(Some(Token::new(
                    TokenKind::Operator(match current {
                        '>' => Operator::Greater,
                        '<' => Operator::Lesser,
                        '&' => Operator::BitAnd,
                        '|' => Operator::BitOr,
                        _ => unreachable!(),
                    }),
                    self.line,
                )));
            }
            '~' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Operator(Operator::BitNot),
                    self.line,
                )));
            }
            '(' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Parenthesis {
                        closing: false,
                        kind: Parenthesis::Normal,
                    },
                    self.line,
                )));
            }
            ')' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Parenthesis {
                        closing: true,
                        kind: Parenthesis::Normal,
                    },
                    self.line,
                )));
            }
            '[' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Parenthesis {
                        closing: false,
                        kind: Parenthesis::Square,
                    },
                    self.line,
                )));
            }
            ']' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Parenthesis {
                        closing: true,
                        kind: Parenthesis::Square,
                    },
                    self.line,
                )));
            }
            '{' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Parenthesis {
                        closing: false,
                        kind: Parenthesis::Curly,
                    },
                    self.line,
                )));
            }
            '}' => {
                let _ = self.cursor.advance(1);
                return Ok(Some(Token::new(
                    TokenKind::Parenthesis {
                        closing: true,
                        kind: Parenthesis::Curly,
                    },
                    self.line,
                )));
            }
            '"' => {
                return self
                    .parse_string(ParseStringOptions { raw: false })
                    .map(Some)
            }
            '#' => {
                return self
                    .parse_multiline_string(ParseStringOptions { raw: false })
                    .map(Some)
            }
            c if c.is_ascii_whitespace() => unreachable!(),
            c if c.is_alphabetic() => return self.parse_ident().map(Some),
            c => {
                println!("unimplemented character: {c:?}@{}", self.cursor.position);
                todo!();
            }
        }
        Ok(None)
    }

    fn parse_string_escape(&mut self, buf: &mut String) -> Result<(), Error> {
        let Some(esc_c) = self.cursor.next() else {
            return Err(Error::new(
                UNFINISHED_STRING_ESCAPE,
                "unfinished string escape",
                self.file.as_ref(),
                self.line,
            )
            .with_note("maybe finish the string with a '\"'")
            .with_note("if you wanted to make a raw string, add r before the string"));
        };
        match esc_c {
            '\\' => buf.push('\\'),
            '\'' => buf.push('\''),
            '"' => buf.push('"'),
            'a' => buf.push('\x07'),
            'b' => buf.push('\x08'),
            'f' => buf.push('\x0c'),
            'n' => buf.push('\n'),
            'r' => buf.push('\r'),
            't' => buf.push('\t'),
            'v' => buf.push('\x0b'),
            'x' => {
                let mut n = String::new();
                for _ in 0..2 {
                    let Some(c) = self.cursor.next() else {
                        return Err(Error::new(
                            UNFINISHED_STRING_ESCAPE,
                            "unfinished string escape",
                            self.file.as_ref(),
                            self.line,
                        )
                        .with_note("maybe finish the string with a '\"'")
                        .with_note("if you wanted to make a raw string, add r before the string"));
                    };
                    n.push(c);
                }
                let codepoint = match u8::from_str_radix(&n, 16) {
                    Ok(cp) => cp,
                    Err(_) => {
                        return Err(Error::new(
                            INVALID_STRING_ESCAPE,
                            &format!("invalid string escape: '\\x{n}'"),
                            self.file.as_ref(),
                            self.line,
                        ))
                    }
                };
                buf.push(codepoint as char);
            }
            'u' => {
                let mut n = String::new();
                for _ in 0..4 {
                    let Some(c) = self.cursor.next() else {
                        return Err(Error::new(
                            UNFINISHED_STRING_ESCAPE,
                            "unfinished string escape",
                            self.file.as_ref(),
                            self.line,
                        )
                        .with_note("maybe finish the string with a '\"'")
                        .with_note("if you wanted to make a raw string, add r before the string"));
                    };
                    n.push(c);
                }
                let codepoint = match u32::from_str_radix(&n, 16) {
                    Ok(cp) => cp,
                    Err(_) => {
                        return Err(Error::new(
                            INVALID_STRING_ESCAPE,
                            &format!("invalid string escape: '\\u{n}'"),
                            self.file.as_ref(),
                            self.line,
                        ))
                    }
                };
                // Unsafe rationale:
                // Here, invalid chars are supposed to pass.
                buf.push(unsafe { char::from_u32_unchecked(codepoint) });
            }
            'U' => {
                let mut n = String::new();
                for _ in 0..8 {
                    let Some(c) = self.cursor.next() else {
                        return Err(Error::new(
                            UNFINISHED_STRING_ESCAPE,
                            "unfinished string escape",
                            self.file.as_ref(),
                            self.line,
                        )
                        .with_note("maybe finish the string with a '\"'")
                        .with_note("if you wanted to make a raw string, add r before the string"));
                    };
                    n.push(c);
                }
                let codepoint = match u32::from_str_radix(&n, 16) {
                    Ok(cp) => cp,
                    Err(_) => {
                        return Err(Error::new(
                            INVALID_STRING_ESCAPE,
                            &format!("invalid string escape: '\\U{n}'"),
                            self.file.as_ref(),
                            self.line,
                        ))
                    }
                };
                // Unsafe rationale:
                // Here, invalid chars are supposed to pass.
                buf.push(unsafe { char::from_u32_unchecked(codepoint) });
            }
            _ => {
                return Err(Error::new(
                    INVALID_STRING_ESCAPE,
                    &format!("invalid string escape: '\\{esc_c}'"),
                    self.file.as_ref(),
                    self.line,
                ))
            }
        }
        Ok(())
    }

    fn parse_string(
        &mut self,
        ParseStringOptions { raw }: ParseStringOptions,
    ) -> Result<Token, Error> {
        assert_eq!(self.cursor.next(), Some('"'));
        let mut buf = String::new();
        loop {
            let ch = match self.cursor.next() {
                Some(ch) => ch,
                None => {
                    return Err(Error::new(
                        UNFINISHED_STRING,
                        "unfinished string",
                        self.file.as_ref(),
                        self.line,
                    )
                    .with_note("maybe finish the string with a '\"'"))
                }
            };
            if ch == '"' {
                break;
            }
            if ch == '\n' {
                return Err(Error::new(
                    UNFINISHED_STRING,
                    "unfinished string",
                    self.file.as_ref(),
                    self.line,
                )
                .with_note("maybe finish the string with a '\"'"));
            }
            if !raw && ch == '\\' {
                self.parse_string_escape(&mut buf)?;
                continue;
            }
            buf.push(ch);
        }
        Ok(Token::new(
            TokenKind::Literal(Literal::String, buf.into()),
            self.line,
        ))
    }

    fn parse_multiline_string(
        &mut self,
        ParseStringOptions { raw }: ParseStringOptions,
    ) -> Result<Token, Error> {
        let start_line = self.line;
        let mut octothorp_count = 0;
        loop {
            let x = self.cursor.next();
            let x = match x {
                Some(x) => x,
                None => {
                    return Err(Error::new(
                        UNEXPECTED,
                        "expected one of '#' or '\"' but found end of file",
                        self.file.as_ref(),
                        start_line,
                    ))
                }
            };
            if x != '#' && x != '"' {
                return Err(Error::new(
                    UNEXPECTED,
                    &format!("expected one of '#' or '\"' but found {x:?}"),
                    self.file.as_ref(),
                    start_line,
                ));
            }
            if x == '"' {
                break;
            }
            octothorp_count += 1;
        }
        let mut buf = String::new();
        'tloop: loop {
            let ch = match self.cursor.next() {
                Some(ch) => ch,
                None => {
                    return Err(Error::new(
                        UNFINISHED_STRING,
                        "unfinished string",
                        self.file.as_ref(),
                        start_line,
                    )
                    .with_note(&format!(
                        "maybe finish the string with a '\"{}'",
                        "#".repeat(octothorp_count)
                    )));
                }
            };
            if ch == '"' {
                let mut mdb = String::from('"');
                for _ in 0..octothorp_count {
                    let Some(x) = self.cursor.next() else {
                        return Err(Error::new(
                            UNFINISHED_STRING,
                            "unfinished string",
                            self.file.as_ref(),
                            start_line,
                        )
                        .with_note(&format!(
                            "maybe finish the string with a '\"{}'",
                            "#".repeat(octothorp_count)
                        )));
                    };
                    if x == '#' {
                        mdb.push(x);
                    } else {
                        self.cursor.rewind(1);
                        buf.push_str(&mdb);
                        continue 'tloop;
                    }
                }
                break;
            }
            if ch == '\n' {
                self.line += 1;
            }
            if !raw && ch == '\\' {
                self.parse_string_escape(&mut buf)?;
                continue;
            }
            buf.push(ch);
        }
        Ok(Token::new(
            TokenKind::Literal(Literal::String, buf.into()),
            start_line,
        ))
    }

    fn parse_ident(&mut self) -> Result<Token, Error> {
        let mut buf = String::new();
        while let Some(x) = self.cursor.next() {
            if buf.as_str().to_ascii_lowercase() == "r" {
                if x == '#' {
                    self.cursor.rewind(1);
                    return self.parse_multiline_string(ParseStringOptions { raw: true });
                }
                if x == '"' {
                    self.cursor.rewind(1);
                    return self.parse_string(ParseStringOptions { raw: true });
                }
            }
            if !x.is_alphanumeric() && !x.is_combining_character() {
                self.cursor.rewind(1);
                return Ok(Token::new(
                    KEYWORDS
                        .get(&buf)
                        .map(|kw| TokenKind::Keyword(*kw))
                        .unwrap_or(TokenKind::Identifier(buf.into())),
                    self.line,
                ));
            }
            buf.push(x);
        }
        Ok(Token::new(
            KEYWORDS
                .get(&buf)
                .map(|kw| TokenKind::Keyword(*kw))
                .unwrap_or(TokenKind::Identifier(buf.into())),
            self.line,
        ))
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        match self.next_token() {
            Ok(x) => x.map(Ok),
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests;
