mod util;

use program::Ident;
use rtcore::value::Value;
use simulator::Simulator;

#[test]
fn misc() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        declare bus B(7:0), C(7:0)

        X <- (1 > 0) + (X = X); # 1
        X(3:0) <- (not 0) xor "1010"; # 2
        X <- sxt B(0), B <- 1; # 3
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 1
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("10").unwrap()
    );

    // 2
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("0101").unwrap()
    );

    // 3
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("11111111").unwrap()
    );
}

#[test]
fn literals() {
    const SOURCE: &'static str = r#"
        declare register X

        # 0000
        X <- "1000" + "1000" = "0000";
        X <- "1000" + "1000" = 0b0000;

        # 00000
        X <- "1000" + "1000" = "00000";
        X <- "1000" + "1000" = 0b00000;
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 0000
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("1").unwrap()
    );
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("1").unwrap()
    );

    // 00000
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("0").unwrap()
    );
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("X".to_string())).unwrap(),
        Value::parse_bin("1").unwrap()
    );
}
