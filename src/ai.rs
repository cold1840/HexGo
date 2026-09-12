use bevy::{
    ecs::system::ResMut,
    log::{error, info},
};

use crate::{
    ai::mcts::Mcts,
    client::SessionResource,
    game::{board::VertexId, state::GameStatus},
    session::{GameMode, GameSession, SessionCommand, SessionError},
    time::Timer,
};

mod mcts;

fn choose_ai_move(session: &GameSession) -> Option<VertexId> {
    let game = session.game();

    let mut mcts = Mcts::new();
    let t = Timer::now();
    let vertex = mcts.choose_move(game, 1000);
    if game.status() == GameStatus::Playing {
        info!("MCTS took {:.2} ms", t.elapsed_ms());
    }
    vertex
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
        if let Err(err) = session.0.submit(SessionCommand::Pass)
            && err != SessionError::GameOver
        {
            error!("AI fail: {:?}", err);
        }
        return;
    };

    if let Err(err) = session.0.submit(SessionCommand::Place(vertex))
        && err != SessionError::GameOver
    {
        error!("AI fail: {:?}", err);
    }
}
