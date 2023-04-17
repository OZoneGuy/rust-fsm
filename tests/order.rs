use fsm_macro::fsm;

fsm! {
    initial = A
    end = C, B

        A -> B: b
        A -> A: a
        B -> C: c

        A
        B
        C
}

fn main() {}
