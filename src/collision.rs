// collision.rs - Sistema de colisiones 

use crate::maze::Maze;

pub struct CollisionSystem {
    maze: Maze,
    block_size: usize,
}

impl CollisionSystem {
    pub fn new(maze: Maze, block_size: usize) -> Self {
        CollisionSystem { maze, block_size }
    }

    /// Verifica si una posición está dentro de los límites del maze
    fn is_within_bounds(&self, grid_x: usize, grid_y: usize) -> bool {
        grid_y < self.maze.len() && grid_x < self.maze.get(0).map_or(0, |row| row.len())
    }

    /// Convierte coordenadas del mundo a coordenadas de grid de forma segura
    fn world_to_grid(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        if x < 0.0 || y < 0.0 {
            return None;
        }

        let grid_x = (x / self.block_size as f32) as usize;
        let grid_y = (y / self.block_size as f32) as usize;

        if self.is_within_bounds(grid_x, grid_y) {
            Some((grid_x, grid_y))
        } else {
            None
        }
    }

    /// Obtiene el contenido de una celda de forma segura
    pub fn get_cell(&self, x: f32, y: f32) -> char {
        match self.world_to_grid(x, y) {
            Some((grid_x, grid_y)) => self.maze[grid_y][grid_x],
            None => '#', // Tratar áreas fuera de bounds como muros
        }
    }

    /// Verifica si el jugador puede moverse a una posición específica
    pub fn can_move_to(&self, x: f32, y: f32) -> bool {
        match self.world_to_grid(x, y) {
            Some((grid_x, grid_y)) => {
                let cell = self.maze[grid_y][grid_x];
                // Permitir movimiento a través de espacios vacíos y elementos interactivos
                matches!(cell, ' ' | 'k' | 't' | 'l' | 'c' | 'e')
            }
            None => false, // No permitir movimiento fuera de bounds
        }
    }

    /// Verifica colisión con múltiples puntos del jugador (más preciso)
    pub fn can_move_to_with_radius(&self, center_x: f32, center_y: f32, radius: f32) -> bool {
        // Verificar el centro y los puntos cardinales del radio del jugador
        let points = [
            (center_x, center_y),
            (center_x + radius, center_y),
            (center_x - radius, center_y),
            (center_x, center_y + radius),
            (center_x, center_y - radius),
            (center_x + radius * 0.7, center_y + radius * 0.7),
            (center_x - radius * 0.7, center_y - radius * 0.7),
            (center_x + radius * 0.7, center_y - radius * 0.7),
            (center_x - radius * 0.7, center_y + radius * 0.7),
        ];

        // Todos los puntos deben ser válidos para permitir el movimiento
        points.iter().all(|&(x, y)| self.can_move_to(x, y))
    }

    /// Actualiza el maze (para cambios dinámicos)
    pub fn update_maze(&mut self, new_maze: Maze) {
        self.maze = new_maze;
    }

    /// Obtiene las dimensiones del maze
    pub fn get_dimensions(&self) -> (usize, usize) {
        let height = self.maze.len();
        let width = self.maze.get(0).map_or(0, |row| row.len());
        (width, height)
    }

    /// Método seguro para raycasting
    pub fn safe_raycast_check(&self, x: f32, y: f32) -> char {
        if x < 0.0 || y < 0.0 {
            return '#';
        }

        let grid_x = (x / self.block_size as f32) as usize;
        let grid_y = (y / self.block_size as f32) as usize;

        if self.is_within_bounds(grid_x, grid_y) {
            self.maze[grid_y][grid_x]
        } else {
            '#' // Muro por defecto fuera de bounds
        }
    }
}

// Funciones de conveniencia para mantener compatibilidad
pub fn can_move_to_with_maze_safe(x: f32, y: f32, maze: &Maze, block_size: usize) -> bool {
    let collision_system = CollisionSystem::new(maze.clone(), block_size);
    collision_system.can_move_to(x, y)
}