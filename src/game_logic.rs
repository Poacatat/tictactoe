use crate::{board::{Cell, BOARD_SIZE}, draw_board};
use std::io::{self, Write};


pub fn game_loop(){
    let mut board: [[Cell; BOARD_SIZE];BOARD_SIZE] = [[Cell::Empty; BOARD_SIZE];BOARD_SIZE];

   // make_move(&mut board, 1, 1, Cell::Cross);
    let mut piece_type: Cell = Cell::Cross;
    let mut viable:bool;

    let mut board_location: usize;
    let mut grid_location : usize;

   // draw_board(&board);
    draw_board(&board);

    print!("Enter a move: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    
    // Read the input from the user
    io::stdin().read_line(&mut input).unwrap();

    board_location = input.trim().parse().unwrap();

    loop {
        print!("Enter a move: ");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");


        //shitty fix handle this correctly
        grid_location =0;

        match parse_grid_location(&input) {
            Ok(grid_location) => {
                println!("Valid input: {}", grid_location);
                println!("{}", grid_location);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        }

        //grid_location = input.trim().parse().unwrap();
        viable = make_move(&mut board, board_location,grid_location,piece_type);

        if viable{
            draw_board(&board, board_location);
            if piece_type == Cell::Cross {piece_type = Cell::Naught}else{piece_type=Cell::Cross}
            board_location = grid_location; 
        }else{
            println!("That spot is unavailable");
        }

        

        
    }
}


pub fn make_move(board: &mut [[Cell;BOARD_SIZE];BOARD_SIZE], board_location: usize, grid_location: usize, piece_type: Cell)->bool{
    if  board[board_location][grid_location] != Cell::Empty{
        return false;
    }
    board[board_location][grid_location] = piece_type;

    check_three_in_a_row(&board[board_location], grid_location);

    return true;
    
    //check if move is viable
    //place piece
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


pub fn game_logic_test(){
    let mut board: [[Cell; BOARD_SIZE];BOARD_SIZE] = [[Cell::Empty; BOARD_SIZE];BOARD_SIZE];

    board[8][0] = Cell::Cross;
    board[8][4] = Cell::Cross;
    board[8][7] = Cell::Cross;
    assert_eq!(check_three_in_a_row(&board[8], 7), true);
    assert_eq!(check_diagonal(&board[8]), true);
    assert_eq!(check_across(&board[8], 7), false);
    assert_eq!(check_down(&board[8], 7), false);
}



fn parse_grid_location(input: &str) -> Result<usize, String> {
    match input.trim().parse::<isize>() {
        Ok(num) if num < 0 => Err("Number cannot be negative".to_string()),
        Ok(num) if num > 8 => Err("Number cannot be greater than 8".to_string()),
        Ok(num) => Ok(num as usize), // Convert to usize safely
        Err(_) => Err("Invalid input: Please enter a valid number between 0 and 8".to_string()),
    }
}