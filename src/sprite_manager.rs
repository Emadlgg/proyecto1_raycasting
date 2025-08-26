// sprite_manager.rs - Sistema de sprites

use raylib::prelude::*;
use std::collections::HashMap;
use crate::maze::Maze;
use crate::player::Player;
use crate::framebuffer::Framebuffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteType {
    KeyGold,
    Checkpoint,
    ExitPortal,
    ExtraLife,
    TrapSpike,
}

impl SpriteType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'k' => Some(SpriteType::KeyGold),
            'c' => Some(SpriteType::Checkpoint),
            'e' => Some(SpriteType::ExitPortal),
            'l' => Some(SpriteType::ExtraLife),
            't' => Some(SpriteType::TrapSpike),
            _ => None,
        }
    }

    pub fn get_file_path(&self) -> &'static str {
        match self {
            SpriteType::KeyGold => "assets/sprites/key_gold.png",
            SpriteType::Checkpoint => "assets/sprites/checkpoint.png",
            SpriteType::ExitPortal => "assets/sprites/exit_portal.png",
            SpriteType::ExtraLife => "assets/sprites/extra_life.png",
            SpriteType::TrapSpike => "assets/sprites/trap_spike.png",
        }
    }

    pub fn get_fallback_color(&self) -> Color {
        match self {
            SpriteType::KeyGold => Color::GOLD,
            SpriteType::Checkpoint => Color::CYAN,
            SpriteType::ExitPortal => Color::GREEN,
            SpriteType::ExtraLife => Color::PURPLE,
            SpriteType::TrapSpike => Color::RED,
        }
    }

    pub fn get_base_scale(&self) -> f32 {
        match self {
            SpriteType::KeyGold => 0.8,
            SpriteType::Checkpoint => 1.2,
            SpriteType::ExitPortal => 1.5,
            SpriteType::ExtraLife => 0.9,
            SpriteType::TrapSpike => 1.0,
        }
    }

    pub fn is_animated(&self) -> bool {
        matches!(self, 
            SpriteType::KeyGold | 
            SpriteType::Checkpoint | 
            SpriteType::ExitPortal | 
            SpriteType::ExtraLife
        )
    }
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub sprite_type: SpriteType,
    pub world_x: f32,
    pub world_y: f32,
    pub scale: f32,
    pub rotation: f32,
    pub active: bool,
    pub animation_time: f32,
}

