#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum Cell{
    Empty,
    Cross,
    Naught
}

pub const BOARD_SIZE: usize = 9;
