// player.rs - Sistema de jugador 

use raylib::prelude::*;
use std::f32::consts::PI;
use crate::maze::Maze;

#[derive(Clone)]
pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
    pub radius: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vector2::new(150.0, 150.0),
            a: PI / 3.0,
            fov: PI / 3.0,
            radius: 20.0,
        }
    }

    pub fn new_with_pos(x: f32, y: f32, angle: f32) -> Self {
        Player {
            pos: Vector2::new(x, y),
            a: angle,
            fov: PI / 3.0,
            radius: 20.0,
        }
    }
}

// Función principal de procesamiento de eventos con validación de colisiones
pub fn process_events_with_maze_safe(
    player: &mut Player, 
    rl: &RaylibHandle, 
    maze: &Maze, 
    block_size: usize
) {
    const MOVE_SPEED: f32 = 18.0;
    const ROTATION_SPEED: f32 = PI / 18.0;

    // Rotación con teclas
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_E) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_Q) {
        player.a -= ROTATION_SPEED;
    }

    // Normalizar ángulo
    while player.a < 0.0 {
        player.a += 2.0 * PI;
    }
    while player.a >= 2.0 * PI {
        player.a -= 2.0 * PI;
    }

    // Movimiento hacia adelante
    if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();
        
        if can_move_to_safe(new_x, new_y, maze, block_size, player.radius) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        } else {
            // Intentar movimiento deslizante (sliding)
            if can_move_to_safe(new_x, player.pos.y, maze, block_size, player.radius) {
                player.pos.x = new_x;
            } else if can_move_to_safe(player.pos.x, new_y, maze, block_size, player.radius) {
                player.pos.y = new_y;
            }
        }
    }

    // Movimiento hacia atrás
    if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();
        
        if can_move_to_safe(new_x, new_y, maze, block_size, player.radius) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        } else {
            // Intentar movimiento deslizante
            if can_move_to_safe(new_x, player.pos.y, maze, block_size, player.radius) {
                player.pos.x = new_x;
            } else if can_move_to_safe(player.pos.x, new_y, maze, block_size, player.radius) {
                player.pos.y = new_y;
            }
        }
    }

    // Movimiento lateral (strafe)
    if rl.is_key_down(KeyboardKey::KEY_D) {
        let strafe_angle = player.a + PI / 2.0;
        let new_x = player.pos.x + MOVE_SPEED * 0.7 * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * 0.7 * strafe_angle.sin();
        
        if can_move_to_safe(new_x, new_y, maze, block_size, player.radius) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
    
    if rl.is_key_down(KeyboardKey::KEY_A) {
        let strafe_angle = player.a - PI / 2.0;
        let new_x = player.pos.x + MOVE_SPEED * 0.7 * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * 0.7 * strafe_angle.sin();
        
        if can_move_to_safe(new_x, new_y, maze, block_size, player.radius) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
}

// Función de colisión segura que verifica múltiples puntos del jugador
fn can_move_to_safe(x: f32, y: f32, maze: &Maze, block_size: usize, radius: f32) -> bool {
    // Verificar que las coordenadas estén dentro de rangos razonables
    if x < 0.0 || y < 0.0 || x > 10000.0 || y > 10000.0 {
        return false;
    }

    if maze.is_empty() {
        return false;
    }

    // Verificar múltiples puntos del jugador para colisión más precisa
    let points = [
        (x, y),                                    // Centro
        (x + radius, y),                           // Derecha
        (x - radius, y),                           // Izquierda  
        (x, y + radius),                           // Abajo
        (x, y - radius),                           // Arriba
        (x + radius * 0.7, y + radius * 0.7),     // Diagonal SE
        (x - radius * 0.7, y - radius * 0.7),     // Diagonal NW
        (x + radius * 0.7, y - radius * 0.7),     // Diagonal NE
        (x - radius * 0.7, y + radius * 0.7),     // Diagonal SW
    ];

    // Todos los puntos deben ser válidos para permitir el movimiento
    points.iter().all(|&(px, py)| {
        is_position_walkable_safe(px, py, maze, block_size)
    })
}

// Verifica si una posición específica es caminable
fn is_position_walkable_safe(x: f32, y: f32, maze: &Maze, block_size: usize) -> bool {
    if x < 0.0 || y < 0.0 {
        return false;
    }

    let grid_x = (x / block_size as f32) as usize;
    let grid_y = (y / block_size as f32) as usize;

    if grid_y >= maze.len() {
        return false;
    }
    
    if let Some(row) = maze.get(grid_y) {
        if grid_x >= row.len() {
            return false;
        }
        
        let cell = row[grid_x];
        
        // Las trampas son técnicamente caminables, pero peligrosas
        if cell == 't' {
            return true;
        }
        
        // Permitir movimiento a través de espacios vacíos y elementos interactivos
        matches!(cell, ' ' | 'k' | 't' | 'l' | 'c' | 'e')
    } else {
        false
    }
}

// Función para obtener el contenido de una celda de forma segura
pub fn get_cell_safe(x: f32, y: f32, maze: &Maze, block_size: usize) -> char {
    if x < 0.0 || y < 0.0 {
        return '#';
    }

    if maze.is_empty() {
        return '#';
    }

    let grid_x = (x / block_size as f32) as usize;
    let grid_y = (y / block_size as f32) as usize;

    if grid_y >= maze.len() {
        return '#';
    }

    if let Some(row) = maze.get(grid_y) {
        if grid_x >= row.len() {
            '#'
        } else {
            row[grid_x]
        }
    } else {
        '#'
    }
}

// Función de entrada de mouse mejorada
pub fn process_mouse_input_safe(player: &mut Player, mouse_delta: f32) {
    const MOUSE_SENSITIVITY: f32 = 0.002;
    player.a -= mouse_delta * MOUSE_SENSITIVITY;
    
    // Normalizar ángulo
    while player.a < 0.0 {
        player.a += 2.0 * PI;
    }
    while player.a >= 2.0 * PI {
        player.a -= 2.0 * PI;
    }
}

// Funciones de compatibilidad
pub fn process_events_with_maze(player: &mut Player, rl: &RaylibHandle, maze: &Maze, block_size: usize) {
    process_events_with_maze_safe(player, rl, maze, block_size);
}

pub fn process_mouse_input(player: &mut Player, mouse_delta: f32) {
    process_mouse_input_safe(player, mouse_delta);
}

pub fn can_move_to_with_maze(x: f32, y: f32, maze: &Maze, block_size: usize) -> bool {
    can_move_to_safe(x, y, maze, block_size, 20.0)
}

// Función legacy para process_events
pub fn process_events(player: &mut Player, rl: &RaylibHandle, game_state: &crate::game_state::GameState, block_size: usize) {
    process_events_with_maze_safe(player, rl, &game_state.data.maze, block_size);
}