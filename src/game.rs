#![allow(dead_code)]

use bevy::platform::collections::HashSet;

use crate::game::{
    board::{BoardGraph, VertexId},
    player::Player,
    state::VertexState,
};

use std::collections::VecDeque;

pub mod board;
pub mod player;
pub mod state;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveError {
    InvalidVertex,
    Occupied,
    Suicide,
}

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

        Some(self.occupancy[id.index()])
    }

    pub fn group(&self, start: VertexId) -> Option<Vec<VertexId>> {
        let state = self.vertex_state(start)?;

        if state == VertexState::Empty {
            return None;
        }

        let mut group = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(cur) = queue.pop_front() {
            group.push(cur);

            for &neighbor in self.board().get_neighbors(cur)? {
                if self.vertex_state(neighbor) != Some(state) {
                    continue;
                }

                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        Some(group)
    }
    // Returns the liberties of the group containing the vertex.
    pub fn liberties(&self, start: VertexId) -> Option<Vec<VertexId>> {
        let group = self.group(start)?;

        let mut liberties = HashSet::new();

        for vertex in group {
            for &neighbor in self.board().get_neighbors(vertex)? {
                if self.vertex_state(neighbor) == Some(VertexState::Empty) {
                    liberties.insert(neighbor);
                }
            }
        }

        Some(liberties.into_iter().collect())
    }

    pub fn liberty_count(&self, start: VertexId) -> Option<usize> {
        Some(self.liberties(start)?.len())
    }

    pub fn has_liberty(&self, start: VertexId) -> Option<bool> {
        Some(!self.liberties(start)?.is_empty())
    }

    pub fn remove_group(&mut self, start: VertexId) -> Option<Vec<VertexId>> {
        let gruop = self.group(start)?;

        for vertex in gruop.iter() {
            self.occupancy[vertex.index()] = VertexState::Empty;
        }

        Some(gruop)
    }

    pub fn play_move(&mut self, vertex: VertexId) -> Result<(), MoveError> {
        let Some(state) = self.vertex_state(vertex) else {
            return Err(MoveError::InvalidVertex);
        };

        if state != VertexState::Empty {
            return Err(MoveError::Occupied);
        }

        let player = self.current_player();

        let opponent = player.opponent();

        let neighbors = self
            .board()
            .get_neighbors(vertex)
            .ok_or(MoveError::InvalidVertex)?
            .to_vec();

        self.occupancy[vertex.index()] = VertexState::Occupied(player);

        let mut captured = Vec::new();

        // Capture opponent groups with no liberties.
        for neighbor in neighbors {
            if self.vertex_state(neighbor) != Some(VertexState::Occupied(opponent)) {
                continue;
            }

            if self.has_liberty(neighbor) == Some(false)
                && let Some(group) = self.remove_group(neighbor)
            {
                for stone in group {
                    captured.push(stone);
                }
            }
        }

        // Prevent suicide moves.
        if self.has_liberty(vertex) == Some(false) {
            self.occupancy[vertex.index()] = VertexState::Empty;

            // Restore captured stones.
            for stone in captured {
                self.occupancy[stone.index()] = VertexState::Occupied(opponent);
            }

            return Err(MoveError::Suicide);
        }

        self.current_player = opponent;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::game::board::VertexId;

    use super::*;

    fn create_test_game() -> Game {
        // 0 --- 1 --- 2
        //       |
        //       3 --- 4
        let edges = vec![
            (VertexId::new(0), VertexId::new(1)),
            (VertexId::new(1), VertexId::new(2)),
            (VertexId::new(1), VertexId::new(3)),
            (VertexId::new(3), VertexId::new(4)),
        ];

        let board = BoardGraph::from_edges(5, edges).unwrap();

        Game::new(board)
    }

    #[test]
    fn test_new_game() {
        let game = create_test_game();

        assert_eq!(game.current_player(), Player::Black);

        for vertex in game.board().vertices() {
            assert_eq!(game.vertex_state(vertex), Some(VertexState::Empty));
        }

        assert_eq!(game.vertex_state(VertexId::new(5)), None);
    }

    #[test]
    fn test_group() {
        let mut game = create_test_game();

        // 0(B) --- 1(B) --- 2(W)
        //           |
        //          3(B) --- 4(Empty)

        game.occupancy[0] = VertexState::Occupied(Player::Black);
        game.occupancy[1] = VertexState::Occupied(Player::Black);
        game.occupancy[2] = VertexState::Occupied(Player::White);
        game.occupancy[3] = VertexState::Occupied(Player::Black);

        let group = game.group(VertexId::new(0)).unwrap();

        assert_eq!(group.len(), 3);
        assert!(group.contains(&VertexId::new(0)));
        assert!(group.contains(&VertexId::new(1)));
        assert!(group.contains(&VertexId::new(3)));

        assert!(!group.contains(&VertexId::new(2)));
        assert!(!group.contains(&VertexId::new(4)));
    }

    #[test]
    fn test_group_single_vertex() {
        let mut game = create_test_game();

        game.occupancy[2] = VertexState::Occupied(Player::White);

        let group = game.group(VertexId::new(2)).unwrap();

        assert_eq!(group, vec![VertexId::new(2)]);
    }

    #[test]
    fn test_group_empty_vertex() {
        let game = create_test_game();

        assert_eq!(game.group(VertexId::new(0)), None);
    }

    #[test]
    fn test_group_invalid_vertex() {
        let game = create_test_game();

        assert_eq!(game.group(VertexId::new(100)), None);
    }

    #[test]
    fn test_liberties() {
        let mut game = create_test_game();

        // 0(B) --- 1(B) --- 2(Empty)
        //           |
        //          3(B) --- 4(Empty)

        game.occupancy[0] = VertexState::Occupied(Player::Black);
        game.occupancy[1] = VertexState::Occupied(Player::Black);
        game.occupancy[3] = VertexState::Occupied(Player::Black);

        let liberties = game.liberties(VertexId::new(0)).unwrap();

        assert_eq!(liberties.len(), 2);
        assert!(liberties.contains(&VertexId::new(2)));
        assert!(liberties.contains(&VertexId::new(4)));

        assert_eq!(game.liberty_count(VertexId::new(0)), Some(2));

        assert_eq!(game.has_liberty(VertexId::new(0)), Some(true));
    }

    #[test]
    fn test_no_liberties() {
        let mut game = create_test_game();

        // 全部占满
        //
        // 0(B) --- 1(B) --- 2(W)
        //           |
        //          3(B) --- 4(W)

        game.occupancy[0] = VertexState::Occupied(Player::Black);
        game.occupancy[1] = VertexState::Occupied(Player::Black);
        game.occupancy[2] = VertexState::Occupied(Player::White);
        game.occupancy[3] = VertexState::Occupied(Player::Black);
        game.occupancy[4] = VertexState::Occupied(Player::White);

        let liberties = game.liberties(VertexId::new(0)).unwrap();

        assert!(liberties.is_empty());

        assert_eq!(game.liberty_count(VertexId::new(0)), Some(0));

        assert_eq!(game.has_liberty(VertexId::new(0)), Some(false));
    }

    #[test]
    fn test_liberties_empty_vertex() {
        let game = create_test_game();

        assert_eq!(game.liberties(VertexId::new(0)), None);

        assert_eq!(game.liberty_count(VertexId::new(0)), None);

        assert_eq!(game.has_liberty(VertexId::new(0)), None);
    }

    #[test]
    fn test_remove_group() {
        let mut game = create_test_game();

        // 0(B) --- 1(B) --- 2(W)
        //           |
        //          3(B) --- 4(W)

        game.occupancy[0] = VertexState::Occupied(Player::Black);
        game.occupancy[1] = VertexState::Occupied(Player::Black);
        game.occupancy[2] = VertexState::Occupied(Player::White);
        game.occupancy[3] = VertexState::Occupied(Player::Black);
        game.occupancy[4] = VertexState::Occupied(Player::White);

        let removed = game.remove_group(VertexId::new(0)).unwrap();

        assert_eq!(removed.len(), 3);

        assert!(removed.contains(&VertexId::new(0)));
        assert!(removed.contains(&VertexId::new(1)));
        assert!(removed.contains(&VertexId::new(3)));

        assert_eq!(
            game.vertex_state(VertexId::new(0)),
            Some(VertexState::Empty)
        );

        assert_eq!(
            game.vertex_state(VertexId::new(1)),
            Some(VertexState::Empty)
        );

        assert_eq!(
            game.vertex_state(VertexId::new(3)),
            Some(VertexState::Empty)
        );

        // White stones should not be removed accidentally.
        assert_eq!(
            game.vertex_state(VertexId::new(2)),
            Some(VertexState::Occupied(Player::White))
        );

        assert_eq!(
            game.vertex_state(VertexId::new(4)),
            Some(VertexState::Occupied(Player::White))
        );
    }

    #[test]
    fn test_remove_single_vertex_group() {
        let mut game = create_test_game();

        game.occupancy[2] = VertexState::Occupied(Player::White);

        let removed = game.remove_group(VertexId::new(2)).unwrap();

        assert_eq!(removed, vec![VertexId::new(2)]);

        assert_eq!(
            game.vertex_state(VertexId::new(2)),
            Some(VertexState::Empty)
        );
    }

    #[test]
    fn test_remove_empty_vertex() {
        let mut game = create_test_game();

        let removed = game.remove_group(VertexId::new(0));

        assert_eq!(removed, None);

        assert_eq!(
            game.vertex_state(VertexId::new(0)),
            Some(VertexState::Empty)
        );
    }

    #[test]
    fn test_remove_invalid_vertex() {
        let mut game = create_test_game();

        let removed = game.remove_group(VertexId::new(100));

        assert_eq!(removed, None);
    }

    #[test]
    fn test_play_move() {
        let edges = vec![
            (VertexId::new(0), VertexId::new(1)),
            (VertexId::new(1), VertexId::new(2)),
        ];

        let board = BoardGraph::from_edges(3, edges).unwrap();
        let mut game = Game::new(board);

        assert_eq!(game.play_move(VertexId::new(1)), Ok(()));

        assert_eq!(
            game.vertex_state(VertexId::new(1)),
            Some(VertexState::Occupied(Player::Black))
        );

        // Switch to the other player after a valid move.
        assert_eq!(game.current_player(), Player::White);
    }

    #[test]
    fn test_play_move_on_occupied_vertex() {
        let edges = vec![(VertexId::new(0), VertexId::new(1))];

        let board = BoardGraph::from_edges(2, edges).unwrap();
        let mut game = Game::new(board);

        game.occupancy[0] = VertexState::Occupied(Player::White);

        assert_eq!(game.play_move(VertexId::new(0)), Err(MoveError::Occupied));

        // Player should not change after an invalid move.
        assert_eq!(game.current_player(), Player::Black);
    }

    #[test]
    fn test_play_move_invalid_vertex() {
        let board = BoardGraph::from_edges(2, vec![]).unwrap();
        let mut game = Game::new(board);

        assert_eq!(
            game.play_move(VertexId::new(100)),
            Err(MoveError::InvalidVertex)
        );

        assert_eq!(game.current_player(), Player::Black);
    }

    #[test]
    fn test_play_move_captures_opponent() {
        //       0(B)
        //        |
        // 2(B) - 1(W) - 3(?)
        //
        // Black plays at 3, leaving White at 1 with no liberties.

        let edges = vec![
            (VertexId::new(1), VertexId::new(0)),
            (VertexId::new(1), VertexId::new(2)),
            (VertexId::new(1), VertexId::new(3)),
        ];

        let board = BoardGraph::from_edges(4, edges).unwrap();
        let mut game = Game::new(board);

        game.occupancy[0] = VertexState::Occupied(Player::Black);
        game.occupancy[1] = VertexState::Occupied(Player::White);
        game.occupancy[2] = VertexState::Occupied(Player::Black);

        assert_eq!(game.play_move(VertexId::new(3)), Ok(()));

        // The white stone should be captured.
        assert_eq!(
            game.vertex_state(VertexId::new(1)),
            Some(VertexState::Empty)
        );

        assert_eq!(
            game.vertex_state(VertexId::new(3)),
            Some(VertexState::Occupied(Player::Black))
        );

        assert_eq!(game.current_player(), Player::White);
    }

    #[test]
    fn test_play_move_prevents_suicide() {
        //     4
        //     |
        //     0(W)
        //     |
        // 2(W)-1(?)-3(W)
        // |         |
        // 5         6
        //
        // Black plays at 1.
        // All neighboring white stones still have liberties,
        // so none are captured, while Black has no liberties.

        let edges = vec![
            (VertexId::new(1), VertexId::new(0)),
            (VertexId::new(1), VertexId::new(2)),
            (VertexId::new(1), VertexId::new(3)),
            (VertexId::new(0), VertexId::new(4)),
            (VertexId::new(2), VertexId::new(5)),
            (VertexId::new(3), VertexId::new(6)),
        ];

        let board = BoardGraph::from_edges(7, edges).unwrap();
        let mut game = Game::new(board);

        game.occupancy[0] = VertexState::Occupied(Player::White);
        game.occupancy[2] = VertexState::Occupied(Player::White);
        game.occupancy[3] = VertexState::Occupied(Player::White);

        assert_eq!(game.play_move(VertexId::new(1)), Err(MoveError::Suicide));

        // The attempted move must be rolled back.
        assert_eq!(
            game.vertex_state(VertexId::new(1)),
            Some(VertexState::Empty)
        );

        // Player must not change after an illegal move.
        assert_eq!(game.current_player(), Player::Black);

        // White stones must remain untouched.
        for id in [0, 2, 3] {
            assert_eq!(
                game.vertex_state(VertexId::new(id)),
                Some(VertexState::Occupied(Player::White))
            );
        }
    }
}
