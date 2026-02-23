//! This is the schoolbook approach for a state engine, with trait objects and new generation of states 
//! after every transition. For games this may be disadvantageous because of the performance overhead that comes
//! with the new construction of every game state.


#[derive(Default)]
struct BlackBoard {
    menu_data: u32,
    game_data: u32,
}

trait StateControl {
    fn enter_state(&mut self, data: &mut BlackBoard);
    fn update_state(&mut self) -> Option<Box<dyn StateControl>>;
    fn exit_state(&mut self, data: &mut BlackBoard);
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

    fn update_state(&mut self) -> Option<Box<dyn StateControl>> {
        self.data += 1;
        println!("Menu: {}", self.data);
        self.data.is_multiple_of(3).then(|| Box::new(GameState::default()) as Box<dyn StateControl>)
    }

    fn exit_state(&mut self, data: &mut BlackBoard) {
        data.menu_data = self.data;
    }
}

impl StateControl for GameState {
    fn enter_state(&mut self, data: &mut BlackBoard) {
        self.data = data.game_data;
    }

    fn update_state(&mut self) -> Option<Box<dyn StateControl>> {
        self.data += 1;
        println!("Game: {}", self.data);
        self.data.is_multiple_of(3).then(|| Box::new(MenuState::default()) as Box<dyn StateControl>)
    }

    fn exit_state(&mut self, data: &mut BlackBoard) {
        data.game_data = self.data;
    }
}

struct StateMachine {
    game_state: Box<dyn StateControl>,
    black_board: BlackBoard,
}

impl StateMachine {
    fn new() -> Self {
        let mut game_state = MenuState::default();
        let mut black_board = BlackBoard::default();
        game_state.enter_state(&mut black_board);
        Self {
            game_state: Box::new(game_state),
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
