mod utils;

extern crate fixedbitset;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
  width: u32,
  height: u32,
  cells: FixedBitSet,
}

/**
 * Private methods
 */
impl Universe {
  /**
   * The universe is a 2D grid. However, in memory, the universe is 
   * a linear array. This function is used to get the index from that
   * array
   */
  fn get_index(&self, row: u32, col: u32) -> usize {
      (row * self.width + col) as usize
  }

  /**
   * Get the number of live neighbours around a cell at (row, col)
   * This assumes a periodic universe, where cells on the edges have
   * neighbours that wrap around to the other side of the universe.
   */
  fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
    let mut count = 0;

    let north = if row == 0 {
        self.height - 1
    } else {
        row - 1
    };

    let south = if row == self.height - 1 {
        0
    } else {
        row + 1
    };

    let west = if col == 0 {
        self.width - 1
    } else {
        col - 1
    };

    let east = if col == self.width - 1 {
        0
    } else {
        col + 1
    };

    let nw = self.get_index(north, west);
    count += self.cells[nw] as u8;

    let n = self.get_index(north, col);
    count += self.cells[n] as u8;

    let ne = self.get_index(north, east);
    count += self.cells[ne] as u8;

    let w = self.get_index(row, west);
    count += self.cells[w] as u8;

    let e = self.get_index(row, east);
    count += self.cells[e] as u8;

    let sw = self.get_index(south, west);
    count += self.cells[sw] as u8;

    let s = self.get_index(south, col);
    count += self.cells[s] as u8;

    let se = self.get_index(south, east);
    count += self.cells[se] as u8;

    count
  }

  pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
    for (row, col) in cells.iter().cloned() {
      let idx = self.get_index(row, col);
      self.cells.set(idx, true);
    }
  }
}

/**
 * Public methods, exported to JavaScript.
 */
#[wasm_bindgen]
impl Universe {
  /**
   * A single time-step in the universe
   */
  pub fn tick(&mut self) {
    // Make a copy of all the cells in the universe; we will modify this copy
    let mut next = self.cells.clone();

    // Step through the entire universe and set the value of each cell
    // in the next tick
    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);

        let cell = self.cells[idx];
        let live_neighbours = self.live_neighbor_count(row, col);
        
        let next_cell = match (cell, live_neighbours) {
          // Rule 1: Any live cell with fewer than 2 live neighbours
          // dies, as if caused by underpopulation.
          (true, x) if x < 2 => false,
          // Rule 2: Any live cell with 2 or 3 live neighbours lives
          // on to the next generation.
          (true, 2) | (true, 3) => true,
          // Rule 3: Any live cell with more than 3 live neighbours
          // dies, as if by overpopulation.
          (true, x) if x > 3 => false,
          // Rule 4: Any dead cell with exactly 3 live neighbours
          // becomes a live cell, as if by reproduction.
          (false, 3) => true,
          // All other cells remain in the same state.
          (state, _) => state,
        };
        
        next.set(idx, next_cell);
      }
    }

    // Update the universe
    self.cells = next;
  }

  /**
   * Constructor
   */
  pub fn new() -> Universe {
    utils::set_panic_hook();

    let width = 128;
    let height = 128;

    // Initialize cells
    let size = (width * height) as usize;
    let mut cells = FixedBitSet::with_capacity(size);

    for i in 0..size {
      cells.set(i, i % 2 == 0 || i % 7 == 0);
    }

    Universe { width, height, cells }
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const u32 {
    self.cells.as_slice().as_ptr()
  }

  pub fn set_width(&mut self, width: u32) {
    self.width = width;

    let size = (width * self.height) as usize;
    for i in 0..size {
      self.cells.set(i, false);
    }
  }

  pub fn set_height(&mut self, height: u32) {
    self.height = height;
    
    let size = (height * self.width) as usize;
    for i in 0..size {
      self.cells.set(i, false);
    }
  }
}
