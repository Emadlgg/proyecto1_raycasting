// main.rs - Sistema principal optimizado

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;
mod game_state;
mod audio;
mod texture_manager;
mod sprite_manager;
mod ui;
mod notification;
mod collision;

use framebuffer::Framebuffer;
use player::{process_events_with_maze_safe, process_mouse_input_safe};
use game_state::{GameState, GameMode};
use audio::{AudioManager, GameAudioEvent, MusicType};
use texture_manager::TextureManager;
use sprite_manager::SpriteManager;
use ui::{render_fps, render_minimap, render_welcome_screen, render_game_over, render_victory};
use caster::{render_world_with_sprites};

use raylib::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("The Backrooms Escape - PROYECTO 1")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::new(20, 20, 30, 255));

    let mut game_state = GameState::new();
    
    // Inicializar sistemas de renderizado
    let texture_manager = TextureManager::new(&mut window, &raylib_thread);
    let mut sprite_manager = SpriteManager::new(&mut window, &raylib_thread);
    
    // Inicializar sistema de audio
    let mut audio_manager = AudioManager::new();
    
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();
    let mut current_fps = 0.0;

    let mut last_mouse_x = window.get_mouse_x();
    let mut delta_time = 0.016; // ~60 FPS inicial
    let mut was_moving = false; // Para detectar cambios de movimiento
    
    while !window.window_should_close() {
        let frame_start = Instant::now();
        
        // Actualizar sistema de audio
        audio_manager.update(delta_time);
        
        // Calcular FPS
        fps_counter += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            current_fps = fps_counter as f32;
            fps_counter = 0;
            fps_timer = Instant::now();
        }

        framebuffer.clear();

        match game_state.mode {
            GameMode::Welcome => {
                // Reproducir mÃºsica de menÃº si no estÃ¡ sonando
                if !audio_manager.is_music_playing() || 
                    audio_manager.get_current_music_type() != Some(MusicType::Menu) {
                    audio_manager.play_menu_music();
                }
                
                let selected_level = render_welcome_screen(&mut framebuffer, &window);
                if let Some(level) = selected_level {
                    game_state.start_game(level);
                    
                    // Cargar sprites del maze
                    sprite_manager.load_sprites_from_maze(&game_state.data.maze, block_size);
                    
                    // Cambiar a mÃºsica de fondo del juego
                    audio_manager.play_background_music();
                }
            },
            GameMode::Playing => {
                // Manejar entrada del mouse
                let mouse_x = window.get_mouse_x();
                let mouse_delta = mouse_x - last_mouse_x;
                last_mouse_x = mouse_x;

                let maze_clone = game_state.data.maze.clone();
                
                if !maze_clone.is_empty() && !maze_clone[0].is_empty() {
                    // Detectar si el jugador se estÃ¡ moviendo
                    let is_moving = window.is_key_down(KeyboardKey::KEY_W) ||
                                   window.is_key_down(KeyboardKey::KEY_S) ||
                                   window.is_key_down(KeyboardKey::KEY_A) ||
                                   window.is_key_down(KeyboardKey::KEY_D) ||
                                   window.is_key_down(KeyboardKey::KEY_Q) ||
                                   window.is_key_down(KeyboardKey::KEY_E) ||
                                   window.is_key_down(KeyboardKey::KEY_UP) ||
                                   window.is_key_down(KeyboardKey::KEY_DOWN) ||
                                   window.is_key_down(KeyboardKey::KEY_LEFT) ||
                                   window.is_key_down(KeyboardKey::KEY_RIGHT);

                    // Actualizar sonido de pasos
                    if is_moving != was_moving {
                        audio_manager.handle_player_movement(is_moving);
                        was_moving = is_moving;
                    }

                    // Procesar eventos de entrada
                    process_events_with_maze_safe(
                        &mut game_state.data.player, 
                        &window, 
                        &maze_clone, 
                        block_size
                    );
                    process_mouse_input_safe(&mut game_state.data.player, mouse_delta as f32);

                    // Actualizar sprites
                    sprite_manager.update_sprites(delta_time);

                    // Actualizar estado del juego
                    game_state.update_with_audio(&mut audio_manager, block_size);

                    // Renderizar mundo con sprites
                    render_world_with_sprites(
                        &mut framebuffer,
                        &maze_clone,
                        &texture_manager,
                        &mut sprite_manager,
                        block_size,
                        &game_state.data.player,
                    );
                    
                    game_state.data.notification_manager.render(&mut framebuffer);

                    // Renderizar UI
                    render_hud_enhanced(&mut framebuffer, &game_state.data, current_fps);

                    render_minimap(
                        &mut framebuffer,
                        &maze_clone,
                        &game_state.data.player,
                        block_size,
                    );

                    render_fps(&mut framebuffer, current_fps);
                    
                } else {
                    render_error_screen(&mut framebuffer, "ERROR: Maze not loaded correctly");
                }

                // Reset de emergencia
                if window.is_key_pressed(KeyboardKey::KEY_R) {
                    game_state.load_level(game_state.data.current_level);
                    sprite_manager.load_sprites_from_maze(&game_state.data.maze, block_size);
                }

                // Control de volumen con teclas
                if window.is_key_pressed(KeyboardKey::KEY_MINUS) {
                    let new_volume = (audio_manager.get_music_volume() - 0.1).max(0.0);
                    audio_manager.set_music_volume(new_volume);
                }
                if window.is_key_pressed(KeyboardKey::KEY_EQUAL) {
                    let new_volume = (audio_manager.get_music_volume() + 0.1).min(1.0);
                    audio_manager.set_music_volume(new_volume);
                }
            },
            GameMode::GameOver => {
                if render_game_over(&mut framebuffer, &window) {
                    game_state.reset();
                    audio_manager.stop_background_music();
                }
            },
            GameMode::Victory => {
                let should_continue = render_victory(&mut framebuffer, &window, game_state.data.current_level);
                
                if should_continue {
                    if game_state.data.current_level < 3 {
                        let next_level = game_state.data.current_level + 1;

                        game_state.clear_notifications();
                        
                        // Reproducir sonido de victoria
                        audio_manager.play_game_event(GameAudioEvent::LevelComplete);
                        
                        // Cargar el siguiente nivel directamente
                        game_state.load_level(next_level);
                        sprite_manager.load_sprites_from_maze(&game_state.data.maze, block_size);
                        game_state.mode = GameMode::Playing;
                        
                        // Reanudar mÃºsica de fondo
                        audio_manager.resume_background_music_now();
                    } else {
                        audio_manager.play_game_event(GameAudioEvent::LevelComplete);
                        game_state.reset();
                    }
                }
            }
        }

        // Mostrar framebuffer
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        // Calcular delta time para prÃ³ximo frame
        let frame_time = frame_start.elapsed();
        delta_time = frame_time.as_secs_f32().min(0.033); // Cap a ~30 FPS mÃ­nimo
        
        // Limitar FPS
        std::thread::sleep(Duration::from_millis(16));
    }
}

