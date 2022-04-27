mod util;

use compiler::{CompilerErrorKind, Error};

#[test]
fn register_array_read() {
    const SOURCE: &'static str = r#"
        declare register array ARR(7:0)[64]
        declare register X(7:0)
        ARR[0] <- 12 + ARR[1] + ARR[1], X <- ARR[2];
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::RegisterArrayTooManyReads { .. } => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn register_array_read_if() {
    const SOURCE: &'static str = r#"
        declare register array ARR(7:0)[64]
        declare register X(7:0)
        ARR[0] <- 12 + ARR[1] + ARR[1], if X(0) then X <- ARR[2] fi;
    "#;

    match util::check_err(SOURCE) {
        Error::Errors(errors) => {
            for error in errors {
                match error.kind {
                    CompilerErrorKind::RegisterArrayTooManyReads { .. } => (),
                    other => panic!("unexpected error: {:?}", other),
                }
            }
        }
        other => panic!("unexpected error: {:?}", other),
    }
}

#[test]
fn register_array_read_ok() {
    const SOURCE: &'static str = r#"
        declare register array ARR(7:0)[64]
        declare register X(7:0)
        if X(0) then ARR[0] <- 12 + ARR[1] + ARR[1] else X <- ARR[2] fi;
    "#;

    util::check(SOURCE);
}

#[test]
fn register_array_bit_range_lvalue() {
    const SOURCES: &'static [&'static str] = &[
        r#"
            declare register array ARR(7:0)[64]
            ARR[0](2:1) <- 3;
        "#,
        r#"
            declare register array ARR(2:0)[32]
            ARR[2](0) <- 1;
        "#,
        r#"
            declare register array ARR(7:0)[64]
            declare register X(3:0)
            X(0).ARR[0](7:0).X(1) <- -1;
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::AssignmentLhsRegisterArrayWithBitRange => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}

#[test]
fn register_array_bit_range_expr() {
    const SOURCE: &'static str = r#"
        declare register array ARR(7:0)[64]
        declare register X(7:0)

        X <- ARR[3](3:1);
        X <- ARR[2](0);
        X <- ARR[1](2:2);
    "#;

    util::check(SOURCE);
}
