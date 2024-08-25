#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCode(pub u16);

impl ErrorCode {
    const LEXER: u16 = 1 << 12;
    const PARSER: u16 = 1 << 13;
    const ANALYSIS: u16 = 1 << 14;
    const BACKEND: u16 = 1 << 15;

    pub const fn lexer(code: u16) -> Self {
        debug_assert!(code < Self::LEXER);
        Self(code | Self::LEXER)
    }

    pub const fn parser(code: u16) -> Self {
        debug_assert!(code < Self::LEXER);
        Self(code | Self::PARSER)
    }

    pub const fn analysis(code: u16) -> Self {
        debug_assert!(code < Self::LEXER);
        Self(code | Self::ANALYSIS)
    }

    pub const fn backend(code: u16) -> Self {
        debug_assert!(code < Self::LEXER);
        Self(code | Self::BACKEND)
    }
}

impl std::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E{:o}", self.0)
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Error {
    pub code: ErrorCode,
    pub message: Box<str>,
    pub file: Box<str>,
    pub line: usize,
    pub notes: Vec<Box<str>>,
}

impl Error {
    pub fn new(code: ErrorCode, message: &str, file: &str, line: usize) -> Self {
        Self {
            code,
            message: message.into(),
            file: file.into(),
            line,
            notes: Vec::new(),
        }
    }

    pub fn with_note(mut self, note: &str) -> Self {
        self.notes.push(note.into());
        self
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}: {}", self.code, self.message)?;
        writeln!(f, "{}:{}", self.file, self.line)?;
        // TODO: add line printing
        for note in self.notes.iter() {
            writeln!(f, "note: {note}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
