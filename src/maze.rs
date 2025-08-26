// maze.rs - Sistema de maze 
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

#[derive(Debug, Clone)]
pub struct MazeData {
    pub grid: Maze,
    pub width: usize,
    pub height: usize,
}

impl MazeData {
    pub fn new(grid: Maze) -> Self {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };
        
        MazeData {
            grid,
            width,
            height,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> char {
        if y < self.height && x < self.width {
            self.grid[y][x]
        } else {
            '#' // Retornar muro por defecto fuera de bounds
        }
    }

    pub fn get_cell_safe(&self, x: f32, y: f32, block_size: usize) -> char {
        if x < 0.0 || y < 0.0 {
            return '#';
        }

        let grid_x = (x / block_size as f32) as usize;
        let grid_y = (y / block_size as f32) as usize;

        self.get_cell(grid_x, grid_y)
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        let cell = self.get_cell(x, y);
        matches!(cell, '#' | '+' | '-' | '|' | 'r' | 'b' | 'g')
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        let cell = self.get_cell(x, y);
        matches!(cell, ' ' | 'k' | 't' | 'l' | 'c' | 'e')
    }

    pub fn can_move_to(&self, world_x: f32, world_y: f32, block_size: usize) -> bool {
        if world_x < 0.0 || world_y < 0.0 {
            return false;
        }

        let grid_x = (world_x / block_size as f32) as usize;
        let grid_y = (world_y / block_size as f32) as usize;

        self.is_walkable(grid_x, grid_y)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: char) {
        if y < self.height && x < self.width {
            self.grid[y][x] = cell;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<char>> {
        self.grid.iter()
    }

    pub fn get(&self, index: usize) -> Option<&Vec<char>> {
        self.grid.get(index)
    }

    pub fn len(&self) -> usize {
        self.height
    }

    pub fn is_empty(&self) -> bool {
        self.height == 0
    }
}

pub fn load_maze(filename: &str) -> Maze {
    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut maze: Maze = reader
                .lines()
                .map(|line| line.unwrap_or_default().chars().collect())
                .collect();

            // Asegurar que todas las filas tengan la misma longitud
            if let Some(max_width) = maze.iter().map(|row| row.len()).max() {
                for row in &mut maze {
                    while row.len() < max_width {
                        row.push(' ');
                    }
                }
            }

            maze
        },
        Err(_) => {
            create_default_maze()
        }
    }
}

pub fn load_maze_data(filename: &str) -> MazeData {
    let grid = load_maze(filename);
    MazeData::new(grid)
}

fn create_default_maze() -> Maze {
    vec![
        "################".chars().collect(),
        "#              #".chars().collect(),
        "# ############ #".chars().collect(),
        "#              #".chars().collect(),
        "# ####### # # ###".chars().collect(),
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

// Funciones de conveniencia para compatibilidad
pub fn get_cell_safe(x: f32, y: f32, maze: &Maze, block_size: usize) -> char {
    let maze_data = MazeData::new(maze.clone());
    maze_data.get_cell_safe(x, y, block_size)
}

pub fn can_move_to_safe(x: f32, y: f32, maze: &Maze, block_size: usize) -> bool {
    let maze_data = MazeData::new(maze.clone());
    maze_data.can_move_to(x, y, block_size)
}