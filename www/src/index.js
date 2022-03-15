// Game of Life Engine
import { Universe, INIT } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm"; // WebAssembly's linear memory

const CELL_SIZE = 5; // [px]
const GRID_COLOR = "#EEEEEE";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const WIDTH = universe.width();
const HEIGHT = universe.height();
const SIZE = WIDTH * HEIGHT;

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * WIDTH + 1;
canvas.height = (CELL_SIZE + 1) * HEIGHT + 1;
const ctx = canvas.getContext('2d');

let animationId = null;

/**
 * Get buffer index
 * @param {number} row 
 * @param {number} col 
 * @returns 
 */
 const getIndex = (row, col) => {
  return row * WIDTH + col;
};

/**
 * Draw grid
 */
 const drawGrid = () => {
  
  ctx.beginPath();
  
  // Draw vertical lines
  for (let i = 0; i <= WIDTH; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
  }
  
  // Draw horizontal lines
  for (let i = 0; i <= HEIGHT; i++) {
    ctx.moveTo(0, i * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, i * (CELL_SIZE + 1) + 1);
  }
  
  // Assign color
  ctx.strokeStyle = GRID_COLOR;
  ctx.stroke();
}

/**
 * Draw cells
 */
 const drawCells = () => {
  const cellsPtr = universe.cells(); // get pointer to WebAssembly's linear memory
  const cells = new Uint8Array(memory.buffer, cellsPtr, SIZE);

  ctx.beginPath();

  // Alive cells
  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      const idx = getIndex(row, col);
      if (cells[idx]) {
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  }

  // Dead cells
  ctx.fillStyle = DEAD_COLOR;
  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      const idx = getIndex(row, col);
      if (!cells[idx]) {
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  }

  ctx.stroke();
}

/**
 * Toggle selected cell
 */
 const onToggleCell = (e) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
  const canvasTop = (e.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), HEIGHT - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), WIDTH - 1);

  universe.toggle_cell(row, col);

  drawGrid();
  drawCells();
}

/**
 * Initialize universe
 * @param {INIT} init 
 */
const initUniverse = (init) => {
  switch (init) {
    case INIT.Empty:
      universe.init(INIT.Empty);
      break;
    case INIT.Random:
      universe.init(INIT.Random);
      break;
    case INIT.Type1:
      universe.init(INIT.Type1);
      break;
    default:
      break;
  }

  drawGrid();
  drawCells();
}

/**
 * Initialization buttons
 */
document.getElementById("clear").addEventListener("click", () => initUniverse(INIT.Empty))
document.getElementById("random").addEventListener("click", () => initUniverse(INIT.Random))
document.getElementById("type-1").addEventListener("click", () => initUniverse(INIT.Type1))

/**
 * Interaction button
 */
const playPauseButton = document.getElementById("play-pause");
playPauseButton.addEventListener("click", () => {
  if (!animationId) {
    console.log('play')
    playPauseButton.textContent = "⏸";
    renderLoop();
  } else {
    console.log('pause')
    playPauseButton.textContent = "⏯︎";
    cancelAnimationFrame(animationId);
    animationId = null;
  }
});

/**
 * Class to measure FPS performance
 */
const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Compute frames per second
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now; // update timestamp
    const fps = 1 / delta * 1000; // [Hz]

    // Save the 100 most recent FPS values
    this.frames.push(fps);
    if (this.frames.length >= 100) { // handle overflow
      this.frames.shift();
    }

    // Find the max, min, and mean of the last 100 FPS values
    let max = Math.max(...this.frames);
    let min = Math.min(...this.frames);
    let sum = this.frames.reduce((total, currentValue) => {return total + currentValue});
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `FPS: ${Math.round(fps)} (Avg: ${Math.round(mean)}, Min: ${Math.round(min)}, Max: ${Math.round(max)})`.trim();
  }
};

/**
 * Render loop
 */
 const renderLoop = () => {
  fps.render();

  // console.time();
  universe.tick();
  // console.timeEnd();

  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};

// Initialize universe
canvas.addEventListener('click', onToggleCell);
initUniverse(INIT.Random);
