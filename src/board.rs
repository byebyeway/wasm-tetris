use wasm_bindgen::prelude::*;
use super::*;

#[wasm_bindgen]
struct Board {
    width : u32,
    height : u32,
    cells : Vec<CellState>,
    active_position : Point,
    active_block: Block
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Copy)]
pub enum CellState {
    Empty,
    Ocupied
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
        // log!("init new board");
        utils::set_panic_hook();
        let mut v : Vec<CellState> = (0..width * height).map(|_| CellState::Empty).collect();
        let block = Block::get_block();
        Board {
            width,
            height,
            cells : v,
            active_position : Point {
                x : 0 ,
                y : 0
            },
            active_block : block
        }
    }

    pub fn new_round(&mut self) {
        self.active_position = Point{
            x : (self.width / 2) as i32,
            y : 1
        };
    }

    pub fn draw_current_block(&mut self){
        let center_index = self.get_index(self.active_position.y as u32, self.active_position.x as u32);
        // log!("center index is {}",center_index);
        for x  in [self.width -1, 0, 1]{
            for y  in [self.height -1, 0, 1]{
                let line = (y  + self.active_position.y as u32)%self.height;
                let col = (x +self.active_position.x as u32)%self.width;
                let index = self.get_index(line,  col);
                let mapped_index = 4 - (self.active_position.x - col as i32) ;
                // log!("neighbor index is {}",index);
                self.cells[index as usize] = self.active_block.shape_array[(4-(center_index as i32 - index as i32)) as usize];
            }
        }
    }

    fn left_to_edge_length(&self) -> u32 {
        self.active_position.x - self.active_block.width as u32 /2
    }

    fn right_to_edge_length(&self) -> u32 {
        
    }

    pub fn up(&mut self) {
        let next_position = Point { 
            x: self.active_position.x ,
            y: self.active_position.y - 1
        };

        self.draw_current_block();
        // self.move_block(next_position);
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

#[wasm_bindgen]
#[derive(Clone)]
struct Block {
    name : String,
    width : u32,
    height : u32,
    shape_array : [CellState ; 9]
}

trait BasicBlockOperation {
    
    fn get_init_block() -> Block{
        let mut array = [CellState::Empty ; 9];
        array[4] = CellState::Ocupied;
        Block {
            name : String::from("basic"),
            width : 3,
            height : 3,
            shape_array : array
        }
    }

    fn get_block() -> Block;

}

impl BasicBlockOperation for Block {

    fn get_block() -> Block{
        let mut b = Self::get_init_block();
        b.shape_array[5] =CellState::Ocupied;
        b.shape_array[7] =CellState::Ocupied;
        b.shape_array[8] =CellState::Ocupied;
        b
    }



}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn mapping_block_to_board() {
        let block = Block::get_block();
        let mut board = Board::new_board(6, 6);
        board.new_round();
        board.draw_current_block();
 
    }
}