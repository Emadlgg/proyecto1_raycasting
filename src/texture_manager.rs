// texture_manager.rs - Sistema de texturas

use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureInfo {
    pub texture: Texture2D,
    pub width: i32,
    pub height: i32,
    pub fallback_color: Color,
}

impl TextureInfo {
    pub fn new(texture: Texture2D, fallback_color: Color) -> Self {
        let width = texture.width;
        let height = texture.height;
        
        TextureInfo {
            texture,
            width,
            height,
            fallback_color,
        }
    }

    // Función segura para obtener color - usar variación procedural
    pub fn get_pixel_color_safe(&self, tx: f32, ty: f32) -> Color {
        let variation = ((tx * 16.0) as i32 + (ty * 16.0) as i32) % 8;
        let factor = 0.9 + (variation as f32 * 0.02);
        
        Color::new(
            ((self.fallback_color.r as f32) * factor).min(255.0) as u8,
            ((self.fallback_color.g as f32) * factor).min(255.0) as u8,
            ((self.fallback_color.b as f32) * factor).min(255.0) as u8,
            self.fallback_color.a,
        )
    }
}

pub struct TextureManager {
    textures: HashMap<char, TextureInfo>,
    fallback_colors: HashMap<char, Color>,
    default_texture_size: usize,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut texture_manager = TextureManager {
            textures: HashMap::new(),
            fallback_colors: HashMap::new(),
            default_texture_size: 64,
        };

        texture_manager.init_fallback_colors();
        texture_manager.load_wall_textures_safe(rl, thread);
        
