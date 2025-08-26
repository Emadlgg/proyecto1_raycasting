// caster.rs - Sistema de raycasting 

use raylib::color::Color;
use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::texture_manager::TextureManager;
use crate::sprite_manager::SpriteManager;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub texture_x: f32,
    pub texture_y: f32,
    pub side: WallSide,
}

#[derive(Debug, Clone, Copy)]
pub enum WallSide {
    North,
    South,
    East,
    West,
}

// Constantes de renderizado
const MAX_DISTANCE: f32 = 1000.0;
const STEP_SIZE: f32 = 1.5;
const MAX_ITERATIONS: i32 = 400;
const PROJECTION_PLANE_DISTANCE: f32 = 100.0;

pub fn cast_ray_textured(maze: &Maze, player: &Player, angle: f32, block_size: usize) -> Intersect {
    if maze.is_empty() {
        return create_default_intersect();
    }

    let maze_height = maze.len();
    let maze_width = maze.get(0).map_or(0, |row| row.len());
    
    if maze_width == 0 {
        return create_default_intersect();
    }

    let cos_a = angle.cos();
    let sin_a = angle.sin();
    let mut distance = STEP_SIZE;
    
    for _ in 0..MAX_ITERATIONS {
        if distance > MAX_DISTANCE {
            break;
        }

        let ray_x = player.pos.x + distance * cos_a;
        let ray_y = player.pos.y + distance * sin_a;

        // Verificación de bounds
        if ray_x < 0.0 || ray_y < 0.0 {
            return create_wall_intersect(distance);
        }

        let grid_x = (ray_x / block_size as f32) as usize;
        let grid_y = (ray_y / block_size as f32) as usize;

        if grid_y >= maze_height || grid_x >= maze_width {
            return create_wall_intersect(distance);
        }

        // Acceso seguro a la celda
        let current_cell = maze.get(grid_y)
            .and_then(|row| row.get(grid_x))
            .copied()
            .unwrap_or('#');

        if is_wall_cell(current_cell) {
            // Calcular coordenadas de textura
            let cell_x = ray_x % block_size as f32;
            let cell_y = ray_y % block_size as f32;
            
            let norm_x = (cell_x / block_size as f32).clamp(0.0, 1.0);
            let norm_y = (cell_y / block_size as f32).clamp(0.0, 1.0);
            
            let side = determine_wall_side(norm_x, norm_y);
            let (texture_x, texture_y) = calculate_texture_coordinates(side, norm_x, norm_y);

            return Intersect {
                distance,
                impact: current_cell,
                texture_x: texture_x.clamp(0.0, 1.0),
                texture_y: texture_y.clamp(0.0, 1.0),
                side,
            };
        }

        distance += STEP_SIZE;
    }

    create_default_intersect()
}

#[inline]
fn create_default_intersect() -> Intersect {
    Intersect {
        distance: MAX_DISTANCE,
        impact: '#',
        texture_x: 0.0,
        texture_y: 0.0,
        side: WallSide::North,
    }
}

#[inline]
fn create_wall_intersect(distance: f32) -> Intersect {
    Intersect {
        distance,
        impact: '#',
        texture_x: 0.0,
        texture_y: 0.0,
        side: WallSide::North,
    }
}

#[inline]
fn is_wall_cell(cell: char) -> bool {
    matches!(cell, '#' | '+' | '-' | '|' | 'r' | 'b' | 'g')
}

#[inline]
fn determine_wall_side(norm_x: f32, norm_y: f32) -> WallSide {
    let distances = [
        (norm_x, WallSide::West),
        (1.0 - norm_x, WallSide::East),
        (norm_y, WallSide::North),
        (1.0 - norm_y, WallSide::South),
    ];
    
    distances.iter()
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .map(|(_, side)| *side)
        .unwrap_or(WallSide::North)
}

#[inline]
fn calculate_texture_coordinates(side: WallSide, norm_x: f32, norm_y: f32) -> (f32, f32) {
    match side {
        WallSide::North | WallSide::South => (norm_x, norm_y),
        WallSide::East | WallSide::West => (norm_y, norm_x),
    }
}