impl Sprite {
    pub fn new(sprite_type: SpriteType, world_x: f32, world_y: f32) -> Self {
        Sprite {
            sprite_type,
            world_x,
            world_y,
            scale: sprite_type.get_base_scale(),
            rotation: 0.0,
            active: true,
            animation_time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.active || !self.sprite_type.is_animated() {
            return;
        }

        self.animation_time += delta_time;
        
        match self.sprite_type {
            SpriteType::KeyGold => {
                // Rotación lenta y pulsación suave
                self.rotation = self.animation_time * 45.0;
                self.scale = self.sprite_type.get_base_scale() + 
                           (self.animation_time * 2.0).sin() * 0.1;
            },
            SpriteType::Checkpoint => {
                // Pulsación más intensa
                self.scale = self.sprite_type.get_base_scale() + 
                           (self.animation_time * 4.0).sin() * 0.2;
            },
            SpriteType::ExitPortal => {
                // Rotación y pulsación del portal
                self.rotation = self.animation_time * 30.0;
                self.scale = self.sprite_type.get_base_scale() + 
                           (self.animation_time * 3.0).sin() * 0.3;
            },
            SpriteType::ExtraLife => {
                // Latido del corazón
                self.scale = self.sprite_type.get_base_scale() + 
                           (self.animation_time * 6.0).sin() * 0.15;
            },
            _ => {}
        }
    }

    pub fn distance_to_player(&self, player: &Player) -> f32 {
        let dx = self.world_x - player.pos.x;
        let dy = self.world_y - player.pos.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn is_visible_to_player(&self, player: &Player) -> bool {
        let dx = self.world_x - player.pos.x;
        let dy = self.world_y - player.pos.y;
        
        let angle_to_sprite = dy.atan2(dx);
        let mut relative_angle = angle_to_sprite - player.a;
        
        // Normalizar ángulo
        while relative_angle > std::f32::consts::PI {
            relative_angle -= 2.0 * std::f32::consts::PI;
        }
        while relative_angle < -std::f32::consts::PI {
            relative_angle += 2.0 * std::f32::consts::PI;
        }
        
        let half_fov = player.fov / 2.0;
        relative_angle.abs() <= half_fov
    }
}

pub struct SpriteInfo {
    pub texture: Texture2D,
    pub width: i32,
    pub height: i32,
    pub fallback_color: Color,
    pub loaded_successfully: bool,
}

impl SpriteInfo {
    pub fn new(texture: Texture2D, fallback_color: Color, loaded_successfully: bool) -> Self {
        let width = texture.width;
        let height = texture.height;
        
        SpriteInfo {
            texture,
            width,
            height,
            fallback_color,
            loaded_successfully,
        }
    }
}

pub struct SpriteManager {
    sprite_infos: HashMap<SpriteType, SpriteInfo>,
    sprites: Vec<Sprite>,
    z_buffer: Vec<f32>, // Para depth testing
}

impl SpriteManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut sprite_manager = SpriteManager {
            sprite_infos: HashMap::new(),
            sprites: Vec::new(),
            z_buffer: Vec::new(),
        };
        
        sprite_manager.load_all_sprites(rl, thread);
        sprite_manager
    }

    fn load_all_sprites(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let sprite_types = [
            SpriteType::KeyGold,
            SpriteType::Checkpoint,
            SpriteType::ExitPortal,
            SpriteType::ExtraLife,
            SpriteType::TrapSpike,
        ];

        for sprite_type in sprite_types.iter() {
            let path = sprite_type.get_file_path();
            let fallback_color = sprite_type.get_fallback_color();
            
            let sprite_info = match rl.load_texture(thread, path) {
                Ok(texture) => {
                    SpriteInfo::new(texture, fallback_color, true)
                },
                Err(_) => {
                    // Crear textura de fallback
                    match self.create_fallback_sprite_texture(rl, thread, *sprite_type) {
                        Ok(texture) => {
                            SpriteInfo::new(texture, fallback_color, false)
                        },
                        Err(_) => {
                            continue;
                        }
                    }
                }
            };
            
            self.sprite_infos.insert(*sprite_type, sprite_info);
        }
    }

    fn create_fallback_sprite_texture(&self, rl: &mut RaylibHandle, thread: &RaylibThread, sprite_type: SpriteType) -> Result<Texture2D, String> {
        let size = 32;
        let base_color = sprite_type.get_fallback_color();
        
        // Crear imagen procedural basada en el tipo de sprite
        let image = match sprite_type {
            SpriteType::KeyGold => self.create_key_pattern(size, base_color),
            SpriteType::Checkpoint => self.create_cross_pattern(size, base_color),
            SpriteType::ExitPortal => self.create_portal_pattern(size, base_color),
            SpriteType::ExtraLife => self.create_heart_pattern(size, base_color),
            SpriteType::TrapSpike => self.create_spike_pattern(size, base_color),
        };
        
        rl.load_texture_from_image(thread, &image)
            .map_err(|e| format!("Failed to create fallback texture: {:?}", e))
    }

    fn create_key_pattern(&self, size: i32, base_color: Color) -> Image {
        let mut image = Image::gen_image_color(size, size, Color::new(0, 0, 0, 0));
        
        let center_x = size / 2;
        let center_y = size / 2;
        
        // Cabeza de la llave (círculo)
        for y in 0..size {
            for x in 0..size {
                let dx = x - (center_x - size/4);
                let dy = y - center_y;
                if dx*dx + dy*dy <= (size/6)*(size/6) {
                    image.draw_pixel(x, y, base_color);
                }
            }
        }
        
        // Cuerpo de la llave
        for x in center_x..(center_x + size/2) {
            for y in (center_y - 1)..(center_y + 2) {
                if x < size && y >= 0 && y < size {
                    image.draw_pixel(x, y, base_color);
                }
            }
        }
        
        image
    }

