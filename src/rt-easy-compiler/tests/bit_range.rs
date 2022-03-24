mod util;

use rt_easy_compiler::{CompilerErrorKind, Error};

#[test]
fn bit_range_ok() {
    const SOURCE: &'static str = r#"
        declare register R(7:0)
        declare bus B(7:0)
        declare input I(7:0)
        declare output OUT(7:0)
        declare register array ARR(7:0)[64]

        declare register X(3:0)

        X <- R(6:3);
        X <- B(7:4);
        X <- I(5:4);
        X <- OUT(4:1);
        X <- ARR[0](3:0);
        X <- ARR[0](7:7);
    "#;

    util::check(SOURCE);
}

#[test]
fn bit_range_err() {
    const SOURCES: &'static [&'static str] = &[
        // Register
        r#"
            declare register X(7:0)
            declare register R(7:0)
            X <- R(2:3);
        "#,
        r#"
            declare register X(7:0)
            declare register R(7:0)
            X <- R(8:1);
        "#,
        // Bus
        r#"
            declare register X(7:0)
            declare bus B(7:0)
            X <- B(2:3);
        "#,
        r#"
            declare register X(7:0)
            declare bus B(7:0)
            X <- B(8:1);
        "#,
        // Input
        r#"
            declare register X(7:0)
            declare input I(7:0)
            X <- I(2:3);
        "#,
        r#"
            declare register X(7:0)
            declare input I(7:0)
            X <- I(8:1);
        "#,
        // Output
        r#"
            declare register X(7:0)
            declare output OUT(7:0)
            X <- OUT(2:3);
        "#,
        r#"
            declare register X(7:0)
            declare output OUT(7:0)
            X <- OUT(8:1);
        "#,
        // Register Array
        r#"
            declare register X(7:0)
            declare register array ARR(7:0)[64]
            X <- ARR[1](2:3);
        "#,
        r#"
            declare register X(7:0)
            declare register array ARR(7:0)[64]
            X <- ARR[2](8:1);
        "#,
    ];

    for source in SOURCES {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::RangeMismatch { .. } => (),
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
