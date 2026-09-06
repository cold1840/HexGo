#![allow(dead_code)]

use crate::game::{
    board::{BoardGraph, VertexId},
    player::Player,
    state::VertexState,
};

pub mod board;
pub mod player;
pub mod state;
pub struct Game {
    board: BoardGraph,
    occupancy: Vec<VertexState>,
    current_player: Player,
}

impl Game {
    pub fn new(board: BoardGraph) -> Self {
        let count = board.vertex_count();

        Self {
            board,
            occupancy: vec![VertexState::Empty; count],
            current_player: Player::Black,
        }
    }

    pub fn board(&self) -> &BoardGraph {
        &self.board
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn vertex_state(&self, id: VertexId) -> Option<VertexState> {
        if !self.board().contains(id) {
            return None;
        }

        if self.occupancy.len() <= id.index() {
            return None;
        }

        Some(self.occupancy[id.index()])
    }
}

#[cfg(test)]
mod test {
    use crate::game::board::VertexId;

    use super::*;

    #[test]
    fn test_new_board() {
        let edges = vec![
            (VertexId::new(0), VertexId::new(1)),
            (VertexId::new(0), VertexId::new(2)),
            (VertexId::new(0), VertexId::new(3)),
            (VertexId::new(1), VertexId::new(3)),
            (VertexId::new(2), VertexId::new(3)),
            (VertexId::new(1), VertexId::new(2)),
        ];
        let res = BoardGraph::from_edges(4, edges);
        assert!(res.is_ok());
        let board = res.unwrap();

        let game = Game::new(board);
        assert_eq!(game.current_player, Player::Black);

        for vextex in game.board().vertices() {
            assert!(
                game.vertex_state(vextex)
                    .is_some_and(|state| state == VertexState::Empty)
            );
        }

        assert!(game.vertex_state(VertexId::new(4)).is_none());
    }
}
