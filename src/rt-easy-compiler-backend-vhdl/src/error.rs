use rtcore::common::Span;
use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SynthError>;

#[derive(Debug, Error)]
#[error("VHDL backend error")]
pub struct BackendError {
    pub errors: Vec<SynthError>,
    pub signals: rtvhdl::Signals,
}

impl BackendError {
    pub fn pretty_print(&self, source: &str, file_name: Option<&str>, ansi_colors: bool) -> String {
        // Sort errors
        let mut errors = self.errors.iter().collect::<Vec<_>>();
        errors.sort_by(|a, b| a.span.cmp(&b.span));

        // Pretty print all errors
        let mut result = String::new();
        for (idx, error) in errors.iter().enumerate() {
            if idx != 0 {
                result += "\n\n";
            }
            result += &error.pretty_print(source, file_name, ansi_colors);
        }
        result
    }
}

#[derive(Debug, Error)]
#[error("{kind}")]
pub struct SynthError {
    pub kind: SynthErrorKind,
    pub span: Span,
}

impl SynthError {
    pub fn new(kind: SynthErrorKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn pretty_print(&self, source: &str, file_name: Option<&str>, ansi_colors: bool) -> String {
        let message = self.kind.to_string();
        let error_code = format!("[E{:03}]", self.kind.code());

        let mut error = pretty_error::Error::new(&message)
            .with_error_code(&error_code)
            .with_source(source, pretty_error::Span::Range(self.span.range()))
            .with_ansi_colors(ansi_colors);
        if let Some(file_name) = file_name {
            error = error.with_file_name(file_name);
        }

        error.to_string()
    }
}

#[derive(Debug)]
pub enum SynthErrorKind {
    UnclockedGotoDependency,
    ConditionalGotoInFirstState,
}

impl SynthErrorKind {
    pub fn code(&self) -> usize {
        use SynthErrorKind::*;

        match self {
            UnclockedGotoDependency => 200,
            ConditionalGotoInFirstState => 201,
        }
    }
}

impl fmt::Display for SynthErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SynthErrorKind::*;

        match self {
            UnclockedGotoDependency => write!(f, "next state depends on an unclocked item"),
            ConditionalGotoInFirstState => write!(f, "conditional goto in first state"),
        }
    }
}
