//! This is a classical state machine implementation just using enum_dispatch instead of trait objects.
//! States are recreated and destroyed every time, which may save memory but requires extra book keeping
//! and may limit performance.


use enum_dispatch::*;

#[derive(Default)]
struct BlackBoard {
    menu_data: u32,
    game_data: u32,
}

#[enum_dispatch]
trait StateControl {
    fn enter_state(&mut self, data: &mut BlackBoard);
    fn update_state(&mut self) -> Option<State>;
    fn exit_state(&mut self, data: &mut BlackBoard);
}

#[enum_dispatch(StateControl)]
enum State {
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

impl StateControl for MenuState {
    fn enter_state(&mut self, data: &mut BlackBoard) {
        self.data = data.menu_data;
    }

    fn update_state(&mut self) -> Option<State> {
        self.data += 1;
        println!("Menu: {}", self.data);
        self.data.is_multiple_of(3).then(|| State::GameState(GameState::default()))
    }

    fn exit_state(&mut self, data: &mut BlackBoard) {
        data.menu_data = self.data;
    }
}

impl StateControl for GameState {
    fn enter_state(&mut self, data: &mut BlackBoard) {
        self.data = data.game_data;
    }

    fn update_state(&mut self) -> Option<State> {
        self.data += 1;
        println!("Game: {}", self.data);
        self.data.is_multiple_of(3).then(|| State::MenuState(MenuState::default()))
    }

    fn exit_state(&mut self, data: &mut BlackBoard) {
        data.game_data = self.data;
    }
}

struct StateMachine {
    game_state: State,
    black_board: BlackBoard,
}

impl StateMachine {
    fn new() -> Self {
        let mut game_state = MenuState::default();
        let mut black_board = BlackBoard::default();
        game_state.enter_state(&mut black_board);
        Self {
            game_state: State::MenuState(game_state),
            black_board,
        }
    }

    fn update(&mut self) {
        if let Some(new_state) = self.game_state.update_state() {
            self.game_state.exit_state(&mut self.black_board);
            self.game_state = new_state;
            self.game_state.enter_state(&mut self.black_board);
        }
    }
}

pub fn state_test() {
    let mut machine = StateMachine::new();
    for _ in 0..10 {
        machine.update();
    }
}
