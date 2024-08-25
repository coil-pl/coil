use crate::{Error, ErrorCode};

#[test]
fn test_error_new() {
    let code: ErrorCode = ErrorCode(1);
    let message: &str = "message";
    let file: &str = "<inline>";
    let line: usize = 1;
    let notes: Vec<Box<str>> = vec![];
    assert_eq!(
        Error {
            code,
            message: message.into(),
            file: file.into(),
            line,
            notes: notes,
        },
        Error::new(code, message, file, line),
    )
}

#[test]
fn test_error_with_note() {
    let code: ErrorCode = ErrorCode(1);
    let message: &str = "message";
    let file: &str = "<inline>";
    let line: usize = 1;
    let notes: Vec<Box<str>> = vec!["blah blah".into()];
    assert_eq!(
        Error {
            code,
            message: message.into(),
            file: file.into(),
            line,
            notes: notes,
        },
        Error::new(code, message, file, line).with_note("blah blah"),
    )
}
