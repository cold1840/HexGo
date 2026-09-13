use crate::game::board::VertexId;
pub const ACTION_SIZE: usize = 89;
pub const PASS_INDEX: usize = 88;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Move(VertexId),
    Pass,
}
impl Action {
    pub fn index(self) -> usize {
        match self {
            Self::Move(vertex) => vertex.index(),
            Self::Pass => PASS_INDEX,
        }
    }
}
