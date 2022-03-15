mod utils;

// Import `wasm_bindgen`
// https://rustwasm.github.io/wasm-bindgen/
use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator
// https://fitzgeraldnick.com/2018/02/09/wee-alloc.html
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// Global variables
const WIDTH: u16 = 128;
const HEIGHT: u16 = 128;
const SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);

#[wasm_bindgen]
pub struct Universe {
  width: u16, // # of cells
  height: u16, // # of cells
  switch: bool, // true: cells1, false: cells2
  cells1: [bool; SIZE], // cell state: true (alive) or false (dead)
  cells2: [bool; SIZE], // cell state: true (alive) or false (dead)
}

impl Universe {
  //! Private methods
  
  /// The universe is a 2D grid. However, in memory, the universe is 
  /// a linear array. This function is used to get the index from that
  /// array
  fn get_index(&self, row: u16, col: u16) -> usize {
    (row * self.width + col) as usize
  }

  /// Get the number of live neighbours around a cell at (row, col).
  /// This assumes a periodic universe, where cells on the edges have
  /// neighbours that wrap around to the other side of the universe.
  fn live_neighbor_count(&self, row: u16, col: u16) -> u8 {

    // Get neighbour index and handle wrap-arounds
    let north = if row == 0 { self.height - 1 } else { row - 1 };
    let south = if row == self.height - 1 { 0 } else { row + 1 };
    let east = if col == self.width - 1 { 0 } else { col + 1 };
    let west = if col == 0 { self.width - 1 } else { col - 1 };

    // Select buffer
    let cells = if self.switch { &self.cells1[..] } else { &self.cells2[..] };

    // Count cells
    let mut count: u8 = 0;

    let n = self.get_index(north, col);
    count += cells[n] as u8;

    let ne = self.get_index(north, east);
    count += cells[ne] as u8;
    
    let nw = self.get_index(north, west);
    count += cells[nw] as u8;

    let s = self.get_index(south, col);
    count += cells[s] as u8;

    let se = self.get_index(south, east);
    count += cells[se] as u8;

    let sw = self.get_index(south, west);
    count += cells[sw] as u8;
      
    let e = self.get_index(row, east);
    count += cells[e] as u8;

    let w = self.get_index(row, west);
    count += cells[w] as u8;

    count
  }
}

#[wasm_bindgen]
impl Universe {
  //! Public methods
  
  /// A single time-step in the universe
  pub fn tick(&mut self) {
    // Step through the entire universe and set the value of each cell
    // in the next tick
    for row in 0..HEIGHT {
      for col in 0..WIDTH {
        let idx = self.get_index(row, col);
  
        let cell_now = if self.switch { self.cells1[idx] } else { self.cells2[idx] };
        let live_neighbours = self.live_neighbor_count(row, col);
        
        // Get the value of the cell one step into the future
        let cell_next = match (cell_now, live_neighbours) {
          // Rule 1: Any live cell with fewer than 2 live neighbours
          // dies, as if caused by under-population.
          (true, x) if x < 2 => false,
          // Rule 2: Any live cell with 2 or 3 live neighbours lives
          // on to the next generation.
          (true, 2) | (true, 3) => true,
          // Rule 3: Any live cell with more than 3 live neighbours
          // dies, as if by over-population.
          (true, x) if x > 3 => false,
          // Rule 4: Any dead cell with exactly 3 live neighbours
          // becomes a live cell, as if by reproduction.
          (false, 3) => true,
          // All other cells remain in the same state.
          (state, _) => state,
        };

        // Set the value in a temporary buffer
        if !self.switch {
          self.cells1[idx] = cell_next;
        } else {
          self.cells2[idx] = cell_next;
        }
      }
    }
  
    // Update the universe
    self.switch = !self.switch;
  }

  /// Set cell value
  pub fn set_cell(&mut self, row: u16, col: u16, val: bool) {
    let idx = self.get_index(row, col);
    if self.switch {
      self.cells1[idx] = val;
    } else {
      self.cells2[idx] = val;
    }
  }
  
  /// Return pointer to WebAssembly's linear memory
  pub fn cells(&self) -> *const bool {
    if self.switch {
      self.cells1.as_ptr()
    } else {
      self.cells2.as_ptr()
    }
  }

  /// Get universe width
  pub fn width(&self) -> u16 {
    self.width
  }
  
  /// Get universe height
  pub fn height(&self) -> u16 {
    self.height
  }

  /// Constructor
  pub fn new() -> Universe {
    utils::set_panic_hook();

    let width = WIDTH;
    let height = HEIGHT;
    let switch = true;
    let mut cells1 = [false; SIZE];
    let mut cells2 = [false; SIZE];
    for i in 0..SIZE {
      cells1[i] = i % 2 == 0 || i % 7 == 0;
      cells2[i] = cells1[i];
    }

    Universe { width, height, cells1, cells2, switch }
  }
}