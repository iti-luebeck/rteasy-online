use ast::Ast;

#[allow(dead_code)] // Not used by every test file
pub fn parse(source: &str) -> Ast<'_> {
    match parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", parser::pretty_print_error(&e, source, None, false)),
    }
}

#[allow(dead_code)] // Not used by every test file
pub fn parse_err(source: &str) -> toktok::Error<parser::Token> {
    match parser::parse(source) {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    }
}
