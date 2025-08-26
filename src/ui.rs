// ui.rs - Sistema de UI optimizado

use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::game_state::GameData;

// Bitmap font 5x7 para caracteres ASCII básicos
const FONT_WIDTH: u32 = 5;
const FONT_HEIGHT: u32 = 7;

// Función para obtener bitmap de caracteres esenciales
fn get_char_bitmap(c: char) -> [u8; 35] {
    match c.to_ascii_uppercase() {
        'A' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1],
        'B' => [1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0],
        'C' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,1, 0,1,1,1,0],
        'D' => [1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0],
        'E' => [1,1,1,1,1, 1,0,0,0,0, 1,0,0,0,0, 1,1,1,1,0, 1,0,0,0,0, 1,0,0,0,0, 1,1,1,1,1],
        'F' => [1,1,1,1,1, 1,0,0,0,0, 1,0,0,0,0, 1,1,1,1,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0],
        'G' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,0, 1,0,1,1,1, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        'H' => [1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1],
        'I' => [1,1,1,1,1, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 1,1,1,1,1],
        'K' => [1,0,0,0,1, 1,0,0,1,0, 1,0,1,0,0, 1,1,0,0,0, 1,0,1,0,0, 1,0,0,1,0, 1,0,0,0,1],
        'L' => [1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0, 1,1,1,1,1],
        'M' => [1,0,0,0,1, 1,1,0,1,1, 1,0,1,0,1, 1,0,1,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1],
        'N' => [1,0,0,0,1, 1,1,0,0,1, 1,0,1,0,1, 1,0,1,0,1, 1,0,0,1,1, 1,0,0,0,1, 1,0,0,0,1],
        'O' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        'P' => [1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0, 1,0,0,0,0, 1,0,0,0,0, 1,0,0,0,0],
        'R' => [1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 1,1,1,1,0, 1,0,1,0,0, 1,0,0,1,0, 1,0,0,0,1],
        'S' => [0,1,1,1,1, 1,0,0,0,0, 1,0,0,0,0, 0,1,1,1,0, 0,0,0,0,1, 0,0,0,0,1, 1,1,1,1,0],
        'T' => [1,1,1,1,1, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0],
        'U' => [1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        'V' => [1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 0,1,0,1,0, 0,0,1,0,0],
        'W' => [1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 1,0,1,0,1, 1,0,1,0,1, 1,1,0,1,1, 1,0,0,0,1],
        'X' => [1,0,0,0,1, 1,0,0,0,1, 0,1,0,1,0, 0,0,1,0,0, 0,1,0,1,0, 1,0,0,0,1, 1,0,0,0,1],
        'Y' => [1,0,0,0,1, 1,0,0,0,1, 1,0,0,0,1, 0,1,0,1,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0],
        'Z' => [1,1,1,1,1, 0,0,0,0,1, 0,0,0,1,0, 0,0,1,0,0, 0,1,0,0,0, 1,0,0,0,0, 1,1,1,1,1],
        '0' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,1,1, 1,0,1,0,1, 1,1,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        '1' => [0,0,1,0,0, 0,1,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,1,1,1,0],
        '2' => [0,1,1,1,0, 1,0,0,0,1, 0,0,0,0,1, 0,0,0,1,0, 0,0,1,0,0, 0,1,0,0,0, 1,1,1,1,1],
        '3' => [1,1,1,1,1, 0,0,0,0,1, 0,0,0,1,0, 0,0,1,1,0, 0,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        '4' => [0,0,0,1,0, 0,0,1,1,0, 0,1,0,1,0, 1,0,0,1,0, 1,1,1,1,1, 0,0,0,1,0, 0,0,0,1,0],
        '5' => [1,1,1,1,1, 1,0,0,0,0, 1,1,1,1,0, 0,0,0,0,1, 0,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        '6' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,0, 1,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        '7' => [1,1,1,1,1, 0,0,0,0,1, 0,0,0,1,0, 0,0,1,0,0, 0,1,0,0,0, 0,1,0,0,0, 0,1,0,0,0],
        '8' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        '9' => [0,1,1,1,0, 1,0,0,0,1, 1,0,0,0,1, 0,1,1,1,1, 0,0,0,0,1, 1,0,0,0,1, 0,1,1,1,0],
        ':' => [0,0,0,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,0,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,0,0,0],
        '!' => [0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,1,0,0, 0,0,0,0,0, 0,0,1,0,0],
        '/' => [0,0,0,0,1, 0,0,0,1,0, 0,0,0,1,0, 0,0,1,0,0, 0,1,0,0,0, 0,1,0,0,0, 1,0,0,0,0],
        '-' => [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 1,1,1,1,1, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        '(' => [0,0,0,1,0, 0,0,1,0,0, 0,1,0,0,0, 0,1,0,0,0, 0,1,0,0,0, 0,0,1,0,0, 0,0,0,1,0],
        ')' => [0,1,0,0,0, 0,0,1,0,0, 0,0,0,1,0, 0,0,0,1,0, 0,0,0,1,0, 0,0,1,0,0, 0,1,0,0,0],
        ' ' => [0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        _ => [1,1,1,1,1, 1,0,0,0,1, 1,0,1,0,1, 1,0,0,0,1, 1,0,1,0,1, 1,0,0,0,1, 1,1,1,1,1],
    }
}

fn render_char(framebuffer: &mut Framebuffer, c: char, x: u32, y: u32) {
    let bitmap = get_char_bitmap(c);
    
    for row in 0..FONT_HEIGHT {
        for col in 0..FONT_WIDTH {
            let index = (row * FONT_WIDTH + col) as usize;
            if bitmap[index] == 1 {
                if x + col < framebuffer.width && y + row < framebuffer.height {
                    framebuffer.set_pixel(x + col, y + row);
                }
            }
        }
    }
}

pub fn render_fps(framebuffer: &mut Framebuffer, fps: f32) {
    framebuffer.set_current_color(Color::new(0, 0, 0, 180));
    for y in 5..25 {
        for x in 5..95 {
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.set_pixel(x, y);
            }
        }
    }
    
    let color = if fps >= 50.0 {
        Color::GREEN
    } else if fps >= 30.0 {
        Color::YELLOW
    } else {
        Color::RED
    };
    
    framebuffer.set_current_color(color);
    let fps_text = format!("FPS {:.0}", fps);
    render_text(framebuffer, &fps_text, 8, 10);
}

pub fn render_welcome_screen(framebuffer: &mut Framebuffer, window: &RaylibHandle) -> Option<usize> {
    render_gradient_background(
        framebuffer,
        Color::new(10, 15, 25, 255),
        Color::new(40, 30, 50, 255)
    );
    
    let center_x = framebuffer.width / 2;
    let center_y = framebuffer.height / 2;
    
    render_border_frame(
        framebuffer,
        50,
        80,
        framebuffer.width - 100,
        framebuffer.height - 160,
        Color::new(100, 100, 150, 255)
    );
    
    // Título principal
    framebuffer.set_current_color(Color::new(30, 30, 30, 255));
    render_text_centered(framebuffer, "THE BACKROOMS", 120, 4);
    render_text_centered(framebuffer, "ESCAPE", 170, 4);
    
    framebuffer.set_current_color(Color::new(220, 200, 100, 255));
    render_text_centered(framebuffer, "THE BACKROOMS", 117, 4);
    render_text_centered(framebuffer, "ESCAPE", 167, 4);
    
    framebuffer.set_current_color(Color::new(180, 180, 180, 255));
    render_text_centered(framebuffer, "UNA AVENTURA EN JUNTO A UN RAYCASTER 3D", 220, 1);
    
    // Menú de selección de nivel
    let menu_start_y = center_y - 50;
    render_border_frame(
        framebuffer,
        center_x - 200,
        menu_start_y - 20,
        400,
        160,
        Color::new(80, 80, 120, 255)
    );
    
    framebuffer.set_current_color(Color::new(200, 200, 250, 255));
    render_text_centered(framebuffer, "SELECCIONA EL NIVEL", menu_start_y - 5, 2);
    
    let level_colors = [
        Color::new(255, 215, 0, 255),
        Color::new(255, 69, 0, 255),
        Color::new(138, 43, 226, 255),
    ];
    
    let level_descriptions = [
        "1 - THE YELLOW HALLS     (PRINCIPIANTE)",
        "2 - THE RED CHAMBERS     (INTERMEDIO)", 
        "3 - THE FINAL ESCAPE     (AVANZADO)"
    ];
    
    for (i, (desc, color)) in level_descriptions.iter().zip(level_colors.iter()).enumerate() {
        framebuffer.set_current_color(*color);
        let level_y = menu_start_y + 40 + (i as u32 * 25);
        render_text_centered(framebuffer, desc, level_y, 1);
    }
    
    // Controles
    let controls_y = framebuffer.height - 180;
    render_border_frame(
        framebuffer,
        80,
        controls_y - 10,
        framebuffer.width - 160,
        140,
        Color::new(60, 60, 100, 255)
    );
    
    framebuffer.set_current_color(Color::new(150, 200, 255, 255));
    render_text_centered(framebuffer, "CONTROLES", controls_y + 5, 2);
    
    let controls = [
        "WASD / ARROWS - MOVERTE Y ROTAR",
        "MOUSE         - MIRAR ALREDEDOR",
        "R             - RESETEAR NIVEL",
        "",
        "JUNTA TODAS LAS MONEDAS Y DESBLOQUEA EL PORTAL!",
        "CUIDADO CON LAS TRAMPAS"
    ];
    
    framebuffer.set_current_color(Color::new(200, 200, 200, 255));
    for (i, control) in controls.iter().enumerate() {
        if !control.is_empty() {
            let control_y = controls_y + 35 + (i as u32 * 15);
            render_text_centered(framebuffer, control, control_y, 1);
        }
    }
    
    // Prompt de inicio con animación
    let animation_offset = ((window.get_time() * 3.0).sin() * 10.0) as i32;
    framebuffer.set_current_color(Color::new(100 + animation_offset.abs() as u8, 255, 100, 255));
    render_text_centered(framebuffer, "PRESIONA 1, 2 O 3 PARA ELEGIR UN NIVEL", framebuffer.height - 40, 1);
    
    // Detectar entrada de teclado
    if window.is_key_pressed(KeyboardKey::KEY_ONE) {
        return Some(1);
    } else if window.is_key_pressed(KeyboardKey::KEY_TWO) {
        return Some(2);
    } else if window.is_key_pressed(KeyboardKey::KEY_THREE) {
        return Some(3);
    }
    
    None
}

pub fn render_game_over(framebuffer: &mut Framebuffer, window: &RaylibHandle) -> bool {
    render_gradient_background(
        framebuffer,
        Color::new(60, 10, 10, 255),
        Color::new(30, 5, 5, 255)
    );
    
    let center_y = framebuffer.height / 2;
    
    render_border_frame(
        framebuffer,
        100,
        center_y - 100,
        framebuffer.width - 200,
        200,
        Color::new(150, 50, 50, 255)
    );
    
    // Texto de Game Over con efecto de sombra
    framebuffer.set_current_color(Color::new(50, 0, 0, 255));
    render_text_centered(framebuffer, "GAME OVER", center_y - 70, 4);
    
    framebuffer.set_current_color(Color::new(255, 100, 100, 255));
    render_text_centered(framebuffer, "GAME OVER", center_y - 73, 4);
    
    framebuffer.set_current_color(Color::new(200, 150, 150, 255));
    render_text_centered(framebuffer, "TE PERDISTE EN LOS BACKROMMS...", center_y - 20, 1);
    
    // Prompt animado para regresar al menú
    let animation_offset = ((window.get_time() * 4.0).sin() * 20.0) as i32;
    framebuffer.set_current_color(Color::new(255, 255 - animation_offset.abs() as u8, 255, 255));
    render_text_centered(framebuffer, "PRESIONA ESPACIO PARA VOLVER AL MENU", center_y + 20, 1);
    
    window.is_key_pressed(KeyboardKey::KEY_SPACE)
}

pub fn render_victory(framebuffer: &mut Framebuffer, window: &RaylibHandle, level: usize) -> bool {
    render_gradient_background(
        framebuffer,
        Color::new(10, 60, 10, 255),
        Color::new(5, 30, 5, 255)
    );
    
    let center_y = framebuffer.height / 2;
    
    render_border_frame(
        framebuffer,
        100,
        center_y - 120,
        framebuffer.width - 200,
        240,
        Color::new(50, 150, 50, 255)
    );
    
    // Texto de victoria con efecto de sombra
    framebuffer.set_current_color(Color::new(0, 50, 0, 255));
    render_text_centered(framebuffer, "NIVEL COMPLETADO!", center_y - 90, 3);
    
    framebuffer.set_current_color(Color::new(100, 255, 100, 255));
    render_text_centered(framebuffer, "NIVEL COMPLETADO!", center_y - 93, 3);
    
    framebuffer.set_current_color(Color::new(150, 255, 150, 255));
    if level < 3 {
        let level_text = format!("NIVEL {} COMPLETO!", level);
        render_text_centered(framebuffer, &level_text, center_y - 30, 2);
        render_text_centered(framebuffer, "LISTO PARA LO SIGUIENTE?", center_y - 5, 1);
        
        let animation_offset = ((window.get_time() * 4.0).sin() * 30.0) as i32;
        framebuffer.set_current_color(Color::new(255, 255, 100 + animation_offset.abs() as u8, 255));
        let next_level_text = format!("PRESIONA ESPACIO PARA INICIAL NIVEL {}", level + 1);
        render_text_centered(framebuffer, &next_level_text, center_y + 30, 1);
    } else {
        render_text_centered(framebuffer, "FELICIDADES!", center_y - 30, 2);
        render_text_centered(framebuffer, "ESCAPASTE DE LOS BACKROOMS!", center_y - 5, 1);
        render_text_centered(framebuffer, "ERES LIBRE!", center_y + 15, 1);
        
        let animation_offset = ((window.get_time() * 4.0).sin() * 30.0) as i32;
        framebuffer.set_current_color(Color::new(255, 255, 100 + animation_offset.abs() as u8, 255));
        render_text_centered(framebuffer, "PRESIONA ESPACIO PARA VOLVER A IR AL MENU", center_y + 45, 1);
    }
    
    window.is_key_pressed(KeyboardKey::KEY_SPACE)
}

pub fn render_hud(framebuffer: &mut Framebuffer, game_data: &GameData) {
    let hud_height = 120;
    let hud_width = 200;
    
    // Fondo del HUD
    framebuffer.set_current_color(Color::new(0, 0, 0, 150));
    for y in 5..hud_height {
        for x in 5..hud_width {
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.set_pixel(x, y);
            }
        }
    }
    
    render_border_frame(framebuffer, 5, 5, hud_width - 5, hud_height - 5, Color::new(100, 100, 150, 255));
    
    framebuffer.set_current_color(Color::WHITE);
    render_text(framebuffer, "ESTADO", 15, 15);
    
    // Vidas con color dinámico
    let lives_color = if game_data.lives > 2 {
        Color::GREEN
    } else if game_data.lives == 2 {
        Color::YELLOW
    } else {
        Color::RED
    };
    framebuffer.set_current_color(lives_color);
    let lives_text = format!("VIDAS: {}", game_data.lives);
    render_text(framebuffer, &lives_text, 15, 30);
    
    // Llaves con progreso
    let keys_color = if game_data.keys_collected >= game_data.keys_needed {
        Color::GOLD
    } else {
        Color::YELLOW
    };
    framebuffer.set_current_color(keys_color);
    let keys_text = format!("LLAVES: {}/{}", game_data.keys_collected, game_data.keys_needed);
    render_text(framebuffer, &keys_text, 15, 45);
    
    framebuffer.set_current_color(Color::CYAN);
    let level_text = format!("NIVEL: {}", game_data.current_level);
    render_text(framebuffer, &level_text, 15, 60);
    
    // Estado de la salida
    if game_data.has_key {
        framebuffer.set_current_color(Color::GREEN);
        render_text(framebuffer, "PUERTA DESBLOQUEADA!", 15, 75);
    } else {
        framebuffer.set_current_color(Color::ORANGE);
        render_text(framebuffer, "LLAVES ENCONTRADAS!", 15, 75);
    }
    
    // Checkpoints para niveles avanzados
    if game_data.current_level > 1 {
        let checkpoints_needed = game_data.current_level - 1;
        let checkpoints_color = if game_data.visited_checkpoints.len() >= checkpoints_needed {
            Color::PURPLE
        } else {
            Color::GRAY
        };
        framebuffer.set_current_color(checkpoints_color);
        let checkpoints_text = format!("CHECKPOINTS: {}/{}", 
            game_data.visited_checkpoints.len(), checkpoints_needed);
        render_text(framebuffer, &checkpoints_text, 15, 90);
    }
}

pub fn render_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
) {
    let minimap_size = 200;
    let minimap_x = framebuffer.width - minimap_size - 15;
    let minimap_y = 15;
    
    let maze_height = maze.len();
    let maze_width = if maze_height > 0 { maze[0].len() } else { 0 };
    
    if maze_width == 0 || maze_height == 0 {
        return;
    }
    
    let scale_x = (minimap_size - 20) as f32 / maze_width as f32;
    let scale_y = (minimap_size - 20) as f32 / maze_height as f32;
    let scale = scale_x.min(scale_y).max(2.0) as u32;
    
    let maze_pixel_width = maze_width as u32 * scale;
    let maze_pixel_height = maze_height as u32 * scale;
    let offset_x = (minimap_size - maze_pixel_width) / 2;
    let offset_y = (minimap_size - maze_pixel_height) / 2;
    
    // Fondo del minimapa
    framebuffer.set_current_color(Color::new(0, 0, 0, 200));
    for y in 0..minimap_size {
        for x in 0..minimap_size {
            if minimap_x + x < framebuffer.width && minimap_y + y < framebuffer.height {
                framebuffer.set_pixel(minimap_x + x, minimap_y + y);
            }
        }
    }
    
    render_border_frame(
        framebuffer, 
        minimap_x, 
        minimap_y, 
        minimap_size, 
        minimap_size, 
        Color::new(100, 100, 150, 255)
    );
    
    framebuffer.set_current_color(Color::WHITE);
    render_text(framebuffer, "MAPA", minimap_x + 5, minimap_y + 5);
    
    // Renderizar celdas del maze
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell != ' ' {
                let color = match cell {
                    '#' | '+' | '-' | '|' | 'r' | 'b' | 'g' => Color::new(180, 180, 120, 255),
                    'k' => Color::GOLD,
                    'e' => Color::GREEN,
                    't' => Color::RED,
                    'l' => Color::PURPLE,
                    'c' => Color::CYAN,
                    _ => Color::WHITE,
                };
                framebuffer.set_current_color(color);
                
                let cell_x = minimap_x + offset_x + (col_index as u32 * scale);
                let cell_y = minimap_y + offset_y + (row_index as u32 * scale);
                
                for dy in 0..scale {
                    for dx in 0..scale {
                        let px = cell_x + dx;
                        let py = cell_y + dy;
                        if px < framebuffer.width && py < framebuffer.height {
                            framebuffer.set_pixel(px, py);
                        }
                    }
                }
            }
        }
    }
    
    // Renderizar jugador
    let player_map_x = minimap_x + offset_x + ((player.pos.x / block_size as f32) * scale as f32) as u32;
    let player_map_y = minimap_y + offset_y + ((player.pos.y / block_size as f32) * scale as f32) as u32;
    
    framebuffer.set_current_color(Color::RED);
    let player_size = 3.max(scale / 3);
    for dy in 0..player_size {
        for dx in 0..player_size {
            let px = player_map_x + dx - player_size/2;
            let py = player_map_y + dy - player_size/2;
            if px < framebuffer.width && py < framebuffer.height {
                framebuffer.set_pixel(px, py);
            }
        }
    }
    
    // Renderizar dirección del jugador
    framebuffer.set_current_color(Color::BLUE);
    let direction_length = (scale * 2) as f32;
    let end_x = player_map_x as f32 + player.a.cos() * direction_length;
    let end_y = player_map_y as f32 + player.a.sin() * direction_length;
    
    for i in 0..((direction_length as u32).max(8)) {
        let t = i as f32 / direction_length;
        let px = (player_map_x as f32 + t * (end_x - player_map_x as f32)) as u32;
        let py = (player_map_y as f32 + t * (end_y - player_map_y as f32)) as u32;
        
        for dy in 0..2 {
            for dx in 0..2 {
                if px + dx < framebuffer.width && py + dy < framebuffer.height {
                    framebuffer.set_pixel(px + dx, py + dy);
                }
            }
        }
    }
}

