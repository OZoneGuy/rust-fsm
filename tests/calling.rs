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
    let fsm = FSM::FSM::new();
    let f = |from: &str, to: &str, ev: &str| {
        println!("{} -> {}: {}", from, to, ev);
    };

    let f_end = |to: &str| {
        println!("ended at: {}", to);
    };
    fsm.a_fn(f).a().a().b_fn(f).c().end_fn(f_end);
}
