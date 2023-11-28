import { Board, CellState } from "wasm-tetris";
import { memory } from "wasm-tetris/wasm_tetris_bg";
const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const height = 9;
const width = 9;

const board = Board.new_board(height, width);
board.new_round()

const canvas = document.getElementById("game-of-tetris");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
  
    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};
  
const drawCells = () => {
    const cellsPtr = board.get_cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  
    ctx.beginPath();
  
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);
  
        ctx.fillStyle = cells[idx] === CellState.Empty
          ? DEAD_COLOR
          : ALIVE_COLOR;
  
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  
    ctx.stroke();
};  

const renderLoop = () => {

    // drawGrid();
    drawCells();
    requestAnimationFrame(renderLoop);
};

window.addEventListener("keydown", (event) => {
	if (event.key == 'w') {
        board.up();
    }
    if (event.key == 's') {
        board.down();
    }
    if (event.key == 'a') {
        board.left();
    }
    if (event.key == 'd') {
        board.right();
    }
});

drawGrid();
requestAnimationFrame(renderLoop);
