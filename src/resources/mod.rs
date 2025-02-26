// This file will manage shared resources for the game

pub mod game_state {
    #[allow(dead_code)]
    pub struct GameState {
        pub current_level: usize,
        pub score: usize,
    }
}

pub mod config {
    #[allow(dead_code)]
    pub struct Config {
        pub window_title: String,
        pub window_width: u32,
        pub window_height: u32,
    }
}
