use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, BufRead};
use std::time::Instant;

fn tilt_vertical<I>(grid: &mut Vec<Vec<char>>, row_range: I, placement_offset: isize) where I: Iterator<Item = usize> + Clone {
    // Iterate over the columns
    for col in 0..grid[0].len() {
        // Iterate over the rows
        let mut movable_stones = 0;
        let mut last_row = 0;
        for row in row_range.clone() {
            // If the current cell is not empty
            match grid[row][col] {
                'O' => {
                    movable_stones += 1;
                    grid[row][col] = '.';
                },
                '#' => {
                    let mut placement_idx = row as isize + placement_offset as isize;
                    for _ in 0..movable_stones {
                        grid[placement_idx as usize][col] = 'O';
                        placement_idx += placement_offset;
                    }

                    movable_stones = 0;
                }
                _ => (),
            }
            last_row = row;
        }

        let mut placement_idx = last_row as isize;
        for _ in 0..movable_stones {
            grid[placement_idx as usize][col] = 'O';
            placement_idx += placement_offset;
        }
    }
}

fn tilt_horizontal<I>(grid: &mut Vec<Vec<char>>, col_range: I, placement_offset: isize) where I: Iterator<Item = usize> + Clone {
    // Iterate over the rows
    for row in 0..grid.len() {
        // Iterate over the columns
        let mut movable_stones = 0;
        let mut last_col = 0;
        for col in col_range.clone() {
            // If the current cell is not empty
            match grid[row][col] {
                'O' => {
                    movable_stones += 1;
                    grid[row][col] = '.';
                },
                '#' => {
                    let mut placement_idx = col as isize + placement_offset as isize;
                    for _ in 0..movable_stones {
                        grid[row][placement_idx as usize] = 'O';
                        placement_idx += placement_offset;
                    }

                    movable_stones = 0;
                }
                _ => (),
            }
            last_col = col;
        }

        let mut placement_idx = last_col as isize;
        for _ in 0..movable_stones {
            grid[row][placement_idx as usize] = 'O';
            placement_idx += placement_offset;
        }
    }
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    tilt_vertical(grid, (0..grid.len()).rev(), 1);
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    tilt_vertical(grid, 0..grid.len(), -1);
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    tilt_horizontal(grid, (0..grid[0].len()).rev(), 1);
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    tilt_horizontal(grid, 0..grid[0].len(), -1);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{:?}", row);
    }
}

fn hash_grid(grid: &Vec<Vec<char>>) -> u64 {
    let mut board_string = String::new();
    for row in grid {
        for cell in row {
            board_string.push(*cell);
        }
    }

    let mut hasher = DefaultHasher::new();
    board_string.hash(&mut hasher);
    hasher.finish()
}

fn spin_circle(grid: &mut Vec<Vec<char>>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn compute_weight(grid: &Vec<Vec<char>>) -> usize {
    let mut weight = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let cell = grid[row][col];
            if cell == 'O' {
                weight += grid.len() - row;
            }
        }
    }
    weight
}

fn run_spins(grid: &mut Vec<Vec<char>>, cicles: usize, short_circuit: bool) {
    let mut seen_states = std::collections::HashMap::new();

    for i in 0..cicles {
        spin_circle(grid);
        let new_hash = hash_grid(&grid);
        if let Some(iter) = seen_states.get(&new_hash) {
            // println!("Found a loop at iteration {} which was seen at {}", i, iter);
            if short_circuit {
                let cycle_length = i - iter;
                let remaining = (cicles - i - 1) % cycle_length;
                for _ in 0..remaining {
                    spin_circle(grid);
                }
                break;
            }
        }
        seen_states.insert(new_hash, i);
    }
}

const CICLES: usize = 1_000_000_000;
const SHORT_CIRCUIT: bool = true;

fn main() {
    // Create a vector to hold the grid
    let mut grid: Vec<Vec<char>> = Vec::new();

    // Read lines from standard input
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // Unwrap the line and collect characters into a vector
        let row: Vec<char> = line.unwrap().chars().collect();
        // Add the row to the grid
        grid.push(row);
    }

    run_spins(&mut grid, CICLES, SHORT_CIRCUIT);

    let weight = compute_weight(&grid);

    println!("{}", weight);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn generate_random_grid(rows: usize, cols: usize) -> Vec<Vec<char>> {
        let mut grid = vec![];
        for _ in 0..rows {
            let mut row = vec![];
            for _ in 0..cols {
                let cell = match rand::random::<u8>() % 3 {
                    0 => '.',
                    1 => 'O',
                    2 => '#',
                    _ => '.',
                };
                row.push(cell);
            }
            grid.push(row);
        }
        grid
    }

    fn clone_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut new_grid = vec![];
        for row in grid {
            new_grid.push(row.clone());
        }
        new_grid
    }
    
    #[test]
    fn test_tilt_north() {
        // Initial grid setup
        let mut grid = vec![
            vec!['.', '#', '.', '.', '.'],
            vec!['.', 'O', 'O', '#', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'O', '.', '.', '.'],
            vec!['.', '.', '.', 'O', '.'],
        ];
        
        // Expected grid after tilting north
        let expected_grid = vec![
            vec!['.', '#', 'O', '.', '.'],
            vec!['.', 'O', '.', '#', '.'],
            vec!['.', 'O', '.', 'O', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        
        // Call the function
        tilt_north(&mut grid);
        
        // Assert the grid matches the expected output
        assert_eq!(grid, expected_grid);
    }

    
    #[test]
    fn test_tilt_south() {
        // Initial grid setup
        let mut grid = vec![
            vec!['.', '#', '.', '.', '.'],
            vec!['.', 'O', 'O', 'O', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'O', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.'],
        ];
        
        // Expected grid after tilting north
        let expected_grid = vec![
            vec!['.', '#', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'O', 'O', '.', '.'],
            vec!['.', 'O', '#', '.', '.'],
            vec!['.', '#', '.', 'O', '.'],
        ];
        
        // Call the function
        tilt_south(&mut grid);
        
        // Assert the grid matches the expected output
        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn test_tilt_west() {
        // Initial grid setup
        let mut grid = vec![
            vec!['.', '#', 'O', '.', '.'],
            vec!['.', '#', '.', 'O', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'O', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.'],
        ];
        
        // Expected grid after tilting north
        let expected_grid = vec![
            vec!['.', '#', 'O', '.', '.'],
            vec!['.', '#', 'O', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['O', '.', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.'],
        ];
        
        // Call the function
        tilt_west(&mut grid);
        
        // Assert the grid matches the expected output
        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn test_tilt_east() {
        // Initial grid setup
        let mut grid = vec![
            vec!['.', '#', 'O', '.', '.'],
            vec!['.', '#', '.', 'O', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'O', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.'],
        ];
        
        // Expected grid after tilting north
        let expected_grid = vec![
            vec!['.', '#', '.', '.', 'O'],
            vec!['.', '#', '.', '.', 'O'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'O', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.'],
        ];
        
        // Call the function
        tilt_east(&mut grid);
        
        // Assert the grid matches the expected output
        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn test_run_spins() {
        // Initial grid setup
        let mut grid = generate_random_grid(10, 10);
        
        // Expected grid after tilting north
        let mut grid_shorted = clone_grid(&grid);
        
        run_spins(&mut grid, 1000, false);

        run_spins(&mut grid_shorted, 1000, true);

        // Assert the grid matches the expected output
        assert_eq!(grid, grid_shorted);
    }
}