fn render_border_frame(framebuffer: &mut Framebuffer, x: u32, y: u32, width: u32, height: u32, color: Color) {
    framebuffer.set_current_color(color);
    
    // LÃ­neas horizontales
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
    
    // LÃ­neas verticales
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

pub fn render_text(framebuffer: &mut Framebuffer, text: &str, x: u32, y: u32) {
    ui::render_text(framebuffer, text, x, y);
}

fn render_hud_enhanced(
    framebuffer: &mut Framebuffer, 
    game_data: &game_state::GameData,
    fps: f32,
) {
    let hud_height = 140;
    let hud_width = 240;
    
    // Fondo del HUD con transparencia
    framebuffer.set_current_color(Color::new(0, 0, 0, 150));
    for y in 5..hud_height {
        for x in 5..hud_width {
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.set_pixel(x, y);
            }
        }
    }
    
    // Marco del HUD
    render_border_frame(framebuffer, 5, 5, hud_width - 5, hud_height - 5, Color::new(100, 100, 150, 255));
    
    // InformaciÃ³n del juego
    framebuffer.set_current_color(Color::WHITE);
    render_text(framebuffer, "ESTADO", 15, 15);
    
    // Vidas con color dinÃ¡mico
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
    
    // Nivel actual
    framebuffer.set_current_color(Color::CYAN);
    let level_text = format!("NIVEL: {}", game_data.current_level);
    render_text(framebuffer, &level_text, 15, 60);
    
    // Estado de la salida
    if game_data.has_key {
        framebuffer.set_current_color(Color::GREEN);
        render_text(framebuffer, "SALIDA BLOQUEADA!", 15, 75);
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
    
    // FPS
    let fps_color = if fps >= 50.0 { 
        Color::GREEN 
    } else if fps >= 30.0 { 
        Color::YELLOW 
    } else { 
        Color::RED 
    };
    framebuffer.set_current_color(fps_color);
    render_text(framebuffer, &format!("FPS: {:.0}", fps), 15, 105);
}

fn render_error_screen(framebuffer: &mut Framebuffer, error_msg: &str) {
    // Fondo rojo de error
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.set_current_color(Color::new(100, 0, 0, 255));
            framebuffer.set_pixel(x, y);
        }
    }
    
    // Texto de error
    framebuffer.set_current_color(Color::WHITE);
    render_error_text(framebuffer, error_msg, 400, 400);
    render_error_text(framebuffer, "PRESIONA R PARA REINTENTAR", 450, 450);
    render_error_text(framebuffer, "Presiona ESC para salir", 450, 480);
}

fn render_error_text(framebuffer: &mut Framebuffer, text: &str, x: u32, y: u32) {
    for (i, c) in text.chars().enumerate() {
        if c != ' ' {
            let char_x = x + (i as u32 * 8);
            for dx in 0..6 {
                for dy in 0..8 {
                    if char_x + dx < framebuffer.width && y + dy < framebuffer.height {
                        framebuffer.set_pixel(char_x + dx, y + dy);
                    }
                }
            }
        }
    }
}