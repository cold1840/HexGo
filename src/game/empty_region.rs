use crate::game::{board::VertexId, player::Player};
use std::collections::HashSet;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyRegion {
    vertices: Vec<VertexId>,
    bordering_players: HashSet<Player>,
}

impl EmptyRegion {
    pub fn new(vertices: Vec<VertexId>, bordering_players: HashSet<Player>) -> Self {
        Self {
            vertices,
            bordering_players,
        }
    }

    pub fn vertices(&self) -> &[VertexId] {
        &self.vertices
    }

    pub fn bordering_players(&self) -> &HashSet<Player> {
        &self.bordering_players
    }
}
