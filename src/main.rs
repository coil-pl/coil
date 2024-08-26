use clap::Parser;
use std::{convert::identity, fs, path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Copy, Default)]
enum Step {
    Lexing,
    Parsing,
    Codegen,
    Linking,
    #[default]
    Finishing,
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        use Step::*;
        match s {
            "l" | "lex" | "lexing" | "lexer" => Ok(Lexing),
            "p" | "parse" | "parsing" | "parser" => Ok(Parsing),
            "c" | "codegen" | "codegenerator" => Ok(Codegen),
            "L" | "link" | "linking" | "linker" => Ok(Linking),
            "f" | "end" | "finish" | "finishing" => Ok(Finishing),
            _ => Err(format!("invalid value: {s}")),
        }
    }
}

impl ToString for Step {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}

#[derive(Parser)]
struct Args {
    /// The source file path
    source: PathBuf,
    #[arg(short, long)]
    /// The output file path
    output: Option<PathBuf>,
    /// The step at which to halt
    #[arg(short, long, default_value_t = Step::Finishing)]
    until: Step,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let source = args.source.canonicalize()?;
    let output = args.output.map_or_else(|| source.with_extension(std::env::consts::EXE_EXTENSION), identity);
    let lexer_source = fs::read_to_string(&source)?;
    let lx = coil_lexer::Lexer::new(&source.into_os_string().into_string().unwrap(), &lexer_source);
    let tokens: Vec<_> = lx.flatten().collect();
    for token in tokens.iter() {
        println!("{token:?}");
    }
    Ok(())
}
