use bevy::{ecs::system::ResMut, log::error};

use crate::{
    ai::mcts::Mcts,
    client::SessionResource,
    game::board::VertexId,
    session::{GameMode, GameSession, SessionCommand},
};

mod mcts;

fn choose_ai_move(session: &GameSession) -> Option<VertexId> {
    let game = session.game();

    let mut mcts = Mcts::new();

    mcts.choose_move(game, 1000)
}

pub(crate) fn update_ai(mut session: ResMut<SessionResource>) {
    match session.0.mode() {
        GameMode::AI(player) => {
            if session.0.current_player() == player {
                return;
            }
        }
        _ => {
            return;
        }
    }

    let Some(vertex) = choose_ai_move(&session.0) else {
        if let Err(err) = session.0.submit(SessionCommand::Pass) {
            error!("AI fail: {:?}", err);
        }
        return;
    };

    if let Err(err) = session.0.submit(SessionCommand::Place(vertex)) {
        error!("AI fail: {:?}", err);
    }
}
