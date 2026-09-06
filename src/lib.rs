use bevy::prelude::*;

pub mod board_layout;
pub mod game;
pub mod session;
pub mod ui;

/// Builds the HexGo application without starting its event loop.
pub fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(ui::primary_window()),
        ..default()
    }))
    .add_plugins(ui::HexGoUiPlugin);
    app
}

/// Builds and runs HexGo using the platform's event loop.
pub fn run() {
    build_app().run();
}

#[cfg(target_os = "android")]
#[bevy_main]
fn main() {
    run();
}