// Función principal optimizada con sprites
pub fn render_world_with_sprites(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    texture_manager: &TextureManager,
    sprite_manager: &mut SpriteManager,
    block_size: usize,
    player: &Player,
) {
    if maze.is_empty() {
        return;
    }

    let screen_width = framebuffer.width;
    let screen_height = framebuffer.height;
    let half_height = screen_height as f32 * 0.5;

    // Renderizar cielo y suelo
    render_sky_and_floor(framebuffer, half_height);

    // Vector para z-buffer (distancias de paredes)
    let mut wall_distances = vec![MAX_DISTANCE; screen_width as usize];

    // Calcular incremento de ángulo
    let angle_increment = player.fov / screen_width as f32;
    let start_angle = player.a - player.fov * 0.5;

    // Renderizar paredes
    for column in 0..screen_width {
        let ray_angle = start_angle + column as f32 * angle_increment;
        let intersect = cast_ray_textured(maze, player, ray_angle, block_size);
        
        let distance = intersect.distance.max(1.0);
        wall_distances[column as usize] = distance;

        // Calcular altura de la columna de pared
        let wall_height = (half_height / distance) * PROJECTION_PLANE_DISTANCE;
        let wall_top = ((half_height - wall_height * 0.5).max(0.0)) as u32;
        let wall_bottom = ((half_height + wall_height * 0.5).min(screen_height as f32)) as u32;

        render_wall_column(
            framebuffer,
            texture_manager,
            column,
            wall_top,
            wall_bottom,
            &intersect,
            distance,
        );
    }

    // Renderizar sprites usando z-buffer
    sprite_manager.render_sprites(framebuffer, player, &wall_distances);
}

// Función optimizada sin sprites
pub fn render_world_textured(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    texture_manager: &TextureManager,
    block_size: usize,
    player: &Player,
) {
    if maze.is_empty() {
        return;
    }

    let screen_width = framebuffer.width;
    let screen_height = framebuffer.height;
    let half_height = screen_height as f32 * 0.5;

    render_sky_and_floor(framebuffer, half_height);

    let angle_increment = player.fov / screen_width as f32;
    let start_angle = player.a - player.fov * 0.5;

    for column in 0..screen_width {
        let ray_angle = start_angle + column as f32 * angle_increment;
        let intersect = cast_ray_textured(maze, player, ray_angle, block_size);
        
        let distance = intersect.distance.max(1.0);
        let wall_height = (half_height / distance) * PROJECTION_PLANE_DISTANCE;
        let wall_top = ((half_height - wall_height * 0.5).max(0.0)) as u32;
        let wall_bottom = ((half_height + wall_height * 0.5).min(screen_height as f32)) as u32;

        render_wall_column(
            framebuffer,
            texture_manager,
            column,
            wall_top,
            wall_bottom,
            &intersect,
            distance,
        );
    }
}

#[inline]
fn render_sky_and_floor(framebuffer: &mut Framebuffer, half_height: f32) {
    let screen_height = framebuffer.height;
    let half_height_u32 = half_height as u32;

    // Renderizar cielo (parte superior)
    for y in 0..half_height_u32.min(screen_height) {
        let depth_factor = y as f32 / half_height;
        let sky_color = Color::new(
            (10.0 + depth_factor * 15.0) as u8,
            (10.0 + depth_factor * 15.0) as u8,
            (20.0 + depth_factor * 25.0) as u8,
            255
        );
        
        framebuffer.set_current_color(sky_color);
        for x in 0..framebuffer.width {
            framebuffer.set_pixel(x, y);
        }
    }

    // Renderizar suelo (parte inferior)
    for y in half_height_u32..screen_height {
        let depth_factor = (y as f32 - half_height) / half_height;
        let floor_color = Color::new(
            (40.0 + depth_factor * 60.0) as u8,
            (30.0 + depth_factor * 45.0) as u8,
            (20.0 + depth_factor * 30.0) as u8,
            255
        );
        
        framebuffer.set_current_color(floor_color);
        for x in 0..framebuffer.width {
            framebuffer.set_pixel(x, y);
        }
    }
}

