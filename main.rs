public struct cell{
    empty,
    cross,
    naught
}

let mut board Vec<Vec<cell>> = new Vec();

public function check_three_in_a_row(grid: &Vec<cell>, placed: u32) -> bool{
    let bool cell_type = grid[placed]

    

    if placed <= 2{
        let bool down = cell_type == grid[placed + 3] && cell_type == grid[placed + 6]
    }else if placed <= 5{
        let bool down = cell_type == grid[placed - 3] && cell_type == grid[placed + 3]
    }else {
        let bool down = cell_type == grid[placed - 3] && cell_type == grid[placed - 6]
    }


}


public function check_across(grid: &Vec<cell>, placed u32) -> bool{
    if placed % 3 == 0{
        let bool across = cell_type == grid[placed + 1] && cell_type == grid[placed + 2]
    }else if placed % 3 == 1{
        let bool across = cell_type == grid[placed - 1] && cell_type == grid[placed + 1]
    }else {
        let bool across = cell_type == grid[placed - 2] && cell_type == grid[placed - 1]
    }
}