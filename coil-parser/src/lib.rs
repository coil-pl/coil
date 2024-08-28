mod expr;
use std::{ffi::{OsStr, OsString}, path::PathBuf, str::FromStr};

use coil_error::Error;
use coil_lexer::{Keyword, Lexer, Literal, Operator, Token, TokenKind};
pub use expr::*;

pub struct Parser {
    lexer: Lexer,
    saved_token: Option<Token>,
    line: usize,
    maybe_insert_semicolon: bool,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            saved_token: None,
            line: 1,
            maybe_insert_semicolon: false,
        }
    }

    pub fn get_token(&mut self) -> Result<Option<Token>, Error> {
        let saved = self.saved_token.take();
        let Some(x) = (if saved.is_some() {
            saved
        } else {
            self.lexer.next_token()?
        }) else {
            return Ok(None);
        };
        if self.maybe_insert_semicolon && self.line < x.line {
            self.saved_token = Some(x);
            self.maybe_insert_semicolon = false;
            return Ok(Some(Token::new(TokenKind::Operator(Operator::Semicolon), self.line)));
        }
        self.line = x.line;
        self.maybe_insert_semicolon = match x.kind {
            TokenKind::Identifier(_)
            | TokenKind::Keyword(Keyword::Break | Keyword::Continue | Keyword::Fallthrough | Keyword::Return)
            | TokenKind::Literal(_, _)
            | TokenKind::Parenthesis { closing: true, .. } => true,
            _ => false,
        };
        Ok(Some(x))
    }

    pub fn parse(&mut self) -> Expr {
        self.lexer.reset();
        let filename = self.lexer.file.as_ref();
        let filename = OsString::from_str(filename).unwrap();
        let filename = PathBuf::from(filename);
        let filename = filename.file_stem().unwrap();
        let filename: String = filename.to_owned().into_string().unwrap();
        let mut module = Statement::Module {
            name: Box::new(Expr::Identifier(filename.into())),
            children: vec![],
        };
        Expr::Statement(module)
    }
}

#[cfg(test)]
mod tests;
