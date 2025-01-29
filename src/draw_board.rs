//TODO add highligt to the selected grid.


use crate::board::{Cell, BOARD_SIZE};

pub fn draw_board(board:&[[Cell;BOARD_SIZE];BOARD_SIZE], highlight : usize){
    for row in 0..BOARD_SIZE {
        draw_board_row(board, row, highlight);
        //println!();
        if row == 2 || row == 5{
            println!();
            println!("════════════╪═════════════╪═══════════════");
        }
    }
    println!();
    println!("Jansson.sus");

}

fn draw_board_row(board : &[[Cell;BOARD_SIZE];BOARD_SIZE], row: usize, highlight : usize){
   
    for i in 0..3{
        for ii in 0..3{
            //println!("({} ,{} )", i+row-row%3, ii+(row%3)*3);
            

            print!("{}", match board[i+row-row%3][ii+(row%3)*3] {
                Cell::Empty => "   ",   
                Cell::Cross => " X ",   
                Cell::Naught => " O ",
            });
            if ii != 2{
                print!("|");
            }
        }
        if i !=2{
            print!(" | ");
        }
    }
    if row % 3 == 2{return}
    println!();
    println!("---+---+--- | ---+---+--- | ---+---+---");
}