#[inline]
fn render_wall_column(
    framebuffer: &mut Framebuffer,
    texture_manager: &TextureManager,
    column: u32,
    wall_top: u32,
    wall_bottom: u32,
    intersect: &Intersect,
    distance: f32,
) {
    if wall_bottom <= wall_top || wall_top >= framebuffer.height {
        return;
    }

    let column_height = (wall_bottom - wall_top) as f32;
    let texture_y_step = 1.0 / column_height.max(1.0);

    // Calcular factores de iluminación
    let distance_attenuation = calculate_distance_attenuation(distance);
    let side_attenuation = calculate_side_attenuation(intersect.side);
    let final_brightness = distance_attenuation * side_attenuation;

    // Renderizar cada pixel de la columna
    for y in wall_top..wall_bottom.min(framebuffer.height) {
        let texture_y = (y - wall_top) as f32 * texture_y_step;

        let base_color = texture_manager.get_wall_color_textured(
            intersect.impact,
            intersect.texture_x,
            texture_y,
        );

        let final_color = apply_lighting(base_color, final_brightness);
        
        framebuffer.set_current_color(final_color);
        framebuffer.set_pixel(column, y);
    }
}

#[inline]
fn calculate_distance_attenuation(distance: f32) -> f32 {
    let max_distance = 600.0;
    let min_brightness = 0.3;
    let attenuation = (distance / max_distance).min(0.8);
    (1.0 - attenuation * 0.7).max(min_brightness)
}

#[inline]
fn calculate_side_attenuation(side: WallSide) -> f32 {
    match side {
        WallSide::North | WallSide::South => 1.0,
        WallSide::East | WallSide::West => 0.92,
    }
}

#[inline]
fn apply_lighting(color: Color, brightness: f32) -> Color {
    Color::new(
        ((color.r as f32) * brightness) as u8,
        ((color.g as f32) * brightness) as u8,
        ((color.b as f32) * brightness) as u8,
        color.a,
    )
}

// Renderizado de objetos simples (para compatibilidad)
pub fn render_world_objects(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    _texture_manager: &TextureManager,
    player: &Player,
    block_size: usize,
) {
    if maze.is_empty() {
        return;
    }

    let screen_center_y = framebuffer.height as i32 / 2;
    
    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if is_object_cell(cell) {
                let obj_x = col_idx as f32 * block_size as f32 + (block_size as f32 * 0.5);
                let obj_y = row_idx as f32 * block_size as f32 + (block_size as f32 * 0.5);
                
                render_simple_sprite(framebuffer, player, obj_x, obj_y, cell, screen_center_y);
            }
        }
    }
}

#[inline]
fn is_object_cell(cell: char) -> bool {
    matches!(cell, 'k' | 'c' | 'l' | 't')
}

fn render_simple_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    obj_x: f32,
    obj_y: f32,
    obj_type: char,
    screen_center_y: i32,
) {
    let dx = obj_x - player.pos.x;
    let dy = obj_y - player.pos.y;
    let distance = (dx * dx + dy * dy).sqrt();
    
    // Culling por distancia
    if distance > 400.0 || distance < 1.0 {
        return;
    }
    
    // Calcular ángulo y verificar FOV
    let angle_to_object = dy.atan2(dx);
    let mut relative_angle = angle_to_object - player.a;
    
    // Normalizar ángulo
    while relative_angle > PI { relative_angle -= 2.0 * PI; }
    while relative_angle < -PI { relative_angle += 2.0 * PI; }
    
    let half_fov = player.fov * 0.5;
    if relative_angle.abs() > half_fov {
        return;
    }
    
    // Calcular posición y tamaño en pantalla
    let screen_x = framebuffer.width as f32 * 0.5 + 
                  (relative_angle / half_fov) * (framebuffer.width as f32 * 0.5);
    
    let sprite_scale = get_sprite_scale(obj_type);
    let sprite_size = ((framebuffer.height as f32 * sprite_scale) / distance) * PROJECTION_PLANE_DISTANCE;
    
    if sprite_size < 2.0 {
        return;
    }
    
    let brightness = calculate_distance_attenuation(distance);
    render_sprite_shape(framebuffer, screen_x as i32, screen_center_y, sprite_size as u32, obj_type, brightness);
}

#[inline]
fn get_sprite_scale(obj_type: char) -> f32 {
    match obj_type {
        'k' => 0.15,
        'c' => 0.2,
        'l' => 0.18,
        't' => 0.16,
        _ => 0.15,
    }
}

