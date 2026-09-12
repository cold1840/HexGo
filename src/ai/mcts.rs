#![allow(dead_code)]
use crate::game::{Game, board::VertexId, player::Player};

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
    player: Player,
}

impl Mcts {
    pub fn new(player: Player) -> Self {
        Mcts {
            nodes: Vec::new(),
            player,
        }
    }

    pub fn choose_move(&mut self, game: &Game) -> Option<VertexId> {
        let legal_moves = game.legal_moves();

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

        // TODO: MCTS iterations

        None
    }
}
