use bevy::prelude::*;

mod board_layout;
mod game;
mod session;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(ui::primary_window()),
            ..default()
        }))
        .add_plugins(ui::HexGoUiPlugin)
        .run();
}
