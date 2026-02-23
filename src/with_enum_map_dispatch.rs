//! This method uses enum_dispatch instead of trait objects and saves the state in a vector. 
//! Saving the state is interesting to save time on construction on objects, think about a level state
//! and an ingame menu state. As in typical high level engine states one state is consuming the bulk of the 
//! memory anyway, adding the memory of all states up, does not play that much of a role.

use enum_dispatch::*;
use enum_map::*;

#[derive(Default)]
struct BlackBoard;

#[enum_dispatch]
trait StateControlDoubled {
    fn enter_state(&mut self, _data: &mut BlackBoard) {}
    fn update_state(&mut self) -> Option<StateIndex>;
    fn exit_state(&mut self, _data: &mut BlackBoard) {}
}

#[derive(Clone, Copy, Enum)]
enum StateIndex {
    MenuState,
    GameState,
}

#[enum_dispatch(StateControlDoubled)]
enum StateDoubled {
    MenuState(MenuState),
    GameState(GameState),
}

#[derive(Default)]
struct MenuState {
    data: u32,
}

#[derive(Default)]
struct GameState {
    data: u32,
}

impl StateControlDoubled for MenuState {
    fn update_state(&mut self) -> Option<StateIndex> {
        self.data += 1;
        println!("Menu: {}", self.data);
        self.data.is_multiple_of(3).then_some(StateIndex::GameState)
    }
}

impl StateControlDoubled for GameState {
    fn update_state(&mut self) -> Option<StateIndex> {
        self.data += 1;
        println!("Game: {}", self.data);
        self.data.is_multiple_of(3).then_some(StateIndex::MenuState)
    }
}

struct StateMachine {
    current: StateIndex,
    state_list: EnumMap<StateIndex, StateDoubled>,
    black_board: BlackBoard,
}

impl StateMachine {
    fn new() -> Self {
        let state_list = enum_map! {
            StateIndex::MenuState =>  StateDoubled::MenuState(MenuState::default()),
            StateIndex::GameState =>  StateDoubled::GameState(GameState::default()),
        };
        Self {
            current: StateIndex::GameState,
            state_list,
            black_board: BlackBoard,
        }
    }

    fn update(&mut self) {
        if let Some(new_state) = self.state_list[self.current].update_state() {
            self.state_list[self.current].exit_state(&mut self.black_board);
            self.current = new_state;
            self.state_list[self.current].enter_state(&mut self.black_board);
        }
    }
}

pub fn state_test() {
    let mut machine = StateMachine::new();
    for _ in 0..10 {
        machine.update();
    }
}
