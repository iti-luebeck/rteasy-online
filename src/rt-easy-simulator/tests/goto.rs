mod util;

use rt_easy_simulator::Simulator;
use rtcore::value::Value;
use rtprogram::Ident;

#[test]
fn cond_bus_after_pipe() {
    const SOURCE: &'static str = r#"
        declare output OUT(7:0)
        declare register X(7:0)
        declare bus A(7:0), B(7:0)
        
        A <- X, X <- 1 | if A = 1 then goto SKIP_1 fi;
        goto END;
        SKIP_1: OUT <- 1; # 1
        
        A <- B + 3, B <- X + 2, X <- 1 | if A = 6 then goto SKIP_2 fi;
        goto END;
        SKIP_2: OUT <- 2; # 2
        
        if B + X = 4 then A <- B + 5 fi, B <- X + 2, X <- 1 | if A = 8 then goto SKIP_3 fi;
        goto END;
        SKIP_3: OUT <- 3; # 3
        
        END:
    "#;

    let mut simulator = Simulator::init(util::compile(SOURCE));

    // 1
    simulator.step(false).unwrap();
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("OUT".to_string())).unwrap(),
        Value::parse_dec("1").unwrap()
    );

    // 2
    simulator.step(false).unwrap();
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("OUT".to_string())).unwrap(),
        Value::parse_dec("2").unwrap()
    );

    // 3
    simulator.step(false).unwrap();
    simulator.step(false).unwrap();
    assert_eq!(
        simulator.register_value(&Ident("OUT".to_string())).unwrap(),
        Value::parse_dec("3").unwrap()
    );
}
