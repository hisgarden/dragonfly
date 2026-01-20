//! Defrag animation - retro block visualization
//!
//! This module provides the "80s defrag" style animation that displays
//! while scanning the disk.

use std::time::{Duration, Instant};

/// Block state in the defrag grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockState {
    Free,    // . (empty space)
    Used,    // # (allocated)
    Moving,  // @ (currently being "moved")
}

impl BlockState {
    /// Get the character representation of this block
    pub fn as_char(&self) -> char {
        match self {
            BlockState::Free => '·',
            BlockState::Used => '█',
            BlockState::Moving => '▓',
        }
    }
}

/// Defrag animation state
pub struct DefragAnimation {
    /// Grid of blocks (80 columns x 20 rows by default)
    grid: Vec<Vec<BlockState>>,
    /// Current "moving" cluster position
    moving_pos: (usize, usize),
    /// Direction of movement
    direction: (i8, i8),
    /// Last update time
    last_update: Instant,
    /// Update interval
    update_interval: Duration,
    /// Animation frame counter
    frame: usize,
}

impl DefragAnimation {
    /// Create a new animation with the given dimensions
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut grid = vec![vec![BlockState::Free; cols]; rows];
        
        // Initialize with some "used" blocks in a scattered pattern
        for row in 0..rows {
            for col in 0..cols {
                if (row + col) % 3 == 0 || (row * col) % 7 == 0 {
                    grid[row][col] = BlockState::Used;
                }
            }
        }
        
        Self {
            grid,
            moving_pos: (0, 0),
            direction: (1, 0),
            last_update: Instant::now(),
            update_interval: Duration::from_millis(50),
            frame: 0,
        }
    }
    
    /// Create a default 80x20 animation
    pub fn default_size() -> Self {
        Self::new(80, 20)
    }
    
    /// Update the animation state
    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) < self.update_interval {
            return;
        }
        
        self.last_update = now;
        self.frame += 1;
        
        // Clear previous moving blocks
        for row in &mut self.grid {
            for block in row {
                if *block == BlockState::Moving {
                    *block = BlockState::Used;
                }
            }
        }
        
        // Move the "cluster"
        let (mut x, mut y) = self.moving_pos;
        let (dx, dy) = self.direction;
        
        x = ((x as i32 + dx as i32) as usize) % self.grid[0].len();
        y = ((y as i32 + dy as i32) as usize) % self.grid.len();
        
        self.moving_pos = (x, y);
        
        // Change direction occasionally
        if self.frame % 40 == 0 {
            self.direction = match self.frame % 4 {
                0 => (1, 0),   // right
                1 => (0, 1),   // down
                2 => (-1, 0),  // left
                _ => (0, -1),  // up
            };
        }
        
        // Mark moving cluster (3x3 block)
        for dy in 0..3 {
            for dx in 0..3 {
                let nx = (x + dx) % self.grid[0].len();
                let ny = (y + dy) % self.grid.len();
                if self.grid[ny][nx] == BlockState::Used {
                    self.grid[ny][nx] = BlockState::Moving;
                }
            }
        }
        
        // Simulate "consolidation" - occasionally convert free blocks to used on the left
        if self.frame % 20 == 0 {
            for row in &mut self.grid {
                if let Some(first_free) = row.iter().position(|&b| b == BlockState::Free) {
                    if first_free < row.len() / 2 {
                        row[first_free] = BlockState::Used;
                    }
                }
            }
        }
    }
    
    /// Render the grid to a string
    pub fn render(&self) -> String {
        let mut output = String::new();
        for row in &self.grid {
            for block in row {
                output.push(block.as_char());
            }
            output.push('\n');
        }
        output
    }
    
    /// Get grid dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_animation_creation() {
        let anim = DefragAnimation::new(40, 10);
        assert_eq!(anim.dimensions(), (40, 10));
    }
    
    #[test]
    fn test_animation_update() {
        let mut anim = DefragAnimation::new(40, 10);
        let initial_frame = anim.frame;
        
        // Force update by setting last_update to past
        anim.last_update = Instant::now() - Duration::from_secs(1);
        anim.update();
        
        assert!(anim.frame > initial_frame);
    }
    
    #[test]
    fn test_animation_render() {
        let anim = DefragAnimation::default_size();
        let rendered = anim.render();
        
        // Should have 20 lines (rows)
        assert_eq!(rendered.lines().count(), 20);
        
        // Each line should have 80 characters (cols)
        for line in rendered.lines() {
            assert_eq!(line.chars().count(), 80);
        }
    }
}
