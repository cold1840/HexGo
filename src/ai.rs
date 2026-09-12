use bevy::{ecs::system::ResMut, log::error};

use crate::{
    client::SessionResource,
    game::board::VertexId,
    session::{GameMode, GameSession, SessionCommand},
};

fn choose_ai_move(_session: &GameSession) -> Option<VertexId> {
    None
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
