use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
mod ai;
mod board_layout;
mod client;
mod game;
mod session;
mod window;
/// Builds the HexGo application without starting its event loop.
pub fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        },
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window::primary_window()),
            ..default()
        }),
    ))
    .add_plugins(client::ClientPlugin);
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