fn render_sprite_shape(
    framebuffer: &mut Framebuffer,
    center_x: i32,
    center_y: i32,
    size: u32,
    sprite_type: char,
    brightness: f32,
) {
    let base_color = get_sprite_color(sprite_type);
    let final_color = apply_lighting(base_color, brightness);
    framebuffer.set_current_color(final_color);
    
    let half_size = size as i32 / 2;
    let start_x = (center_x - half_size).max(0) as u32;
    let end_x = (center_x + half_size).min(framebuffer.width as i32) as u32;
    let start_y = (center_y - half_size).max(0) as u32;
    let end_y = (center_y + half_size).min(framebuffer.height as i32) as u32;
    
    match sprite_type {
        'k' => render_key_shape(framebuffer, start_x, start_y, end_x, end_y),
        'c' => render_cross_shape(framebuffer, start_x, start_y, end_x, end_y),
        'l' => render_circle_shape(framebuffer, center_x, center_y, half_size),
        't' => render_triangle_shape(framebuffer, start_x, start_y, end_x, end_y),
        _ => render_square_shape(framebuffer, start_x, start_y, end_x, end_y),
    }
}

#[inline]
fn get_sprite_color(sprite_type: char) -> Color {
    match sprite_type {
        'k' => Color::GOLD,
        'c' => Color::CYAN,
        'l' => Color::PURPLE,
        't' => Color::RED,
        _ => Color::WHITE,
    }
}

fn render_key_shape(framebuffer: &mut Framebuffer, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
    let mid_y = (start_y + end_y) / 2;
    let quarter_x = start_x + (end_x - start_x) / 4;
    
    // Cabeza de la llave
    for y in start_y..mid_y {
        for x in start_x..quarter_x {
            framebuffer.set_pixel(x, y);
        }
    }
    
    // Cuerpo de la llave
    for x in quarter_x..end_x {
        framebuffer.set_pixel(x, mid_y);
        if mid_y + 1 < framebuffer.height {
            framebuffer.set_pixel(x, mid_y + 1);
        }
    }
}

fn render_cross_shape(framebuffer: &mut Framebuffer, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
    let mid_x = (start_x + end_x) / 2;
    let mid_y = (start_y + end_y) / 2;
    
    // Línea vertical
    for y in start_y..end_y {
        framebuffer.set_pixel(mid_x, y);
        if mid_x + 1 < framebuffer.width {
            framebuffer.set_pixel(mid_x + 1, y);
        }
    }
    
    // Línea horizontal
    for x in start_x..end_x {
        framebuffer.set_pixel(x, mid_y);
        if mid_y + 1 < framebuffer.height {
            framebuffer.set_pixel(x, mid_y + 1);
        }
    }
}

fn render_circle_shape(framebuffer: &mut Framebuffer, center_x: i32, center_y: i32, radius: i32) {
    let radius_sq = radius * radius;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius_sq {
                let x = center_x + dx;
                let y = center_y + dy;
                if x >= 0 && y >= 0 && (x as u32) < framebuffer.width && (y as u32) < framebuffer.height {
                    framebuffer.set_pixel(x as u32, y as u32);
                }
            }
        }
    }
}

fn render_triangle_shape(framebuffer: &mut Framebuffer, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
    let mid_x = (start_x + end_x) / 2;
    let height = end_y - start_y;
    
    for y in start_y..end_y {
        let row_progress = (y - start_y) as f32 / height as f32;
        let row_width = ((end_x - start_x) as f32 * (1.0 - row_progress)) as u32;
        let row_start = mid_x - row_width / 2;
        let row_end = mid_x + row_width / 2;
        
        for x in row_start..row_end {
            if x < framebuffer.width {
                framebuffer.set_pixel(x, y);
            }
        }
    }
}

fn render_square_shape(framebuffer: &mut Framebuffer, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
    for y in start_y..end_y {
        for x in start_x..end_x {
            framebuffer.set_pixel(x, y);
        }
    }
}

// Funciones de compatibilidad
pub fn cast_ray_safe(maze: &Maze, player: &Player, angle: f32, block_size: usize) -> Intersect {
    cast_ray_textured(maze, player, angle, block_size)
}

pub fn render_world_safe(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    texture_manager: &TextureManager,
    block_size: usize,
    player: &Player,
) {
    render_world_textured(framebuffer, maze, texture_manager, block_size, player);
}