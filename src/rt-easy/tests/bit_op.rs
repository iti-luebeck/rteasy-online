use rt_easy::{
    rtcore::{program::Ident, value::Value},
    simulator::Simulator,
};

const SOURCE: &'static str = r#"
declare register A(31:0)

A <- 0b1110 xor 0b0101;
A <- 0b1110 or 0b0101;
A <- 0b1110 nor 0b0101;
A <- 0b1110 and 0b0101;
A <- 0b1110 nand 0b0101;
"#;

#[test]
fn mult() {
    let mut simulator = compile(SOURCE);

    // xor
    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("1011", false).unwrap()
    );

    // or
    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("1111", false).unwrap()
    );

    // nor
    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("11111111111111111111111111110000", false).unwrap()
    );

    // and
    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("100", false).unwrap()
    );

    // nand
    simulator.step().unwrap();
    assert_eq!(
        simulator.register_value(&Ident("A".to_string())).unwrap(),
        Value::parse_bin("11111111111111111111111111111011", false).unwrap()
    );
}

fn compile(source: &str) -> Simulator {
    let ast = match rt_easy::parser::parse(source) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", rt_easy::parser::pretty_print_error(&e, source)),
    };

    let backend = rt_easy::compiler_backend_simulator::BackendSimulator;
    match rt_easy::compiler::compile(&backend, ast, &Default::default()) {
        Ok(program) => Simulator::init(program),
        Err(e) => panic!("{:#?}", e),
    }
}