    fn create_cross_pattern(&self, size: i32, base_color: Color) -> Image {
        let mut image = Image::gen_image_color(size, size, Color::new(0, 0, 0, 0));
        
        let center_x = size / 2;
        let center_y = size / 2;
        let thickness = 2;
        
        // Línea vertical
        for y in thickness..(size - thickness) {
            for dx in -thickness..=thickness {
                let x = center_x + dx;
                if x >= 0 && x < size {
                    image.draw_pixel(x, y, base_color);
                }
            }
        }
        
        // Línea horizontal
        for x in thickness..(size - thickness) {
            for dy in -thickness..=thickness {
                let y = center_y + dy;
                if y >= 0 && y < size {
                    image.draw_pixel(x, y, base_color);
                }
            }
        }
        
        image
    }

    fn create_portal_pattern(&self, size: i32, base_color: Color) -> Image {
        let mut image = Image::gen_image_color(size, size, Color::new(0, 0, 0, 0));
        
        let center_x = size as f32 / 2.0;
        let center_y = size as f32 / 2.0;
        let max_radius = (size as f32 / 2.0) * 0.8;
        
        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx*dx + dy*dy).sqrt();
                
                if distance <= max_radius {
                    let intensity = 1.0 - (distance / max_radius);
                    let color = Color::new(
                        (base_color.r as f32 * intensity) as u8,
                        (base_color.g as f32 * intensity) as u8,
                        (base_color.b as f32 * intensity) as u8,
                        (255.0 * intensity) as u8,
                    );
                    image.draw_pixel(x, y, color);
                }
            }
        }
        
        image
    }

    fn create_heart_pattern(&self, size: i32, base_color: Color) -> Image {
        let mut image = Image::gen_image_color(size, size, Color::new(0, 0, 0, 0));
        
        let center_x = size as f32 / 2.0;
        let center_y = size as f32 / 2.0;
        
        for y in 0..size {
            for x in 0..size {
                let nx = (x as f32 - center_x) / (size as f32 / 4.0);
                let ny = (y as f32 - center_y) / (size as f32 / 4.0);
                
                // Ecuación simplificada de corazón
                let heart_eq = (nx*nx + ny*ny - 1.0).powi(3) - nx*nx * ny*ny*ny;
                
                if heart_eq <= 0.0 {
                    image.draw_pixel(x, y, base_color);
                }
            }
        }
        
        image
    }

    fn create_spike_pattern(&self, size: i32, base_color: Color) -> Image {
        let mut image = Image::gen_image_color(size, size, Color::new(0, 0, 0, 0));
        
        let num_spikes = 5;
        let spike_width = size / num_spikes;
        
        for spike in 0..num_spikes {
            let spike_center_x = spike * spike_width + spike_width / 2;
            
            // Dibujar triángulo
            for y in 0..size {
                let spike_height = size - y;
                let half_width = (spike_height * spike_width / 2) / size;
                
                for dx in -half_width..=half_width {
                    let x = spike_center_x + dx;
                    if x >= 0 && x < size {
                        image.draw_pixel(x, y, base_color);
                    }
                }
            }
        }
        
        image
    }

    pub fn load_sprites_from_maze(&mut self, maze: &Maze, block_size: usize) {
        self.sprites.clear();
        
        for (row_idx, row) in maze.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if let Some(sprite_type) = SpriteType::from_char(cell) {
                    let world_x = col_idx as f32 * block_size as f32 + (block_size as f32 / 2.0);
                    let world_y = row_idx as f32 * block_size as f32 + (block_size as f32 / 2.0);
                    
                    let sprite = Sprite::new(sprite_type, world_x, world_y);
                    self.sprites.push(sprite);
                }
            }
        }
    }

    pub fn update_sprites(&mut self, delta_time: f32) {
        for sprite in &mut self.sprites {
            if sprite.active {
                sprite.update(delta_time);
            }
        }
    }

    pub fn render_sprites(
        &mut self,
        framebuffer: &mut Framebuffer,
        player: &Player,
        wall_distances: &[f32], // Z-buffer de las paredes
    ) {
        if self.sprites.is_empty() {
            return;
        }

        // Inicializar z-buffer si es necesario
        if self.z_buffer.len() != framebuffer.width as usize {
            self.z_buffer.resize(framebuffer.width as usize, f32::INFINITY);
        }
        
        // Copiar distancias de paredes al z-buffer
        for (i, &distance) in wall_distances.iter().enumerate() {
            if i < self.z_buffer.len() {
                self.z_buffer[i] = distance;
            }
        }

        // Ordenar sprites por distancia (más lejanos primero)
        let mut sprite_distances: Vec<(usize, f32)> = self.sprites
            .iter()
            .enumerate()
            .filter(|(_, sprite)| sprite.active && sprite.is_visible_to_player(player))
            .map(|(i, sprite)| (i, sprite.distance_to_player(player)))
            .collect();
        
        sprite_distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Renderizar sprites en orden de profundidad
        for (sprite_idx, distance) in sprite_distances {
            if let Some(sprite) = self.sprites.get(sprite_idx) {
                self.render_single_sprite(framebuffer, player, sprite, distance);
            }
        }
    }

    fn render_single_sprite(
        &self,
        framebuffer: &mut Framebuffer,
        player: &Player,
        sprite: &Sprite,
        distance: f32,
    ) {
        // Calcular posición en pantalla
        let dx = sprite.world_x - player.pos.x;
        let dy = sprite.world_y - player.pos.y;
        
        let angle_to_sprite = dy.atan2(dx);
        let mut relative_angle = angle_to_sprite - player.a;
        
        // Normalizar ángulo
        while relative_angle > std::f32::consts::PI {
            relative_angle -= 2.0 * std::f32::consts::PI;
        }
        while relative_angle < -std::f32::consts::PI {
            relative_angle += 2.0 * std::f32::consts::PI;
        }
        
        // Calcular posición horizontal en pantalla
        let half_fov = player.fov / 2.0;
        let screen_x = framebuffer.width as f32 * 0.5 + 
                      (relative_angle / half_fov) * (framebuffer.width as f32 * 0.5);
        
        // Calcular tamaño del sprite
        let distance_to_projection_plane = 100.0;
        let sprite_size = (framebuffer.height as f32 * sprite.scale / distance) * distance_to_projection_plane;
        
        if sprite_size < 1.0 {
            return; // Muy pequeño para renderizar
        }
        
        self.render_sprite_column(
            framebuffer,
            sprite,
            screen_x as i32,
            sprite_size as u32,
            distance,
        );
    }

    fn render_sprite_column(
        &self,
        framebuffer: &mut Framebuffer,
        sprite: &Sprite,
        center_x: i32,
        size: u32,
        distance: f32,
    ) {
        if let Some(_sprite_info) = self.sprite_infos.get(&sprite.sprite_type) {
            let half_size = size as i32 / 2;
            let start_x = (center_x - half_size).max(0) as u32;
            let end_x = (center_x + half_size).min(framebuffer.width as i32) as u32;
            
            let center_y = framebuffer.height as i32 / 2;
            let start_y = (center_y - half_size).max(0) as u32;
            let end_y = (center_y + half_size).min(framebuffer.height as i32) as u32;
            
            // Factor de atenuación por distancia
            let distance_factor = (distance / 400.0).min(0.7).max(0.0);
            let brightness = 1.0 - distance_factor * 0.3;
            
            for y in start_y..end_y {
                for x in start_x..end_x {
                    // Verificar z-buffer
                    if (x as usize) < self.z_buffer.len() && distance < self.z_buffer[x as usize] {
                        // Calcular coordenadas de textura
                        let tx = (x - start_x) as f32 / (end_x - start_x) as f32;
                        let ty = (y - start_y) as f32 / (end_y - start_y) as f32;
                        
                        let color = self.get_fallback_sprite_color(sprite, tx, ty, brightness);
                        
                        // Solo dibujar píxeles no transparentes
                        if color.a > 0 {
                            framebuffer.set_current_color(color);
                            framebuffer.set_pixel(x, y);
                        }
                    }
                }
            }
        }
    }

    fn get_fallback_sprite_color(&self, sprite: &Sprite, tx: f32, ty: f32, brightness: f32) -> Color {
        let base_color = sprite.sprite_type.get_fallback_color();
        
        // Patrón específico según el tipo de sprite
        let (intensity, alpha) = match sprite.sprite_type {
            SpriteType::KeyGold => {
                // Patrón metálico dorado
                let metallic = ((tx * 4.0).sin() * (ty * 4.0).cos() * 0.3 + 0.7).abs();
                (metallic, 255)
            },
            SpriteType::Checkpoint => {
                // Patrón de cruz brillante
                let is_cross = (tx > 0.4 && tx < 0.6) || (ty > 0.4 && ty < 0.6);
                if is_cross { (1.0, 255) } else { (0.0, 0) }
            },
            SpriteType::ExitPortal => {
                // Patrón circular del portal
                let center_dist = ((tx - 0.5).powi(2) + (ty - 0.5).powi(2)).sqrt() * 2.0;
                if center_dist <= 1.0 {
                    let wave = (center_dist * 6.0 + sprite.animation_time * 3.0).sin() * 0.5 + 0.5;
                    (wave, (255.0 * (1.0 - center_dist)) as u8)
                } else {
                    (0.0, 0)
                }
            },
            SpriteType::ExtraLife => {
                // Patrón de corazón
                let heart_shape = self.is_heart_shape(tx, ty);
                if heart_shape { (1.0, 255) } else { (0.0, 0) }
            },
            SpriteType::TrapSpike => {
                // Patrón de picos
                let spike_pattern = ((tx * 5.0) as i32 % 2 == 0) && (ty > 0.3);
                if spike_pattern { (1.0, 255) } else { (0.0, 0) }
            },
        };

        Color::new(
            ((base_color.r as f32) * intensity * brightness).min(255.0) as u8,
            ((base_color.g as f32) * intensity * brightness).min(255.0) as u8,
            ((base_color.b as f32) * intensity * brightness).min(255.0) as u8,
            alpha,
        )
    }

    fn is_heart_shape(&self, tx: f32, ty: f32) -> bool {
        let x = (tx - 0.5) * 4.0;
        let y = (ty - 0.5) * 4.0;
        
        // Ecuación aproximada de corazón
        let heart_eq = (x*x + y*y - 1.0).powi(3) - x*x * y*y*y;
        heart_eq <= 0.0
    }

    // Funciones públicas de utilidad
    pub fn remove_sprite_at(&mut self, world_x: f32, world_y: f32, tolerance: f32) -> Option<SpriteType> {
        if let Some(index) = self.sprites.iter().position(|sprite| {
            sprite.active &&
            (sprite.world_x - world_x).abs() < tolerance &&
            (sprite.world_y - world_y).abs() < tolerance
        }) {
            let sprite_type = self.sprites[index].sprite_type;
            self.sprites[index].active = false;
            Some(sprite_type)
        } else {
            None
        }
    }

    pub fn sprite_count(&self) -> usize {
        self.sprites.iter().filter(|s| s.active).count()
    }
}

impl Drop for SpriteManager {
    fn drop(&mut self) {
        self.sprite_infos.clear();
        self.sprites.clear();
    }
}