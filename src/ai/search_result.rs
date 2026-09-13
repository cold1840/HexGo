use crate::game::action::Action;

pub struct SearchResult {
    pub action: Action,
    pub policy: Vec<(Action, f32)>,
}
