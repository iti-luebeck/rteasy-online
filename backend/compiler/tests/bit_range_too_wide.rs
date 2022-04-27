mod util;

use compiler::{CompilerErrorKind, Error};

#[test]
fn bit_range_too_wide() {
    let sources = &[
        (
            format!(
                r#"
                declare register A({0}:0)
                declare register B(0:{0})
                "#,
                usize::MAX
            ),
            u16::MAX as usize + 1,
            None,
        ),
        (
            r#"
            declare register A(131071:0)
            "#
            .to_string(),
            u16::MAX as usize + 1,
            Some(131072),
        ),
        (
            r#"
            declare bus B(0:65536)
            "#
            .to_string(),
            u16::MAX as usize + 1,
            Some(65537),
        ),
        (
            r#"
            declare register A(128:64), B
            declare memory MEM(A, B)
            "#
            .to_string(),
            64,
            Some(65),
        ),
    ];

    for (source, e_max_size, e_size) in sources {
        match util::check_err(source) {
            Error::Errors(errors) => {
                for error in errors {
                    match error.kind {
                        CompilerErrorKind::BitRangeTooWide { max_size, size } => {
                            assert_eq!(max_size, *e_max_size);
                            assert_eq!(size, *e_size);
                        }
                        other => panic!("unexpected error: {:?}", other),
                    }
                }
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }
}
