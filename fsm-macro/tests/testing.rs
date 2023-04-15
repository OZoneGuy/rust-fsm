use fsm_macro::fsm;

fsm! {
    initial = A
        A
        B
        C
        end = C, B

        A -> B: b
        A -> A: a
        B -> C: c
}

#[test]
fn compiles() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile.rs");
    t.pass("tests/calling.rs");
    t.pass("tests/history.rs");
}
