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

fn main() {}
