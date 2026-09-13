#![allow(dead_code)]
use crate::game::{Game, GameResult, board::VertexId, player::Player, state::VertexState};

use rand::{Rng, RngExt};

struct MctsNode {
    parent: Option<usize>,
    children: Vec<usize>,

    action: Option<VertexId>,

    visits: u32,
    value: f64,

    untried_moves: Vec<VertexId>,
}

pub struct Mcts {
    nodes: Vec<MctsNode>,
}

impl Mcts {
    pub fn new() -> Self {
        Mcts { nodes: Vec::new() }
    }

    fn uct(&self, parent: usize, child: usize, is_root_player: bool) -> f64 {
        let parent = &self.nodes[parent];
        let child = &self.nodes[child];

        if child.visits == 0 {
            return f64::INFINITY;
        }

        let root_exploit = child.value / child.visits as f64;

        let exploitation = if is_root_player {
            root_exploit
        } else {
            1.0 - root_exploit
        };
        let parent_visits = (parent.visits as f64).max(1.0);
        let exploration = parent_visits.ln() / child.visits as f64;

        exploitation + 1.414 * exploration.sqrt()
    }

    fn select(&self, root_game: &Game, root_player: Player) -> (usize, Game) {
        let mut node = 0;
        let mut game = root_game.clone();

        loop {
            let current = &self.nodes[node];

            if !current.untried_moves.is_empty() {
                return (node, game);
            }

            if current.children.is_empty() {
                return (node, game);
            }

            let is_root_turn = game.current_player() == root_player;

            node = current
                .children
                .iter()
                .copied()
                .max_by(|&a, &b| {
                    self.uct(node, a, is_root_turn)
                        .partial_cmp(&self.uct(node, b, is_root_turn))
                        .unwrap()
                })
                .unwrap();

            let action = self.nodes[node].action.unwrap();
            game.play_move(action).unwrap();
        }
    }

    fn expand(&mut self, node: usize, game: &Game) -> Option<(usize, Game)> {
        let action = *self.nodes[node].untried_moves.last()?;
        let mut child_game = game.clone();
        child_game.play_move(action).ok()?;
        self.nodes[node].untried_moves.pop();

        let child_id = self.nodes.len();

        let child = MctsNode {
            parent: Some(node),
            children: Vec::new(),
            action: Some(action),
            visits: 0,
            value: 0.0,
            untried_moves: child_game.legal_moves(),
        };

        self.nodes.push(child);

        self.nodes[node].children.push(child_id);

        Some((child_id, child_game))
    }

    fn random_legal_move(game: &Game, rng: &mut impl Rng) -> Option<VertexId> {
        let mut candidates: Vec<VertexId> = (0..game.board().vertex_count())
            .map(VertexId::new)
            .filter(|&v| game.vertex_state(v) == Some(VertexState::Empty))
            .collect();

        while !candidates.is_empty() {
            let index = rng.random_range(0..candidates.len());
            let vertex = candidates.swap_remove(index);

            if game.is_legal_move(vertex) {
                return Some(vertex);
            }
        }

        None
    }

    fn simulate(&self, mut game: Game, root_player: Player) -> f64 {
        let mut rng = rand::rng();

        while let Some(action) = Self::random_legal_move(&game, &mut rng) {
            game.play_move(action).unwrap();
        }

        match game.score_result() {
            GameResult::WinByScore { winner, margin: _ } if winner == root_player => 1.0,
            GameResult::Draw => 0.5,
            _ => 0.0,
        }
    }

    fn backpropagate(&mut self, mut node: usize, result: f64) {
        loop {
            self.nodes[node].visits += 1;
            self.nodes[node].value += result;

            let Some(parent) = self.nodes[node].parent else {
                break;
            };

            node = parent;
        }
    }

