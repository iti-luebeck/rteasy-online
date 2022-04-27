use compiler::PrettyPrintError;
use program::Program;
use unit_test::unit_test::UnitTest;

#[allow(dead_code)] // Not used by every test file
pub fn compile(source: &str) -> Program {
    let ast = match parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", parser::pretty_print_error(&e, source, None, false)),
    };

    let backend = compiler_backend_simulator::BackendSimulator;
    match compiler::compile(&backend, (), ast, &Default::default()) {
        Ok(program) => program,
        Err(e) => panic!("{}", e.pretty_print(source, None, false)),
    }
}

#[allow(dead_code)] // Not used by every test file
pub fn compile_unit_test(source: &str) -> UnitTest {
    match unit_test::parser::parse(&source) {
        Ok(unit_test) => unit_test,
        Err(e) => {
            panic!("{}", unit_test::parser::pretty_print_error(&e, &source, None, false))
        }
    }
}

#[allow(dead_code)] // Not used by every test file
pub fn compile_unit_test_err(source: &str) -> toktok::Error<unit_test::parser::Token> {
    match unit_test::parser::parse(&source) {
        Ok(_unit_test) => panic!("Expected error"),
        Err(e) => e,
    }
}
