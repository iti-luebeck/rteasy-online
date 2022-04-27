mod util;

#[test]
fn zero() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        declare register X
    "#;

    let vhdl = util::compile(SOURCE);
    assert!(vhdl.signals.condition_signals().is_empty());
    assert!(vhdl.signals.control_signals().is_empty());
}

#[test]
fn zero_nop_goto_assert() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        declare register X

        START: nop; goto START; assert 1;
    "#;

    let vhdl = util::compile(SOURCE);
    assert!(vhdl.signals.condition_signals().is_empty());
    assert!(vhdl.signals.control_signals().is_empty());
}

#[test]
fn one_condition_signal() {
    const SOURCE: &'static str = r#"
        declare register X(3:0)

        if X = 2 then nop fi;
    "#;

    let vhdl = util::compile(SOURCE);
    assert_eq!(vhdl.signals.condition_signals(), vec!["X = 2"]);
    assert!(vhdl.signals.control_signals().is_empty());
}

#[test]
fn one_control_signal() {
    const SOURCE: &'static str = r#"
        declare register X(3:0)

        X <- 2;
    "#;

    let vhdl = util::compile(SOURCE);
    assert!(vhdl.signals.condition_signals().is_empty());
    assert_eq!(vhdl.signals.control_signals(), vec!["X <- 2"]);
}

#[test]
fn duplicate_condition_signal() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        declare register X(3:0)

        if B + X = 1 then nop fi;
        if X(0) then nop fi;
        if B + X = 1 then nop fi; # duplicate
    "#;

    let vhdl = util::compile(SOURCE);
    assert_eq!(vhdl.signals.condition_signals(), vec!["B + X = 1", "X(0)"]);
    assert!(vhdl.signals.control_signals().is_empty());
}

#[test]
fn duplicate_control_signal() {
    const SOURCE: &'static str = r#"
        declare bus B(7:0)
        declare register X(3:0)

        X(3) <- 1, X(2:0) <- 2; X(2:0) <- 2;
    "#;

    let vhdl = util::compile(SOURCE);
    assert!(vhdl.signals.condition_signals().is_empty());
    assert_eq!(vhdl.signals.control_signals(), vec!["X(3) <- 1", "X(2:0) <- 2"]);
}

#[test]
fn remove_full_range() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        if X(7:0) = 1 then X(7:0) <- X(7:0) fi;
    "#;

    let vhdl = util::compile(SOURCE);
    assert_eq!(vhdl.signals.condition_signals(), vec!["X = 1"]);
    assert_eq!(vhdl.signals.control_signals(), vec!["X <- X"]);
}

#[test]
fn duplicate_full_range() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        X <- X;
        X <- X(7:0);
        X(7:0) <- X;
        X(7:0) <- X(7:0);
    "#;

    let vhdl = util::compile(SOURCE);
    assert!(vhdl.signals.condition_signals().is_empty());
    assert_eq!(vhdl.signals.control_signals(), vec!["X <- X"]);
}

#[test]
fn duplicate_number_different_kind() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        X <- "1100";
        X <- 0b1100; X <- 0B001100; X <- %0000000000000000001100;
        X <- 12; X <- 012;
        X <- 0xc; X <- 0X000C; X <- $00000c;

        X <- 4; X <- "100";
    "#;

    let vhdl = util::compile(SOURCE);
    assert!(vhdl.signals.condition_signals().is_empty());
    assert_eq!(vhdl.signals.control_signals(), vec!["X <- \"1100\"", "X <- 4"]);
}

#[test]
fn different_ctx_size() {
    const SOURCE: &'static str = r#"
        declare register X(7:0)
        X <- X(3:0) + X(3:0) = "0000";
        X <- X(3:0) + X(3:0) = "00000";
    "#;

    let vhdl = util::compile(SOURCE);
    assert!(vhdl.signals.condition_signals().is_empty());
    assert_eq!(
        vhdl.signals.control_signals(),
        vec!["X <- X(3:0) + X(3:0) = \"0000\"", "X <- X(3:0) + X(3:0) = \"00000\""],
    );
}

#[test]
fn bus_ordering() {
    const SOURCE: &'static str = r#"
        declare register X(7:0), Y(7:0)
        declare bus B(7:0), C(7:0)
        X <- B, B <- 1;
    "#;

    let vhdl = util::compile(SOURCE);
    assert!(vhdl.signals.condition_signals().is_empty());
    assert_eq!(vhdl.signals.control_signals(), vec!["B <- 1", "X <- B"]);
}

#[test]
fn misc() {
    const SOURCE: &'static str = r#"
        declare register X(7:0), Y(7:0)
        declare bus B(7:0), C(7:0)

        X <- 12 + 5;
        assert X = 17;

        B <- 0, X <- 12 + 5;

        X <- B, B <- -1;
        assert X = 0b11111111;

        SW:
        switch Y {
            case 0: Y <- Y + 1, goto SW
            case 1 and 1: Y <- Y + 1, goto SW
            case 1 + 1: Y <- Y + 1, goto SW
            case 3: if 0 then nop else nop fi
            default: nop, nop
        };

        X <- 1;
        assert X + 2 = 4;

        nop;
    "#;

    let vhdl = util::compile(SOURCE);
    assert_eq!(
        vhdl.signals.condition_signals(),
        vec!["Y = 0", "Y = (1 and 1)", "Y = 1 + 1", "Y = 3", "0"],
    );
    assert_eq!(
        vhdl.signals.control_signals(),
        vec!["X <- 12 + 5", "B <- 0", "B <- -1", "X <- B", "Y <- Y + 1", "X <- 1"],
    );
}
