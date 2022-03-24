#![deny(rust_2018_idioms)]

mod build_mir;
mod check_ast;
mod check_mir;
mod error;
mod symbols;
mod util;

pub mod mir;
pub use self::error::{CompilerError, CompilerErrorKind, Error, InternalError};
pub use self::symbols::SymbolType;

use std::convert::Infallible;

pub trait Backend {
    type Args;
    type Output;
    type Error: PrettyPrintError;

    fn generate(&self, mir: mir::Mir<'_>, args: Self::Args) -> Result<Self::Output, Self::Error>;
}

// Dummy Backend
impl Backend for Infallible {
    type Args = Infallible;
    type Output = Infallible;
    type Error = Infallible;

    fn generate(&self, _: mir::Mir<'_>, _: Self::Args) -> Result<Self::Output, Self::Error> {
        match *self {}
    }
}

pub trait PrettyPrintError {
    fn pretty_print(&self, source: &str, file_name: Option<&str>, ansi_colors: bool) -> String;
}

// Dummy PrettyPrintError
impl PrettyPrintError for Infallible {
    fn pretty_print(&self, _: &str, _: Option<&str>, _: bool) -> String {
        match *self {}
    }
}

#[derive(Debug, Default)]
pub struct Options {
    pub print_mir_unordered: bool,
    pub print_mir: bool,
}

pub fn compile<B>(
    backend: &B,
    args: B::Args,
    ast: rtast::Ast<'_>,
    options: &Options,
) -> Result<B::Output, Error<B>>
where
    B: Backend,
{
    let (_symbols, mir) = check_(ast, options)?;

    match backend.generate(mir, args) {
        Ok(output) => Ok(output),
        Err(e) => Err(Error::Backend(e)),
    }
}

pub fn check(ast: rtast::Ast<'_>, options: &Options) -> Result<(), Error<Infallible>> {
    check_(ast, options)?;
    Ok(())
}

fn check_<'s, B: Backend>(
    ast: rtast::Ast<'s>,
    options: &Options,
) -> Result<(symbols::Symbols<'s>, mir::Mir<'s>), Error<B>> {
    // Check ast
    let symbols = check_ast::check(&ast)?;

    // Build and check mir
    let mut mir = build_mir::build_mir(ast, &symbols)?;
    check_mir::check(&symbols, &mut mir, options)?;

    Ok((symbols, mir))
}
