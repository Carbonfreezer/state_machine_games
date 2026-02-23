//! This is the minimalistic way of implementing a state machine in games, which is perfect for small states within a game object
//! like simply is animating, transitioning fighting etc.

#[derive(Clone, Copy,Default)]
enum StateIndex {
    #[default]
    MenuState,
    GameState,
}

#[derive(Default)]
struct StateMachine {
    state: StateIndex,
    menu_data : u32,
    game_data : u32,
}


impl StateMachine {
    fn update(&mut self) {
        use StateIndex::*;
        match self.state {
            MenuState => {
                self.menu_data += 1;
                println!("Menu: {}", self.menu_data);
                if self.menu_data.is_multiple_of(3) {
                    self.state = GameState;
                }
            }
            GameState => {
                self.game_data += 1;
                println!("Game: {}", self.game_data);
                if self.game_data.is_multiple_of(3) {
                    self.state = MenuState;
                }
            }
        }
    }
}

pub fn state_test() {
    let mut machine = StateMachine::default();
    for _ in 0..10 {
        machine.update();
    }
}