    pub fn choose_move(&mut self, game: &Game, iterations: usize) -> Option<VertexId> {
        let legal_moves = game.legal_moves();
        let root_player = game.current_player();
        if legal_moves.is_empty() {
            return None;
        }

        self.nodes.clear();

        self.nodes.push(MctsNode {
            parent: None,
            children: Vec::new(),
            action: None,
            visits: 0,
            value: 0.0,
            untried_moves: legal_moves,
        });

        for _ in 0..iterations {
            let (node, game) = self.select(game, root_player);

            let (node, simulation_game) = match self.expand(node, &game) {
                Some(result) => result,
                None => (node, game),
            };

            let result = self.simulate(simulation_game, root_player);

            self.backpropagate(node, result);
        }

        let best_child = self.nodes[0]
            .children
            .iter()
            .copied()
            .max_by_key(|&child| self.nodes[child].visits)?;

        self.nodes[best_child].action
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::board::BoardGraph;

    fn test_game() -> Game {
        let board = BoardGraph::from_edges(
            6,
            [
                (VertexId::new(0), VertexId::new(1)),
                (VertexId::new(0), VertexId::new(2)),
                (VertexId::new(1), VertexId::new(2)),
                (VertexId::new(1), VertexId::new(3)),
                (VertexId::new(2), VertexId::new(4)),
                (VertexId::new(3), VertexId::new(4)),
                (VertexId::new(3), VertexId::new(5)),
                (VertexId::new(4), VertexId::new(5)),
            ],
        )
        .unwrap();

        Game::new(board)
    }

    #[test]
    fn choose_move_returns_legal_move() {
        let game = test_game();

        let mut mcts = Mcts::new();

        let action = mcts.choose_move(&game, 100);

        assert!(action.is_some());

        let action = action.unwrap();

        assert!(
            game.legal_moves().contains(&action),
            "MCTS returned illegal move: {action:?}"
        );
    }

    #[test]
    fn choose_move_does_not_modify_game() {
        let game = test_game();
        let before = game.clone();

        let mut mcts = Mcts::new();

        let _ = mcts.choose_move(&game, 100);

        assert_eq!(game, before, "choose_move modified the original game");
    }

    #[test]
    fn root_is_visited_every_iteration() {
        let game = test_game();
        let mut mcts = Mcts::new();

        let iterations = 100;

        let _ = mcts.choose_move(&game, iterations);

        assert_eq!(
            mcts.nodes[0].visits, iterations as u32,
            "root should be visited once per iteration"
        );
    }

    #[test]
    fn root_has_children() {
        let game = test_game();
        let mut mcts = Mcts::new();

        let _ = mcts.choose_move(&game, 100);

        assert!(
            !mcts.nodes[0].children.is_empty(),
            "root should have children after MCTS"
        );
    }

    #[test]
    fn root_children_are_legal_moves() {
        let game = test_game();
        let legal_moves = game.legal_moves();

        let mut mcts = Mcts::new();

        let _ = mcts.choose_move(&game, 100);

        for &child in &mcts.nodes[0].children {
            let node = &mcts.nodes[child];

            let action = node.action.expect("root child should have an action");

            assert!(
                legal_moves.contains(&action),
                "root child has illegal action: {action:?}"
            );
        }
    }

    #[test]
    fn root_children_have_visits() {
        let game = test_game();
        let mut mcts = Mcts::new();

        let _ = mcts.choose_move(&game, 100);

        for &child in &mcts.nodes[0].children {
            assert!(
                mcts.nodes[child].visits > 0,
                "expanded root child should have been visited"
            );
        }
    }

    fn replay_node(mcts: &Mcts, root: &Game, node: usize) -> Option<Game> {
        let mut actions = Vec::new();
        let mut current = node;

        while let Some(parent) = mcts.nodes[current].parent {
            actions.push(mcts.nodes[current].action?);
            current = parent;
        }

        let mut game = root.clone();

        for action in actions.into_iter().rev() {
            game.play_move(action).ok()?;
        }

        Some(game)
    }

    #[test]
    fn node_actions_reconstruct_valid_game() {
        let game = test_game();
        let mut mcts = Mcts::new();

        let _ = mcts.choose_move(&game, 100);

        for node in 1..mcts.nodes.len() {
            let reconstructed =
                replay_node(&mcts, &game, node).expect("node path should be reconstructable");

            assert!(
                !reconstructed
                    .legal_moves()
                    .contains(&mcts.nodes[node].action.unwrap()),
                "node action should already have been played"
            );
        }
    }

    #[test]
    fn values_are_within_valid_range() {
        let game = test_game();
        let mut mcts = Mcts::new();

        let _ = mcts.choose_move(&game, 100);

        for (id, node) in mcts.nodes.iter().enumerate() {
            if node.visits == 0 {
                continue;
            }

            assert!(node.value >= 0.0, "node {id} has negative value");

            assert!(
                node.value <= node.visits as f64,
                "node {id} value exceeds visits"
            );
        }
    }

    #[test]
    fn best_move_is_one_of_root_children() {
        let game = test_game();
        let mut mcts = Mcts::new();

        let action = mcts.choose_move(&game, 100);

        let action = action.expect("MCTS should return a move");

        let root_contains_action = mcts.nodes[0]
            .children
            .iter()
            .any(|&child| mcts.nodes[child].action == Some(action));

        assert!(
            root_contains_action,
            "chosen move must correspond to a root child"
        );
    }

    #[test]
    fn larger_search_builds_deeper_tree() {
        let game = test_game();
        let mut mcts = Mcts::new();

        let _ = mcts.choose_move(&game, 1_000);

        assert!(
            mcts.nodes.iter().any(|node| node.parent.is_some()),
            "MCTS should build nodes below root"
        );

        assert!(
            mcts.nodes.iter().any(|node| {
                node.parent
                    .and_then(|parent| mcts.nodes[parent].parent)
                    .is_some()
            }),
            "MCTS should build a tree deeper than one level"
        );
    }
}
