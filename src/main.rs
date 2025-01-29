mod board;
mod draw_board;
mod game_logic;


use board::Cell;
use draw_board::draw_board;
use game_logic::make_move;
use game_logic::game_loop;


const ROW_SIZE    : usize = 3;
const COLUMN_SIZE : usize = 3;
const BOARD_SIZE  : usize = COLUMN_SIZE * ROW_SIZE;

fn main(){
    game_loop();
}



fn test(){
    

}



