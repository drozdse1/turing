use std::time::Duration;
use std::fmt::{Display, Formatter, Result};
use std::collections::HashSet;
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};
use std::{thread,time};
use log;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum TapeMove {
    // An `enum` may either be `unit-like`,
    Left,
    Right
}

impl Display for TapeMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TapeMove::Left => write!(f, "LEFT"),
            TapeMove::Right => write!(f, "RIGHT")
        }
    }
}

type Index = i32;

pub struct TuringMachine<T>{
    pub blank_symbol: T,
    pub input_symbols: Vec<T>,
    pub tape_symbols: Vec<T>,
    pub states: HashSet<State<T>>,
    pub current_state: &'static str,
    pub tape: HashMap<Index, T>,
    pub current_position: Index,
    pub final_states: Vec<&'static str>,
    waiting_ms: Duration,
    clean_screen: bool,
} 

impl<T> Hash for State<T> where T:Hash {
    fn hash<H: Hasher>(&self, state: &mut H)  { 
        self.tape_symbol.hash(state);
        self.name.hash(state);
    }
}

impl Display for State<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}

impl Display for TuringMachine<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.status(true, false))
    }
}

impl<T> PartialEq for State<T> where T:PartialEq {
    fn eq(&self, other: &State<T>) -> bool { 
        self.tape_symbol == other.tape_symbol && self.name == other.name 
    }
}

#[derive(Eq, Clone, Copy)]
pub struct State<T> {
    pub name: &'static str,
    pub tape_symbol: T,
    pub write_symbol: T,
    pub move_tape: TapeMove,
    pub next_state: &'static str 
}

impl TuringMachine<char> {
    pub fn init() -> TuringMachine<char> {
        let mut _states = HashSet::new();
        _states.insert(State {name: "q1", tape_symbol: '0', write_symbol: '1', move_tape: TapeMove::Left, next_state: "q2"});
        _states.insert(State {name: "q2", tape_symbol: '0', write_symbol: '1', move_tape: TapeMove::Right, next_state: "q2"});
        _states.insert(State {name: "q3", tape_symbol: '0', write_symbol: '1', move_tape: TapeMove::Left, next_state: "q3"});

        let _tm: TuringMachine<char> = Self {
            blank_symbol : '_',
            input_symbols : vec!('1', '0'),
            tape_symbols : vec!('0', '1', '_'),
            states: _states,
            current_state : "q1",
            tape : HashMap::new(),
            current_position: 0,
            final_states: vec!("q3"),
            waiting_ms: time::Duration::from_millis(50),
            clean_screen: true,
        };
        return _tm;  
    } 

    pub fn new(blank_symbol: char, input_symbols: Vec<char>, tape_symbols: Vec<char>, states: &[State<char>], inital_state: &'static str, final_states: Vec<&'static str>, initial_tape: &'static str) -> TuringMachine<char> { 
        let mut _states = HashSet::new();

        // build hashset based on array
        for state in states {
            _states.insert(*state);
        }

        let mut _tm: TuringMachine<char> = Self {
            blank_symbol : blank_symbol,
            input_symbols : input_symbols,
            tape_symbols : tape_symbols,
            states: _states,
            current_state : inital_state,
            tape : HashMap::new(),
            current_position: 0,
            final_states: final_states,
            waiting_ms: time::Duration::from_millis(50),
            clean_screen: true,
        };

        _tm.init_tape(initial_tape);
        
        return _tm;
    }

    pub fn status(&self, seperator: bool, detailed: bool) -> String {
        let seperator_char = "=";
        let mut result = String::from("");

        if seperator {
            result = result + &format!("{}\n", seperator_char.repeat(self.length() + 1));
        }

        result = result + &format!("{}\n", self.show_tape());
        if self.current_position < 0 {
            result = result + &format!("\n");
        } else {
            result = result + &format!("{}{}\n", String::from(" ").repeat(self.relative_position()), "^");
        }
        

        if detailed {
            result = result + &format!("State:'{}, Pos: '{}, Next State:{}, Relative Pos.{}'\n", self.current_state, self.current_position, self.transition(),self.relative_position());
        }
        if seperator {
            result = result + &format!("{}\n", seperator_char.repeat(self.length() + 1));
        }

        return result;
    }

