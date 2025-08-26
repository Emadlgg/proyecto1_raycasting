// game_state.rs - Sistema de estado de juego optimizado

use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::{Maze, load_maze};
use crate::player::Player;
use crate::audio::{AudioManager, GameAudioEvent};
use crate::notification::{NotificationManager};

#[derive(Clone, PartialEq)]
pub enum GameMode {
    Welcome,
    Playing,
    GameOver,
    Victory,
}

#[derive(Clone)]
pub struct GameData {
    pub maze: Maze,
    pub player: Player,
    pub current_level: usize,
    pub lives: i32,
    pub has_key: bool,
    pub keys_collected: i32,
    pub keys_needed: i32,
    pub visited_checkpoints: Vec<(usize, usize)>,
    pub animation_time: f32,
    pub notification_manager: NotificationManager,
}

pub struct GameState {
    pub mode: GameMode,
    pub data: GameData,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            mode: GameMode::Welcome,
            data: GameData {
                maze: vec![],
                player: Player::new(),
                current_level: 1,
                lives: 3,
                has_key: false,
                keys_collected: 0,
                keys_needed: 1,
                visited_checkpoints: vec![],
                animation_time: 0.0,
                notification_manager: NotificationManager::new(),
            },
        }
    }

    pub fn start_game(&mut self, level: usize) {
        self.mode = GameMode::Playing;
        self.load_level(level);
    }

    pub fn load_level(&mut self, level: usize) {
        let level_file = match level {
            1 => "assets/levels/level1.txt",
            2 => "assets/levels/level2.txt", 
            3 => "assets/levels/level3.txt",
            _ => "assets/levels/level1.txt",
        };
        
        self.data.maze = load_maze(level_file);
        
        if self.data.maze.is_empty() || self.data.maze.iter().any(|row| row.is_empty()) {
            self.data.maze = self.create_emergency_maze();
        }
        
        self.data.current_level = level;
        self.data.has_key = false;
        self.data.keys_collected = 0;
        self.data.visited_checkpoints.clear();
        
        self.data.keys_needed = match level {
            1 => 1,
            2 => 2,
            3 => 3,
            _ => 1,
        };

        self.set_player_start_position(level);
        self.validate_player_position();
    }

    fn create_emergency_maze(&self) -> Maze {
        vec![
            "###################".chars().collect(),
            "#                 #".chars().collect(),
            "# ############### #".chars().collect(),
            "#                 #".chars().collect(),
            "# ####### # # ####".chars().collect(),
            "# #     # # #   #".chars().collect(),
            "# # ### # # ### #".chars().collect(),
            "# #   # # #   # #".chars().collect(),
            "# ### # # ### # #".chars().collect(),
            "# #k  # #   # # #".chars().collect(),
            "# ##### ### # # #".chars().collect(),
            "#           # # #".chars().collect(),
            "############# # #".chars().collect(),
            "#             # #".chars().collect(),
            "#           l  ##".chars().collect(),
            "##############e##".chars().collect(),
        ]
    }

    fn set_player_start_position(&mut self, level: usize) {
        let spawn_pos = self.find_spawn_position();
        
        let (start_pos, start_angle) = if let Some((x, y)) = spawn_pos {
            (Vector2::new(x, y), PI / 4.0)
        } else {
            match level {
                1 => (Vector2::new(150.0, 150.0), PI / 4.0),
                2 => (Vector2::new(150.0, 150.0), 0.0),
                3 => (Vector2::new(150.0, 150.0), PI),
                _ => (Vector2::new(150.0, 150.0), PI / 4.0),
            }
        };

        self.data.player = Player {
            pos: start_pos,
            a: start_angle,
            fov: PI / 3.0,
            radius: 20.0,
        };

        if !self.is_position_safe(start_pos.x, start_pos.y) {
            self.find_safe_start_position();
        }
    }

    fn find_spawn_position(&self) -> Option<(f32, f32)> {
        if self.data.maze.is_empty() {
            return None;
        }
        
        for (y, row) in self.data.maze.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == ' ' {
                    let world_x = (x as f32 * 100.0) + 50.0;
                    let world_y = (y as f32 * 100.0) + 50.0;
                    
                    if self.has_spawn_clearance(x, y) {
                        return Some((world_x, world_y));
                    }
                }
            }
        }
        
        None
    }

    fn has_spawn_clearance(&self, x: usize, y: usize) -> bool {
        if self.data.maze.is_empty() {
            return false;
        }
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                let check_x = x as i32 + dx;
                let check_y = y as i32 + dy;
                
                if check_x >= 0 && check_y >= 0 {
                    let check_x = check_x as usize;
                    let check_y = check_y as usize;
                    
                    if check_y < self.data.maze.len() && check_x < self.data.maze[check_y].len() {
                        let cell = self.data.maze[check_y][check_x];
                        if matches!(cell, '#' | '+' | '-' | '|' | 'r' | 'b' | 'g') {
                            return false;
                        }
                    }
                }
            }
        }
        
        true
    }

    fn find_safe_start_position(&mut self) {
        for (y, row) in self.data.maze.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == ' ' {
                    let world_x = (x as f32 * 100.0) + 50.0;
                    let world_y = (y as f32 * 100.0) + 50.0;
                    
                    if self.is_position_safe(world_x, world_y) {
                        self.data.player.pos = Vector2::new(world_x, world_y);
                        return;
                    }
                }
            }
        }
        
        self.data.player.pos = Vector2::new(150.0, 150.0);
    }

    pub fn next_level(&mut self) {
        if self.data.current_level < 3 {
            let next_level = self.data.current_level + 1;
            self.load_level(next_level);
            self.mode = GameMode::Playing;
        } else {
            self.mode = GameMode::Victory;
        }
    }

    pub fn reset(&mut self) {
        self.mode = GameMode::Welcome;
        self.data.lives = 3;
        self.data.current_level = 1;
        self.data.has_key = false;
        self.data.keys_collected = 0;
        self.data.visited_checkpoints.clear();
    }

    pub fn update_with_audio(&mut self, audio_manager: &mut AudioManager, block_size: usize) {
        self.data.animation_time += 0.1;
        self.data.notification_manager.update(0.016);

        if self.data.maze.is_empty() {
            return;
        }

        self.check_player_interactions_with_notifications(audio_manager, block_size);
        
        if self.check_win_condition_safe() {
            self.mode = GameMode::Victory;
            audio_manager.play_game_event(GameAudioEvent::LevelComplete);
        }

        if self.data.lives <= 0 {
            self.mode = GameMode::GameOver;
        }
    }

    fn check_player_interactions_with_notifications(&mut self, audio_manager: &mut AudioManager, block_size: usize) {
        let player_x = self.data.player.pos.x;
        let player_y = self.data.player.pos.y;
        
        if player_x < 0.0 || player_y < 0.0 || self.data.maze.is_empty() {
            return;
        }

        let player_grid_x = (player_x / block_size as f32) as usize;
        let player_grid_y = (player_y / block_size as f32) as usize;

        if player_grid_y >= self.data.maze.len() {
            return;
        }

        let row_len = self.data.maze.get(player_grid_y).map(|r| r.len()).unwrap_or(0);
        if player_grid_x >= row_len {
            return;
        }

        let current_cell = self.data.maze[player_grid_y][player_grid_x];

        match current_cell {
            'k' => {
                self.data.maze[player_grid_y][player_grid_x] = ' ';
                self.data.keys_collected += 1;
                if self.data.keys_collected >= self.data.keys_needed {
                    self.data.has_key = true;
                }
                audio_manager.play_game_event(GameAudioEvent::KeyPickup);
                self.data.notification_manager.show_key_collected(self.data.keys_collected, self.data.keys_needed);
            },
            't' => {
                self.data.maze[player_grid_y][player_grid_x] = ' ';
                if self.data.lives > 0 {
                    self.data.lives -= 1;
                }
                
                audio_manager.play_game_event(GameAudioEvent::TrapTriggered);
                audio_manager.play_game_event(GameAudioEvent::PlayerHurt);
                
                self.data.notification_manager.show_life_lost(self.data.lives);
                self.data.notification_manager.show_trap_activated();
                
                self.data.player.pos.x -= 10.0;
                self.data.player.pos.y -= 10.0;
                
                if self.data.player.pos.x < 50.0 {
                    self.data.player.pos.x = 50.0;
                }
                if self.data.player.pos.y < 50.0 {
                    self.data.player.pos.y = 50.0;
                }
            },
            'l' => {
                self.data.maze[player_grid_y][player_grid_x] = ' ';
                self.data.lives += 1;
                audio_manager.play_game_event(GameAudioEvent::KeyPickup);
                self.data.notification_manager.show_extra_life(self.data.lives);
            },
            'c' => {
                let checkpoint = (player_grid_x, player_grid_y);
                if !self.data.visited_checkpoints.contains(&checkpoint) {
                    self.data.visited_checkpoints.push(checkpoint);
                    audio_manager.play_game_event(GameAudioEvent::CheckpointReached);
                    self.data.notification_manager.show_checkpoint_reached(
                        self.data.visited_checkpoints.len(), 
                        self.data.current_level
                    );
                }
            },
            'e' => {
                if !self.data.has_key {
                    self.data.notification_manager.show_exit_blocked("no_key");
                } else {
                    let checkpoints_needed = match self.data.current_level {
                        1 => 0,
                        2 => 1, 
                        3 => 2,
                        _ => 0,
                    };

                    if self.data.visited_checkpoints.len() < checkpoints_needed {
                        self.data.notification_manager.show_exit_blocked("no_checkpoints");
                    }
                }
            },
            _ => {}
        }
    }

    pub fn clear_notifications(&mut self) {
        self.data.notification_manager.clear_all();
    }

    pub fn update(&mut self, audio_manager: &mut AudioManager, block_size: usize) {
        self.data.animation_time += 0.1;

        if self.data.maze.is_empty() {
            return;
        }

        self.check_player_interactions_simple(audio_manager, block_size);
        
        if self.check_win_condition_safe() {
            self.mode = GameMode::Victory;
        }

        if self.data.lives <= 0 {
            self.mode = GameMode::GameOver;
        }
    }

    fn check_player_interactions_simple(&mut self, _audio_manager: &mut AudioManager, block_size: usize) {
        let player_x = self.data.player.pos.x;
        let player_y = self.data.player.pos.y;
        
        if player_x < 0.0 || player_y < 0.0 || self.data.maze.is_empty() {
            return;
        }

        let player_grid_x = (player_x / block_size as f32) as usize;
        let player_grid_y = (player_y / block_size as f32) as usize;

        if player_grid_y >= self.data.maze.len() {
            return;
        }

        let row_len = self.data.maze.get(player_grid_y).map(|r| r.len()).unwrap_or(0);
        if player_grid_x >= row_len {
            return;
        }

        let current_cell = self.data.maze[player_grid_y][player_grid_x];

        match current_cell {
            'k' => {
                self.data.maze[player_grid_y][player_grid_x] = ' ';
                self.data.keys_collected += 1;
                if self.data.keys_collected >= self.data.keys_needed {
                    self.data.has_key = true;
                }
            },
            't' => {
                self.data.maze[player_grid_y][player_grid_x] = ' ';
                if self.data.lives > 0 {
                    self.data.lives -= 1;
                }
                
                self.data.player.pos.x -= 10.0;
                self.data.player.pos.y -= 10.0;
                
                if self.data.player.pos.x < 50.0 {
                    self.data.player.pos.x = 50.0;
                }
                if self.data.player.pos.y < 50.0 {
                    self.data.player.pos.y = 50.0;
                }
            },
            'l' => {
                self.data.maze[player_grid_y][player_grid_x] = ' ';
                self.data.lives += 1;
            },
            'c' => {
                let checkpoint = (player_grid_x, player_grid_y);
                if !self.data.visited_checkpoints.contains(&checkpoint) {
                    self.data.visited_checkpoints.push(checkpoint);
                }
            },
            _ => {}
        }
    }

    fn is_position_safe(&self, x: f32, y: f32) -> bool {
        if x < 0.0 || y < 0.0 {
            return false;
        }

        if self.data.maze.is_empty() {
            return false;
        }

        let grid_x = (x / 100.0) as usize;
        let grid_y = (y / 100.0) as usize;
        
        if grid_y >= self.data.maze.len() {
            return false;
        }
        
        if let Some(row) = self.data.maze.get(grid_y) {
            if grid_x >= row.len() {
                return false;
            }
            
            let cell = row[grid_x];
            matches!(cell, ' ' | 'k' | 't' | 'l' | 'c' | 'e')
        } else {
            false
        }
    }

    pub fn can_player_move_to(&self, x: f32, y: f32, _block_size: usize) -> bool {
        let radius = self.data.player.radius;
        let points = [
            (x, y),
            (x + radius, y),
            (x - radius, y),
            (x, y + radius),
            (x, y - radius),
            (x + radius * 0.7, y + radius * 0.7),
            (x - radius * 0.7, y - radius * 0.7),
            (x + radius * 0.7, y - radius * 0.7),
            (x - radius * 0.7, y + radius * 0.7),
        ];

        points.iter().all(|&(px, py)| {
            self.is_position_safe(px, py)
        })
    }

    fn check_win_condition_safe(&self) -> bool {
        let player_x = self.data.player.pos.x;
        let player_y = self.data.player.pos.y;
        
        if player_x < 0.0 || player_y < 0.0 {
            return false;
        }

        if self.data.maze.is_empty() {
            return false;
        }

        let player_grid_x = (player_x / 100.0) as usize;
        let player_grid_y = (player_y / 100.0) as usize;

        if player_grid_y >= self.data.maze.len() {
            return false;
        }

        if let Some(row) = self.data.maze.get(player_grid_y) {
            if player_grid_x >= row.len() {
                return false;
            }

            let current_cell = row[player_grid_x];
            
            if current_cell == 'e' {
                let checkpoints_needed = match self.data.current_level {
                    1 => 0,
                    2 => 1,
                    3 => 2,
                    _ => 0,
                };

                let has_required_checkpoints = self.data.visited_checkpoints.len() >= checkpoints_needed;
                return self.data.has_key && has_required_checkpoints;
            }
        }
        
        false
    }

    pub fn emergency_reset(&mut self) {
        self.load_level(self.data.current_level);
    }

    fn validate_player_position(&mut self) {
        let player_pos = self.data.player.pos;
        if !self.can_move_to_safe(player_pos.x, player_pos.y, self.data.player.radius) {
            if let Some((safe_x, safe_y)) = self.find_nearest_safe_position(player_pos.x, player_pos.y, self.data.player.radius) {
                self.data.player.pos.x = safe_x;
                self.data.player.pos.y = safe_y;
            } else {
                self.data.player.pos.x = 150.0;
                self.data.player.pos.y = 150.0;
            }
        }
    }

    fn can_move_to_safe(&self, x: f32, y: f32, radius: f32) -> bool {
        let points = [
            (x, y),
            (x + radius, y),
            (x - radius, y),
            (x, y + radius),
            (x, y - radius),
            (x + radius * 0.7, y + radius * 0.7),
            (x - radius * 0.7, y - radius * 0.7),
            (x + radius * 0.7, y - radius * 0.7),
            (x - radius * 0.7, y + radius * 0.7),
        ];

        points.iter().all(|&(px, py)| {
            self.is_position_safe(px, py)
        })
    }

    fn find_nearest_safe_position(&self, x: f32, y: f32, radius: f32) -> Option<(f32, f32)> {
        if self.data.maze.is_empty() {
            return None;
        }

        let start_grid_x = (x / 100.0) as usize;
        let start_grid_y = (y / 100.0) as usize;
        
        for search_radius in 1..=5 {
            for dy in -(search_radius as i32)..=(search_radius as i32) {
                for dx in -(search_radius as i32)..=(search_radius as i32) {
                    let check_x = start_grid_x as i32 + dx;
                    let check_y = start_grid_y as i32 + dy;
                    
                    if check_x >= 0 && check_y >= 0 {
                        let world_x = check_x as f32 * 100.0 + 50.0;
                        let world_y = check_y as f32 * 100.0 + 50.0;
                        
                        if self.can_move_to_safe(world_x, world_y, radius) {
                            return Some((world_x, world_y));
                        }
                    }
                }
            }
        }
        
        None
    }
}