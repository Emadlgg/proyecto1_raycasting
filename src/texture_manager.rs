// texture_manager.rs - Sistema de texturas con crate image

use raylib::prelude::*;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::collections::HashMap;

pub struct RealTextureInfo {
    pub pixels: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub fallback_color: Color,
}

impl RealTextureInfo {
    pub fn from_png(path: &str, fallback_color: Color) -> Result<Self, String> {
        match image::open(path) {
            Ok(img) => {
                let rgba_img = img.to_rgba8();
                let (width, height) = rgba_img.dimensions();
                
                Ok(RealTextureInfo {
                    pixels: rgba_img.into_raw(),
                    width: width as i32,
                    height: height as i32,
                    fallback_color,
                })
            },
            Err(e) => Err(format!("Error loading PNG {}: {}", path, e))
        }
    }

    pub fn get_pixel_color(&self, texture_x: f32, texture_y: f32) -> Color {
        let x = (texture_x.clamp(0.0, 1.0) * (self.width - 1) as f32) as usize;
        let y = (texture_y.clamp(0.0, 1.0) * (self.height - 1) as f32) as usize;
        
        let index = (y * self.width as usize + x) * 4;
        
        if index + 3 < self.pixels.len() {
            Color::new(
                self.pixels[index],     // R
                self.pixels[index + 1], // G
                self.pixels[index + 2], // B
                self.pixels[index + 3], // A
            )
        } else {
            self.fallback_color
        }
    }
}

pub struct TextureManager {
    real_textures: HashMap<char, RealTextureInfo>,
    fallback_colors: HashMap<char, Color>,
    default_texture_size: usize,
}

impl TextureManager {
    pub fn new(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Self {
        let mut texture_manager = TextureManager {
            real_textures: HashMap::new(),
            fallback_colors: HashMap::new(),
            default_texture_size: 64,
        };

        texture_manager.init_fallback_colors();
        texture_manager.load_real_textures();
        
        texture_manager
    }

    fn init_fallback_colors(&mut self) {
        self.fallback_colors.insert('#', Color::new(200, 200, 120, 255));
        self.fallback_colors.insert('+', Color::new(180, 180, 100, 255)); 
        self.fallback_colors.insert('-', Color::new(220, 220, 140, 255));
        self.fallback_colors.insert('|', Color::new(190, 190, 110, 255));
        self.fallback_colors.insert('r', Color::new(180, 60, 60, 255));
        self.fallback_colors.insert('b', Color::new(60, 60, 180, 255));
        self.fallback_colors.insert('e', Color::new(100, 255, 100, 255));
    }

    fn load_real_textures(&mut self) {
        let texture_mappings = [
            ('#', "assets/textures/walls/wall_yellow.png"),
            ('+', "assets/textures/walls/wall_yellow.png"),
            ('-', "assets/textures/walls/wall_yellow.png"),
            ('|', "assets/textures/walls/wall_yellow.png"),
            ('r', "assets/textures/walls/wall_red.png"),
            ('b', "assets/textures/walls/wall_blue.png"),
        ];

        for (wall_char, texture_path) in texture_mappings.iter() {
            let fallback_color = *self.fallback_colors.get(wall_char).unwrap_or(&Color::GRAY);
            
            if let Ok(texture_info) = RealTextureInfo::from_png(texture_path, fallback_color) {
                self.real_textures.insert(*wall_char, texture_info);
            }
        }
    }

    pub fn get_wall_color_textured(&self, ch: char, texture_x: f32, texture_y: f32) -> Color {
        if let Some(real_texture) = self.real_textures.get(&ch) {
            real_texture.get_pixel_color(texture_x, texture_y)
        } else {
            self.get_wall_color_simple(ch, texture_x, texture_y)
        }
    }

    pub fn get_wall_color_simple(&self, ch: char, texture_x: f32, texture_y: f32) -> Color {
        let base_color = self.fallback_colors.get(&ch).copied().unwrap_or(Color::GRAY);
        
        let variation_factor = match ch {
            '#' | '+' | '-' | '|' => {
                let grid_x = (texture_x * 8.0) as i32;
                let grid_y = (texture_y * 8.0) as i32;
                let is_border = (grid_x % 8 == 0) || (grid_y % 8 == 0);
                let is_corner = (grid_x % 4 == 0) && (grid_y % 4 == 0);
                
                if is_corner { 0.85 }
                else if is_border { 0.9 }
                else { 1.0 }
            },
            'r' => {
                let brick_row = (texture_y * 6.0) as i32;
                let brick_col = (texture_x * 12.0 + (brick_row % 2) as f32 * 6.0) as i32;
                
                let is_mortar_h = (texture_y * 6.0) % 1.0 < 0.1;
                let is_mortar_v = (brick_col % 12 == 0);
                
                if is_mortar_h || is_mortar_v { 0.7 }
                else { 1.0 + ((brick_col % 3) as f32 * 0.05) }
            },
            'b' => {
                let panel_x = (texture_x * 4.0) as i32;
                let panel_y = (texture_y * 4.0) as i32;
                let center_dist = ((texture_x - 0.5) * (texture_x - 0.5) + 
                                 (texture_y - 0.5) * (texture_y - 0.5)).sqrt();
                
                0.9 + (panel_x % 2) as f32 * 0.1 + center_dist * 0.2
            },
            'e' => {
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

    pub fn has_texture(&self, ch: char) -> bool {
        self.real_textures.contains_key(&ch)
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
        self.real_textures.clear();
        self.fallback_colors.clear();
    }
}