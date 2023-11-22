use wasm_bindgen::prelude::*;
use super::*;

#[wasm_bindgen]
struct Board {
    width : u32,
    height : u32,
    cells : Vec<CellState>
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Copy)]
pub enum CellState {
    Empty,
    Ocupied
}

#[wasm_bindgen]
impl Board {

    pub fn get_cells(&self) -> *const CellState {
        self.cells.as_ptr()
    }

    fn get_index(&self, line: u32, column : u32 ) -> u32 {
        self.width * line + column 
    }

    pub fn new( width : u32, height : u32) -> Board {
        utils::set_panic_hook();
        // log!(
        //     "start to init!! width is {} height is {}",
        //     width,
        //     height
        // );
        let mut v : Vec<CellState> = (0..width * height).map(|_| CellState::Empty).collect();
        
        Board {
            width,
            height,
            cells : v
        }
    }

}