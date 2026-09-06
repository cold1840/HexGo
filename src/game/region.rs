use crate::game::{board::VertexId, player::Player};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Territory {
    Owned(Player),
    Neutral,
}

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

    pub fn territory(&self) -> Territory {
        if self.bordering_players.len() != 1 {
            return Territory::Neutral;
        }

        let player = *self.bordering_players.iter().next().unwrap();

        Territory::Owned(player)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_black_owned_territory() {
        let region = EmptyRegion::new(
            vec![VertexId::new(0), VertexId::new(1)],
            HashSet::from([Player::Black]),
        );

        assert_eq!(region.territory(), Territory::Owned(Player::Black));
    }

    #[test]
    fn test_white_owned_territory() {
        let region = EmptyRegion::new(
            vec![VertexId::new(0), VertexId::new(1)],
            HashSet::from([Player::White]),
        );

        assert_eq!(region.territory(), Territory::Owned(Player::White));
    }

    #[test]
    fn test_mixed_border_is_neutral() {
        let region = EmptyRegion::new(
            vec![VertexId::new(0)],
            HashSet::from([Player::Black, Player::White]),
        );

        assert_eq!(region.territory(), Territory::Neutral);
    }

    #[test]
    fn test_region_without_bordering_players_is_neutral() {
        let region = EmptyRegion::new(vec![VertexId::new(0), VertexId::new(1)], HashSet::new());

        assert_eq!(region.territory(), Territory::Neutral);
    }

    #[test]
    fn test_vertices() {
        let vertices = vec![VertexId::new(0), VertexId::new(1), VertexId::new(2)];

        let region = EmptyRegion::new(vertices.clone(), HashSet::from([Player::Black]));

        assert_eq!(region.vertices(), vertices.as_slice());
    }

    #[test]
    fn test_bordering_players() {
        let bordering_players = HashSet::from([Player::Black, Player::White]);

        let region = EmptyRegion::new(vec![VertexId::new(0)], bordering_players.clone());

        assert_eq!(region.bordering_players(), &bordering_players);
    }
}
