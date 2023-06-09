# Rust-FSM

A statically checked finite state machine written in rust.

## Motivation

Because I can. And I wanted to practice creating proc macros in rust. They can be fun...

## Usage

To create a state machine, use the `fsm` macro.

```rust
fsm!{

  initial = init_state
  state_1
  state_2
  ...
  end = end_state_1, end_state_2,...

  state_n -> state_m: transition_k
  ...
}

// --snip--

// Initialize the finite state machine
let mut fsm = FSM::new();

// run transition and save new struct
fsm = fsm.transition_k();
let f = |from: &str, to: &str, event: &str| {
    println!("from: {from}, to: {to}, event: {event}");
    // other actions to be performed
}

// You can pass a callback function to be called with the transition
fsm = fsm.transition_l_fn(f);

// Call the end function. Confirm that the FSM reached a final state.
fsm.end();

// or pass a closure
f_end = |state: %str| {
      println!("done!");
      // maybe do something else as well...
}

fsm.end_fn(f_end);
```

## How does it work

The macro creates types for all the states and an FSM struct. The FSM struct is what the users should use. The state types allow it to define and restrict what transitions/functions can be called.

### Sample generated code

```rust
type struct state_1;
...

type struct FSM<T> {
  ...
  pub history: Vec<Transition>,
}

type struct Transition{
  pub from: String,
  pub to: String,
  pub event: string,
}

impl FSM {

  pub fn new() -> FSM<initial_state> {
    return FSM{
      // initialization here
    }
  }

}

impl FSM<state_1> {
  pub fn transition_k(self) -> FSM<state_2> {
    return FSM {
      // initialization here as well
    }
  }

  pub fn transition_k_fn(self, f: Fn(&str, &str, &str)) -> FSM<state_2> {
    f("state_1", "state_2", "transition_k");
    return self.transition_k();
  }
}

...

impl FSM<end_state> {

  ...
  // note that it destroys the object.
  // Do what you want with it before calling this function
  pub fn end() {}

  pub fn end_fn(f: Fn(&str)) {
    f("end_state");
  }
}
```

## Issues

1. Sanitization
 * The generated code is placed in the same module as the macro call. It should ideally create a new module and place the new struct there.
1. Macros and LSP
 * Macros don't have the greatest lsp support. Doesn't always give intellisence and might not warn about bad code.

## Features to add

 * [x] Provide function to print to FSM to a graph format
   * probably will stick to dot for now
 * [ ] Check for unreachable states
   * should give some sort of warning then...
 * [ ] Allow for multiple transition labels on the same line
   * something like: `A -> B: a,b,c,...`