    /* returns the length of the TM tape */
    pub fn length(&self) -> usize { 
        return (
            self.tape.keys()
                .max()
                .unwrap_or(&0)         
            - self.tape.keys()
                .min()
                .unwrap_or(&0)
        ) as usize;
     }


    fn init_tape(&mut self, tape_string: &str) {
        self.tape = HashMap::new();
        let mut _current_position = 0;

        for symbol in tape_string.chars() {
            if self.input_symbols.contains(&symbol) {
                self.tape.insert(_current_position, symbol);
                _current_position += 1;
            }
        }
    }

    pub fn move_head(&mut self, direction: TapeMove) {
        match direction {
            TapeMove::Left => self.current_position -= 1,
            TapeMove::Right => self.current_position += 1
        }
    }

    pub fn read_head(&self) -> char
    {
        match self.tape.get(&self.current_position) {
            Some(symbol) => *symbol,
            None => self.blank_symbol
        }
    }

    pub fn write_head(&mut self, symbol: char) {
        self.tape.insert(self.current_position, symbol);
    }

    fn step(&mut self) -> bool{
        let current_state = self.state(self.current_state, self.read_head());

        let next_state = self.transition();
        let write_symbol = current_state.write_symbol ;
        let tape_move = current_state.move_tape;



        // alter machine == begin
        self.current_state = next_state;
        self.write_head(write_symbol);
        self.move_head(tape_move);
        // alter machine == end

        if self.final_states.contains(&self.current_state) {
            return false;
        }

        log::debug!("Tape moved {} to State: [{}] and wrote Symbol [{}]", &tape_move, self.current_state, &write_symbol);

        if self.clean_screen {
            print!("{esc}c", esc = 27 as char);
        }

        print!("{}", self.status(true, true));

        return true;
    }

    fn state(&self, name: &'static str, tape_symbol: char) -> &State<char> {
         // todo: currently need to create an complete State object for indexing hashset 
        let state = self.states.get(&State{name: name, tape_symbol: tape_symbol, write_symbol: '0', move_tape: TapeMove::Left, next_state: "q3"});
        
        match state {
            Some(item) => {return item},
            None => panic!("State:'{}' not found for '{}'", name, tape_symbol),
        }

    }
    
    fn transition(&self) -> &'static str {
        let current_symbol = self.read_head();

        log::debug!("Current state: [{}]", self.current_state);
        log::debug!("Current symbol: [{}]", current_symbol);

        let state = self.state(self.current_state, current_symbol);

        return state.next_state;
    }

    pub fn print_tape_status(&self) {

    }

    pub fn run(&mut self) ->  &'static str {

        while self.step() {
            thread::sleep(self.waiting_ms);
        }

        return self.current_state;

    }

    pub fn show_tape(&self) -> String {
        let mut result = String::new();

        let max_pos = self.tape.keys()
                               .max()
                               .unwrap_or(&0);

        let min_pos = self.tape.keys()
                               .min()
                               .unwrap_or(&0);

        // iiterate through min to max position
        for pos in *min_pos ..= *max_pos {
            result.push(*self.tape.get(&pos).unwrap_or(&self.blank_symbol))
        } 

        return result;
    }

    pub fn relative_position(&self) -> usize {
        return (
            self.current_position    
            - self.tape.keys()
                .min()
                .unwrap_or(&0)
        ) as usize;
    }
}

#[cfg(test)]
mod tests {
use std::collections::HashSet;
use crate::tm::State;
use crate::tm::{TuringMachine, TapeMove};
    
