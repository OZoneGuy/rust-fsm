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

fn main() {
    let fsm = FSM::new();
    debug_assert!(fsm.history.is_empty());
    let fsm = fsm.a().a().a();
    debug_assert!(fsm.history.len() == 3);
    let fsm = fsm.b();
    debug_assert!(fsm.history.len() == 4);
    debug_assert!(
        fsm.history.last()
            == Some(&Transition {
                from: "A".to_string(),
                to: "B".to_string(),
                event: "b".to_string(),
            })
    );
    let fsm = fsm.c();
    debug_assert!(fsm.history.len() == 5);
    fsm.end();
}
