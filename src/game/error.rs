#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveError {
    InvalidVertex,
    Occupied,
    Suicide,
    Superko,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassError {
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResignError {
    GameOver,
}