    #[test]
    fn test1() {
        let mut test = TuringMachine::init();

        test.init_tape("0101");

        assert_eq!(test.tape.len(), 4);

        test.move_head(TapeMove::Left);

        test.write_head('0');

        test.move_head(TapeMove::Left);

        test.write_head('0');

        assert_eq!(test.tape.len(), 6);

        assert_eq!(test.relative_position(), 0);

        assert_eq!(test.current_position, -2);
    }

    #[test]
    fn test2() {
        let _states = [
            State {name: "q1", tape_symbol: '_', write_symbol: '0', move_tape: TapeMove::Right, next_state: "q2"}
         ];

        let mut test = TuringMachine::new(
            '_',
            vec!('1', '0'),
            vec!('0', '1', '_'),
            &_states,
            "q1",
            vec!("q1"),
            "");

        assert_eq!(test.read_head(), '_');
        test.write_head('0');
        test.move_head(TapeMove::Left);
        assert_eq!(test.read_head(), '_');
        test.write_head('1');
        test.move_head(TapeMove::Left);
        assert_eq!(test.read_head(), '_');
        test.write_head('0');
        test.move_head(TapeMove::Left);

        assert_eq!(test.current_position, -3);

        test.move_head(TapeMove::Right);
        assert_eq!(test.read_head(), '0');

        test.move_head(TapeMove::Right);
        assert_eq!(test.read_head(), '1');

        test.move_head(TapeMove::Right);
        assert_eq!(test.read_head(), '0');
        
        assert_eq!(test.current_position, 0);
        
        assert_eq!(test.relative_position(), 2);
        
    }


    #[test]
    fn simple_check() {
        let mut test = TuringMachine::init();

        test.init_tape("0101");
        
        assert_eq!(test.show_tape(), String::from("0101"))
    }

    #[test]
    fn another_simple_check() {
        let mut test = TuringMachine::init();

        test.write_head('0');
        test.move_head(TapeMove::Left);
        test.write_head(' ');
        test.move_head(TapeMove::Left);
        test.write_head(' ');
        test.move_head(TapeMove::Left);
        test.write_head('1');
        test.move_head(TapeMove::Right);
        test.write_head('1');
       
        assert_eq!(test.show_tape(), String::from("11 0"))
    }

      
    #[test]
    fn check_if_states_are_distinct() {
        let mut test = TuringMachine::init();

        // should add new state
        test.states.insert(State {name: "q1", tape_symbol: '1', write_symbol: '1', move_tape: TapeMove::Left, next_state: "q2"});

        assert_eq!(test.states.len(), 4);
        
        // should override existing state
        test.states.insert(State {name: "q1", tape_symbol: '0', write_symbol: '0', move_tape: TapeMove::Right, next_state: "q9"});

        assert_eq!(test.states.len(), 4);
    }

    #[test]
    fn test_transition() {

        let _states = [
            State {name: "q1", tape_symbol: '_', write_symbol: '0', move_tape: TapeMove::Right, next_state: "q2"}, 
            State {name: "q2", tape_symbol: '_', write_symbol: '0', move_tape: TapeMove::Right, next_state: "q3"},
            State {name: "q3", tape_symbol: '_', write_symbol: '0', move_tape: TapeMove::Right, next_state: "q4"}
        ];

        let mut test = TuringMachine::new(
            '_',
            vec!('1', '0'),
            vec!('0', '1', '_'),
            &_states,
            "q1",
            vec!("q4"),
            "_");

        assert_eq!(
            test.states.contains(&crate::tm::State{name: "q1", tape_symbol: '_', write_symbol: '0', move_tape: TapeMove::Left, next_state: "q3"}), true);

        test.step();

        assert_eq!(test.current_position, 1);
        assert_eq!(test.current_state, "q2");

        test.step();

        assert_eq!(test.current_position, 2);
        assert_eq!(test.current_state, "q3");

        test.step();
        
        assert_eq!(test.current_position, 3);
        assert_eq!(test.current_state, "q4");
 
    }
}
