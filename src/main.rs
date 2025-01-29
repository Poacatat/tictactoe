mod board;
mod draw_board;

use board::Cell;
use draw_board::draw_board;


const ROW_SIZE    : usize = 3;
const COLUMN_SIZE : usize = 3;
const BOARD_SIZE  : usize = COLUMN_SIZE * ROW_SIZE;

fn main(){
    let mut board: [[Cell; BOARD_SIZE];BOARD_SIZE] = [[Cell::Empty; BOARD_SIZE];BOARD_SIZE];

    board[0][0] = Cell::Cross;
    board[0][7] = Cell::Cross;
    board[8][0] = Cell::Cross;
    board[8][4] = Cell::Cross;
    board[8][7] = Cell::Cross;

    dbg!(check_three_in_a_row(&board[8], 7));

    draw_board(&board);
}



fn test(){
    let mut board: [[Cell; BOARD_SIZE];BOARD_SIZE] = [[Cell::Empty; BOARD_SIZE];BOARD_SIZE];

    board[8][0] = Cell::Cross;
    board[8][4] = Cell::Cross;
    board[8][7] = Cell::Cross;
    assert_eq!(check_three_in_a_row(&board[8], 7), true);
    assert_eq!(check_diagonal(&board[8]), true);
    assert_eq!(check_across(&board[8], 7), false);
    assert_eq!(check_down(&board[8], 7), false);

}

fn check_three_in_a_row(grid: &[Cell;BOARD_SIZE], placed: usize) -> bool{

    let down:     bool = check_down(grid, placed);
    let across:   bool = check_across(grid, placed);
    let diagonal: bool = check_diagonal(grid); 
    dbg!(down, across, diagonal);
    return down || across || diagonal

}

fn check_down(grid: &[Cell;BOARD_SIZE], placed: usize) -> bool{

    let cell_type: Cell = grid[placed];

    dbg!(cell_type);

    if placed <= 2{
       return cell_type == grid[placed + 3] && cell_type == grid[placed + 6]
    }else if placed <= 5{
        return cell_type == grid[placed - 3] && cell_type == grid[placed + 3]
    }else {
        return cell_type == grid[placed - 3] && cell_type == grid[placed - 6]
    }
}


fn check_across(grid: &[Cell;BOARD_SIZE], placed: usize) -> bool{

    let cell_type: Cell = grid[placed];

    if placed % 3 == 0{
        return cell_type == grid[placed + 1] && cell_type == grid[placed + 2]
    }else if placed % 3 == 1{
        return cell_type == grid[placed - 1] && cell_type == grid[placed + 1]
    }else {
        return cell_type == grid[placed - 2] && cell_type == grid[placed - 1]
    }

}

fn check_diagonal(grid: &[Cell;BOARD_SIZE]) -> bool{
    return grid[0] == grid[4] && grid[4] == grid[8] ||
       grid[2] == grid[4] && grid[4] == grid[6]
}


