use crate::ai::search_result::SearchResult;
use crate::game::Game;

pub trait Search {
    fn search(&mut self, game: &Game, iterations: usize) -> Option<SearchResult>;

    fn choose_action(
        &mut self,
        game: &Game,
        iterations: usize,
    ) -> Option<crate::game::action::Action> {
        self.search(game, iterations).map(|result| result.action)
    }
}
