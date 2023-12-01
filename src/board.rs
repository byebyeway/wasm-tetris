use wasm_bindgen::prelude::*;
use super::*;
use getrandom::getrandom;

#[wasm_bindgen]
#[derive(Debug)]
struct Board {
    width : u32,
    height : u32,
    cells : Vec<CellState>,
    active_position : Point,
    active_block:  Block
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Copy, Debug)]
pub enum CellState {
    Empty,
    MovingOcupied,
    FixedOcupied
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Copy, Debug)]
pub enum CollisionResult {
    EdgeCollision,
    NoEdgeCollision,
    HitBottom,
    HorizonHitBlock,
    VerticalHitBlock
}

#[wasm_bindgen]
#[derive(Clone,Copy, Debug)]
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
        let block = BlockType::get_random_block();
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
        let block = BlockType::get_random_block();
        self.active_block = block;
    }

    pub fn draw_current_block(&mut self, next_position : Point, draw_active : bool){
        self.active_position = next_position;
        let center_index = self.get_index(self.active_position.y as u32, self.active_position.x as u32);
        for x  in [self.width -1, 0, 1]{
            for y  in [self.height -1, 0, 1]{
                let line = (y  + self.active_position.y as u32)%self.height;
                if  y == self.height -1 && line > self.active_position.y as u32 {
                    continue;
                }
                let col = (x +self.active_position.x as u32)%self.width;
                if  x == self.width -1 && col > self.active_position.x as u32 {
                    continue;
                }
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
                match self.active_block.shape_array[mapping_index as usize] {
                    CellState::MovingOcupied if draw_active && self.cells[index as usize] == CellState::Empty => self.cells[index as usize] = self.active_block.shape_array[mapping_index as usize],
                    CellState::MovingOcupied if !draw_active => {
                        self.cells[index as usize] = CellState::FixedOcupied;
                    },
                    CellState::Empty if self.cells[index as usize] == CellState::FixedOcupied => {
                        ()},
                    _ => {
                        self.cells[index as usize] = self.active_block.shape_array[mapping_index as usize]
                    }
                }
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
            CollisionResult::EdgeCollision | CollisionResult::HorizonHitBlock => return (),
            CollisionResult::HitBottom => {
                self.draw_current_block(self.active_position, false);
                self.new_round();
            },
            _ => {
                self.clear_last_position();
                self.draw_current_block(next_position, true);
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
        self.cells[index as usize] = CellState::MovingOcupied;
    }

    /// return true when crash edge
    fn edge_collison_check(&self, next_position : & Point) -> CollisionResult {
        let horizon_block_check = self.horizon_block_check(next_position);
        log!("horizon_edge collision is {:?}",horizon_block_check);
        match horizon_block_check {
            CollisionResult::HorizonHitBlock => return CollisionResult::HorizonHitBlock,
            _ => ()
        };
        
        let vertical_edge_collision = self.check_vertical_edge_collision(next_position);
        log!("vertical edge collision is {:?}",vertical_edge_collision);
        match vertical_edge_collision {
            CollisionResult::HitBottom => return CollisionResult::HitBottom,
            _ => ()
        };
        
        let horizon_edge_collision = self.check_horizon_edge_collision(next_position);
        log!("horizon_edge collision is {:?}",horizon_edge_collision);
        match horizon_edge_collision {
            CollisionResult::EdgeCollision => return CollisionResult::EdgeCollision,
            _ => ()
        };
        CollisionResult::NoEdgeCollision

    }

    fn horizon_block_check(&self, next_position : & Point) -> CollisionResult {
        let center_index = self.get_index(next_position.y as u32, next_position.x as u32);
        for x  in [self.width -1, 0, 1]{
            for y  in [self.height -1, 0, 1]{
                let line = (y  + next_position.y as u32)%self.height;
                if  y == self.height -1 && line > next_position.y as u32 {
                    continue;
                }
                let col = (x +next_position.x as u32)%self.width;
                if  x == self.width -1 && col > next_position.x as u32 {
                    continue;
                }
                let index = self.get_index(line,  col);
                
                let width_diff = match next_position.x - col as i32 {
                    x if  x > 0 => 0,
                    x if x == 0 => 1,
                    x if x < 0 => 2 ,
                    _ => 0
                };

                let height_diff = match next_position.y - line as i32 {
                    x if  x > 0 => 0,
                    x if x == 0 => 1,
                    x if x < 0 => 2 ,
                    _ => 0
                };
                let mapping_index = width_diff + height_diff * self.active_block.width;
                match self.active_block.shape_array[mapping_index as usize] {
                    CellState::MovingOcupied if self.cells[index as usize] == CellState::FixedOcupied => return CollisionResult::HorizonHitBlock,
                    _ => continue
                }
            }
        }
        CollisionResult::NoEdgeCollision
    }

    fn left_to_edge_length(&self, next_x : i32) -> i32 {
        next_x + self.active_block.left_most_cell  - self.active_block.center_x_offset
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
        CollisionResult::HitBottom
    }


}

#[wasm_bindgen]
#[derive(Clone, Debug)]
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

#[wasm_bindgen]
struct BlockType {}

impl BlockType {

    fn get_random_block() -> Block {
        let mut random_number_array : [u8; 1] = [0 ; 1];
        getrandom(&mut random_number_array);
        let index = random_number_array[0] % 5;
        match index {
            0 => Self::get_square_block(),
            1 => Self::get_i_block(),
            2 => Self::get_l_block(),
            3 => Self::get_t_block(),
            4 => Self::get_z_block(),
            _ => Self::get_basic_block()
        }
    }

    fn get_basic_block() -> Block{
        let mut array = [CellState::Empty ; 9];
        array[4] = CellState::MovingOcupied;
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

    fn get_square_block () -> Block {
        let mut b = Self::get_basic_block();
        b.shape_array[5] =CellState::MovingOcupied;
        b.shape_array[7] =CellState::MovingOcupied;
        b.shape_array[8] =CellState::MovingOcupied;
        b.calculate_most_cell();
        b
    }

    fn get_z_block () -> Block {
        let mut b = Self::get_basic_block();
        b.shape_array[3] =CellState::MovingOcupied;
        b.shape_array[7] =CellState::MovingOcupied;
        b.shape_array[8] =CellState::MovingOcupied;
        b.calculate_most_cell();
        b
    }

    fn get_t_block () -> Block {
        let mut b = Self::get_basic_block();
        b.shape_array[0] =CellState::MovingOcupied;
        b.shape_array[1] =CellState::MovingOcupied;
        b.shape_array[2] =CellState::MovingOcupied;
        b.shape_array[7] =CellState::MovingOcupied;
        b.calculate_most_cell();
        b
    }

    fn get_i_block () -> Block {
        let mut b = Self::get_basic_block();
        b.shape_array[1] =CellState::MovingOcupied;
        b.shape_array[1] =CellState::MovingOcupied;
        b.shape_array[7] =CellState::MovingOcupied;
        b.calculate_most_cell();
        b
    }

    fn get_l_block () -> Block {
        let mut b = Self::get_basic_block();
        b.shape_array[1] =CellState::MovingOcupied;
        b.shape_array[1] =CellState::MovingOcupied;
        b.shape_array[7] =CellState::MovingOcupied;
        b.shape_array[8] =CellState::MovingOcupied;
        b.calculate_most_cell();
        b
    }

}


impl Block {

    fn get_init_block() -> Block{
        let mut array = [CellState::Empty ; 9];
        array[4] = CellState::MovingOcupied;
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

    fn get_index(&self , line: u32, column : u32 ) -> u32 {
        self.width * line + column 
    }

    fn calculate_most_cell(&mut self){
        let mut left_most_cell : i32 = -1;
        let mut right_most_cell : i32 = -1;
        let mut top_most_cell : i32 = -1;
        let mut bottom_most_cell : i32 = -1;
        for i in 0..self.width {
            for j in 0..self.height {
                let index =  self.get_index(j, i);
                if self.shape_array[index as usize] == CellState::MovingOcupied && left_most_cell == -1 {
                    left_most_cell = i as i32;
                }
                if self.shape_array[index as usize] == CellState::MovingOcupied && i as i32 > right_most_cell{
                    right_most_cell = i as i32;
                }
                
                if self.shape_array[index as usize] == CellState::MovingOcupied && top_most_cell == -1 {
                    top_most_cell = j as i32;
                }
                if self.shape_array[index as usize] == CellState::MovingOcupied && j as i32 > bottom_most_cell{
                    bottom_most_cell = j as i32;
                }
            }
        }
        self.left_most_cell = left_most_cell;
        self.right_most_cell = right_most_cell;
        self.top_most_cell = top_most_cell;
        self.bottom_most_cell = bottom_most_cell;
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn mapping_block_to_board() {
        let block = BlockType::get_random_block();
        let mut board = Board::new_board(6, 6);
        board.new_round();
        // board.draw_current_block();
 
    }

    #[test]
    fn test_collision() {
        let block = BlockType::get_random_block();
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
        let mut block = BlockType::get_z_block();
        print!("left{}, right{}, top{}, bottom{}",block.left_most_cell, block.right_most_cell, block.top_most_cell, block.bottom_most_cell);
    }


}