        texture_manager
    }

    fn init_fallback_colors(&mut self) {
        self.fallback_colors.insert('#', Color::new(200, 200, 120, 255)); // Backrooms amarillo
        self.fallback_colors.insert('+', Color::new(180, 180, 100, 255)); 
        self.fallback_colors.insert('-', Color::new(220, 220, 140, 255));
        self.fallback_colors.insert('|', Color::new(190, 190, 110, 255));
        self.fallback_colors.insert('r', Color::new(180, 60, 60, 255));   // Rojo
        self.fallback_colors.insert('b', Color::new(60, 60, 180, 255));   // Azul
        self.fallback_colors.insert('g', Color::new(60, 180, 60, 255));   // Verde
        self.fallback_colors.insert('e', Color::new(100, 255, 100, 255)); // Portal
    }

    fn load_wall_textures_safe(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let texture_mappings = [
            ('#', "assets/textures/walls/wall_yellow.png"),
            ('+', "assets/textures/walls/wall_yellow.png"),
            ('-', "assets/textures/walls/wall_yellow.png"),
            ('|', "assets/textures/walls/wall_yellow.png"),
            ('r', "assets/textures/walls/wall_red.png"),
            ('b', "assets/textures/walls/wall_blue.png"),
            ('g', "assets/textures/walls/wall_green.png"),
            ('e', "assets/textures/walls/exit_portal.png"),
        ];

        for (wall_char, texture_path) in texture_mappings.iter() {
            let fallback_color = *self.fallback_colors.get(wall_char).unwrap_or(&Color::GRAY);
            
            match self.load_texture_safe(rl, thread, texture_path, fallback_color) {
                Ok(texture_info) => {
                    self.textures.insert(*wall_char, texture_info);
                },
                Err(_) => {
                    // Crear textura de respaldo simple
                    if let Ok(fallback_texture) = self.create_simple_fallback(rl, thread, fallback_color) {
                        self.textures.insert(*wall_char, fallback_texture);
                    }
                }
            }
        }
    }

    fn load_texture_safe(&self, rl: &mut RaylibHandle, thread: &RaylibThread, path: &str, fallback_color: Color) -> Result<TextureInfo, String> {
        match rl.load_texture(thread, path) {
            Ok(texture) => {
                Ok(TextureInfo::new(texture, fallback_color))
            },
            Err(_) => {
                Err(format!("Could not load texture from {}", path))
            }
        }
    }

    fn create_simple_fallback(&self, rl: &mut RaylibHandle, thread: &RaylibThread, color: Color) -> Result<TextureInfo, String> {
        let size = self.default_texture_size as i32;
        
        // Crear imagen simple como fallback
        let simple_image = Image::gen_image_color(size, size, color);
        
        match rl.load_texture_from_image(thread, &simple_image) {
            Ok(texture) => Ok(TextureInfo::new(texture, color)),
            Err(_) => Err("Failed to create fallback texture".to_string())
        }
    }

    // Función principal para obtener color de textura
    pub fn get_wall_color_textured(&self, ch: char, texture_x: f32, texture_y: f32) -> Color {
        if let Some(texture_info) = self.textures.get(&ch) {
            texture_info.get_pixel_color_safe(texture_x, texture_y)
        } else {
            self.get_wall_color_simple(ch, texture_x, texture_y)
        }
    }

    // Función de respaldo con patrones procedurales
    pub fn get_wall_color_simple(&self, ch: char, texture_x: f32, texture_y: f32) -> Color {
        let base_color = self.fallback_colors.get(&ch).copied().unwrap_or(Color::GRAY);
        
        let variation_factor = match ch {
            '#' | '+' | '-' | '|' => {
                // Patrón de cuadrícula para backrooms
                let grid_x = (texture_x * 4.0) as i32;
                let grid_y = (texture_y * 4.0) as i32;
                let is_border = (grid_x % 4 == 0) || (grid_y % 4 == 0);
                if is_border { 0.9 } else { 1.0 }
            },
            'r' => {
                // Patrón de ladrillos
                let brick_pattern = ((texture_x * 6.0) as i32 + (texture_y * 3.0) as i32) % 4;
                match brick_pattern {
                    0 => 1.0,
                    1 => 0.95,
                    2 => 1.05,
                    _ => 0.92,
                }
            },
            'b' => {
                // Patrón metálico
                let metal_pattern = ((texture_x * 8.0) as i32 + (texture_y * 8.0) as i32) % 3;
                match metal_pattern {
                    0 => 1.0,
                    1 => 0.97,
                    _ => 1.03,
                }
            },
            'g' => {
                // Patrón orgánico
                let organic_pattern = ((texture_x * 5.0) as i32 + (texture_y * 7.0) as i32) % 5;
                if organic_pattern < 2 { 1.0 } else { 0.9 }
            },
            'e' => {
                // Efecto pulsante para portal
                let time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs_f32();
                let pulse = (time * 3.0).sin() * 0.3 + 0.7;
                pulse
            },
            _ => 1.0
        };

        Color::new(
            ((base_color.r as f32) * variation_factor).min(255.0) as u8,
            ((base_color.g as f32) * variation_factor).min(255.0) as u8,
            ((base_color.b as f32) * variation_factor).min(255.0) as u8,
            base_color.a,
        )
    }

    // Funciones de compatibilidad
    pub fn get_wall_color(&self, wall_type: char) -> Color {
        self.fallback_colors.get(&wall_type).copied().unwrap_or(Color::GRAY)
    }

    pub fn get_wall_color_fallback(&self, ch: char) -> Color {
        self.fallback_colors.get(&ch).copied().unwrap_or(Color::GRAY)
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch).map(|info| &info.texture)
    }

    pub fn has_texture(&self, ch: char) -> bool {
        self.textures.contains_key(&ch)
    }

    pub fn is_wall_cell(&self, cell: char) -> bool {
        matches!(cell, '#' | '+' | '-' | '|' | 'r' | 'b' | 'g' | 'e')
    }

    pub fn get_texture_size(&self) -> usize {
        self.default_texture_size
    }
}

impl Drop for TextureManager {
    fn drop(&mut self) {
        self.textures.clear();
        self.fallback_colors.clear();
    }
}