pub fn render_text(framebuffer: &mut Framebuffer, text: &str, x: u32, y: u32) {
    for (i, c) in text.chars().enumerate() {
        let char_x = x + (i as u32 * (FONT_WIDTH + 1));
        if char_x < framebuffer.width {
            render_char(framebuffer, c, char_x, y);
        }
    }
}

fn render_text_with_scale(framebuffer: &mut Framebuffer, text: &str, x: u32, y: u32, scale: u32) {
    for (i, c) in text.chars().enumerate() {
        let bitmap = get_char_bitmap(c);
        let char_x = x + (i as u32 * (FONT_WIDTH * scale + 2));
        
        for row in 0..FONT_HEIGHT {
            for col in 0..FONT_WIDTH {
                let index = (row * FONT_WIDTH + col) as usize;
                if bitmap[index] == 1 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let px = char_x + col * scale + dx;
                            let py = y + row * scale + dy;
                            if px < framebuffer.width && py < framebuffer.height {
                                framebuffer.set_pixel(px, py);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_text_centered(framebuffer: &mut Framebuffer, text: &str, y: u32, scale: u32) {
    let text_width = text.len() as u32 * (FONT_WIDTH * scale + 2);
    let center_x = (framebuffer.width - text_width) / 2;
    render_text_with_scale(framebuffer, text, center_x, y, scale);
}

fn render_gradient_background(framebuffer: &mut Framebuffer, color1: Color, color2: Color) {
    for y in 0..framebuffer.height {
        let factor = y as f32 / framebuffer.height as f32;
        let r = (color1.r as f32 * (1.0 - factor) + color2.r as f32 * factor) as u8;
        let g = (color1.g as f32 * (1.0 - factor) + color2.g as f32 * factor) as u8;
        let b = (color1.b as f32 * (1.0 - factor) + color2.b as f32 * factor) as u8;
        
        framebuffer.set_current_color(Color::new(r, g, b, 255));
        for x in 0..framebuffer.width {
            framebuffer.set_pixel(x, y);
        }
    }
}

fn render_border_frame(framebuffer: &mut Framebuffer, x: u32, y: u32, width: u32, height: u32, color: Color) {
    framebuffer.set_current_color(color);
    
    // Líneas horizontales
    for px in x..=(x + width) {
        if px < framebuffer.width {
            if y < framebuffer.height {
                framebuffer.set_pixel(px, y);
            }
            if y + height < framebuffer.height {
                framebuffer.set_pixel(px, y + height);
            }
        }
    }
    
    // Líneas verticales
    for py in y..=(y + height) {
        if py < framebuffer.height {
            if x < framebuffer.width {
                framebuffer.set_pixel(x, py);
            }
            if x + width < framebuffer.width {
                framebuffer.set_pixel(x + width, py);
            }
        }
    }
}