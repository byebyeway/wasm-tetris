use wasm_bindgen::prelude::*;
use super::*;

#[wasm_bindgen]
struct Board {
    width : u32,
    height : u32,
    cells : Vec<CellState>,
    active_position : Point
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Copy)]
pub enum CellState {
    Empty,
    Ocupied
}

struct Block {
    name : String,
    width : u32,
    height : u32,
    
}

#[derive(Clone,Copy)]
struct Point {
    x : i32,
    y : i32
}

#[wasm_bindgen]
impl Board {

    pub fn get_cells(&self) -> *const CellState {
        self.cells.as_ptr()
    }

    fn get_index(&self, line: u32, column : u32 ) -> u32 {
        self.width * line + column 
    }

    pub fn new_board( width : u32, height : u32) -> Board {
        utils::set_panic_hook();
        let mut v : Vec<CellState> = (0..width * height).map(|_| CellState::Empty).collect();
        Board {
            width,
            height,
            cells : v,
            active_position : Point {
                x : 0 ,
                y : 0
            }
        }
    }

    pub fn new_round(&mut self) {
        self.active_position = Point{
            x : 0,
            y : 0
        };
        self.cells[0] = CellState::Ocupied;
    }

    pub fn up(&mut self) {
        let next_position = Point { 
            x: self.active_position.x ,
            y: self.active_position.y - 1
        };

        self.move_block(next_position);
    }

    pub fn down(&mut self) {
        let next_position = Point { 
            x: self.active_position.x ,
            y: self.active_position.y + 1
        };
        self.move_block(next_position);

    }

    pub fn left(&mut self) {
        let next_position = Point { 
            x: self.active_position.x - 1,
            y: self.active_position.y 
        };
        self.move_block(next_position);
    }

    pub fn right(&mut self) {
        let next_position = Point { 
            x: self.active_position.x + 1 ,
            y: self.active_position.y 
        };
        self.move_block(next_position);
    }

    fn move_block(&mut self, next_position : Point){
        if self.edge_collison_check(&next_position){
            return
        }else{
            self.clear_last_position();
            self.refresh_active(next_position);
        }
    }

    /// return true when crash edge
    fn edge_collison_check(&self, next_position : & Point) -> bool {
        if(next_position.x < 0 || next_position.x > ((self.width-1) as i32)){
            return true;
        }
        if(next_position.y<0 || next_position.y > ((self.height-1) as i32)){
            return true;
        }
        return false;
    }

    fn clear_last_position(&mut self){
        let index = self.get_index(self.active_position.y as u32 , self.active_position.x as u32);
        self.cells[index as usize] = CellState::Empty;
    }

    fn refresh_active(&mut self, next_position : Point) {
        self.active_position = next_position;
        let index = self.get_index(next_position.y as u32 , next_position.x as u32);
        self.cells[index as usize] = CellState::Ocupied;
    }


}

