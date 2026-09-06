#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn oppoent(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_player_oppoent() {
        assert_eq!(Player::Black.oppoent(), Player::White);
        assert_eq!(Player::White.oppoent(), Player::Black);
    }
}
