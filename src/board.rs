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

#[wasm_bindgen]
#[derive(Clone, PartialEq, Copy, Debug)]
pub enum CollisionResult {
    EdgeCollision,
    NoEdgeCollision,
    HitBottom
}

#[wasm_bindgen]
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

    pub fn draw_current_block(&mut self, next_position : Point){
        self.active_position = next_position;
        let center_index = self.get_index(self.active_position.y as u32, self.active_position.x as u32);
        // log!("center index is {}",center_index);
        for x  in [self.width -1, 0, 1]{
            for y  in [self.height -1, 0, 1]{
                let line = (y  + self.active_position.y as u32)%self.height;
                let col = (x +self.active_position.x as u32)%self.width;
                let index = self.get_index(line,  col);
                
                let width_diff = match self.active_position.x - col as i32 {
                    x if  x > 0 => 0,
                    x if x == 0 => 1,
                    x if x < 0 => 2 ,
                    _ => 0
                };

                let height_diff = match self.active_position.y - line as i32 {
                    x if  x > 0 => 0,
                    x if x == 0 => 1,
                    x if x < 0 => 2 ,
                    _ => 0
                };
                let mapping_index = width_diff + height_diff * self.active_block.width;
                // log!("neighbor index is {}",index);
                self.cells[index as usize] = self.active_block.shape_array[mapping_index as usize];
            }
        }
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
        match self.edge_collison_check(& next_position) {
            CollisionResult::EdgeCollision => return (),
            _ => {
                self.clear_last_position();
                self.draw_current_block(next_position);
            }
        }
    }

    fn clear_last_position(&mut self){
        for x  in [self.width -1, 0, 1]{
            for y  in [self.height -1, 0, 1]{
                let line = (y  + self.active_position.y as u32)%self.height;
                let col = (x +self.active_position.x as u32)%self.width;
                let index = self.get_index(line,  col);
                self.cells[index as usize] = CellState::Empty;
            }
        }
    }

    ///un used
    fn refresh_active(&mut self, next_position : Point) {
        self.active_position = next_position;
        let index = self.get_index(next_position.y as u32 , next_position.x as u32);
        self.cells[index as usize] = CellState::Ocupied;
    }

    /// return true when crash edge
    fn edge_collison_check(&self, next_position : & Point) -> CollisionResult {
        let horizon_edge_collision = self.check_horizon_edge_collision(next_position);
        let vertical_edge_collision = self.check_vertical_edge_collision(next_position);
        log!("horizon_edge collision is {:?}",horizon_edge_collision);
        log!("vertical edge collision is {:?}",vertical_edge_collision);
        match horizon_edge_collision {
            CollisionResult::EdgeCollision => return CollisionResult::EdgeCollision,
            _ => ()
        };
        match vertical_edge_collision {
            CollisionResult::EdgeCollision => return CollisionResult::EdgeCollision,
            _ => ()
        };

        CollisionResult::NoEdgeCollision

    }

    fn left_to_edge_length(&self, next_x : i32) -> i32 {
        next_x +  self.active_block.left_most_cell - self.active_block.center_x_offset
    }

    fn right_to_edge_length(&self, next_x : i32) -> i32 {
        self.width as i32 - 1 + (self.active_block.width as i32 - self.active_block.right_most_cell - self.active_block.center_x_offset) - next_x - self.active_block.width as i32 /2
    }

    fn check_horizon_edge_collision(&self, next_position : & Point) -> CollisionResult {
        let a = self.right_to_edge_length(next_position.x);
        let b = self.left_to_edge_length(next_position.x);
        if(self.left_to_edge_length(next_position.x) >= 0 && self.right_to_edge_length(next_position.x) >= 0){
            return CollisionResult::NoEdgeCollision;
        }
        CollisionResult::EdgeCollision
    }

    fn up_to_edge_length(&self, next_y : i32) -> i32 {
        next_y + self.active_block.top_most_cell  - self.active_block.center_y_offset
    }

    fn down_to_edge_length(&self, next_y : i32) -> i32 {
        self.height as i32 - 1 + (self.active_block.height as i32 - self.active_block.bottom_most_cell - self.active_block.center_y_offset) - next_y - self.active_block.height as i32 /2
    }

    fn check_vertical_edge_collision(&self, next_position : & Point) -> CollisionResult {
        if(self.up_to_edge_length(next_position.y) >= 0 && self.down_to_edge_length(next_position.y) >= 0){
            return CollisionResult::NoEdgeCollision;
        }
        CollisionResult::EdgeCollision
    }


}

#[wasm_bindgen]
#[derive(Clone)]
struct Block {
    name : String,
    width : u32,
    height : u32,
    shape_array : [CellState ; 9],
    left_most_cell : i32,
    right_most_cell : i32,
    top_most_cell : i32,
    bottom_most_cell : i32,
    center_x_offset: i32,
    center_y_offset: i32
}

trait BasicBlockOperation {
    
    fn get_init_block() -> Block{
        let mut array = [CellState::Empty ; 9];
        array[4] = CellState::Ocupied;
        Block {
            name : String::from("basic"),
            width : 3,
            height : 3,
            shape_array : array ,
            left_most_cell : 0,
            right_most_cell : 0,
            top_most_cell : 0,
            bottom_most_cell : 0,
            center_x_offset : 1,
            center_y_offset: 1
        }
    }

    fn get_block() -> Block;

}

impl BasicBlockOperation for Block {

    fn get_block() -> Block{
        let mut b = Self::get_init_block();
        b.shape_array[3] =CellState::Ocupied;
        b.shape_array[6] =CellState::Ocupied;
        b.shape_array[7] =CellState::Ocupied;
        // b.shape_array[5] =CellState::Ocupied;
        // b.shape_array[7] =CellState::Ocupied;
        // b.shape_array[8] =CellState::Ocupied;
        let (x,y,m,n) = b.calculate_most_cell();
        b.left_most_cell = x;
        b.right_most_cell = y;
        b.top_most_cell = m;
        b.bottom_most_cell = n;
        b
    }

}

impl Block {
    fn get_index(&self , line: u32, column : u32 ) -> u32 {
        self.width * line + column 
    }

    fn calculate_most_cell(&self) -> (i32,i32,i32,i32){
        println!("enter");
        let mut left_most_cell : i32 = -1;
        let mut right_most_cell : i32 = -1;
        let mut top_most_cell : i32 = -1;
        let mut bottom_most_cell : i32 = -1;
        for i in 0..self.width {
            for j in 0..self.height {
                let index =  self.get_index(j, i);
                if self.shape_array[index as usize] == CellState::Ocupied && left_most_cell == -1 {
                    left_most_cell = i as i32;
                } else if self.shape_array[index as usize] == CellState::Ocupied {
                    right_most_cell = i as i32;
                }
                
                if self.shape_array[index as usize] == CellState::Ocupied && top_most_cell == -1 {
                    top_most_cell = j as i32;
                } else if self.shape_array[index as usize] == CellState::Ocupied {
                    bottom_most_cell = j as i32;
                }
            }
        }
        (left_most_cell, right_most_cell, top_most_cell, bottom_most_cell)
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
        // board.draw_current_block();
 
    }

    #[test]
    fn test_collision() {
        let block = Block::get_block();
        let mut board = Board::new_board(6, 6);
        board.new_round();
        let next_position = Point {
            x: 4,
            y: 3
        };
        board.check_horizon_edge_collision(& next_position);
    }

    #[test]
    fn test_block_most_cell() {
        let block = Block::get_block();
        let (a,b,c,d) = block.calculate_most_cell();
    }


}