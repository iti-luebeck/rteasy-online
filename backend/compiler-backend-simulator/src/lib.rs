#![deny(rust_2018_idioms)]

mod concat;
mod declaration;
mod expression;
mod helper;
mod operation;
mod program;

#[derive(Debug)]
pub struct BackendSimulator;

impl compiler::Backend for BackendSimulator {
    type Args = ();
    type Output = ::program::Program;
    type Error = std::convert::Infallible;

    fn generate(&self, mir: compiler::mir::Mir<'_>, _args: Self::Args) -> Result<Self::Output> {
        ::program::Program::generate(mir)
    }
}

type Result<T> = std::result::Result<T, std::convert::Infallible>;

trait Generate<I>: Sized {
    fn generate(input: I) -> Result<Self>;
}

fn gen_label(l: compiler::mir::Label<'_>) -> ::program::Label {
    ::program::Label(l.0.to_owned())
}

fn gen_ident(id: compiler::mir::Ident<'_>) -> ::program::Ident {
    ::program::Ident(id.0.to_owned())
}
