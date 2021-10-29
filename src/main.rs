pub mod tm;


fn main() {

    let _states = [
        tm::State { name: "q0",  tape_symbol: '0', write_symbol: 'x', move_tape: tm::TapeMove::Right, next_state: "q1"  },
        tm::State { name: "q0",  tape_symbol: 'x', write_symbol: 'x', move_tape: tm::TapeMove::Left,  next_state: "q10" },
        tm::State { name: "q1",  tape_symbol: '0', write_symbol: '0', move_tape: tm::TapeMove::Right, next_state: "q1"  },
        tm::State { name: "q1",  tape_symbol: 'x', write_symbol: 'x', move_tape: tm::TapeMove::Right, next_state: "q2"  },
        tm::State { name: "q2",  tape_symbol: '0', write_symbol: '1', move_tape: tm::TapeMove::Right, next_state: "q4"  },
        tm::State { name: "q2",  tape_symbol: '=', write_symbol: '=', move_tape: tm::TapeMove::Left,  next_state: "q3"  },
        tm::State { name: "q3",  tape_symbol: '1', write_symbol: '0', move_tape: tm::TapeMove::Left,  next_state: "q3"  },
        tm::State { name: "q3",  tape_symbol: 'x', write_symbol: 'x', move_tape: tm::TapeMove::Left,  next_state: "q9"  },
        tm::State { name: "q4",  tape_symbol: '0', write_symbol: '0', move_tape: tm::TapeMove::Right, next_state: "q4"  },
        tm::State { name: "q4",  tape_symbol: '=', write_symbol: '=', move_tape: tm::TapeMove::Right, next_state: "q5"  },
        tm::State { name: "q5",  tape_symbol: ' ', write_symbol: '0', move_tape: tm::TapeMove::Left,  next_state: "q6"  },
        tm::State { name: "q5",  tape_symbol: '0', write_symbol: '0', move_tape: tm::TapeMove::Right, next_state: "q5"  },
        tm::State { name: "q6",  tape_symbol: '=', write_symbol: '=', move_tape: tm::TapeMove::Left,  next_state: "q7"  },
        tm::State { name: "q6",  tape_symbol: '0', write_symbol: '0', move_tape: tm::TapeMove::Left,  next_state: "q6"  },
        tm::State { name: "q7",  tape_symbol: '1', write_symbol: '1', move_tape: tm::TapeMove::Right, next_state: "q8"  },
        tm::State { name: "q7",  tape_symbol: '0', write_symbol: '0', move_tape: tm::TapeMove::Left,  next_state: "q7"  },
        tm::State { name: "q8",  tape_symbol: '=', write_symbol: '=', move_tape: tm::TapeMove::Left,  next_state: "q3"  },
        tm::State { name: "q8",  tape_symbol: '0', write_symbol: '1', move_tape: tm::TapeMove::Right, next_state: "q4"  },
        tm::State { name: "q9",  tape_symbol: 'x', write_symbol: 'x', move_tape: tm::TapeMove::Right, next_state: "q0"  },
        tm::State { name: "q9",  tape_symbol: '0', write_symbol: '0', move_tape: tm::TapeMove::Left,  next_state: "q9"  },
        tm::State { name: "q10", tape_symbol: 'x', write_symbol: '0', move_tape: tm::TapeMove::Left,  next_state: "q10" },
        tm::State { name: "q10", tape_symbol: ' ', write_symbol: ' ', move_tape: tm::TapeMove::Right, next_state: "q11" },
        tm::State { name: "q11", tape_symbol: '0', write_symbol: ' ', move_tape: tm::TapeMove::Right, next_state: "q11" },
        tm::State { name: "q11", tape_symbol: 'x', write_symbol: ' ', move_tape: tm::TapeMove::Right, next_state: "q11" },
        tm::State { name: "q11", tape_symbol: '=', write_symbol: ' ', move_tape: tm::TapeMove::Right, next_state: "q12" },
    ];

    let mut test = tm::TuringMachine::new(
        ' ',
        vec!('0', 'x', '='),
        vec!('0', '1', 'x', '='),
        &_states,
        "q0",
        vec!("q12"),
        "0000x00000="
    );

    test.run();

    println!("Tata!!!");
}

